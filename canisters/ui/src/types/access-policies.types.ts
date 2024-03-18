import { BasicUser, Resource, UserGroup } from '~/generated/wallet/wallet.did';

export enum ResourceTypeEnum {
  User = 'User',
  UserGroup = 'UserGroup',
  Account = 'Account',
  Transfer = 'Transfer',
  AccessPolicy = 'AccessPolicy',
  ProposalPolicy = 'ProposalPolicy',
  ChangeCanister = 'ChangeCanister',
  Settings = 'Settings',
  Proposal = 'Proposal',
  AddressBook = 'AddressBook',
}

export enum ResourceActionEnum {
  List = 'List',
  Create = 'Create',
  Read = 'Read',
  Update = 'Update',
  Delete = 'Delete',
  Transfer = 'Transfer',
  ReadSensitiveConfig = 'ReadSensitiveConfig',
  ReadPublicConfig = 'ReadPublicConfig',
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

export interface ResourceAccessPolicySpecifier {
  action: ResourceActionEnum;
  resource: Resource;
  allow: ResourceAccessAllowLevels;
  canEdit: boolean;
}

export interface AggregatedResouceAccessPolicies {
  resourceType: ResourceTypeEnum;
  resources: ResourceAccessPolicySpecifier[];
  match(a: Resource, b: Resource): boolean;
}
