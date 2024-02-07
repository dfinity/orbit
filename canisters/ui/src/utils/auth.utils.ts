import { unreachable } from '~/core/utils.core';
import { Privilege, RequiredSessionState } from '~/types/auth.types';
import { useSessionStore } from '~/ui/stores/session';
import { useWalletStore } from '~/ui/stores/wallet';

export const hasRequiredSession = (
  requiredSessionState: RequiredSessionState,
  session = useSessionStore(),
): boolean => {
  switch (requiredSessionState) {
    case RequiredSessionState.Guest:
      return !session.isAuthenticated;
    case RequiredSessionState.Authenticated:
      return session.isAuthenticated;
    case RequiredSessionState.ConnectedToWallet:
      return session.data.selectedWallet.hasAccess;
    case RequiredSessionState.Any:
      return true;
  }

  unreachable(requiredSessionState);
};

export const hasRequiredPrivilege = (
  {
    anyOf,
  }: {
    anyOf?: Privilege[];
  },
  wallet = useWalletStore(),
): boolean => {
  const userPrivileges = new Set<string>();
  wallet.privileges.forEach(userPrivilege => {
    const privelegeId = Object.keys(userPrivilege)?.[0];
    if (privelegeId) {
      userPrivileges.add(privelegeId);
    }
  });

  if (!anyOf?.length) {
    return true;
  }

  return anyOf.some(requiredPrivilege => userPrivileges.has(requiredPrivilege));
};
