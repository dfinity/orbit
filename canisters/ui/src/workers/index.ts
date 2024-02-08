import { Principal } from '@dfinity/principal';
import { logger } from '~/core/logger.core';
import { useWalletStore } from '~/stores/wallet.store';
import { unreachable } from '~/utils/helper.utils';
import type { AccountsWorker } from '~/workers/accounts.worker';
import type { NotificationsWorker } from '~/workers/notifications.worker';

export let accountsWorker: AccountsWorker | undefined;
export let notificationsWorker: NotificationsWorker | undefined;

export const installWebWorkers = async () => {
  const AccountsWorker = await import('~/workers/accounts.worker?worker');
  accountsWorker = new AccountsWorker.default();

  const NotificationsWorker = await import('~/workers/notifications.worker?worker');
  notificationsWorker = new NotificationsWorker.default();
};

export const initWorkers = async (): Promise<void> => {
  await installWebWorkers();

  registerAccountWorkerEventListener();
  registerNotificationsWorkerEventListener();
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
