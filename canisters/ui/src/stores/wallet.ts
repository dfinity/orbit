import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { logger } from '~/core/logger.core';
import {
  Account,
  Notification,
  Proposal,
  UUID,
  User,
  UserPrivilege,
  WalletAsset,
  Config,
} from '~/generated/wallet/wallet.did';
import { WalletService } from '~/services';
import { BlockchainStandard, BlockchainType } from '~/types';
import { i18n, services, startWalletWorkers, stopWalletWorkers } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';
import { LoadableItem } from '~/ui/types';
import { computedWalletName, redirectToWalletSettings } from '~/ui/utils';

export interface WalletMetrics {
  accounts: number;
  transfers: {
    completed: number;
    pending: number;
  };
  notifications: number;
}

export enum WalletConnectionStatus {
  Disconnected = 'disconnected',
  UnregisteredUser = 'unregistered-user',
  Connecting = 'connecting',
  Connected = 'connected',
  Failed = 'failed',
}

export interface WalletStoreState {
  connectionStatus: WalletConnectionStatus;
  canisterId: string;
  loading: boolean;
  user: User;
  privileges: UserPrivilege[];
  configuration: {
    loading: boolean;
    details: Config;
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

export const createUserInitialAccount = async (
  userId: UUID,
  wallet = useWalletStore(),
): Promise<void> => {
  await wallet.service.createProposal({
    title: [],
    summary: [],
    execution_plan: [{ Immediate: null }],
    operation: {
      AddAccount: {
        name: i18n.global.t('app.initial_account_name'),
        blockchain: BlockchainType.InternetComputer,
        standard: BlockchainStandard.Native,
        metadata: [],
        owners: [userId],
        policies: {
          edit: [],
          transfer: [],
        },
      },
    },
  });
};

const initialStoreState = (): WalletStoreState => {
  return {
    connectionStatus: WalletConnectionStatus.Disconnected,
    canisterId: Principal.anonymous().toText(),
    loading: false,
    user: {
      id: '',
      name: [],
      status: { Inactive: null },
      groups: [],
      last_modification_timestamp: '',
      identities: [],
    },
    privileges: [],
    configuration: {
      loading: false,
      details: {
        supported_assets: [],
      },
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
};

export const useWalletStore = defineStore('wallet', {
  state: (): WalletStoreState => initialStoreState(),
  getters: {
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
      return this.configuration.details?.supported_assets ?? [];
    },
    walletId(): Principal {
      return Principal.fromText(this.canisterId);
    },
    service(): WalletService {
      return services().wallet.withWalletId(this.walletId);
    },
    name(state): string {
      return computedWalletName({ canisterId: Principal.fromText(state.canisterId) });
    },
  },
  actions: {
    reset(): void {
      const initialState = initialStoreState();

      this.connectionStatus = initialState.connectionStatus;
      this.canisterId = initialState.canisterId;
      this.accounts = initialState.accounts;
      this.configuration = initialState.configuration;
      this.notifications = initialState.notifications;
      this.user = initialState.user;
      this.privileges = initialState.privileges;

      stopWalletWorkers();
    },
    async connectTo(walletId: Principal): Promise<WalletConnectionStatus> {
      const app = useAppStore();

      try {
        if (this.loading) {
          logger.warn(`Wallet is already loading`);
          return this.connectionStatus;
        }

        // reset the store to the initial state before connecting to a new wallet, this makes sure that
        // the store is in a consistent state and that the user is not seeing any stale data
        this.reset();

        this.loading = true;
        this.connectionStatus = WalletConnectionStatus.Connecting;
        this.canisterId = walletId.toText();
        const myUser = await this.service.myUser();
        if (!myUser) {
          logger.warn(`User not registered in the selected wallet`);
          this.connectionStatus = WalletConnectionStatus.UnregisteredUser;
          return this.connectionStatus;
        }

        this.user = myUser.me;
        this.privileges = myUser.privileges;

        // these calls do not need to be awaited, it will be loaded in the background making the initial load faster
        this.loadAccountList();
        this.loadConfiguration();

        startWalletWorkers(walletId);

        this.connectionStatus = WalletConnectionStatus.Connected;
      } catch (err) {
        logger.error(`Failed to connect to wallet`, { err });
        this.connectionStatus = WalletConnectionStatus.Failed;

        app.sendNotification({
          type: 'error',
          message: i18n.global.t('wallets.user_load_error'),
        });

        redirectToWalletSettings();
      } finally {
        this.loading = false;
      }

      return this.connectionStatus;
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
        // todo: add pagination support
        this.accounts.items = (await this.service.listAccounts()).accounts;
      } finally {
        this.accounts.loading = false;
      }
    },
    async loadConfiguration(): Promise<void> {
      try {
        this.configuration.loading = true;
        this.configuration.details = await this.service.config();
      } finally {
        this.configuration.loading = false;
      }
    },
  },
});
