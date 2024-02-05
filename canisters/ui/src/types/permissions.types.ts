import { BasicUser, UUID, UserGroup } from '~/generated/wallet/wallet.did';

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

export interface AccessPolicyForMembersOfGroup {
  policyId?: UUID;
  groups: Record<UUID, UserGroup>;
}

export interface AccessPolicyForSpecificUsers {
  policyId?: UUID;
  users: Record<UUID, BasicUser>;
}

export interface AccessPolicyForAllUsers {
  policyId?: UUID;
}

export interface ResourceAccessUserSpecifiers {
  allUsers: AccessPolicyForAllUsers;
  membersOfGroup: AccessPolicyForMembersOfGroup;
  specificUsers: AccessPolicyForSpecificUsers;
}
