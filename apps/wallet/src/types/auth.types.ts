export enum Privilege {
  Capabilities = 'Capabilities',
  SystemInfo = 'SystemInfo',
  AddUserGroup = 'AddUserGroup',
  ListUserGroups = 'ListUserGroups',
  AddUser = 'AddUser',
  ListUsers = 'ListUsers',
  AddRequestPolicy = 'AddRequestPolicy',
  ListRequestPolicies = 'ListRequestPolicies',
  ListAccounts = 'ListAccounts',
  ListPermissions = 'ListPermissions',
  AddAccount = 'AddAccount',
  ListAddressBookEntries = 'ListAddressBookEntries',
  AddAddressBookEntry = 'AddAddressBookEntry',
  ListRequests = 'ListRequests',
  ChangeCanister = 'ChangeCanister',
}

export enum RequiredSessionState {
  Authenticated = 'authenticated',
  AuthenticatedNoStation = 'authenticated-no-station',
  AuthenticatedHasStations = 'authenticated-has-stations',
  ConnectedToStation = 'connected-to-station',
  Guest = 'guest',
  Any = 'any',
}

export interface PermissionRequirements {
  session: RequiredSessionState;
  privileges?: Privilege[];
}

export interface AuthRouteMeta {
  check: PermissionRequirements;
}

declare module 'vue-router' {
  interface RouteMeta {
    // must be declared by every route
    auth: AuthRouteMeta;
  }
}
