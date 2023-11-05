import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import {
  User,
  BankAsset,
  BankFeatures,
  Proposal,
  ProposalId,
  Account,
} from '~/generated/bank/bank.did';
import { BankService } from '~/services';
import { i18n, services } from '~/ui/modules';
import { useAuthStore, useSettingsStore, useWorkerStore } from '~/ui/stores';
import { LoadableItem } from '~/ui/types';

export interface BankMetrics {
  accounts: number;
  transfers: {
    completed: number;
    pending: number;
  };
  pendingProposals: number;
}

export interface ActiveBankStoreState {
  _bankId: string;
  loading: boolean;
  _user: User | null;
  features: {
    loading: boolean;
    details: BankFeatures | null;
  };
  accounts: {
    loading: boolean;
    items: Account[];
  };
  pendingProposals: {
    loading: boolean;
    items: LoadableItem<Proposal>[];
  };
}

export const useActiveBankStore = defineStore('activeBank', {
  state: (): ActiveBankStoreState => {
    return {
      _bankId: Principal.anonymous().toString(),
      loading: false,
      _user: null,
      features: {
        loading: false,
        details: null,
      },
      accounts: {
        loading: false,
        items: [],
      },
      pendingProposals: {
        loading: false,
        items: [],
      },
    };
  },
  getters: {
    hasUser(): boolean {
      return !!this._user;
    },
    user(): User {
      if (!this._user) {
        throw new Error('User not loaded');
      }

      return this._user as User;
    },
    sortedAccounts(): Account[] {
      return this.accounts.items.sort((a, b) => {
        const firstDt = new Date(a.last_modification_timestamp).getTime();
        const secondDt = new Date(b.last_modification_timestamp).getTime();

        return secondDt - firstDt;
      });
    },
    lastPendingProposalDate(): Date | null {
      if (!this.pendingProposals.items.length) {
        return null;
      }

      return new Date(this.pendingProposals.items[0].data.created_at);
    },
    lastPendingProposalId(): ProposalId | null {
      if (!this.pendingProposals.items.length) {
        return null;
      }

      return this.pendingProposals.items[0].data.id;
    },
    sortedPendingProposals(): LoadableItem<Proposal>[] {
      return this.pendingProposals.items.sort((a, b) => {
        const firstDt = new Date(a.data.created_at);
        const secondDt = new Date(b.data.created_at);

        return secondDt.getTime() - firstDt.getTime();
      });
    },
    hasPendingProposals(): boolean {
      return this.pendingProposals.items.length > 0;
    },
    bankId(): Principal {
      return Principal.fromText(this._bankId);
    },
    metrics(): BankMetrics {
      return {
        accounts: this.accounts.items.length,
        transfers: {
          completed: 0,
          pending: 0,
        },
        pendingProposals: this.pendingProposals.items.length,
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
        this._user = null;
      }

      this._bankId = bankId.toText();
    },
    reset(): void {
      this._bankId = Principal.anonymous().toText();
      this._user = null;
      this.accounts.items = [];
      this.features.details = null;
      this.pendingProposals.items = [];
    },
    async registerUser(): Promise<User | null> {
      const auth = useAuthStore();
      const bankService = services().bank.withBankId(this.bankId);

      const hasMultipleIdentities = auth.identities.length > 1;
      if (!hasMultipleIdentities) {
        const user = await bankService.register({
          identities: auth.identities,
        });

        return user;
      }

      return this.registerWithMultiIdentityFlow();
    },
    async registerWithMultiIdentityFlow(): Promise<User | null> {
      // TODO: implement multi identity register flow

      return null;
    },
    async saveDecision(
      proposalId: ProposalId,
      decision: { approve?: boolean; reason?: string; read?: boolean },
    ): Promise<Proposal | null> {
      const settings = useSettingsStore();
      const pendingProposal = this.pendingProposals.items.find(item => item.data.id === proposalId);
      if (pendingProposal) {
        pendingProposal.loading = true;
      }

      try {
        return await this.service
          .voteOnProposal({
            proposal_id: proposalId,
            approve: decision.approve !== undefined ? [decision.approve] : [],
            read: decision.read !== undefined ? [decision.read] : [],
            reason: decision.reason !== undefined ? [decision.reason] : [],
          })
          .then(proposal => {
            this.pendingProposals.items = this.pendingProposals.items.filter(item => {
              if (item.data.id !== proposal.id) {
                return true;
              }
              const isPending = 'Pending' in proposal.status;
              const isRead = proposal.votes.some(
                vote => vote.user_id === this.user.id && vote.read,
              );

              return isPending && !isRead;
            });

            return proposal;
          });
      } catch (err) {
        logger.error(`Failed to save proposal`, { err });

        settings.setNotification({
          show: true,
          type: 'error',
          message: i18n.global.t('banks.proposal_failed_to_save'),
        });
      } finally {
        if (pendingProposal) {
          pendingProposal.loading = false;
        }
      }

      return null;
    },
    async loadAccountList(): Promise<void> {
      if (this.accounts.loading) {
        return;
      }
      try {
        this.accounts.loading = true;
        const bankService = services().bank.withBankId(this.bankId);
        this.accounts.items = await bankService.listAccounts();
      } finally {
        this.accounts.loading = false;
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
      this.loadAccountList();
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
        const user = await bankService.myUser();
        if (user) {
          this._user = user;
          this.loadDetailsAsync();
          return;
        }

        const registeredUser = await this.registerUser();

        this._user = registeredUser;

        if (registeredUser) {
          this.loadDetailsAsync();
        }
      } catch (err) {
        logger.error(`Failed to load bank user`, { err });

        settings.setNotification({
          show: true,
          type: 'error',
          message: i18n.global.t('banks.user_load_error'),
        });
      } finally {
        this.loading = false;
      }
    },
  },
});
