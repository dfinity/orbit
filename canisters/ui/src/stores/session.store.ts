import { AnonymousIdentity, Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { Ref } from 'vue';
import { icAgent } from '~/core/ic-agent.core';
import { logger } from '~/core/logger.core';
import { User } from '~/generated/control-panel/control_panel.did';
import { i18n } from '~/plugins/i18n.plugin';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { WalletConnectionStatus, useWalletStore } from '~/stores/wallet.store';
import { afterLoginRedirect, redirectToLogin } from '~/utils/app.utils';
import { unreachable } from '~/utils/helper.utils';
import { objectDeserialize, objectSerialize, useStorage } from '~/utils/storage.utils';
import { disableWalletWorkers, enableWalletWorkers } from '~/workers';

export interface UserWallet {
  main: boolean;
  name: string | null;
  canisterId: string;
}

export interface SelectedUserWallet {
  canisterId: Ref<string | null>;
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
      data: {
        wallets: [],
        selectedWallet: {
          canisterId: useStorage({
            deserialize: objectDeserialize,
            serialize: objectSerialize,
            key: 'selected-wallet',
            storage: sessionStorage,
            deepWatch: true,
            initial: () => null,
          }),
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
      try {
        if (this.initialized === InitializationStatus.Initialized) {
          return;
        }

        const sessionExpirationService = services().sessionExpiration;

        sessionExpirationService.subscribe(msg => {
          switch (msg) {
            case 'otherTabActive':
              sessionExpirationService.resetInactivityTimeout();
              break;
            case 'otherTabSignedIn':
              this.setReauthenticated();
              break;
            case 'otherTabSignedOut':
              this.signOut(false);
              break;
            case 'sessionExpired':
              this.requireReauthentication();
              break;
            case 'userInactive': {
              const authService = services().auth;
              authService.logout();
              this.requireReauthentication();
              break;
            }
            default:
              unreachable(msg);
          }
        });

        const authService = services().auth;
        const cachedAuthenticatedIdentity = await authService.identity();

        if (!cachedAuthenticatedIdentity) {
          icAgent.get().replaceIdentity(new AnonymousIdentity());
          this.lastLoginPrincipal = Principal.anonymous().toText();
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
      const wallet = useWalletStore();

      this.loading = false;
      this.isAuthenticated = false;
      this.principal = Principal.anonymous().toText();
      this.lastLoginPrincipal = Principal.anonymous().toText();
      this.reauthenticationNeeded = false;
      this.data.wallets = [];
      this.data.selectedWallet.canisterId = null;
      this.data.selectedWallet.hasAccess = false;
      wallet.reset();
    },
    async signIn(resetOnError = false): Promise<void> {
      const authService = services().auth;
      const sessionExpirationService = services().sessionExpiration;

      try {
        authService.invalidateAuthClient();
        const identity = await authService.login();

        sessionExpirationService.notifySignedIn();
        await this.initializeAuthenticated(identity);
      } catch (error) {
        disableWalletWorkers();
        if (resetOnError) {
          this.reset();
        }
        throw error;
      }
    },
    async signOut(notifyOtherTabs = true): Promise<void> {
      disableWalletWorkers();

      const sessionExpirationService = services().sessionExpiration;

      sessionExpirationService.clearInactivityTimer();
      sessionExpirationService.clearSessionTimer();

      if (notifyOtherTabs) {
        sessionExpirationService.notifySignedOut();
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

        let initialWalletId = null;

        if (this.data.selectedWallet.canisterId) {
          initialWalletId = Principal.fromText(this.data.selectedWallet.canisterId);
        } else {
          initialWalletId = controlPanelUser.main_wallet?.[0]
            ? controlPanelUser.main_wallet?.[0]
            : controlPanelUser.wallets?.[0]?.canister_id;
        }
        const sameUser = this.isAuthenticated && this.principal === controlPanelUser.id.toText();

        this.isAuthenticated = true;
        this.populateUser(controlPanelUser);

        if (!sameUser && initialWalletId) {
          return this.connectWallet(initialWalletId);
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
      this.data.wallets = user.wallets.map(wallet => ({
        main: wallet.canister_id.toText() === user.main_wallet?.[0]?.toText(),
        name: wallet.name?.[0] ?? null,
        canisterId: wallet.canister_id.toText(),
      }));

      const hasWallet = this.data.wallets.some(wallet => wallet.canisterId === selectedWalletId);
      if (!sameUser || !hasWallet) {
        this.disconnectWallet();
      }
    },
    disconnectWallet(): void {
      const wallet = useWalletStore();

      this.data.selectedWallet.hasAccess = false;
      this.data.selectedWallet.canisterId = null;

      wallet.reset();
    },
    async connectWallet(walletId: Principal): Promise<void> {
      const wallet = useWalletStore();

      this.data.selectedWallet.canisterId = walletId.toText();
      this.data.selectedWallet.hasAccess = false;
      const connectionStatus = await wallet.connectTo(walletId);

      if (connectionStatus === WalletConnectionStatus.Connected) {
        this.data.selectedWallet.hasAccess = true;
      }
    },

    requireReauthentication() {
      this.reauthenticationNeeded = true;

      const sessionExpirationService = services().sessionExpiration;
      sessionExpirationService.clearInactivityTimer();
      sessionExpirationService.clearSessionTimer();

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

    async initializeAuthenticated(newIdentity: Identity) {
      const authService = services().auth;
      icAgent.get().replaceIdentity(newIdentity);

      if (
        this.lastLoginPrincipal !== null &&
        this.lastLoginPrincipal !== newIdentity.getPrincipal().toText()
      ) {
        this.reset();
      }

      this.reauthenticationNeeded = false;
      enableWalletWorkers();

      const sessionExpirationService = services().sessionExpiration;

      const maybeSessionExpirationTimeMs = await authService.getRemainingSessionTimeMs();
      if (maybeSessionExpirationTimeMs) {
        sessionExpirationService.resetSessionTimeout(maybeSessionExpirationTimeMs);
      }
      sessionExpirationService.resetInactivityTimeout();

      const controlPanelService = services().controlPanel;
      const isRegistered = await controlPanelService.hasRegistration();

      if (!isRegistered) {
        await controlPanelService.register({
          // a new user is created with an empty list of wallets, they can add them later
          wallet_id: [],
          // TODO: e-mail of the user
          email: "john@example.com",
        });
      }

      // loads information about the authenticated user
      await this.load();

      // if the user was not signed in before, or the user signed in with a different identity
      if (
        this.lastLoginPrincipal !== null &&
        this.lastLoginPrincipal !== newIdentity.getPrincipal().toText()
      ) {
        afterLoginRedirect();
      }

      this.lastLoginPrincipal = newIdentity.getPrincipal().toText();
    },
  },
});
