import { logger, unreachable } from '~/core';
import { useWalletStore } from '~/ui/stores/wallet';
import { accountsWorker, authWorker, installWebWorkers, notificationsWorker } from '~/workers';
import { useSessionStore } from '../stores/session';
import { Principal } from '@dfinity/principal';

const registerAuthWorkerEventListener = (): void => {
  if (!authWorker) {
    return;
  }

  const sessionStore = useSessionStore();

  authWorker.onmessage = ({ data: msg }) => {
    switch (msg.type) {
      case 'sessionExpired':
        sessionStore.requireReauthentication();
        break;
      case 'sessionValid':
        sessionStore.setReauthenticated();
        break;
      case 'signedOut':
        sessionStore.signOut();
        break;
      default:
        unreachable(msg);
        logger.warn('Unknown message received from accounts worker', { msg });
    }
  };
};

const registerAccountWorkerEventListener = (): void => {
  if (!accountsWorker) {
    return;
  }

  const wallet = useWalletStore();

  accountsWorker.onmessage = ({ data: msg }) => {
    switch (msg.type) {
      case 'accounts': {
        const { accounts } = msg.data;
        wallet.accounts.items = accounts;
        break;
      }
      case 'stopped':
        // do nothing on worker stop as this is expected
        break;
      default:
        logger.warn('Unknown message received from accounts worker', { msg });
    }
  };
};

const registerNotificationsWorkerEventListener = (): void => {
  if (!notificationsWorker) {
    return;
  }

  const wallet = useWalletStore();

  notificationsWorker.onmessage = ({ data: msg }) => {
    switch (msg.type) {
      case 'notifications': {
        const { notifications } = msg.data;
        wallet.notifications.items = notifications.map(n => {
          return {
            loading: false,
            data: n,
          };
        });
        break;
      }
      case 'stopped':
        // do nothing on worker stop as this is expected
        break;
      case 'error':
        // do nothing on worker error
        break;
      default:
        logger.warn('Unknown message received from notifications worker', { msg });
        unreachable(msg);
    }
  };
};

export const initWorkers = async (): Promise<void> => {
  await installWebWorkers();

  registerAuthWorkerEventListener();
  registerAccountWorkerEventListener();
  registerNotificationsWorkerEventListener();
};

export function startWalletWorkers(walletId: Principal) {
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
}
export function stopWalletWorkers() {
  accountsWorker?.postMessage({
    type: 'stop',
  });
  notificationsWorker?.postMessage({
    type: 'stop',
  });
}

export function enableWalletWorkers() {
  accountsWorker?.postMessage({
    type: 'enable',
  });
  notificationsWorker?.postMessage({
    type: 'enable',
  });
}

export function disableWalletWorkers() {
  accountsWorker?.postMessage({
    type: 'disable',
  });
  notificationsWorker?.postMessage({
    type: 'disable',
  });
}

export function startAuthWorker() {
  authWorker?.postMessage({
    type: 'start',
  });
}
export function stopAuthWorker() {
  authWorker?.postMessage({
    type: 'stop',
  });
}
