import { Identity } from '@dfinity/agent';
import { AuthService } from '~/services/auth.service';
import { useSessionStore } from '~/stores/session.store';
import { useWalletStore } from '~/stores/wallet.store';
import { Privilege, RequiredSessionState } from '~/types/auth.types';
import { unreachable } from '~/utils/helper.utils';

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

export const loadIdentity = async (): Promise<Identity | null> => {
  const authService = new AuthService();

  return authService.identity();
};
