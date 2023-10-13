import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import {
  Account,
  BankAsset,
  BankFeatures,
  Operation,
  OperationId,
  Wallet,
} from '~/generated/bank/bank.did';
import { BankService } from '~/services';
import { i18n, services } from '~/ui/modules';
import { useAuthStore, useSettingsStore, useWorkerStore } from '~/ui/stores';
import { LoadableItem } from '~/ui/types';

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
  features: {
    loading: boolean;
    details: BankFeatures | null;
  };
  wallets: {
    loading: boolean;
    items: Wallet[];
  };
  pendingOperations: {
    loading: boolean;
    items: LoadableItem<Operation>[];
  };
}

export const useActiveBankStore = defineStore('activeBank', {
  state: (): ActiveBankStoreState => {
    return {
      _bankId: Principal.anonymous().toString(),
      loading: false,
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
    sortedWallets(): Wallet[] {
      return this.wallets.items.sort((a, b) => {
        const firstDt = new Date(a.last_modification_timestamp).getTime();
        const secondDt = new Date(b.last_modification_timestamp).getTime();

        return secondDt - firstDt;
      });
    },
    lastPendingOperationDate(): Date | null {
      if (!this.pendingOperations.items.length) {
        return null;
      }

      return new Date(this.pendingOperations.items[0].data.created_at);
    },
    lastPendingOperationId(): OperationId | null {
      if (!this.pendingOperations.items.length) {
        return null;
      }

      return this.pendingOperations.items[0].data.id;
    },
    sortedPendingOperations(): LoadableItem<Operation>[] {
      return this.pendingOperations.items.sort((a, b) => {
        const firstDt = new Date(a.data.created_at);
        const secondDt = new Date(b.data.created_at);

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

      return this.registerWithMultiIdentityFlow();
    },
    async registerWithMultiIdentityFlow(): Promise<Account | null> {
      // TODO: implement multi identity register flow

      return null;
    },
    async saveDecision(
      operationId: OperationId,
      decision: { approve?: boolean; reason?: string; read?: boolean },
    ): Promise<Operation | null> {
      const settings = useSettingsStore();
      const pendingOperation = this.pendingOperations.items.find(
        item => item.data.id === operationId,
      );
      if (pendingOperation) {
        pendingOperation.loading = true;
      }

      try {
        return await this.service
          .submitOperationDecision({
            operation_id: operationId,
            approve: decision.approve !== undefined ? [decision.approve] : [],
            read: decision.read !== undefined ? [decision.read] : [],
            reason: decision.reason !== undefined ? [decision.reason] : [],
          })
          .then(operation => {
            this.pendingOperations.items = this.pendingOperations.items.filter(item => {
              if (item.data.id !== operation.id) {
                return true;
              }
              const isPending = 'Pending' in operation.status;
              const isRead = operation.decisions.some(
                decision => decision.account_id === this.account.id && decision.read,
              );

              return isPending && !isRead;
            });

            return operation;
          });
      } catch (err) {
        logger.error(`Failed to save operation`, { err });

        settings.setNotification({
          show: true,
          type: 'error',
          message: i18n.global.t('banks.operation_failed_to_save'),
        });
      } finally {
        if (pendingOperation) {
          pendingOperation.loading = false;
        }
      }

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
      useWorkerStore().start();
      this.loadWalletList();
      this.loadBankFeatures();
    },
    async load(bankId: Principal): Promise<void> {
      if (this.loading) {
        return;
      }
      useWorkerStore().stop();
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
