import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import { PolicyType } from '~/types';
import { i18n, services } from '~/ui/modules';
import { useSettingsStore } from '~/ui/stores';

export interface WalletItem {
  name: string | null;
  canisterId: string;
}

export interface WalletStoreState {
  loading: boolean;
  initialized: boolean;
  _main: string | null;
  wallets: WalletItem[];
}

export const useWalletStore = defineStore('wallet', {
  state: (): WalletStoreState => {
    return {
      loading: false,
      initialized: false,
      _main: null,
      wallets: [],
    };
  },
  getters: {
    hasWallets(): boolean {
      return !!this.wallets.length;
    },
    main(): Principal | null {
      return this._main ? Principal.fromText(this._main) : null;
    },
    policyTypes(): string[] {
      return Object.values(PolicyType);
    },
  },
  actions: {
    async init(): Promise<void> {
      if (this.initialized) {
        return;
      }

      await this.load().finally(() => {
        this.initialized = true;
      });
    },
    computedWalletName(canisterId: Principal, notFoundName = '-'): string {
      const walletIdx = this.wallets.findIndex(wallet => wallet.canisterId === canisterId.toText());

      if (walletIdx === -1) {
        return notFoundName;
      }

      return (
        this.wallets[walletIdx].name ??
        i18n.global.t('wallets.wallet_nr_title', { nr: walletIdx + 1 })
      );
    },
    reset(): void {
      this.initialized = false;
      this._main = null;
      this.wallets = [];
    },
    useWallets(wallets: WalletItem[]): void {
      this.wallets = wallets;
      if (
        this.main &&
        !wallets.some(({ canisterId }) => canisterId == this._main) &&
        wallets.length
      ) {
        this._main = wallets[0].canisterId;
      }
    },
    async load(): Promise<void> {
      this.loading = true;
      const controlPanelService = services().controlPanel;
      const settings = useSettingsStore();
      await Promise.all([controlPanelService.getMainWallet(), controlPanelService.listWallets()])
        .then(([mainWallet, wallets]) => {
          const main = mainWallet ?? wallets?.[0];
          const mainCanisterId = main?.canister_id ?? null;
          if (
            mainCanisterId &&
            wallets.some(({ canister_id }) => canister_id.compareTo(mainCanisterId))
          ) {
            this._main = mainCanisterId.toText();
          }
          this.wallets = wallets.map(wallet => ({
            canisterId: wallet.canister_id.toString(),
            name: wallet.name?.[0] ?? null,
          }));
        })
        .catch(err => {
          logger.error(`Failed to load wallets`, { err });
          settings.setNotification({
            show: true,
            type: 'error',
            message: i18n.global.t('wallets.load_error'),
          });
        })
        .finally(() => {
          this.loading = false;
        });
    },
  },
});
