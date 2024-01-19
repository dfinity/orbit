import type { AccountsWorker } from '~/workers/accounts.worker';
import type { NotificationsWorker } from '~/workers/notifications.worker';
import type { AuthWorker } from './auth.worker';

export let accountsWorker: AccountsWorker | undefined;
export let notificationsWorker: NotificationsWorker | undefined;
export let authWorker: AuthWorker | undefined;

export const installWebWorkers = async () => {
  const AccountsWorker = await import('~/workers/accounts.worker?worker');
  accountsWorker = new AccountsWorker.default();

  const NotificationsWorker = await import('~/workers/notifications.worker?worker');
  notificationsWorker = new NotificationsWorker.default();

  const AuthWorker = await import('~/workers/auth.worker?worker');
  authWorker = new AuthWorker.default();
};
