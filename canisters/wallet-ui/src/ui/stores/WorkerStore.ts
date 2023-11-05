import { defineStore } from 'pinia';
import { logger, timer } from '~/core';
import { Transfer, Account } from '~/generated/bank/bank.did';
import { ChainApiFactory } from '~/services';
import { ChainApi } from '~/types/Chain';
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
      this.pollingJobs.balances = timer(
        () => this.fetchAccountBalances(),
        BALANCE_POLLING_INTERVAL,
      );
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
    async fetchAccountBalances(): Promise<void> {
      try {
        const activeBank = useActiveBankStore();
        if (!activeBank.hasUser || activeBank.accounts.items.length === 0) {
          return;
        }

        const accountApis: { account: Account; api: ChainApi }[] = activeBank.accounts.items
          .map(account => {
            try {
              const api = ChainApiFactory.create(account);
              return { account, api };
            } catch (e) {
              logger.warn('Chain api not supported for account', { error: e, account: account.id });
            }

            return null;
          })
          .filter(entry => entry !== null) as { account: Account; api: ChainApi }[];

        const requests = accountApis.map(async ({ account, api }) =>
          api.fetchBalance().then(balance => ({ account, balance })),
        );

        const balances = (await Promise.all(requests)).flat();
        for (const { account, balance } of balances) {
          account.balance = [
            {
              balance,
              decimals: account.decimals,
              last_update_timestamp: new Date().toISOString(),
            },
          ];
        }
      } catch (error) {
        logger.error("Failed to fetch accounts' balances", { error });
      }
    },
    async fetchNotifications(): Promise<void> {
      try {
        const activeBank = useActiveBankStore();
        if (!activeBank.hasUser) {
          return;
        }

        const newProposals = await activeBank.service.listUnreadPendingProposals(
          activeBank.lastPendingProposalDate ?? undefined,
          activeBank.lastPendingProposalId ?? undefined,
        );

        for (const newProposal of newProposals) {
          if (
            !activeBank.pendingProposals.items.find(current => current.data.id === newProposal.id)
          ) {
            activeBank.pendingProposals.items.push({
              loading: false,
              data: newProposal,
            });
          }
        }
      } catch (error) {
        logger.error('Failed to fetch pending proposals', { error });
      }
    },
  },
});
