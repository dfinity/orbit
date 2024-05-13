import { BasicUser, Resource, UserGroup } from '~/generated/station/station.did';

export enum ResourceTypeEnum {
  User = 'User',
  UserGroup = 'UserGroup',
  Account = 'Account',
  Transfer = 'Transfer',
  Permission = 'Permission',
  RequestPolicy = 'RequestPolicy',
  ChangeCanister = 'ChangeCanister',
  System = 'System',
  Request = 'Request',
  AddressBook = 'AddressBook',
}

export enum ResourceActionEnum {
  List = 'List',
  Create = 'Create',
  Read = 'Read',
  Update = 'Update',
  Delete = 'Delete',
  Transfer = 'Transfer',
  SystemInfoConfig = 'SystemInfoConfig',
  SystemInfoCapabilities = 'SystemInfoCapabilities',
  ManageSystemInfo = 'ManageSystemInfo',
}

export enum AuthScopeEnum {
  Public = 'Public',
  Authenticated = 'Authenticated',
  Restrictred = 'Restrictred',
}

export interface ResourceAccessAllowLevels {
  authScope: AuthScopeEnum;
  membersOfGroup: UserGroup[];
  specificUsers: BasicUser[];
}

export interface ResourcePermissionSpecifier {
  action: ResourceActionEnum;
  resource: Resource;
  allow: ResourceAccessAllowLevels;
  canEdit: boolean;
}

export interface AggregatedResoucePermissions {
  resourceType: ResourceTypeEnum;
  resources: ResourcePermissionSpecifier[];
  match(a: Resource, b: Resource): boolean;
}
