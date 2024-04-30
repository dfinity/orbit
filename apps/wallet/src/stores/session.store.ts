import { AnonymousIdentity, Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { defineStore } from 'pinia';
import { Ref } from 'vue';
import { icAgent } from '~/core/ic-agent.core';
import { logger } from '~/core/logger.core';
import { User } from '~/generated/control-panel/control_panel.did';
import { stationToUserStation } from '~/mappers/stations.mapper';
import { i18n } from '~/plugins/i18n.plugin';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { ConnectionStatus, useStationStore } from '~/stores/station.store';
import { afterLoginRedirect, redirectToLogin } from '~/utils/app.utils';
import { unreachable } from '~/utils/helper.utils';
import { objectDeserialize, objectSerialize, useStorage } from '~/utils/storage.utils';
import { disableWorkers, enableWorkers } from '~/workers';

export interface Station {
  main: boolean;
  name: string;
  canisterId: string;
}

export interface SelectedStation {
  canisterId: Ref<string | null>;
  hasAccess: boolean;
}

export enum InitializationStatus {
  Uninitialized = 'uninitialized',
  Initialized = 'initialized',
  FailedInitialization = 'failed-initialization',
}

export interface SessionStoreState {
  initialized: InitializationStatus;
  loading: boolean;
  lastLoginPrincipal: string | null;
  principal: string;
  isAuthenticated: boolean;
  reauthenticationNeeded: boolean;
  data: {
    stations: Station[];
    selected: SelectedStation;
  };
}

let trackingUserActive: ReturnType<typeof setInterval> | null = null;
const considerTheUserActiveAfterMs = 10_000;
const keepUserActiveEveryMs = 60_000;

// A function that registers that a user is active every after a certain interval of time
const registerUserLastActiveTracking = (): void => {
  if (trackingUserActive) {
    return;
  }

  trackingUserActive = setInterval(() => {
    recordUserIsActive();
  }, keepUserActiveEveryMs);

  setTimeout(() => {
    recordUserIsActive();
  }, considerTheUserActiveAfterMs);
};

const recordUserIsActive = (): void => {
  const session = useSessionStore();
  const controlPanel = services().controlPanel;

  if (session.isAuthenticated && !session.reauthenticationNeeded) {
    controlPanel.setUserActive().catch(err => {
      logger.error(`Failed to set user active`, { err });
    });
  }
};

export const useSessionStore = defineStore('session', {
  state: (): SessionStoreState => {
    return {
      initialized: InitializationStatus.Uninitialized,
      loading: false,
      lastLoginPrincipal: null,
      principal: Principal.anonymous().toText(),
      isAuthenticated: false,
      reauthenticationNeeded: false,
      data: {
        stations: [],
        selected: {
          canisterId: useStorage({
            deserialize: objectDeserialize,
            serialize: objectSerialize,
            key: 'selected-station',
            storage: sessionStorage,
            deepWatch: true,
            initial: () => null,
          }),
          hasAccess: false,
        },
      },
    };
  },
  getters: {
    hasStations(): boolean {
      return !!this.data.stations.length;
    },
    mainStation(): Principal | null {
      const mainStation = this.data.stations.find(station => station.main);

      return mainStation ? Principal.fromText(mainStation.canisterId) : null;
    },
  },
  actions: {
    async initialize(): Promise<void> {
      try {
        if (this.initialized === InitializationStatus.Initialized) {
          return;
        }

        const sessionExpirationService = services().sessionExpiration;

        sessionExpirationService.subscribe(msg => {
          switch (msg) {
            case 'otherTabActive':
              sessionExpirationService.resetInactivityTimeout();
              break;
            case 'otherTabSignedIn':
              this.setReauthenticated();
              break;
            case 'otherTabSignedOut':
              this.signOut(false);
              break;
            case 'sessionExpired':
              this.requireReauthentication();
              break;
            case 'userInactive': {
              const authService = services().auth;
              authService.logout();
              this.requireReauthentication();
              break;
            }
            default:
              unreachable(msg);
          }
        });

        const authService = services().auth;
        const cachedAuthenticatedIdentity = await authService.identity();

        if (!cachedAuthenticatedIdentity) {
          icAgent.get().replaceIdentity(new AnonymousIdentity());
          this.lastLoginPrincipal = Principal.anonymous().toText();
          this.initialized = InitializationStatus.Initialized;
          return;
        }

        await this.initializeAuthenticated(cachedAuthenticatedIdentity);
      } catch (error) {
        this.reset();

        logger.error(`Application failed to initialize the state`, { error });

        this.initialized = InitializationStatus.FailedInitialization;
      } finally {
        registerUserLastActiveTracking();
      }
    },
    reset(): void {
      const station = useStationStore();

      this.loading = false;
      this.isAuthenticated = false;
      this.principal = Principal.anonymous().toText();
      this.lastLoginPrincipal = Principal.anonymous().toText();
      this.reauthenticationNeeded = false;
      this.data.stations = [];
      this.data.selected.canisterId = null;
      this.data.selected.hasAccess = false;

      station.onDisconnected();
    },
    async signIn(resetOnError = false): Promise<void> {
      const authService = services().auth;
      const sessionExpirationService = services().sessionExpiration;

      try {
        const identity = await authService.login();

        sessionExpirationService.notifySignedIn();
        await this.initializeAuthenticated(identity);
      } catch (error) {
        disableWorkers();
        if (resetOnError) {
          this.reset();
        }
        throw error;
      }
    },
    async signOut(notifyOtherTabs = true): Promise<void> {
      disableWorkers();

      const sessionExpirationService = services().sessionExpiration;

      sessionExpirationService.clearInactivityTimer();
      sessionExpirationService.clearSessionTimer();

      if (notifyOtherTabs) {
        sessionExpirationService.notifySignedOut();
      }

      const authService = services().auth;
      await authService.logout();

      this.reset();
      redirectToLogin();
    },

    async load(): Promise<void> {
      const app = useAppStore();

      try {
        if (this.loading) {
          logger.warn(`Session is already loading`);
          return;
        }

        this.loading = true;
        const controlPanelService = services().controlPanel;
        const controlPanelUser = await controlPanelService.getCurrentUser();

        let initialStationId = null;

        if (this.data.selected.canisterId) {
          initialStationId = Principal.fromText(this.data.selected.canisterId);
        } else {
          initialStationId = controlPanelUser.main_station?.[0]
            ? controlPanelUser.main_station?.[0]
            : controlPanelUser.stations?.[0]?.canister_id;
        }
        const sameUser =
          this.isAuthenticated && this.principal === controlPanelUser.identity.toText();

        this.isAuthenticated = true;
        this.populateUser(controlPanelUser);

        if (!sameUser && initialStationId) {
          return this.connectStation(initialStationId);
        }
      } catch (err) {
        logger.error(`Failed to load user session`, { err });

        app.sendNotification({
          type: 'error',
          message: i18n.global.t('app.session_load_error'),
        });
      } finally {
        this.loading = false;
      }
    },
    populateUser(user: User): void {
      const selectedStationId = this.data.selected.canisterId;
      const sameUser = this.isAuthenticated && this.principal === user.identity.toText();
      this.principal = user.identity.toText();
      this.data.stations = user.stations.map(station => ({
        main: station.canister_id.toText() === user.main_station?.[0]?.toText(),
        name: station.name,
        canisterId: station.canister_id.toText(),
      }));

      const hasStation = this.data.stations.some(
        station => station.canisterId === selectedStationId,
      );
      if (!sameUser || !hasStation) {
        this.disconnectStation();
      }
    },
    disconnectStation(): void {
      const station = useStationStore();

      this.data.selected.hasAccess = false;
      this.data.selected.canisterId = null;

      station.onDisconnected();
    },
    async connectStation(stationId: Principal, onConnectedReload = true): Promise<void> {
      const station = useStationStore();

      this.data.selected.canisterId = stationId.toText();
      this.data.selected.hasAccess = false;
      const connectionStatus = await station.connectTo(stationId, onConnectedReload);

      if (connectionStatus === ConnectionStatus.Connected) {
        this.data.selected.hasAccess = true;
      }
    },

    async addStation(canisterId: string, name: string): Promise<void> {
      const controlPanelService = services().controlPanel;

      const user = await controlPanelService.editUser({
        main_station: this.mainStation ? [this.mainStation] : [],
        stations: [
          [
            ...this.data.stations.map(station => stationToUserStation(station)),
            stationToUserStation({
              canisterId: canisterId,
              name: name,
            }),
          ],
        ],
      });

      this.populateUser(user);

      await this.connectStation(Principal.fromText(canisterId));
    },

    requireReauthentication() {
      this.reauthenticationNeeded = true;

      const sessionExpirationService = services().sessionExpiration;
      sessionExpirationService.clearInactivityTimer();
      sessionExpirationService.clearSessionTimer();

      disableWorkers();
    },

    async setReauthenticated() {
      const authService = services().auth;
      await authService.client({ reset: true });
      const maybeIdentity = await authService.identity();
      if (!maybeIdentity) {
        logger.error(`Reauthentication failed, no identity found`);
        return;
      }

      await this.initializeAuthenticated(maybeIdentity);
    },

    async initializeAuthenticated(newIdentity: Identity) {
      const authService = services().auth;
      icAgent.get().replaceIdentity(newIdentity);

      if (
        this.lastLoginPrincipal !== null &&
        this.lastLoginPrincipal !== newIdentity.getPrincipal().toText()
      ) {
        this.reset();
      }

      this.reauthenticationNeeded = false;
      enableWorkers();

      const sessionExpirationService = services().sessionExpiration;

      const maybeSessionExpirationTimeMs = await authService.getRemainingSessionTimeMs();
      if (maybeSessionExpirationTimeMs) {
        sessionExpirationService.resetSessionTimeout(maybeSessionExpirationTimeMs);
      }
      sessionExpirationService.resetInactivityTimeout();

      const controlPanelService = services().controlPanel;
      const isRegistered = await controlPanelService.hasRegistration();

      if (!isRegistered) {
        await controlPanelService.register({
          // a new user is created with an empty list of stations, they can add them later
          station: [],
        });
      }

      // loads information about the authenticated user
      await this.load();

      // if the user was not signed in before, or the user signed in with a different identity
      if (
        this.lastLoginPrincipal !== null &&
        this.lastLoginPrincipal !== newIdentity.getPrincipal().toText()
      ) {
        afterLoginRedirect();
      }

      this.lastLoginPrincipal = newIdentity.getPrincipal().toText();
    },
  },
});
