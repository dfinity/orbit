import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import {
  User,
  WalletAsset,
  WalletFeatures,
  Proposal,
  ProposalId,
  NotificationId,
  Account,
  Notification,
} from '~/generated/wallet/wallet.did';
import { WalletService } from '~/services';
import { i18n, services } from '~/ui/modules';
import { useAuthStore, useSettingsStore, useWorkerStore } from '~/ui/stores';
import { LoadableItem } from '~/ui/types';

export interface WalletMetrics {
  accounts: number;
  transfers: {
    completed: number;
    pending: number;
  };
  notifications: number;
}

export interface PendingAccount {
  name: string;
  proposalId: string;
}

export interface ActiveWalletStoreState {
  _walletId: string;
  loading: boolean;
  _user: User | null;
  features: {
    loading: boolean;
    details: WalletFeatures | null;
  };
  accounts: {
    loading: boolean;
    items: Account[];
  };
  pendingAccounts: {
    loading: boolean;
    items: PendingAccount[];
  };
  notifications: {
    loading: boolean;
    items: LoadableItem<Notification>[];
  };
}

export const useActiveWalletStore = defineStore('activeWallet', {
  state: (): ActiveWalletStoreState => {
    return {
      _walletId: Principal.anonymous().toString(),
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
      pendingAccounts: {
        loading: false,
        items: [],
      },
      notifications: {
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
    lastNotificationDate(): Date | null {
      if (!this.notifications.items.length) {
        return null;
      }

      return new Date(this.notifications.items[0].data.created_at);
    },
    lastNotificationId(): ProposalId | null {
      if (!this.notifications.items.length) {
        return null;
      }

      return this.notifications.items[0].data.id;
    },
    sortedNotifications(): LoadableItem<Notification>[] {
      return this.notifications.items.sort((a, b) => {
        const firstDt = new Date(a.data.created_at);
        const secondDt = new Date(b.data.created_at);

        return secondDt.getTime() - firstDt.getTime();
      });
    },
    hasNotifications(): boolean {
      return this.notifications.items.length > 0;
    },
    walletId(): Principal {
      return Principal.fromText(this._walletId);
    },
    metrics(): WalletMetrics {
      return {
        accounts: this.accounts.items.length,
        transfers: {
          completed: 0,
          pending: 0,
        },
        notifications: this.notifications.items.length,
      };
    },
    supportedAssets(): WalletAsset[] {
      return this.features.details?.supported_assets ?? [];
    },
    service(): WalletService {
      return services().wallet.withWalletId(this.walletId);
    },
  },
  actions: {
    setWalletId(walletId: Principal): void {
      if (walletId !== this.walletId) {
        this._user = null;
      }

      this._walletId = walletId.toText();
    },
    reset(): void {
      this._walletId = Principal.anonymous().toText();
      this._user = null;
      this.accounts.items = [];
      this.features.details = null;
      this.notifications.items = [];
    },
    async registerUser(): Promise<User | null> {
      const auth = useAuthStore();
      const walletService = services().wallet.withWalletId(this.walletId);

      const hasMultipleIdentities = auth.identities.length > 1;
      if (!hasMultipleIdentities) {
        const user = await walletService.register({
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
    async markNotificationRead(notificationId: NotificationId, read: boolean): Promise<void> {
      const settings = useSettingsStore();
      const notification = this.notifications.items.find(item => item.data.id === notificationId);
      if (!notification) {
        return;
      }

      try {
        notification.loading = true;
        await this.service.markNotificationAsRead({
          notification_ids: [notificationId],
          read,
        });

        if (read) {
          this.notifications.items = this.notifications.items.filter(
            item => item.data.id !== notificationId,
          );
        }
      } catch (err) {
        logger.error(`Failed to save notification`, { err });

        settings.setNotification({
          show: true,
          type: 'error',
          message: i18n.global.t('wallets.notification_failed_to_save'),
        });
      } finally {
        notification.loading = false;
      }
    },
    async voteOnProposal(
      proposalId: ProposalId,
      decision: { approve: boolean; reason?: string },
    ): Promise<Proposal | null> {
      const settings = useSettingsStore();

      try {
        return await this.service.voteOnProposal({
          proposal_id: proposalId,
          approve: decision.approve,
          reason: decision.reason !== undefined ? [decision.reason] : [],
        });
      } catch (err) {
        logger.error(`Failed to save proposal`, { err });

        settings.setNotification({
          show: true,
          type: 'error',
          message: i18n.global.t('wallets.proposal_failed_to_save'),
        });
      }

      return null;
    },
    async loadAccountList(): Promise<void> {
      if (this.accounts.loading) {
        return;
      }
      try {
        this.accounts.loading = true;
        const walletService = services().wallet.withWalletId(this.walletId);
        this.accounts.items = await walletService.listAccounts();
      } finally {
        this.accounts.loading = false;
      }
    },
    async loadPendingAccountList(): Promise<void> {
      if (this.pendingAccounts.loading) {
        return;
      }
      try {
        this.pendingAccounts.loading = true;
        const walletService = services().wallet.withWalletId(this.walletId);
        const proposals = await walletService.listProposals({
          operation_type: [{ AddAccount: null }],
          status: [
            [{ Created: null }, { Adopted: null }, { Processing: null }, { Scheduled: null }],
          ],
          from_dt: [],
          to_dt: [],
        });

        this.pendingAccounts.items = proposals
          .map(proposal => {
            if ('AddAccount' in proposal.operation) {
              return {
                name: proposal.operation.AddAccount.name,
                proposalId: proposal.id,
              };
            }

            return null;
          })
          .filter(p => p !== null) as PendingAccount[];
      } finally {
        this.pendingAccounts.loading = false;
      }
    },
    async loadWalletFeatures(): Promise<void> {
      try {
        this.features.loading = true;
        const walletService = services().wallet.withWalletId(this.walletId);
        this.features.details = await walletService.features();
      } finally {
        this.features.loading = false;
      }
    },
    // these calls do not need to be awaited, it will be loaded in the background making the initial load faster
    async loadDetailsAsync(): Promise<void> {
      useWorkerStore().start();
      this.loadAccountList();
      this.loadPendingAccountList();
      this.loadWalletFeatures();
    },
    async load(walletId: Principal): Promise<void> {
      if (this.loading) {
        return;
      }
      useWorkerStore().stop();
      this.reset();
      this.loading = true;
      this.setWalletId(walletId);
      const walletService = services().wallet.withWalletId(this.walletId);
      const settings = useSettingsStore();
      try {
        const user = await walletService.myUser();
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
        logger.error(`Failed to load wallet user`, { err });

        settings.setNotification({
          show: true,
          type: 'error',
          message: i18n.global.t('wallets.user_load_error'),
        });
      } finally {
        this.loading = false;
      }
    },
  },
});
