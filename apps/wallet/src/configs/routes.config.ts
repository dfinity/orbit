export enum Routes {
  Login = 'Login',
  Error = 'Error',
  NotFound = 'NotFound',
  Dashboard = 'Dashboard',
  Accounts = 'Accounts',
  Account = 'Account',
  AccountAsset = 'AccountAsset',
  MySettings = 'MySettings',
  UserGroups = 'UserGroups',
  SystemSettings = 'SystemSettings',
  Disconnected = 'Disconnected',
  Unauthorized = 'Unauthorized',
  Users = 'Users',
  AddressBook = 'AddressBook',
  Initialization = 'Initialization',
  AddStation = 'AddStation',
  Permissions = 'Permissions',
  Assets = 'Assets',
  ExternalCanisters = 'ExternalCanisters',
  ExternalCanister = 'ExternalCanister',
  DisasterRecovery = 'DisasterRecovery',
  // Request Pages
  Requests = 'Requests',
  TransferRequests = 'TransferRequests',
  ApprovalRules = 'ApprovalRules',
}

export enum RouteStatusCode {
  Success = 200,
  NotFound = 404,
  Unauthorized = 401,
  Disconnected = 403,
  Error = 500,
}

export const defaultLoginRoute = Routes.Login;
export const defaultHomeRoute = Routes.Dashboard;
