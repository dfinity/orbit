import { UUID, UserStatus } from '~/generated/wallet/wallet.did';

export enum PolicyType {
  VariableApprovalThreshold = 'VariableApprovalThreshold',
  FixedApprovalThreshold = 'FixedApprovalThreshold',
}

export enum AccountTransferStatus {
  Created = 'created',
  Failed = 'failed',
  Processing = 'processing',
  Completed = 'completed',
  Unknown = 'unknown',
}

export enum WalletProposalType {
  Transfer = 'Transfer',
  AccountEdit = 'AccountEdit',
  Unknown = 'Unknown',
}

export enum Privilege {
  AddUserGroup = 'AddUserGroup',
  ListUserGroups = 'ListUserGroups',
  AddUser = 'AddUser',
  ListUsers = 'ListUsers',
  AddProposalPolicy = 'AddProposalPolicy',
  ListProposalPolicies = 'ListProposalPolicies',
  ListAccounts = 'ListAccounts',
  ListAccessPolicies = 'ListAccessPolicies',
  AddAccessPolicy = 'AddAccessPolicy',
  AddAccount = 'AddAccount',
}

export interface UserInput {
  id?: UUID;
  name?: string;
  status: UserStatus;
  groups: UUID[];
  identities: string[];
}

export enum UserStatusType {
  Active = 'Active',
  Inactive = 'Inactive',
}
