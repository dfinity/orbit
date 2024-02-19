export enum Routes {
  Login = 'Login',
  Error = 'Error',
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
  ProposalPolicies = 'ProposalPolicies',
}

export enum RouteStatusCode {
  Success = 200,
  NotFound = 404,
  Unauthorized = 401,
  Disconnected = 403,
  Error = 500,
}

export const defaultLoginRoute = Routes.Login;
export const defaultHomeRoute = Routes.Accounts;
