export enum Privilege {
  Capabilities = 'Capabilities',
  SystemInfo = 'SystemInfo',
  AddUserGroup = 'AddUserGroup',
  ListUserGroups = 'ListUserGroups',
  AddUser = 'AddUser',
  ListUsers = 'ListUsers',
  AddProposalPolicy = 'AddProposalPolicy',
  ListProposalPolicies = 'ListProposalPolicies',
  ListAccounts = 'ListAccounts',
  ListPermissions = 'ListPermissions',
  AddAccount = 'AddAccount',
  ListAddressBookEntries = 'ListAddressBookEntries',
  AddAddressBookEntry = 'AddAddressBookEntry',
  ListProposals = 'ListProposals',
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

export interface AccessCriteria {
  session: RequiredSessionState;
  privileges?: Privilege[];
}

export interface AuthRouteMeta {
  check: AccessCriteria;
}

declare module 'vue-router' {
  interface RouteMeta {
    // must be declared by every route
    auth: AuthRouteMeta;
  }
}
