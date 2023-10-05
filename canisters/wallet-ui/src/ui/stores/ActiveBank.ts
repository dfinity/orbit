import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import {
  Account,
  BankAsset,
  BankFeatures,
  WalletBalance,
  WalletListItem,
} from '~/generated/bank/bank.did';
import { BankService } from '~/services';
import { i18n, services } from '~/ui/modules';
import { useAuthStore, useSettingsStore } from '~/ui/stores';

export interface BankMetrics {
  wallets: number;
  transfers: {
    completed: number;
    pending: number;
  };
}

export interface ActiveBankStoreState {
  _bankId: string;
  loading: boolean;
  _account: Account | null;
  balanceUpdaterRegistered: boolean;
  features: {
    loading: boolean;
    details: BankFeatures | null;
  };
  wallets: {
    loading: boolean;
    items: WalletListItem[];
  };
}

export const useActiveBankStore = defineStore('activeBank', {
  state: (): ActiveBankStoreState => {
    return {
      _bankId: Principal.anonymous().toString(),
      loading: false,
      balanceUpdaterRegistered: false,
      _account: null,
      features: {
        loading: false,
        details: null,
      },
      wallets: {
        loading: false,
        items: [],
      },
    };
  },
  getters: {
    hasAccount(): boolean {
      return !!this._account;
    },
    account(): Account {
      if (!this._account) {
        throw new Error('Account not loaded');
      }

      return this._account as Account;
    },
    bankId(): Principal {
      return Principal.fromText(this._bankId);
    },
    metrics(): BankMetrics {
      return {
        wallets: this.wallets.items.length,
        transfers: {
          completed: 0,
          pending: 0,
        },
      };
    },
    supportedAssets(): BankAsset[] {
      return this.features.details?.supported_assets ?? [];
    },
    service(): BankService {
      return services().bank.withBankId(this.bankId);
    },
  },
  actions: {
    setBankId(bankId: Principal): void {
      if (bankId !== this.bankId) {
        this._account = null;
      }

      this._bankId = bankId.toText();
    },
    async registerWalletBalanceUpdater(): Promise<void> {
      if (this.balanceUpdaterRegistered) {
        return;
      }
      this.balanceUpdaterRegistered = true;

      do {
        await this.resolveWalletsBalance();
        await new Promise(resolve => setTimeout(resolve, 15000));
      } while (this.balanceUpdaterRegistered);
    },
    unregisterWalletBalanceUpdater(): void {
      this.balanceUpdaterRegistered = false;
    },
    async resolveWalletsBalance(): Promise<void> {
      const walletIds = this.wallets.items.map(wallet => wallet.id);

      for (const walletId of walletIds) {
        const balance = await this.service.walletBalance({ wallet_id: walletId });

        this.wallets.items.forEach(wallet => {
          if (wallet.id === walletId) {
            wallet.balance = [
              {
                balance: balance.balance,
                decimals: balance.decimals,
                last_update_timestamp: balance.last_update_timestamp,
              },
            ];
          }
        });
      }
    },
    reset(): void {
      this._bankId = Principal.anonymous().toText();
      this._account = null;
      this.wallets.items = [];
      this.features.details = null;
    },
    async registerAccount(): Promise<Account | null> {
      const auth = useAuthStore();
      const bankService = services().bank.withBankId(this.bankId);

      const hasMultipleIdentities = auth.identities.length > 1;
      if (!hasMultipleIdentities) {
        const account = await bankService.register({
          identities: auth.identities,
        });

        return account;
      }

      // todo: add logic for multiple identities

      return null;
    },
    async loadWalletList(): Promise<void> {
      if (this.wallets.loading) {
        return;
      }
      try {
        this.wallets.loading = true;
        const bankService = services().bank.withBankId(this.bankId);
        this.wallets.items = await bankService.listWallets();
      } finally {
        this.wallets.loading = false;
      }
    },
    async loadBankFeatures(): Promise<void> {
      try {
        this.features.loading = true;
        const bankService = services().bank.withBankId(this.bankId);
        this.features.details = await bankService.features();
      } finally {
        this.features.loading = false;
      }
    },
    // these calls do not need to be awaited, it will be loaded in the background making the initial load faster
    async loadDetailsAsync(): Promise<void> {
      this.registerWalletBalanceUpdater();
      this.loadWalletList();
      this.loadBankFeatures();
    },
    async load(bankId: Principal): Promise<void> {
      if (this.loading) {
        return;
      }
      this.unregisterWalletBalanceUpdater();
      this.loading = true;
      this.setBankId(bankId);
      const bankService = services().bank.withBankId(this.bankId);
      const settings = useSettingsStore();
      try {
        const account = await bankService.myAccount();
        if (account) {
          this._account = account;
          this.loadDetailsAsync();
          return;
        }

        const registeredAccount = await this.registerAccount();

        this._account = registeredAccount;

        if (registeredAccount) {
          this.loadDetailsAsync();
        }
      } catch (err) {
        logger.error(`Failed to load bank account`, { err });

        settings.setNotification({
          show: true,
          type: 'error',
          message: i18n.global.t('banks.account_load_error'),
        });
      } finally {
        this.loading = false;
      }
    },
  },
});
