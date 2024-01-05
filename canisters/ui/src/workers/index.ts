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
