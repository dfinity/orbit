import { defineStore } from 'pinia';
import { logger } from '~/core';
import { Error as ApiError, TransferListItem, Wallet, WalletId } from '~/generated/bank/bank.did';
import { BankService } from '~/services';
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
    fromDt: Date | null;
    toDt: Date | null;
    page: number;
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
    page: 1,
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
    async loadSentTransfers(fromDt?: Date, toDt?: Date, status?: string): Promise<void> {
      try {
        this.transfers.loading = true;
        this.transfers.items = await this.bankService.listWalletTransfers({
          wallet_id: this.wallet.id,
          from_dt: fromDt ? [fromDt.toISOString()] : [],
          to_dt: toDt ? [toDt.toISOString()] : [],
          status: status ? [status] : [],
        });
      } catch (e) {
        logger.error('Failed to load transfers', { e });
        const settings = useSettingsStore();

        const err = e as ApiError;
        settings.setNotification({
          show: true,
          message: err.message?.[0] ? err.message[0] : err.code,
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

        this.loadSentTransfers();
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
