import { logger } from '~/core';
import { useWalletStore } from '~/ui/stores/wallet';
import { accountsWorker, installWebWorkers, notificationsWorker } from '~/workers';

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
      default:
        logger.warn('Unknown message received from notifications worker', { msg });
    }
  };
};

export const initWorkers = async (): Promise<void> => {
  await installWebWorkers();

  registerAccountWorkerEventListener();
  registerNotificationsWorkerEventListener();
};
