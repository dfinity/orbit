export enum Routes {
  Login = 'Login',
  NotFound = 'NotFound',
  Accounts = 'Accounts',
  Account = 'Account',
  MySettings = 'MySettings',
  UserGroups = 'UserGroups',
  SystemSettings = 'SystemSettings',
  Disconnected = 'Disconnected',
  Unauthorized = 'Unauthorized',
  Users = 'Users',
  AddressBook = 'AddressBook',
  Initialization = 'Initialization',
  AccessPolicies = 'AccessPolicies',
  // Proposal Pages
  Proposals = 'Proposals',
  TransferProposals = 'TransferProposals',
}

export const defaultLoginRoute = Routes.Login;
export const defaultHomeRoute = Routes.Accounts;

