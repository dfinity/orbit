import { AnonymousIdentity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { icAgent, logger, wait } from '~/core';
import { UUID } from '~/generated/wallet/wallet.did';
import { BlockchainStandard, BlockchainType } from '~/types';
import { i18n, services } from '~/ui/modules';
import { useAppStore, useWalletStore } from '~/ui/stores';
import { computedWalletName, redirectToLogin } from '~/ui/utils';

export interface WalletListItem {
  main: boolean;
  name: string | null;
  canisterId: string;
}

export interface UserSession {
  principal: string;
  isAuthenticated: boolean;
  mainWallet: string | null;
  wallets: WalletListItem[];
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
  user: UserSession;
  selectedWallet: string | null;
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

    self.user = {
      ...self.user,
      mainWallet: walletId.toText(),
      wallets:
        controlPanelUser.wallets?.map(wallet => ({
          main: wallet.canister_id === walletId,
          name: wallet.name?.[0] ?? null,
          canisterId: wallet.canister_id.toText(),
        })) ?? [],
    } as UserSession;

    // wait for the wallet to be initialized, this requires one round of consensus
    await wait(6000);

    await self.loadWallet(walletId);

    if (wallet.user) {
      await createInitialAccountForUser(wallet.user.me.id);
    }

    self.environmentStatus = EnvironmentStatus.Ready;
  } catch (err) {
    logger.error('Failed to deploy initial wallet', { err });
    self.environmentStatus = EnvironmentStatus.FailedInitialization;
  }
};

const createInitialAccountForUser = async (
  ownerId: UUID,
  wallet = useWalletStore(),
): Promise<void> => {
  const proposal = await wallet.service.createProposal({
    title: [],
    summary: [],
    execution_plan: [{ Immediate: null }],
    operation: {
      AddAccount: {
        name: i18n.global.t('app.initial_account_name'),
        blockchain: BlockchainType.InternetComputer,
        standard: BlockchainStandard.Native,
        metadata: [],
        owners: [ownerId],
        policies: {
          edit: [],
          transfer: [],
        },
      },
    },
  });

  console.log(proposal);
};

export const useSessionStore = defineStore('session', {
  state: (): SessionStoreState => {
    return {
      initialized: false,
      loading: false,
      environmentStatus: EnvironmentStatus.Uninitialized,
      user: {
        isAuthenticated: false,
        mainWallet: null,
        principal: Principal.anonymous().toText(),
        wallets: [],
      },
      selectedWallet: null,
    };
  },
  getters: {
    isAuthenticated(): boolean {
      return this.user.isAuthenticated;
    },
    hasUser(): boolean {
      return !!this.user;
    },
    hasWallets(): boolean {
      return !!this.user?.wallets.length;
    },
    mainWallet(): Principal | null {
      return this.user?.mainWallet ? Principal.fromText(this.user.mainWallet) : null;
    },
    hasSelectedWallet(): boolean {
      return !!this.selectedWallet;
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
      this.user = {
        isAuthenticated: false,
        mainWallet: null,
        principal: Principal.anonymous().toText(),
        wallets: [],
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

      this.selectedWallet = null;
      wallet.reset();
    },
    async loadWallet(walletId: Principal): Promise<void> {
      const wallet = useWalletStore();

      this.selectedWallet = walletId.toText();
      if (wallet.canisterId) {
        wallet.reset();
      }

      const name = computedWalletName({ canisterId: walletId });

      await wallet.load(walletId, name);
    },
    async loadUser(): Promise<void> {
      const controlPanelService = services().controlPanel;
      const controlPanelUser = await controlPanelService.getCurrentUser();
      const mainWalletId = controlPanelUser.main_wallet?.[0]
        ? controlPanelUser.main_wallet?.[0]
        : controlPanelUser.wallets?.[0]?.canister_id;

      this.user = {
        isAuthenticated: true,
        principal: controlPanelUser.id.toText(),
        mainWallet: mainWalletId?.toText() ?? null,
        wallets:
          controlPanelUser.wallets?.map(wallet => ({
            main: wallet.canister_id === mainWalletId,
            name: wallet.name?.[0] ?? null,
            canisterId: wallet.canister_id.toText(),
          })) ?? [],
      };
    },
    async load(): Promise<void> {
      this.loading = true;
      const app = useAppStore();
      try {
        await this.loadUser();

        if (this.user?.mainWallet) {
          await this.loadWallet(Principal.fromText(this.user.mainWallet));

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
