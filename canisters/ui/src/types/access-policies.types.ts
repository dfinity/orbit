import {
  AccessPolicy,
  BasicUser,
  ResourceSpecifier,
  UUID,
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
  CanisterSettings = 'CanisterSettings',
  Proposal = 'Proposal',
  AddressBook = 'AddressBook',
}

export enum ResourceActionEnum {
  List = 'List',
  Create = 'Create',
  Read = 'Read',
  Update = 'Update',
  Delete = 'Delete',
  ReadSensitiveConfig = 'ReadSensitiveConfig',
  ReadPublicConfig = 'ReadPublicConfig',
}

export interface AccessPolicyItemInfo {
  id: UUID | null;
  canEdit: boolean;
  canRemove: boolean;
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

export interface ResourceAccessUserSpecifiers {
  allUsers: AccessPolicyForAllUsers;
  membersOfGroup: AccessPolicyForMembersOfGroup;
  specificUsers: AccessPolicyForSpecificUsers;
}

export interface ResourceAccessPolicySpecifier {
  action: ResourceActionEnum;
  specifier: ResourceSpecifier;
  users: ResourceAccessUserSpecifiers;
}

export interface AggregatedResouceAccessPolicies {
  resourceType: ResourceTypeEnum;
  specifiers: ResourceAccessPolicySpecifier[];
  match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean;
}
