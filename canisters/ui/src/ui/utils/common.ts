import { Principal } from '@dfinity/principal';
import { i18n } from '~/ui/modules';
import { useAppStore, useSessionStore } from '~/ui/stores';

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

export const computedWalletName = (
  {
    canisterId,
    notFoundName = '-',
  }: {
    canisterId: Principal;
    notFoundName?: string;
  },
  sessionStore = useSessionStore(),
): string => {
  const walletIdx =
    sessionStore.user?.wallets.findIndex(wallet => wallet.canisterId === canisterId.toText()) ?? -1;

  if (walletIdx === -1) {
    return notFoundName;
  }

  return (
    sessionStore.user?.wallets?.[walletIdx].name ??
    i18n.global.t('wallets.wallet_nr_title', { nr: walletIdx + 1 })
  );
};
