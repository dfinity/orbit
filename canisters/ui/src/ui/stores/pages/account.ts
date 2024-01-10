import { defineStore } from 'pinia';
import { endOfDay, logger, startOfDay } from '~/core';
import {
  Error as ApiError,
  Proposal,
  ProposalStatusCode,
  TransferListItem,
  Account,
  UUID,
} from '~/generated/wallet/wallet.did';
import { WalletService, ChainApiFactory } from '~/services';
import { ChainApi, AccountIncomingTransfer } from '~/types';
import { i18n } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';
import { useWalletStore } from '~/ui/stores/wallet';
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
  proposals: {
    loading: boolean;
    items: LoadableItem<Proposal>[];
    fromDt: string | null;
    toDt: string | null;
  };
  deposits: {
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
  proposals: {
    loading: false,
    items: [],
    fromDt: null,
    toDt: null,
  },
  deposits: {
    loading: false,
    items: [],
  },
};

export const useAccountPageStore = defineStore('accountPage', {
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
    sortedDeposits(): AccountIncomingTransfer[] {
      return this.deposits.items.sort((a, b) => {
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
    sortedProposals(): LoadableItem<Proposal>[] {
      return this.proposals.items.sort((a, b) => {
        return new Date(b.data.created_at).getTime() - new Date(a.data.created_at).getTime();
      }) as LoadableItem<Proposal>[];
    },
    defaultEndDt(): string {
      return new Date().toISOString();
    },
    hasLoaded(): boolean {
      return this._account !== null;
    },
    walletService(): WalletService {
      return useWalletStore().service;
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
      this.proposals = reset.proposals;
      this.proposals.fromDt = new Date(this.defaultStartDt).toISOString().split('T')[0];
      this.proposals.toDt = new Date(this.defaultEndDt).toISOString().split('T')[0];
      this.deposits = reset.deposits;
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
    async loadDeposits(): Promise<void> {
      if (!this.chainApi) {
        return;
      }

      const transfers = await this.chainApi.fetchTransfers({
        from_dt: new Date(),
      });

      this.deposits.items = transfers;
    },
    async voteOnProposal(
      proposalId: UUID,
      decision: { approve: boolean; reason?: string },
    ): Promise<void> {
      const activeWallet = useWalletStore();
      const item = this.proposals.items.find(item => item.data.id === proposalId);
      if (!item) {
        logger.warn('Decision not saved, proposal not found', { proposalId });
        return;
      }

      item.loading = true;
      const proposal = await activeWallet
        .voteOnProposal(proposalId, decision)
        .finally(() => (item.loading = false));

      if (!proposal) {
        return;
      }

      this.proposals.items.forEach(item => {
        if (item.data.id === proposal.id) {
          item.data = proposal;
        }
      });
    },
    async loadProposals(fromDt?: Date, toDt?: Date, status?: ProposalStatusCode): Promise<void> {
      try {
        this.proposals.loading = true;
        this.proposals.items = await this.walletService
          .listAccountProposals({
            account_id: this.account.id,
            status: status ? [[status]] : [],
            from_dt: fromDt ? [startOfDay(fromDt).toISOString()] : [],
            to_dt: toDt ? [endOfDay(toDt).toISOString()] : [],
            operation_type: [],
          })
          .then(proposals => {
            return proposals.map(proposal => {
              return {
                loading: false,
                data: proposal,
              };
            });
          });
      } catch (e) {
        logger.error('Failed to load withdraw requests', { e });
        const app = useAppStore();
        this.proposals.items = [];

        app.sendNotification({
          message: i18n.global.t('wallets.load_error_withdraw_requests'),
          type: 'error',
        });
      } finally {
        this.proposals.loading = false;
      }
    },
    async loadSentTransfers(fromDt?: Date, toDt?: Date, status?: string): Promise<void> {
      try {
        this.transfers.loading = true;
        this.transfers.items = await this.walletService.listAccountTransfers({
          account_id: this.account.id,
          from_dt: fromDt ? [startOfDay(fromDt).toISOString()] : [],
          to_dt: toDt ? [endOfDay(toDt).toISOString()] : [],
          status: status ? [status] : [],
        });
      } catch (e) {
        logger.error('Failed to load transfers', { e });
        const app = useAppStore();
        this.transfers.items = [];

        app.sendNotification({
          message: i18n.global.t('wallets.load_error_withdrawal'),
          type: 'error',
        });
      } finally {
        this.transfers.loading = false;
      }
    },
    async load(accountId: UUID): Promise<void> {
      try {
        this.reset();
        this.loading = true;
        const activeWallet = useWalletStore();

        this._account = await this.walletService.getAccount({
          account_id: accountId,
        });

        const updatedBalance = activeWallet.accounts.items.find(item => item.id === accountId)
          ?.balance;
        if (updatedBalance) {
          this._account.balance = updatedBalance;
        }

        this.loadSentTransfers(new Date(this.defaultStartDt), new Date(this.defaultEndDt));
        this.loadProposals(new Date(this.defaultStartDt), new Date(this.defaultEndDt));
        this.loadDeposits();
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
