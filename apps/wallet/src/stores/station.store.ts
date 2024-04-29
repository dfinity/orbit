import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { STATION_ID_QUERY_PARAM } from '~/core/constants.core';
import { InvalidStationError, UnregisteredUserError } from '~/core/errors.core';
import { logger } from '~/core/logger.core';
import {
  Capabilities,
  Notification,
  UUID,
  User,
  UserPrivilege,
  Asset,
} from '~/generated/station/station.did';
import { i18n } from '~/plugins/i18n.plugin';
import { router } from '~/plugins/router.plugin';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { useAppStore } from '~/stores/app.store';
import { BlockchainStandard, BlockchainType } from '~/types/chain.types';
import { LoadableItem } from '~/types/helper.types';
import { computedStationName, isApiError } from '~/utils/app.utils';
import { arrayBatchMaker } from '~/utils/helper.utils';
import { accountsWorker, startWorkers, stopWorkers } from '~/workers';

export enum ConnectionStatus {
  Disconnected = 'disconnected',
  UnregisteredUser = 'unregistered-user',
  Connecting = 'connecting',
  Connected = 'connected',
  Failed = 'failed',
}

export enum ConnectionError {
  NOT_FOUND_USER_IDENTITY = 'not_found_user_identity',
  CANISTER_ERROR = 'canister_error',
  OTHER_ERROR = 'other_error',
}

export interface StationStoreState {
  connectionStatus: ConnectionStatus;
  connectionError?: ConnectionError;
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
  station = useStationStore(),
): Promise<void> => {
  await station.service.createRequest({
    title: [],
    summary: [],
    execution_plan: [{ Immediate: null }],
    operation: {
      AddAccount: {
        name: i18n.global.t('app.initial_account_name'),
        blockchain: BlockchainType.InternetComputer,
        standard: BlockchainStandard.Native,
        metadata: [],
        read_permission: { auth_scope: { Restricted: null }, user_groups: [], users: [userId] },
        transfer_permission: {
          auth_scope: { Restricted: null },
          user_groups: [],
          users: [userId],
        },
        configs_permission: {
          auth_scope: { Restricted: null },
          user_groups: [],
          users: [userId],
        },
        configs_request_policy: [{ Quorum: { min_approved: 100, approvers: { Owner: null } } }],
        transfer_request_policy: [{ Quorum: { min_approved: 100, approvers: { Owner: null } } }],
      },
    },
  });
};

const initialStoreState = (): StationStoreState => {
  return {
    connectionStatus: ConnectionStatus.Disconnected,
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

export const useStationStore = defineStore('station', {
  state: (): StationStoreState => initialStoreState(),
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
    supportedAssets(): Asset[] {
      return this.configuration.details?.supported_assets ?? [];
    },
    stationId(): Principal {
      return Principal.fromText(this.canisterId);
    },
    service(): StationService {
      return services().station.withStationId(this.stationId);
    },
    name(state): string {
      return computedStationName({ canisterId: Principal.fromText(state.canisterId) });
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

      stopWorkers();
    },
    async connectTo(stationId: Principal, onConnectedReload = true): Promise<ConnectionStatus> {
      const app = useAppStore();

      try {
        if (this.loading) {
          logger.warn(`Station is already loading`);
          return this.connectionStatus;
        }

        app.disableBackgroundPolling = true;
        stopWorkers();

        this.loading = true;
        this.connectionStatus = ConnectionStatus.Connecting;

        if (stationId.toText() === Principal.anonymous().toText()) {
          throw new InvalidStationError();
        }

        const stationService = await services().station.withStationId(stationId);
        const myUser = await stationService.myUser();
        if (!myUser) {
          throw new UnregisteredUserError();
        }

        this.user = myUser.me;
        this.privileges = myUser.privileges;

        // loads the capabilities of the station
        this.configuration.details = await stationService.capabilities();

        startWorkers(stationId);

        this.connectionStatus = ConnectionStatus.Connected;
      } catch (err) {
        logger.error(`Failed to connect to station`, { err });

        let connectionStatus = ConnectionStatus.Failed;
        if (err instanceof InvalidStationError) {
          connectionStatus = ConnectionStatus.Disconnected;
        } else if (err instanceof UnregisteredUserError) {
          connectionStatus = ConnectionStatus.UnregisteredUser;
        }

        if (isApiError(err)) {
          switch (err.code) {
            case 'NOT_FOUND_USER_IDENTITY':
              this.connectionError = ConnectionError.NOT_FOUND_USER_IDENTITY;
              break;
            default:
              this.connectionError = ConnectionError.OTHER_ERROR;

              app.sendNotification({
                type: 'error',
                message: i18n.global.t('stations.user_load_error'),
              });
              break;
          }
        } else {
          this.connectionError = ConnectionError.CANISTER_ERROR;

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
        // if the id has changed, force a navigation to re-run the route guards
        if (onConnectedReload && this.canisterId.length && this.canisterId !== stationId.toText()) {
          router.push({
            path: window.location.pathname,
            query: { [STATION_ID_QUERY_PARAM]: stationId.toText() },
          });
        }
        this.canisterId = stationId.toText();
        this.loading = false;
        app.disableBackgroundPolling = false;
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
          message: i18n.global.t('stations.notification_failed_to_save'),
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
          message: i18n.global.t('stations.notification_failed_to_save'),
        });
      } finally {
        notification.loading = false;
      }
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
