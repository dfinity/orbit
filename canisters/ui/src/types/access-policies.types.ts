import {
  AccessPolicy,
  BasicUser,
  Resource,
  ResourceType,
  UserGroup,
} from '~/generated/wallet/wallet.did';

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

export interface AccessPolicyItemInfo {
  resource: ResourceType | null;
  canEdit: boolean;
}

export interface AccessPolicyForMembersOfGroup {
  policy: AccessPolicyItemInfo;
  groups: UserGroup[];
}

export interface AccessPolicyForSpecificUsers {
  policy: AccessPolicyItemInfo;
  users: BasicUser[];
}

export interface AccessPolicyForAllUsers {
  policy: AccessPolicyItemInfo;
}

export interface ResourceAccessAllowLevels {
  allUsers: AccessPolicyForAllUsers;
  membersOfGroup: AccessPolicyForMembersOfGroup;
  specificUsers: AccessPolicyForSpecificUsers;
}

export interface ResourceAccessPolicySpecifier {
  action: ResourceActionEnum;
  resource: Resource;
  allow: ResourceAccessAllowLevels;
}

export interface AggregatedResouceAccessPolicies {
  resourceType: ResourceTypeEnum;
  resources: ResourceAccessPolicySpecifier[];
  match(specifier: Resource, policy: AccessPolicy): boolean;
}
