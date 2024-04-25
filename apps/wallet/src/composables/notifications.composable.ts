import { Proposal } from '~/generated/station/station.did';
import { i18n } from '~/plugins/i18n.plugin';
import { useAppStore } from '~/stores/app.store';

export const useOnFailedOperation = (): void => {
  const app = useAppStore();

  app.sendNotification({
    type: 'error',
    message: i18n.global.t('app.request_failed_message'),
  });
};

export const useOnSuccessfulOperation = (proposal?: Proposal): void => {
  const app = useAppStore();

  if (proposal && 'Rejected' in proposal.status) {
    app.sendNotification({
      type: 'error',
      message: i18n.global.t('app.request_rejected_message'),
    });

    return;
  }

  if (proposal && 'Adopted' in proposal.status) {
    app.sendNotification({
      type: 'success',
      message: i18n.global.t('app.request_adopted_message'),
    });

    return;
  }

  app.sendNotification({
    type: 'warning',
    message: i18n.global.t('app.request_pending_message'),
  });
};
