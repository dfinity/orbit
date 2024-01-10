import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core';
import {
  Account,
  Notification,
  Proposal,
  UUID,
  WalletAsset,
  WalletFeatures,
} from '~/generated/wallet/wallet.did';
import { WalletService } from '~/services';
import { AuthenticatedUser } from '~/types';
import { i18n, services } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';
import { LoadableItem } from '~/ui/types';
import { accountsWorker, notificationsWorker } from '~/workers';

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

export interface WalletStoreState {
  canisterId: string | null;
  loading: boolean;
  name: string | null;
  user: AuthenticatedUser | null;
  features: {
    loading: boolean;
    details: WalletFeatures | null;
  };
  accounts: {
    loading: boolean;
    items: Account[];
  };
  notifications: {
    loading: boolean;
    items: LoadableItem<Notification>[];
  };
}

export const useWalletStore = defineStore('wallet', {
  state: (): WalletStoreState => {
    return {
      canisterId: null,
      loading: false,
      name: null,
      user: null,
      features: {
        loading: false,
        details: null,
      },
      accounts: {
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
      return !!this.user;
    },
    currentUser(): AuthenticatedUser {
      if (!this.user) {
        throw new Error('User not loaded');
      }

      return this.user as AuthenticatedUser;
    },
    userDisplayName(): string | null {
      return this.currentUser.me.name?.[0] ?? null;
    },
    sortedAccounts(): Account[] {
      return this.accounts.items.sort((a, b) => {
        const firstDt = new Date(a.last_modification_timestamp).getTime();
        const secondDt = new Date(b.last_modification_timestamp).getTime();

        return secondDt - firstDt;
      });
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
    activeCanisterId(): Principal {
      if (!this.canisterId) {
        throw new Error('Wallet canister not selected');
      }

      return Principal.fromText(this.canisterId);
    },
    service(): WalletService {
      return services().wallet.withWalletId(this.activeCanisterId);
    },
  },
  actions: {
    reset(): void {
      this.canisterId = null;
      this.user = null;
      this.name = null;
      this.accounts.items = [];
      this.features.details = null;
      this.notifications.items = [];

      accountsWorker?.postMessage({
        type: 'stop',
      });

      notificationsWorker?.postMessage({
        type: 'stop',
      });
    },
    async markNotificationRead(notificationId: UUID, read: boolean): Promise<void> {
      const app = useAppStore();
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

        app.sendNotification({
          type: 'error',
          message: i18n.global.t('wallets.notification_failed_to_save'),
        });
      } finally {
        notification.loading = false;
      }
    },
    async voteOnProposal(
      proposalId: UUID,
      decision: { approve: boolean; reason?: string },
    ): Promise<Proposal | null> {
      const app = useAppStore();

      try {
        return await this.service.voteOnProposal({
          proposal_id: proposalId,
          approve: decision.approve,
          reason: decision.reason !== undefined ? [decision.reason] : [],
        });
      } catch (err) {
        logger.error(`Failed to save proposal`, { err });

        app.sendNotification({
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
        this.accounts.items = await this.service.listAccounts();
      } finally {
        this.accounts.loading = false;
      }
    },
    async loadWalletFeatures(): Promise<void> {
      try {
        this.features.loading = true;
        this.features.details = await this.service.features();
      } finally {
        this.features.loading = false;
      }
    },
    async load(walletId: Principal, name: string): Promise<void> {
      const app = useAppStore();

      try {
        if (this.loading) {
          logger.warn(`Wallet is already loading`);
          return;
        }

        this.name = name;
        this.loading = true;
        this.canisterId = walletId.toText();
        const user = await this.service.myUser();
        if (!user) {
          logger.warn(`User not registered in the selected wallet`);
          return;
        }

        this.user = user;

        // these calls do not need to be awaited, it will be loaded in the background making the initial load faster
        this.loadAccountList();
        this.loadWalletFeatures();

        accountsWorker?.postMessage({
          type: 'start',
          data: {
            walletId,
          },
        });

        notificationsWorker?.postMessage({
          type: 'start',
          data: {
            walletId,
          },
        });
      } catch (err) {
        logger.error(`Failed to load user wallet`, { err });

        app.sendNotification({
          type: 'error',
          message: i18n.global.t('wallets.user_load_error'),
        });
      } finally {
        this.loading = false;
      }
    },
  },
});
