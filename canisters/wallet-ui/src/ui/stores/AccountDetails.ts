import { defineStore } from 'pinia';
import { endOfDay, logger, startOfDay } from '~/core';
import {
  Error as ApiError,
  Operation,
  OperationStatus,
  TransferListItem,
  Account,
  AccountId,
  OperationId,
} from '~/generated/bank/bank.did';
import { BankService, ChainApiFactory } from '~/services';
import { ChainApi, AccountIncomingTransfer } from '~/types';
import { i18n } from '~/ui/modules';
import { useActiveBankStore, useSettingsStore } from '~/ui/stores';
import { LoadableItem } from '~/ui/types';

export interface AccountDetailsStoreState {
  notification: {
    show: boolean;
    type: 'success' | 'error' | 'warning' | 'info';
    message: string | null;
  };
  loading: boolean;
  _account: Account | null;
  transfers: {
    loading: boolean;
    items: TransferListItem[];
    fromDt: string | null;
    toDt: string | null;
  };
  operations: {
    loading: boolean;
    items: LoadableItem<Operation>[];
    fromDt: string | null;
    toDt: string | null;
  };
  receivables: {
    loading: boolean;
    items: AccountIncomingTransfer[];
  };
}

const initialState: AccountDetailsStoreState = {
  loading: false,
  _account: null,
  notification: {
    message: null,
    show: false,
    type: 'success',
  },
  transfers: {
    loading: false,
    items: [],
    fromDt: null,
    toDt: null,
  },
  operations: {
    loading: false,
    items: [],
    fromDt: null,
    toDt: null,
  },
  receivables: {
    loading: false,
    items: [],
  },
};

export const useAccountDetailsStore = defineStore('accountDetails', {
  state: (): AccountDetailsStoreState => {
    return JSON.parse(JSON.stringify(initialState));
  },
  getters: {
    account(state): Account {
      if (!state._account) {
        throw new Error('Account not initialized');
      }

      return state._account;
    },
    defaultStartDt(): string {
      const start = new Date();
      start.setDate(start.getDate() - 7);

      return start.toISOString();
    },
    sortedTransfers(): TransferListItem[] {
      return this.transfers.items.sort((a, b) => {
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
      });
    },
    sortedReceivables(): AccountIncomingTransfer[] {
      return this.receivables.items.sort((a, b) => {
        return new Date(b.created_at ?? 0).getTime() - new Date(a.created_at ?? 0).getTime();
      });
    },
    chainApi(): ChainApi | null {
      try {
        if (!this._account) {
          return null;
        }

        return ChainApiFactory.create(this._account);
      } catch (err) {
        logger.warn('chain api not supported', { err });
        // the account is loaded but with limited real data since the blockchain is not supported by the UI
        return null;
      }
    },
    sortedOperations(): LoadableItem<Operation>[] {
      return this.operations.items.sort((a, b) => {
        return new Date(b.data.created_at).getTime() - new Date(a.data.created_at).getTime();
      });
    },
    defaultEndDt(): string {
      return new Date().toISOString();
    },
    hasLoaded(): boolean {
      return this._account !== null;
    },
    bankService(): BankService {
      return useActiveBankStore().service;
    },
  },
  actions: {
    reset(): void {
      const reset = JSON.parse(JSON.stringify(initialState));
      this.loading = reset.loading;
      this._account = reset._account;
      this.notification = reset.notification;
      this.transfers = reset.transfers;
      this.transfers.fromDt = new Date(this.defaultStartDt).toISOString().split('T')[0];
      this.transfers.toDt = new Date(this.defaultEndDt).toISOString().split('T')[0];
      this.operations = reset.operations;
      this.operations.fromDt = new Date(this.defaultStartDt).toISOString().split('T')[0];
      this.operations.toDt = new Date(this.defaultEndDt).toISOString().split('T')[0];
      this.receivables = reset.receivables;
    },
    showPageNotification(type: 'error' | 'success' | 'warning' | 'info', message: string): void {
      this.notification = {
        show: true,
        type,
        message,
      };
    },
    clearPageNotification(): void {
      this.notification.show = false;
    },
    async loadReceivables(): Promise<void> {
      if (!this.chainApi) {
        return;
      }

      const transfers = await this.chainApi.fetchTransfers({
        from_dt: new Date(),
      });

      this.receivables.items = transfers;
    },
    async saveDecision(
      operationId: OperationId,
      decision: { approve?: boolean; reason?: string; read?: boolean },
    ): Promise<void> {
      const activeBank = useActiveBankStore();
      const item = this.operations.items.find(item => item.data.id === operationId);
      if (!item) {
        logger.warn('Decision not saved, operation not found', { operationId });
        return;
      }

      item.loading = true;
      const operation = await activeBank
        .saveDecision(operationId, decision)
        .finally(() => (item.loading = false));

      if (!operation) {
        return;
      }

      this.operations.items.forEach(item => {
        if (item.data.id === operation.id) {
          item.data = operation;
        }
      });
    },
    async loadOperations(fromDt?: Date, toDt?: Date, status?: OperationStatus): Promise<void> {
      try {
        this.operations.loading = true;
        this.operations.items = await this.bankService
          .listAccountOperations({
            account_id: this.account.id,
            status: status ? [status] : [],
            from_dt: fromDt ? [startOfDay(fromDt).toISOString()] : [],
            to_dt: toDt ? [endOfDay(toDt).toISOString()] : [],
            code: [],
            read: [],
          })
          .then(operations => {
            return operations.map(operation => {
              return {
                loading: false,
                data: operation,
              };
            });
          });
      } catch (e) {
        logger.error('Failed to load operations', { e });
        const settings = useSettingsStore();
        this.operations.items = [];

        settings.setNotification({
          show: true,
          message: i18n.global.t('banks.load_error_operations'),
          type: 'error',
        });
      } finally {
        this.operations.loading = false;
      }
    },
    async loadSentTransfers(fromDt?: Date, toDt?: Date, status?: string): Promise<void> {
      try {
        this.transfers.loading = true;
        this.transfers.items = await this.bankService.listAccountTransfers({
          account_id: this.account.id,
          from_dt: fromDt ? [startOfDay(fromDt).toISOString()] : [],
          to_dt: toDt ? [endOfDay(toDt).toISOString()] : [],
          status: status ? [status] : [],
        });
      } catch (e) {
        logger.error('Failed to load transfers', { e });
        const settings = useSettingsStore();
        this.transfers.items = [];

        settings.setNotification({
          show: true,
          message: i18n.global.t('banks.load_error_transfers'),
          type: 'error',
        });
      } finally {
        this.transfers.loading = false;
      }
    },
    async load(accountId: AccountId): Promise<void> {
      try {
        this.reset();
        this.loading = true;
        const activeBank = useActiveBankStore();

        this._account = await this.bankService.getAccount({
          account_id: accountId,
        });

        const updatedBalance = activeBank.accounts.items.find(item => item.id === accountId)?.balance;
        if (updatedBalance) {
          this._account.balance = updatedBalance;
        }

        this.loadSentTransfers(new Date(this.defaultStartDt), new Date(this.defaultEndDt));
        this.loadOperations(new Date(this.defaultStartDt), new Date(this.defaultEndDt));
        this.loadReceivables();
      } catch (e) {
        logger.error('Failed to load account', { e });

        const err = e as ApiError;
        this.showPageNotification('error', err.message?.[0] ? err.message[0] : err.code);
      } finally {
        this.loading = false;
      }
    },
  },
});
