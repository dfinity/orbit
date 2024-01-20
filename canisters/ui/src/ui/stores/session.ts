import { AnonymousIdentity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { icAgent, logger } from '~/core';
import { User } from '~/generated/control-panel/control_panel.did';
import { disableWalletWorkers, enableWalletWorkers, i18n, services } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';
import { WalletConnectionStatus, useWalletStore } from '~/ui/stores/wallet';
import { redirectToLogin } from '~/ui/utils';
import { AuthCheck } from '../modules/auth-check';

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
  principal: string;
  isAuthenticated: boolean;
  reauthenticationNeeded: boolean;
  authChecker: AuthCheck | null;
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
      principal: Principal.anonymous().toText(),
      isAuthenticated: false,
      reauthenticationNeeded: false,
      authChecker: null,
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

        this.authChecker = new AuthCheck({
          inactivityTimeoutMs: 1000 * 60 * 5,
          onExpired: () => {
            logger.info(`[call] onExpired`);
            this.requireReauthentication();
          },
          onOtherTabReauthenticate: () => {
            logger.info(`[call] onReauthenticated`);
            this.setReauthenticated();
          },
          onInactive: () => {
            logger.info(`[call] onInactive`);
            this.requireReauthentication();
          },
          onOtherTabSignout: () => {
            logger.info(`[call] onOtherTabSignout`);
            this.signOut(false);
          },
        });

        const authService = services().auth;
        const cachedAuthenticatedIdentity = await authService.identity();

        if (!cachedAuthenticatedIdentity) {
          icAgent.get().replaceIdentity(new AnonymousIdentity());
          this.initialized = InitializationStatus.Initialized;
          return;
        }

        this.authChecker.setSignedIn();

        enableWalletWorkers();

        icAgent.get().replaceIdentity(cachedAuthenticatedIdentity);

        await this.load();

        this.initialized = InitializationStatus.Initialized;
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
        icAgent.get().replaceIdentity(identity);

        this.reauthenticationNeeded = false;
        enableWalletWorkers();

        this.authChecker?.setSignedIn();
        this.authChecker?.notifySignedIn();

        const controlPanelService = services().controlPanel;
        const isRegistered = await controlPanelService.hasRegistration();

        if (isRegistered) {
          await this.load();
          return;
        }

        await controlPanelService.register({
          // a new user is created with an empty list of wallets, they can add them later
          wallet_id: [],
        });

        // loads information about the authenticated user
        await this.load();
      } catch (error) {
        disableWalletWorkers();
        this.reset();
        throw error;
      }
    },
    async signOut(notifyOtherTabs = true): Promise<void> {
      logger.info(`[call] signOut`);
      disableWalletWorkers();

      this.authChecker?.setSignedOut();

      if (notifyOtherTabs) {
        this.authChecker?.notifySignedOut();
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
      disableWalletWorkers();
    },

    async reauthenticate() {
      logger.info(`[call] reauthenticate`);
      this.signIn();
    },

    async setReauthenticated() {
      const authService = services().auth;
      authService.invalidateAuthClient();
      const maybeIdentity = await authService.identity();
      if (!maybeIdentity) {
        logger.error(`Reauthentication failed, no identity found`);
        return;
      }

      logger.info(`[call] setReauthenticated`);
      this.reauthenticationNeeded = false;
      icAgent.get().replaceIdentity(maybeIdentity);

      enableWalletWorkers();
    },
  },
});
