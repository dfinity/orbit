import {
  ListProposalsOperationType,
  ProposalStatusCode,
  UUID,
  UserStatus,
} from '~/generated/wallet/wallet.did';

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

export enum ProposalStatusEnum {
  Created = 'Created',
  Adopted = 'Adopted',
  Rejected = 'Rejected',
  Completed = 'Completed',
  Failed = 'Failed',
  Scheduled = 'Scheduled',
  Processing = 'Processing',
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

export interface DateRange {
  fromDt?: Date;
  toDt?: Date;
}

export type SortDirection = 'asc' | 'desc';

export interface ListProposalsArgs {
  limit?: number;
  offset?: number;
  statuses?: ProposalStatusCode[];
  types?: ListProposalsOperationType[];
  created_dt?: DateRange;
  expiration_dt?: DateRange;
  voterIds?: UUID[];
  proposerIds?: UUID[];
  sortBy?:
    | {
        createdAt: SortDirection;
      }
    | {
        expirationDt: SortDirection;
      }
    | {
        lastModified: SortDirection;
      };
}

export enum ProposalDomains {
  Accounts = 'accounts',
  AddressBook = 'address_book',
  Transfers = 'transfers',
  Users = 'users',
  System = 'system',
}
