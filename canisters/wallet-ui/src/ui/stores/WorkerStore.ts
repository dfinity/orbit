import { defineStore } from 'pinia';
import { logger, timer } from '~/core';
import { Transfer, Wallet } from '~/generated/bank/bank.did';
import { WalletApiFactory } from '~/services';
import { WalletApi } from '~/types/Wallet';
import { useActiveBankStore } from '~/ui/stores';

const BALANCE_POLLING_INTERVAL = 30000;
const NOTIFICATIONS_POLLING_INTERVAL = 5000;

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
        if (!activeBank.hasUser || activeBank.wallets.items.length === 0) {
          return;
        }

        const walletApis: { wallet: Wallet; api: WalletApi }[] = activeBank.wallets.items
          .map(wallet => {
            try {
              const api = WalletApiFactory.create(wallet);
              return { wallet, api };
            } catch (e) {
              logger.warn('Wallet api not supported for wallet', { error: e, wallet: wallet.id });
            }

            return null;
          })
          .filter(entry => entry !== null) as { wallet: Wallet; api: WalletApi }[];

        const requests = walletApis.map(async ({ wallet, api }) =>
          api.fetchBalance().then(balance => ({ wallet, balance })),
        );

        const balances = (await Promise.all(requests)).flat();
        for (const { wallet, balance } of balances) {
          wallet.balance = [
            {
              balance,
              decimals: wallet.decimals,
              last_update_timestamp: new Date().toISOString(),
            },
          ];
        }
      } catch (error) {
        logger.error("Failed to fetch wallets' balances", { error });
      }
    },
    async fetchNotifications(): Promise<void> {
      try {
        const activeBank = useActiveBankStore();
        if (!activeBank.hasUser) {
          return;
        }

        const newOperations = await activeBank.service.listUnreadPendingOperations(
          activeBank.lastPendingOperationDate ?? undefined,
          activeBank.lastPendingOperationId ?? undefined,
        );

        for (const newOperation of newOperations) {
          if (
            !activeBank.pendingOperations.items.find(current => current.data.id === newOperation.id)
          ) {
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
