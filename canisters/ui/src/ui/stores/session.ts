import { AnonymousIdentity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { icAgent, logger } from '~/core';
import { User } from '~/generated/control-panel/control_panel.did';
import { disableWalletWorkers, enableWalletWorkers, i18n, services } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';
import { WalletConnectionStatus, useWalletStore } from '~/ui/stores/wallet';
import { afterLoginRedirect, redirectToLogin } from '~/ui/utils';
import { SessionBroadcaseChannel, Timeout } from '../modules/auth-check';
import { Identity } from '@dfinity/agent';

const INACTIVITY_TIMEOUT_MS = 1000 * 60 * 10; // 10 minutes

export interface UserWallet {
  main: boolean;
  name: string | null;
  canisterId: string;
}

export interface SelectedUserWallet {
  canisterId: string | null;
  hasAccess: boolean;
}

export enum InitializationStatus {
  Uninitialized = 'uninitialized',
  Initialized = 'initialized',
  FailedInitialization = 'failed-initialization',
}

export interface SessionStoreState {
  initialized: InitializationStatus;
  loading: boolean;
  lastLoginPrincipal: string | null;
  principal: string;
  isAuthenticated: boolean;
  reauthenticationNeeded: boolean;
  sessionBroadcastChannel: SessionBroadcaseChannel | null;
  sessionTimeout: Timeout | null;
  inactivityTimeout: Timeout | null;
  data: {
    wallets: UserWallet[];
    selectedWallet: SelectedUserWallet;
  };
}

export const useSessionStore = defineStore('session', {
  state: (): SessionStoreState => {
    return {
      initialized: InitializationStatus.Uninitialized,
      loading: false,
      lastLoginPrincipal: null,
      principal: Principal.anonymous().toText(),
      isAuthenticated: false,
      reauthenticationNeeded: false,
      sessionBroadcastChannel: null,
      sessionTimeout: null,
      inactivityTimeout: null,
      data: {
        wallets: [],
        selectedWallet: {
          canisterId: null,
          hasAccess: false,
        },
      },
    };
  },
  getters: {
    hasWallets(): boolean {
      return !!this.data.wallets.length;
    },
    mainWallet(): Principal | null {
      const mainWallet = this.data.wallets.find(wallet => wallet.main);

      return mainWallet ? Principal.fromText(mainWallet.canisterId) : null;
    },
  },
  actions: {
    async initialize(): Promise<void> {
      logger.info(`[call] initialize`);
      try {
        if (this.initialized === InitializationStatus.Initialized) {
          return;
        }

        this.sessionBroadcastChannel = new SessionBroadcaseChannel({
          onOtherTabActive: () => {
            logger.info(`[call] onOtherTabActive`);
            this.inactivityTimeout?.reset(INACTIVITY_TIMEOUT_MS);
          },
          onOtherTabSignout: () => {
            logger.info(`[call] onOtherTabSignout`);
            this.signOut(false);
          },
          onOtherTabSignin: async () => {
            logger.info(`[call] onOtherTabSignin`);
            this.setReauthenticated();
          },
        });

        this.sessionTimeout = new Timeout(() => {
          logger.info(`[call] onExpired`);
          this.requireReauthentication();
        });

        this.inactivityTimeout = new Timeout(() => {
          logger.info(`[call] onInactive`);
          const authService = services().auth;
          authService.logout();
          this.requireReauthentication();
        });

        const authService = services().auth;
        const cachedAuthenticatedIdentity = await authService.identity();

        if (!cachedAuthenticatedIdentity) {
          icAgent.get().replaceIdentity(new AnonymousIdentity());
          this.initialized = InitializationStatus.Initialized;
          return;
        }

        await this.initializeAuthenticated(cachedAuthenticatedIdentity);
      } catch (error) {
        this.reset();

        logger.error(`Application failed to initialize the state`, { error });

        this.initialized = InitializationStatus.FailedInitialization;
      }
    },
    reset(): void {
      logger.info(`[call] reset`);
      const wallet = useWalletStore();

      this.loading = false;
      this.isAuthenticated = false;
      this.principal = Principal.anonymous().toText();
      this.reauthenticationNeeded = false;
      this.data = {
        wallets: [],
        selectedWallet: {
          canisterId: null,
          hasAccess: false,
        },
      };

      wallet.reset();
    },
    async signIn(): Promise<void> {
      logger.info(`[call] signIn`);
      const authService = services().auth;

      try {
        const identity = await authService.login();

        this.sessionBroadcastChannel?.notifySignedIn();
        await this.initializeAuthenticated(identity);
      } catch (error) {
        disableWalletWorkers();
        this.reset();
        throw error;
      }
    },
    async signOut(notifyOtherTabs = true): Promise<void> {
      logger.info(`[call] signOut`);
      disableWalletWorkers();

      this.sessionTimeout?.clear();
      this.inactivityTimeout?.clear();

      if (notifyOtherTabs) {
        this.sessionBroadcastChannel?.notifySignedOut();
      }

      const authService = services().auth;
      await authService.logout();

      this.reset();
      redirectToLogin();
    },

    async load(): Promise<void> {
      const app = useAppStore();

      try {
        if (this.loading) {
          logger.warn(`Session is already loading`);
          return;
        }

        this.loading = true;
        const controlPanelService = services().controlPanel;
        const controlPanelUser = await controlPanelService.getCurrentUser();
        const mainWalletId = controlPanelUser.main_wallet?.[0]
          ? controlPanelUser.main_wallet?.[0]
          : controlPanelUser.wallets?.[0]?.canister_id;
        const sameUser = this.isAuthenticated && this.principal === controlPanelUser.id.toText();

        this.isAuthenticated = true;
        this.populateUser(controlPanelUser);

        if (!sameUser && mainWalletId) {
          return this.connectWallet(mainWalletId);
        }
      } catch (err) {
        logger.error(`Failed to load user session`, { err });

        app.sendNotification({
          type: 'error',
          message: i18n.global.t('app.session_load_error'),
        });
      } finally {
        this.loading = false;
      }
    },
    populateUser(user: User): void {
      const selectedWalletId = this.data.selectedWallet.canisterId;
      const sameUser = this.isAuthenticated && this.principal === user.id.toText();
      this.principal = user.id.toText();
      this.data = {
        selectedWallet: this.data.selectedWallet,
        wallets: user.wallets.map(wallet => ({
          main: wallet.canister_id.toText() === user.main_wallet?.[0]?.toText(),
          name: wallet.name?.[0] ?? null,
          canisterId: wallet.canister_id.toText(),
        })),
      };

      const hasWallet = this.data.wallets.some(wallet => wallet.canisterId === selectedWalletId);
      if (!sameUser || !hasWallet) {
        this.disconnectWallet();
      }
    },
    disconnectWallet(): void {
      const wallet = useWalletStore();

      this.data.selectedWallet = {
        canisterId: null,
        hasAccess: false,
      };

      wallet.reset();
    },
    async connectWallet(walletId: Principal): Promise<void> {
      const wallet = useWalletStore();

      this.data.selectedWallet = {
        canisterId: walletId.toText(),
        hasAccess: false,
      };

      const connectionStatus = await wallet.connectTo(walletId);

      if (connectionStatus === WalletConnectionStatus.Connected) {
        this.data.selectedWallet.hasAccess = true;
      }
    },

    requireReauthentication() {
      logger.info(`[call] requireReauthentication`);
      this.reauthenticationNeeded = true;
      this.inactivityTimeout?.clear();
      this.sessionTimeout?.clear();
      disableWalletWorkers();
    },

    async setReauthenticated() {
      const authService = services().auth;
      authService.invalidateAuthClient();
      const maybeIdentity = await authService.identity();
      if (!maybeIdentity) {
        logger.error(`Reauthentication failed, no identity found`);
        return;
      }

      await this.initializeAuthenticated(maybeIdentity);
    },

    registerActivity() {
      if (this.inactivityTimeout?.isActive()) {
        logger.info(`[call] registerActivity`);
        this.sessionBroadcastChannel?.notifyActive();
        this.inactivityTimeout?.reset(INACTIVITY_TIMEOUT_MS);
      }
    },

    async initializeAuthenticated(newIdentity: Identity) {
      const authService = services().auth;
      icAgent.get().replaceIdentity(newIdentity);

      if (
        this.lastLoginPrincipal !== null &&
        this.lastLoginPrincipal !== newIdentity.getPrincipal().toText()
      ) {
        logger.info(`[call] NEW IDENTITY`);
        this.reset();
      }

      this.reauthenticationNeeded = false;
      enableWalletWorkers();

      const maybeSessionExpirationTimeMs = await authService.getRemainingSessionTimeMs();
      if (maybeSessionExpirationTimeMs) {
        this.sessionTimeout!.reset(maybeSessionExpirationTimeMs);
      }
      this.inactivityTimeout!.reset(INACTIVITY_TIMEOUT_MS);

      const controlPanelService = services().controlPanel;
      const isRegistered = await controlPanelService.hasRegistration();

      if (!isRegistered) {
        await controlPanelService.register({
          // a new user is created with an empty list of wallets, they can add them later
          wallet_id: [],
        });
      }

      // loads information about the authenticated user
      await this.load();

      // if the user was not signed in before, or the user signed in with a different identity
      if (this.lastLoginPrincipal !== newIdentity.getPrincipal().toText()) {
        afterLoginRedirect();
      }

      this.lastLoginPrincipal = newIdentity.getPrincipal().toText();
    },
  },
});
