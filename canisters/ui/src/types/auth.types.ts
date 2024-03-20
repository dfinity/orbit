export enum Privilege {
  AddUserGroup = 'AddUserGroup',
  ListUserGroups = 'ListUserGroups',
  AddUser = 'AddUser',
  ListUsers = 'ListUsers',
  AddProposalPolicy = 'AddProposalPolicy',
  ListProposalPolicies = 'ListProposalPolicies',
  ListAccounts = 'ListAccounts',
  ListAccessPolicies = 'ListAccessPolicies',
  AddAccount = 'AddAccount',
  ListAddressBookEntries = 'ListAddressBookEntries',
  AddAddressBookEntry = 'AddAddressBookEntry',
  ListProposals = 'ListProposals',
  ChangeCanister = 'ChangeCanister',
}

export enum RequiredSessionState {
  Authenticated = 'authenticated',
  ConnectedToWallet = 'connected-to-wallet',
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
