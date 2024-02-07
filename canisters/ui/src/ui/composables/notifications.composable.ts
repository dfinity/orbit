import { Proposal } from '~/generated/wallet/wallet.did';
import { i18n } from '~/ui/modules/i18n';
import { useAppStore } from '~/ui/stores/app';

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
