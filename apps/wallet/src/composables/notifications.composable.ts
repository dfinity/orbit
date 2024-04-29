import { Request } from '~/generated/station/station.did';
import { i18n } from '~/plugins/i18n.plugin';
import { useAppStore } from '~/stores/app.store';

export const useOnFailedOperation = (): void => {
  const app = useAppStore();

  app.sendNotification({
    type: 'error',
    message: i18n.global.t('app.request_failed_message'),
  });
};

export const useOnSuccessfulOperation = (request?: Request): void => {
  const app = useAppStore();

  if (request && 'Rejected' in request.status) {
    app.sendNotification({
      type: 'error',
      message: i18n.global.t('app.request_rejected_message'),
    });

    return;
  }

  if (request && 'Approved' in request.status) {
    app.sendNotification({
      type: 'success',
      message: i18n.global.t('app.request_approved_message'),
    });

    return;
  }

  app.sendNotification({
    type: 'warning',
    message: i18n.global.t('app.request_pending_message'),
  });
};
