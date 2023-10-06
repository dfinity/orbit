import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import {
  Account,
  BankAsset,
  BankFeatures,
  Operation,
  OperationId,
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
  pendingOperations: number;
}

export interface ActiveBankStoreState {
  _bankId: string;
  loading: boolean;
  _account: Account | null;
  pollingJobs: {
    walletBalance?: number;
    pendingOperations?: number;
  };
  features: {
    loading: boolean;
    details: BankFeatures | null;
  };
  wallets: {
    loading: boolean;
    items: WalletListItem[];
  };
  pendingOperations: {
    loading: boolean;
    items: Operation[];
  };
}

export const useActiveBankStore = defineStore('activeBank', {
  state: (): ActiveBankStoreState => {
    return {
      _bankId: Principal.anonymous().toString(),
      loading: false,
      pollingJobs: {
        walletBalance: undefined,
        pendingOperations: undefined,
      },
      _account: null,
      features: {
        loading: false,
        details: null,
      },
      wallets: {
        loading: false,
        items: [],
      },
      pendingOperations: {
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
    lastPendingOperationDate(): Date | null {
      if (!this.pendingOperations.items.length) {
        return null;
      }

      return new Date(this.pendingOperations.items[0].created_at);
    },
    lastPendingOperationId(): OperationId | null {
      if (!this.pendingOperations.items.length) {
        return null;
      }

      return this.pendingOperations.items[0].id;
    },
    sortedPendingOperations(): Operation[] {
      return this.pendingOperations.items.sort((a, b) => {
        const firstDt = new Date(a.created_at);
        const secondDt = new Date(b.created_at);

        return secondDt.getTime() - firstDt.getTime();
      });
    },
    hasPendingOperations(): boolean {
      return this.pendingOperations.items.length > 0;
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
        pendingOperations: this.pendingOperations.items.length,
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
    registerJobs(): void {
      if (!this.pollingJobs.walletBalance) {
        this.fetchWalletsBalance().catch(err => {
          logger.error('Failed to fetch wallets balance', { err });
        });
        this.pollingJobs.walletBalance = setInterval(async () => {
          await this.fetchWalletsBalance().catch(err => {
            logger.error('Failed to fetch wallets balance', { err });
          });
        }, 15000) as unknown as number;
      }
      if (!this.pollingJobs.pendingOperations) {
        this.fetchPendingOperations().catch(err => {
          logger.error('Failed to fetch pending operations', { err });
        });
        this.pollingJobs.pendingOperations = setInterval(async () => {
          await this.fetchPendingOperations().catch(err => {
            logger.error('Failed to fetch pending operations', { err });
          });
        }, 10000) as unknown as number;
      }
    },
    unregisterJobs(): void {
      clearInterval(this.pollingJobs.walletBalance);
      clearInterval(this.pollingJobs.pendingOperations);
    },
    async fetchWalletsBalance(): Promise<void> {
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
    async fetchPendingOperations(): Promise<void> {
      const newOperations = await this.service.listUnreadPendingOperations(
        this.lastPendingOperationDate ?? undefined,
        this.lastPendingOperationId ?? undefined,
      );

      for (const newOperation of newOperations) {
        if (!this.pendingOperations.items.find(current => current.id === newOperation.id)) {
          this.pendingOperations.items.push(newOperation);
        }
      }
    },
    reset(): void {
      this._bankId = Principal.anonymous().toText();
      this._account = null;
      this.wallets.items = [];
      this.features.details = null;
      this.pendingOperations.items = [];
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
    async saveOperation(operation: Operation): Promise<void> {
      const settings = useSettingsStore();

      try {
        const currentOperation = await this.service.getOperation({ operation_id: operation.id });
        let approve: [] | [boolean] = [];
        if ('Pending' in currentOperation.status && 'Adopted' in operation.status) {
          approve = [true];
        } else if ('Pending' in currentOperation.status && 'Rejected' in operation.status) {
          approve = [false];
        }

        let feedback_reason: [] | [string] = [];
        if ('Pending' in currentOperation.status && operation.feedback_reason?.[0]) {
          feedback_reason = [operation.feedback_reason?.[0]];
        }

        await this.service
          .editOperation({
            operation_id: operation.id,
            approve: approve,
            read: [operation.read],
            reason: feedback_reason,
          })
          .then(operation => {
            this.pendingOperations.items = this.pendingOperations.items.filter(item => {
              if (item.id !== operation.id) {
                return true;
              }
              const isPending = 'Pending' in operation.status;
              const isRead = operation.read;

              console.log(isPending, !isRead);

              return isPending && !isRead;
            });
          });
      } catch (err) {
        logger.error(`Failed to save operation`, { err });

        settings.setNotification({
          show: true,
          type: 'error',
          message: i18n.global.t('banks.operation_failed_to_save'),
        });
      }
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
      this.registerJobs();
      this.loadWalletList();
      this.loadBankFeatures();
    },
    async load(bankId: Principal): Promise<void> {
      if (this.loading) {
        return;
      }
      this.unregisterJobs();
      this.reset();
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
