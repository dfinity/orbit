import { useSessionStore } from '~/stores/session.store';
import { useStationStore } from '~/stores/station.store';
import { Privilege, RequiredSessionState } from '~/types/auth.types';
import { unreachable } from '~/utils/helper.utils';

export const hasRequiredSession = (
  requiredSessionState: RequiredSessionState,
  session = useSessionStore(),
): boolean => {
  switch (requiredSessionState) {
    case RequiredSessionState.Guest:
      return !session.isAuthenticated;
    case RequiredSessionState.AuthenticatedNoStation:
      return session.isAuthenticated && !session.data.stations.length;
    case RequiredSessionState.AuthenticatedHasStations:
      return session.isAuthenticated && session.data.stations.length > 0;
    case RequiredSessionState.Authenticated:
      return session.isAuthenticated;
    case RequiredSessionState.ConnectedToStation:
      return session.data.selected.hasAccess;
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
  station = useStationStore(),
): boolean => {
  const userPrivileges = new Set<string>();
  station.privileges.forEach(userPrivilege => {
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
