import { Principal } from '@dfinity/principal';
import { defaultHomeRoute, defaultLoginRoute, i18n, redirectToKey, router } from '~/ui/modules';
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
