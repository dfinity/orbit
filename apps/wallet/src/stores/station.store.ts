import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { appInitConfig } from '~/configs/init.config';
import { createCompatibilityLayer, fetchCanisterVersion } from '~/core/compatibility.core';
import { STATION_ID_QUERY_PARAM } from '~/core/constants.core';
import { InvalidStationError, UnregisteredUserError } from '~/core/errors.core';
import { icAgent } from '~/core/ic-agent.core';
import { logger } from '~/core/logger.core';
import {
  Asset,
  Capabilities,
  CycleObtainStrategy,
  Notification,
  UUID,
  User,
  UserPrivilege,
} from '~/generated/station/station.did';
import { i18n } from '~/plugins/i18n.plugin';
import { router } from '~/plugins/router.plugin';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { useAppStore } from '~/stores/app.store';
import { Privilege } from '~/types/auth.types';
import { BlockchainStandard, BlockchainType } from '~/types/chain.types';
import { LoadableItem } from '~/types/helper.types';
import { computedStationName, isApiError, popRedirectToLocation } from '~/utils/app.utils';
import { hasRequiredPrivilege } from '~/utils/auth.utils';
import { arrayBatchMaker, removeBasePathFromPathname, variantIs } from '~/utils/helper.utils';
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
    cycleObtainStrategy: CycleObtainStrategy;
  };
  notifications: {
    loading: boolean;
    items: LoadableItem<Notification>[];
  };
  versionManagement: {
    loading: boolean;
    updateRequested?: string;
    stationVersion?: string;
    upgraderVersion?: string;
    nextStationVersion?: string;
    nextUpgraderVersion?: string;
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
    expiration_dt: [],
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
        configs_request_policy: [{ Quorum: { min_approved: 1, approvers: { Id: [userId] } } }],
        transfer_request_policy: [{ Quorum: { min_approved: 1, approvers: { Id: [userId] } } }],
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
      name: '',
      status: { Inactive: null },
      groups: [],
      last_modification_timestamp: '',
      identities: [],
    },
    privileges: [],
    configuration: {
      loading: false,
      details: {
        name: '',
        version: '',
        supported_assets: [],
      },
      cycleObtainStrategy: { Disabled: null },
    },
    notifications: {
      loading: false,
      items: [],
    },
    versionManagement: {
      loading: false,
      updateRequested: undefined,
      nextStationVersion: undefined,
      nextUpgraderVersion: undefined,
      stationVersion: undefined,
      upgraderVersion: undefined,
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
    hasNewVersion(): boolean {
      return !!(
        this.versionManagement.nextStationVersion || this.versionManagement.nextUpgraderVersion
      );
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
      this.versionManagement = initialState.versionManagement;

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

        const compat = await createCompatibilityLayer()
          .checkCompatibility(stationId, {
            redirectIfIncompatible: true,
          })
          .catch(err => {
            logger.error(`Failed to check station compatibility`, { err });

            return false;
          });

        // If the compatibility check fails, we warn the user but still allow them to proceed
        // to enable them a chance to use the app with the station, even if it's not fully compatible.
        if (compat === false) {
          app.sendNotification({
            type: 'warning',
            message: i18n.global.t('app.api_compatibility_error'),
          });
        }

        const stationService = services().station.withStationId(stationId);
        const myUser = await stationService.myUser();
        if (!myUser) {
          throw new UnregisteredUserError();
        }

        this.user = myUser.me;
        this.privileges = myUser.privileges;

        if (hasRequiredPrivilege({ anyOf: [Privilege.SystemInfo] })) {
          const systemInfo = await stationService.systemInfo();
          this.configuration.cycleObtainStrategy = systemInfo.system.cycle_obtain_strategy;
        }

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
          // window.location is used because the router is not fully initialized yet in the first load
          const redirectTo = popRedirectToLocation();
          const url = new URL(
            redirectTo ? window.location.origin + redirectTo : window.location.href,
          );
          url.pathname = removeBasePathFromPathname(url.pathname, appInitConfig.versionedBaseUrl);
          url.searchParams.set(STATION_ID_QUERY_PARAM, stationId.toText());

          router.push({
            path: url.pathname,
            query: Array.from(url.searchParams.entries()).reduce(
              (acc: Record<string, string[]>, [key, value]) => {
                if (!acc[key]) {
                  acc[key] = [];
                }

                acc[key].push(value);
                return acc;
              },
              {},
            ),
            hash: url.hash,
          });
        }
        this.canisterId = stationId.toText();
        this.loading = false;
        app.disableBackgroundPolling = false;

        // async check for version updates after the station is connected
        this.checkVersionUpdates();
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
    async loadStationVersion(): Promise<string> {
      const stationVersion = await fetchCanisterVersion(icAgent.get(), this.stationId);

      this.versionManagement.stationVersion = stationVersion;

      return stationVersion;
    },
    async loadUpgraderVersion(): Promise<string> {
      const { system } = await this.service.systemInfo();
      const upgraderVersion = await fetchCanisterVersion(icAgent.get(), system.upgrader_id);

      this.versionManagement.upgraderVersion = upgraderVersion;

      return upgraderVersion;
    },
    async checkVersionUpdates(): Promise<void> {
      if (
        // if the user does not have the privilege to change the canister, we do not need to check for updates
        !this.privileges.some(privilege => variantIs(privilege, 'SystemUpgrade')) ||
        // disables checking for updates if it's already ongoing
        this.versionManagement.loading
      ) {
        return;
      }

      try {
        const controlPanel = services().controlPanel;
        this.versionManagement = {
          ...this.versionManagement,
          loading: true,
        };

        const [stationVersion, upgraderVersion] = await Promise.all([
          this.loadStationVersion(),
          this.loadUpgraderVersion(),
        ]);

        const registryEntry = await controlPanel.findNextModuleVersion({
          name: '@orbit/station',
          currentVersion: stationVersion,
        });

        if (!registryEntry) {
          this.versionManagement.nextStationVersion = undefined;
          this.versionManagement.nextUpgraderVersion = undefined;

          return;
        }

        if (!variantIs(registryEntry.value, 'WasmModule')) {
          throw new Error(`Invalid next version response, expected WasmModule`);
        }

        let nextUpgraderVersion: string | undefined;
        this.versionManagement.nextStationVersion = registryEntry.value.WasmModule.version;
        for (const dependency of registryEntry.value.WasmModule.dependencies) {
          if (dependency.name === '@orbit/upgrader' && dependency.version !== upgraderVersion) {
            nextUpgraderVersion = dependency.version;
            break;
          }
        }

        this.versionManagement.nextUpgraderVersion = nextUpgraderVersion;

        // check if there is an existing request to update the canister, if so, we store the request id
        // to avoid prompting the user to update again if they have already requested an update
        const result = await this.service
          .listRequests({
            limit: 1,
            types: [{ SystemUpgrade: null }],
            statuses: [
              { Approved: null },
              { Processing: null },
              { Scheduled: null },
              { Created: null },
            ],
          })
          // if there is an error, we prevent it from being thrown to avoid breaking the entire
          // new version check flow
          .catch(err => {
            logger.error(`Failed to check if a request to update already exists`, { err });

            return { requests: [] };
          });

        this.versionManagement.updateRequested = result.requests?.[0]?.id;
      } catch (err) {
        logger.error(`Failed to check version updates`, { err });
        this.versionManagement = {
          ...initialStoreState().versionManagement,
          loading: this.versionManagement.loading,
        };
      } finally {
        this.versionManagement.loading = false;
      }
    },
  },
});
