import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { InvalidWalletError, UnregisteredUserError } from '~/core/errors.core';
import { logger } from '~/core/logger.core';
import {
  Capabilities,
  Notification,
  Proposal,
  UUID,
  User,
  UserPrivilege,
  WalletAsset,
} from '~/generated/wallet/wallet.did';
import { i18n } from '~/plugins/i18n.plugin';
import { router } from '~/plugins/router.plugin';
import { services } from '~/plugins/services.plugin';
import { WalletService } from '~/services/wallet.service';
import { useAppStore } from '~/stores/app.store';
import { BlockchainStandard, BlockchainType } from '~/types/chain.types';
import { LoadableItem } from '~/types/helper.types';
import { computedWalletName, isApiError } from '~/utils/app.utils';
import { arrayBatchMaker } from '~/utils/helper.utils';
import { accountsWorker, startWalletWorkers, stopWalletWorkers } from '~/workers';

export enum WalletConnectionStatus {
  Disconnected = 'disconnected',
  UnregisteredUser = 'unregistered-user',
  Connecting = 'connecting',
  Connected = 'connected',
  Failed = 'failed',
}

export enum WalletConnectionError {
  NOT_FOUND_USER_IDENTITY = 'not_found_user_identity',
  OTHER_WALLET_ERROR = 'other_wallet_error',
  CANISTER_ERROR = 'canister_error',
}

export interface WalletStoreState {
  connectionStatus: WalletConnectionStatus;

  connectionError?: WalletConnectionError;
  connectionErrorMessage?: string;

  canisterId: string;
  loading: boolean;
  user: User;
  privileges: UserPrivilege[];
  configuration: {
    loading: boolean;
    details: Capabilities;
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
        read_access_policy: { auth_scope: { Restricted: null }, user_groups: [], users: [userId] },
        transfer_access_policy: {
          auth_scope: { Restricted: null },
          user_groups: [],
          users: [userId],
        },
        update_access_policy: {
          auth_scope: { Restricted: null },
          user_groups: [],
          users: [userId],
        },
        update_approval_policy: [
          { ApprovalThreshold: { threshold: 100, voters: { Owner: null } } },
        ],
        transfer_approval_policy: [
          { ApprovalThreshold: { threshold: 100, voters: { Owner: null } } },
        ],
      },
    },
  });
};

const initialStoreState = (): WalletStoreState => {
  return {
    connectionStatus: WalletConnectionStatus.Disconnected,
    connectionError: undefined,
    connectionErrorMessage: undefined,
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
        version: '',
        supported_assets: [],
      },
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
    onDisconnected(): void {
      const initialState = initialStoreState();

      this.canisterId = initialState.canisterId;
      this.configuration = initialState.configuration;
      this.notifications = initialState.notifications;
      this.user = initialState.user;
      this.privileges = initialState.privileges;

      stopWalletWorkers();
    },
    async connectTo(
      walletId: Principal,
      onConnectedReload = true,
    ): Promise<WalletConnectionStatus> {
      const app = useAppStore();

      try {
        if (this.loading) {
          logger.warn(`Wallet is already loading`);
          return this.connectionStatus;
        }

        stopWalletWorkers();

        this.loading = true;
        this.connectionStatus = WalletConnectionStatus.Connecting;

        if (walletId.toText() === Principal.anonymous().toText()) {
          throw new InvalidWalletError();
        }

        const walletService = await services().wallet.withWalletId(walletId);
        const myUser = await walletService.myUser();
        if (!myUser) {
          throw new UnregisteredUserError();
        }

        this.user = myUser.me;
        this.privileges = myUser.privileges;

        // loads the capabilities of the wallet
        this.configuration.details = await walletService.capabilities();

        startWalletWorkers(walletId);

        this.connectionStatus = WalletConnectionStatus.Connected;
      } catch (err) {
        logger.error(`Failed to connect to wallet`, { err });

        let connectionStatus = WalletConnectionStatus.Failed;
        if (err instanceof InvalidWalletError) {
          connectionStatus = WalletConnectionStatus.Disconnected;
        } else if (err instanceof UnregisteredUserError) {
          connectionStatus = WalletConnectionStatus.UnregisteredUser;
        }

        if (isApiError(err)) {
          switch (err.code) {
            case 'NOT_FOUND_USER_IDENTITY':
              this.connectionError = WalletConnectionError.NOT_FOUND_USER_IDENTITY;
              break;
            default:
              this.connectionError = WalletConnectionError.OTHER_WALLET_ERROR;

              app.sendNotification({
                type: 'error',
                message: i18n.global.t('wallets.user_load_error'),
              });
              break;
          }
        } else {
          this.connectionError = WalletConnectionError.CANISTER_ERROR;

          if (err instanceof Error) {
            this.connectionErrorMessage = err.message;

            app.sendNotification({
              type: 'error',
              message: i18n.global.t('app.session_load_error'),
            });
          }
        }

        this.onDisconnected();
        this.connectionStatus = connectionStatus;
      } finally {
        // if the wallet id has changed, force a navigation to re-run the route guards
        if (onConnectedReload && this.canisterId.length && this.canisterId !== walletId.toText()) {
          router.push({
            path: window.location.pathname,
            query: { walletId: walletId.toText() },
          });
        }
        this.canisterId = walletId.toText();
        this.loading = false;
      }

      return this.connectionStatus;
    },
    async markAllNotificationsRead(): Promise<void> {
      const app = useAppStore();

      try {
        this.notifications.loading = true;
        const notificationIds = this.notifications.items.map(item => item.data.id);
        for (const ids of arrayBatchMaker(notificationIds, 50)) {
          this.notifications.items = this.notifications.items.map(item => {
            item.loading = true;
            return item;
          });

          await this.service.markNotificationAsRead({ notification_ids: ids, read: true });

          this.notifications.items = this.notifications.items.filter(
            item => !ids.includes(item.data.id),
          );
        }
      } catch (err) {
        logger.error(`Failed to mark all notifications as read`, { err });

        app.sendNotification({
          type: 'error',
          message: i18n.global.t('wallets.notification_failed_to_save'),
        });
      } finally {
        this.notifications.loading = false;
      }
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
    trackAccountsBalance(accountIds: UUID[]): void {
      accountsWorker?.postMessage({
        type: 'track',
        data: {
          accountIds,
        },
      });
    },
  },
});
