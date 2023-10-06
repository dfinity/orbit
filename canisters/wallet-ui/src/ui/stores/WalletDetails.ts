import { defineStore } from 'pinia';
import { endOfDay, logger, startOfDay } from '~/core';
import {
  Error as ApiError,
  Operation,
  OperationStatus,
  TransferListItem,
  Wallet,
  WalletId,
} from '~/generated/bank/bank.did';
import { BankService } from '~/services';
import { i18n } from '~/ui/modules';
import { useActiveBankStore, useSettingsStore } from '~/ui/stores';

export interface WalletDetailsStoreState {
  notification: {
    show: boolean;
    type: 'success' | 'error' | 'warning' | 'info';
    message: string | null;
  };
  loading: boolean;
  _wallet: Wallet | null;
  transfers: {
    loading: boolean;
    items: TransferListItem[];
    fromDt: string | null;
    toDt: string | null;
  };
  operations: {
    loading: boolean;
    items: Operation[];
    fromDt: string | null;
    toDt: string | null;
  };
}

const initialState: WalletDetailsStoreState = {
  loading: false,
  _wallet: null,
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
};

export const useWalletDetailsStore = defineStore('walletDetails', {
  state: (): WalletDetailsStoreState => {
    return JSON.parse(JSON.stringify(initialState));
  },
  getters: {
    wallet(state): Wallet {
      if (!state._wallet) {
        throw new Error('Wallet not initialized');
      }

      return state._wallet;
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
    sortedOperations(): Operation[] {
      return this.operations.items.sort((a, b) => {
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
      });
    },
    defaultEndDt(): string {
      return new Date().toISOString();
    },
    hasLoaded(): boolean {
      return this._wallet !== null;
    },
    bankService(): BankService {
      return useActiveBankStore().service;
    },
  },
  actions: {
    reset(): void {
      const reset = JSON.parse(JSON.stringify(initialState));
      this.loading = reset.loading;
      this._wallet = reset._wallet;
      this.notification = reset.notification;
      this.transfers = reset.transfers;
      this.transfers.fromDt = new Date(this.defaultStartDt).toISOString().split('T')[0];
      this.transfers.toDt = new Date(this.defaultEndDt).toISOString().split('T')[0];
      this.operations = reset.operations;
      this.operations.fromDt = new Date(this.defaultStartDt).toISOString().split('T')[0];
      this.operations.toDt = new Date(this.defaultEndDt).toISOString().split('T')[0];
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
    async loadOperations(fromDt?: Date, toDt?: Date, status?: OperationStatus): Promise<void> {
      try {
        this.operations.loading = true;
        this.operations.items = await this.bankService.listWalletOperations({
          wallet_id: this.wallet.id,
          status: status ? [status] : [],
          from_dt: fromDt ? [startOfDay(fromDt).toISOString()] : [],
          to_dt: toDt ? [endOfDay(toDt).toISOString()] : [],
          code: [],
          read: [],
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
        this.transfers.items = await this.bankService.listWalletTransfers({
          wallet_id: this.wallet.id,
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
    async load(walletId: WalletId): Promise<void> {
      try {
        this.reset();
        this.loading = true;

        this._wallet = await this.bankService.getWallet({
          wallet_id: walletId,
        });

        this.loadSentTransfers(new Date(this.defaultStartDt), new Date(this.defaultEndDt));
        this.loadOperations(new Date(this.defaultStartDt), new Date(this.defaultEndDt));
      } catch (e) {
        logger.error('Failed to load wallet', { e });

        const err = e as ApiError;
        this.showPageNotification('error', err.message?.[0] ? err.message[0] : err.code);
      } finally {
        this.loading = false;
      }
    },
  },
});
