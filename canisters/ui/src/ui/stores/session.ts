import { AnonymousIdentity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { icAgent, logger, wait } from '~/core';
import { i18n, services } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';
import {
  WalletConnectionStatus,
  createUserInitialAccount,
  useWalletStore,
} from '~/ui/stores/wallet';
import { computedWalletName, redirectToLogin } from '~/ui/utils';

export interface UserWallet {
  main: boolean;
  name: string | null;
  canisterId: string;
}

export interface SelectedUserWallet {
  canisterId: string | null;
  hasAccess: boolean;
}

export enum EnvironmentStatus {
  Uninitialized = 'uninitialized',
  DeployingInitialWallet = 'deploying-initial-wallet',
  FailedInitialization = 'failed-initialization',
  Ready = 'ready',
}

export interface SessionStoreState {
  initialized: boolean;
  loading: boolean;
  environmentStatus: EnvironmentStatus;
  principal: string;
  isAuthenticated: boolean;
  data: {
    wallets: UserWallet[];
    selectedWallet: SelectedUserWallet;
  };
}

const deployInitialWallet = async (
  self = useSessionStore(),
  wallet = useWalletStore(),
): Promise<void> => {
  if (self.hasWallets) {
    logger.warn('Cannot deploy initial wallet, user already has wallets');
    return;
  } else if (self.environmentStatus === EnvironmentStatus.DeployingInitialWallet) {
    logger.warn('Cannot deploy initial wallet, already deploying');
    return;
  }

  try {
    self.environmentStatus = EnvironmentStatus.DeployingInitialWallet;
    const controlPanelService = services().controlPanel;
    const walletId = await controlPanelService.deployWallet();
    const controlPanelUser = await controlPanelService.getCurrentUser();

    self.data = {
      wallets:
        controlPanelUser.wallets.map(wallet => ({
          main: wallet.canister_id.toText() === controlPanelUser.main_wallet?.[0]?.toText(),
          name: wallet.name?.[0] ?? null,
          canisterId: wallet.canister_id.toText(),
        })) ?? [],
      selectedWallet: {
        canisterId: walletId.toText(),
        hasAccess: true,
      },
    };

    // wait for the wallet to be initialized, this requires one round of consensus
    await wait(6000);

    await self.loadWallet(walletId);

    if (wallet.user) {
      await createUserInitialAccount(wallet.user.id);
    }

    self.environmentStatus = EnvironmentStatus.Ready;
  } catch (err) {
    logger.error('Failed to deploy initial wallet', { err });
    self.environmentStatus = EnvironmentStatus.FailedInitialization;
  }
};

export const useSessionStore = defineStore('session', {
  state: (): SessionStoreState => {
    return {
      initialized: false,
      loading: false,
      environmentStatus: EnvironmentStatus.Uninitialized,
      principal: Principal.anonymous().toText(),
      isAuthenticated: false,
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
      try {
        if (this.initialized) {
          return;
        }
        const authService = services().auth;
        const cachedIdentity = await authService.identity();

        if (!cachedIdentity) {
          icAgent.get().replaceIdentity(new AnonymousIdentity());
          this.initialized = true;
          return;
        }

        icAgent.get().replaceIdentity(cachedIdentity);

        await this.load();

        this.initialized = true;
      } catch (error) {
        this.reset();

        logger.error(`Application failed to initialize the state`, { error });
      }
    },
    reset(): void {
      const wallet = useWalletStore();

      this.loading = false;
      this.isAuthenticated = false;
      this.principal = Principal.anonymous().toText();
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
      const authService = services().auth;

      try {
        const identity = await authService.login();
        icAgent.get().replaceIdentity(identity);

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
        this.reset();
        throw error;
      }
    },
    async signOut(): Promise<void> {
      const authService = services().auth;

      await authService.logout();

      this.reset();
      redirectToLogin();
    },
    unloadWallet(): void {
      const wallet = useWalletStore();

      this.data.selectedWallet = {
        canisterId: null,
        hasAccess: false,
      };

      wallet.reset();
    },
    async loadWallet(walletId: Principal): Promise<void> {
      const wallet = useWalletStore();

      this.data.selectedWallet = {
        canisterId: walletId.toText(),
        hasAccess: false,
      };

      if (wallet.canisterId) {
        wallet.reset();
      }

      const name = computedWalletName({ canisterId: walletId });

      const connectionStatus = await wallet.connectTo(walletId, name);

      if (connectionStatus === WalletConnectionStatus.Connected) {
        this.data.selectedWallet.hasAccess = true;
      }
    },
    async loadUser(): Promise<void> {
      const controlPanelService = services().controlPanel;
      const controlPanelUser = await controlPanelService.getCurrentUser();
      const mainWalletId = controlPanelUser.main_wallet?.[0]
        ? controlPanelUser.main_wallet?.[0]
        : controlPanelUser.wallets?.[0]?.canister_id;

      this.isAuthenticated = true;
      this.principal = controlPanelUser.id.toText();
      this.data = {
        wallets:
          controlPanelUser.wallets?.map(wallet => ({
            main: wallet.canister_id.toText() === mainWalletId.toText(),
            name: wallet.name?.[0] ?? null,
            canisterId: wallet.canister_id.toText(),
          })) ?? [],
        selectedWallet: {
          canisterId: mainWalletId?.toText() ?? null,
          hasAccess: false,
        },
      };
    },
    async load(): Promise<void> {
      this.loading = true;
      const app = useAppStore();
      try {
        await this.loadUser();

        if (this.mainWallet) {
          await this.loadWallet(this.mainWallet);

          this.environmentStatus = EnvironmentStatus.Ready;
        } else {
          // this is not awaited so that the UI can load faster and show early feedback to the user
          deployInitialWallet();
        }
      } catch (err) {
        logger.error(`Failed to load user session`, { err });

        this.environmentStatus = EnvironmentStatus.FailedInitialization;

        app.sendNotification({
          type: 'error',
          message: i18n.global.t('app.session_load_error'),
        });
      } finally {
        this.loading = false;
      }
    },
  },
});
