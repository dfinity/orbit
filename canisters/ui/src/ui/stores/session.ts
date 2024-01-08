import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger, wait } from '~/core';
import { UUID } from '~/generated/wallet/wallet.did';
import { BlockchainStandard, BlockchainType } from '~/types';
import { i18n, services } from '~/ui/modules';
import { useAppStore, useWalletStore } from '~/ui/stores';
import { computedWalletName } from '~/ui/utils';

export interface WalletListItem {
  main: boolean;
  name: string | null;
  canisterId: string;
}

export interface UserSession {
  principal: Principal;
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
  loading: boolean;
  environmentStatus: EnvironmentStatus;
  user: UserSession | null;
  selectedWallet: Principal | null;
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
      environmentStatus: EnvironmentStatus.Uninitialized,
      loading: false,
      user: null,
      selectedWallet: null,
    };
  },
  getters: {
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
    reset(): void {
      const wallet = useWalletStore();

      this.loading = false;
      this.user = null;

      wallet.reset();
    },
    unloadWallet(): void {
      const wallet = useWalletStore();

      this.selectedWallet = null;
      wallet.reset();
    },
    async loadWallet(walletId: Principal): Promise<void> {
      const wallet = useWalletStore();

      this.selectedWallet = walletId;
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
        principal: controlPanelUser.id,
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
