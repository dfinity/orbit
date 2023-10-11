import { defineStore } from 'pinia';
import { arrayBatchMaker, logger, timer } from '~/core';
import { Transfer, WalletBalance } from '~/generated/bank/bank.did';
import { useActiveBankStore } from '~/ui/stores';

const BALANCE_POLLING_INTERVAL = 10000;
const NOTIFICATIONS_POLLING_INTERVAL = 5000;
const BALANCE_FETCH_BATCH_QUANTITY = 5;

export interface WorkerStoreState {
  pollingJobs: {
    balances?: NodeJS.Timeout;
    notifications?: NodeJS.Timeout;
  };
  cachedTransfers: Record<string, Transfer>;
}

export const useWorkerStore = defineStore('cache', {
  state: (): WorkerStoreState => {
    return {
      pollingJobs: {},
      cachedTransfers: {},
    };
  },
  actions: {
    stop(): void {
      clearInterval(this.pollingJobs.balances);
      clearInterval(this.pollingJobs.notifications);
    },
    start(): void {
      this.stop();
      this.pollingJobs.balances = timer(() => this.fetchWalletBalances(), BALANCE_POLLING_INTERVAL);
      this.pollingJobs.notifications = timer(
        () => this.fetchNotifications(),
        NOTIFICATIONS_POLLING_INTERVAL,
      );
    },
    getTransferFromCache(id: string): Transfer {
      const transfer = this.cachedTransfers[id];
      if (!transfer) {
        throw new Error('Transfer not found');
      }

      return transfer;
    },
    async fetchWalletBalances(): Promise<void> {
      try {
        const activeBank = useActiveBankStore();
        if (!activeBank.hasAccount || activeBank.wallets.items.length === 0) {
          return;
        }

        const requests: Promise<WalletBalance[]>[] = arrayBatchMaker(
          activeBank.wallets.items.map(wallet => wallet.id),
          BALANCE_FETCH_BATCH_QUANTITY,
        ).map(walletIds => activeBank.service.fetchWalletBalances({ wallet_ids: walletIds }));

        const balances = (await Promise.all(requests)).flat();
        for (const balance of balances) {
          activeBank.wallets.items.forEach(wallet => {
            if (wallet.id === balance.wallet_id) {
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
      } catch (error) {
        logger.error("Failed to fetch wallets' balances", { error });
      }
    },
    async fetchNotifications(): Promise<void> {
      try {
        const activeBank = useActiveBankStore();
        if (!activeBank.hasAccount) {
          return;
        }

        const newOperations = await activeBank.service.listUnreadPendingOperations(
          activeBank.lastPendingOperationDate ?? undefined,
          activeBank.lastPendingOperationId ?? undefined,
        );

        for (const newOperation of newOperations) {
          if (!activeBank.pendingOperations.items.find(current => current.data.id === newOperation.id)) {
            activeBank.pendingOperations.items.push({
              loading: false,
              data: newOperation,
            });
          }
        }
      } catch (error) {
        logger.error('Failed to fetch pending operations', { error });
      }
    },
  },
});
