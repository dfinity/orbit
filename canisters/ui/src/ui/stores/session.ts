import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import { i18n, services } from '~/ui/modules';
import { useAppStore, useWalletStore } from '~/ui/stores';

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
    loadWallet(walletId: Principal): void {
      const wallet = useWalletStore();

      this.selectedWallet = walletId;
      if (wallet.canisterId) {
        wallet.reset();
      }

      wallet.load(walletId);
    },
    async load(): Promise<void> {
      this.loading = true;
      const controlPanelService = services().controlPanel;
      const app = useAppStore();
      try {
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

        if (mainWalletId) {
          await this.loadMainWallet(mainWalletId);

          this.environmentStatus = EnvironmentStatus.Ready;
        } else {
          // this is not awaited so that the UI can load faster and show early feedback to the user
          this.deployInitialWallet();
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
    async loadMainWallet(walletId: Principal): Promise<void> {
      this.selectedWallet = walletId;
      const wallet = useWalletStore();
      await wallet.load(walletId);
    },
    async deployInitialWallet(): Promise<void> {
      if (this.hasWallets) {
        logger.warn('Cannot deploy initial wallet, user already has wallets');
        return;
      } else if (this.environmentStatus === EnvironmentStatus.DeployingInitialWallet) {
        logger.warn('Cannot deploy initial wallet, already deploying');
        return;
      }

      try {
        this.environmentStatus = EnvironmentStatus.DeployingInitialWallet;
        const controlPanelService = services().controlPanel;
        const walletId = await controlPanelService.deployWallet();
        const controlPanelUser = await controlPanelService.getCurrentUser();

        this.user = {
          ...this.user,
          mainWallet: walletId.toText(),
          wallets:
            controlPanelUser.wallets?.map(wallet => ({
              main: wallet.canister_id === walletId,
              name: wallet.name?.[0] ?? null,
              canisterId: wallet.canister_id.toText(),
            })) ?? [],
        } as UserSession;

        await this.loadMainWallet(walletId);
        this.environmentStatus = EnvironmentStatus.Ready;
      } catch (err) {
        logger.error('Failed to deploy initial wallet', { err });
        this.environmentStatus = EnvironmentStatus.FailedInitialization;
      }
    },
    computedWalletName(canisterId: Principal, notFoundName = '-'): string {
      const walletIdx =
        this.user?.wallets.findIndex(wallet => wallet.canisterId === canisterId.toText()) ?? -1;

      if (walletIdx === -1) {
        return notFoundName;
      }

      return (
        this.user?.wallets?.[walletIdx].name ??
        i18n.global.t('wallets.wallet_nr_title', { nr: walletIdx + 1 })
      );
    },
  },
});
