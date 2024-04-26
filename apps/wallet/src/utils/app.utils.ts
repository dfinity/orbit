import { Principal } from '@dfinity/principal';
import { Routes, defaultHomeRoute, defaultLoginRoute } from '~/configs/routes.config';
import { ApiError } from '~/generated/control-panel/control_panel.did';
import { i18n } from '~/plugins/i18n.plugin';
import { redirectToKey, router } from '~/plugins/router.plugin';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';

export const copyToClipboard = (
  args: {
    textToCopy: string;
    sendNotification?: boolean;
    notificationMessage?: string;
  },
  appStore = useAppStore(),
): void => {
  navigator.clipboard.writeText(args.textToCopy);

  if (args.sendNotification) {
    appStore.sendNotification({
      type: 'success',
      message: args.notificationMessage || i18n.global.t('app.copied_to_clipboard'),
    });
  }
};

export const computedStationName = (
  {
    canisterId,
    notFoundName = '-',
  }: {
    canisterId: Principal;
    notFoundName?: string;
  },
  sessionStore = useSessionStore(),
): string => {
  const stationIdx =
    sessionStore.data.stations.findIndex(station => station.canisterId === canisterId.toText()) ??
    -1;

  if (stationIdx === -1) {
    return notFoundName;
  }

  return (
    sessionStore.data.stations?.[stationIdx].name ??
    i18n.global.t('stations.station_nr_title', { nr: stationIdx + 1 })
  );
};

export const redirectToLogin = (): void => {
  router.push({ name: defaultLoginRoute });
};

export const afterLoginRedirect = (): void => {
  const lastRequestedPage = window?.sessionStorage.getItem(redirectToKey);
  if (lastRequestedPage) {
    window?.sessionStorage.removeItem(redirectToKey);
    router.push(lastRequestedPage);
    return;
  }

  router.push({ name: defaultHomeRoute });
};

export const redirectToStationSettings = (): void => {
  router.push({ name: Routes.SystemSettings });
};

// To be used in catch blocks to determine if the error is an ApiError
export function isApiError(e: unknown): e is ApiError {
  return typeof e === 'object' && e !== null && 'code' in e && 'message' in e && 'details' in e;
}

const beforeUnloadCallback = (e: BeforeUnloadEvent): boolean => {
  e.preventDefault();

  return true;
};

export const registerBeforeUnloadConfirmation = (): void => {
  window.addEventListener('beforeunload', beforeUnloadCallback);
};

export const unregisterBeforeUnloadConfirmation = (): void => {
  window.removeEventListener('beforeunload', beforeUnloadCallback);
};
