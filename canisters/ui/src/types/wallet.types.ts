import {
  DisplayUser,
  ListProposalsOperationType,
  ProposalStatusCode,
  UUID,
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
  Cancelled = 'Cancelled',
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
  All = 'all',
  Accounts = 'accounts',
  AddressBook = 'address_book',
  Transfers = 'transfers',
  Users = 'users',
  System = 'system',
}

export interface ListAccountsArgs {
  limit?: number;
  offset?: number;
  searchTerm?: string;
}

export enum ChangeCanisterTargetType {
  UpgradeWallet = 'UpgradeWallet',
  UpgradeUpgrader = 'UpgradeUpgrader',
}

export enum ProposalSpecifierEnum {
  EditAccessPolicy = 'EditAccessPolicy',
  AddUserGroup = 'AddUserGroup',
  RemoveProposalPolicy = 'RemoveProposalPolicy',
  AddUser = 'AddUser',
  EditUserGroup = 'EditUserGroup',
  RemoveAddressBookEntry = 'RemoveAddressBookEntry',
  EditAddressBookEntry = 'EditAddressBookEntry',
  AddProposalPolicy = 'AddProposalPolicy',
  ChangeCanister = 'ChangeCanister',
  EditProposalPolicy = 'EditProposalPolicy',
  EditUser = 'EditUser',
  Transfer = 'Transfer',
  EditAccount = 'EditAccount',
  AddAddressBookEntry = 'AddAddressBookEntry',
  RemoveUserGroup = 'RemoveUserGroup',
  AddAccount = 'AddAccount',
}

export enum ProposalCriteriaEnum {
  AutoAdopted = 'AutoAdopted',
  HasAddressBookMetadata = 'HasAddressBookMetadata',
  MinimumVotes = 'MinimumVotes',
  ApprovalThreshold = 'ApprovalThreshold',
  And = 'And',
  Or = 'Or',
  Not = 'Not',
}

export enum ProposalCriteriaUserSpecifierEnum {
  Owner = 'Owner',
  Proposer = 'Proposer',
  Any = 'Any',
  Group = 'Group',
  Id = 'Id',
}

export interface ListAddressBookEntriesArgs {
  limit?: number;
  offset?: number;
  addresses?: string[];
  blockchain?: string;
  standard?: string;
  ids?: UUID[];
}

export type MetadataItem = { key: string; value: string };

export interface ProposalDetails {
  can_vote: boolean;
  proposer_name?: string;
  voters: DisplayUser[];
}

export enum ProposalOperationEnum {
  AddUser = 'AddUser',
  EditUser = 'EditUser',
  AddUserGroup = 'AddUserGroup',
  EditUserGroup = 'EditUserGroup',
  RemoveUserGroup = 'RemoveUserGroup',
  AddAccount = 'AddAccount',
  EditAccount = 'EditAccount',
  AddAddressBookEntry = 'AddAddressBookEntry',
  EditAddressBookEntry = 'EditAddressBookEntry',
  RemoveAddressBookEntry = 'RemoveAddressBookEntry',
  AddProposalPolicy = 'AddProposalPolicy',
  EditProposalPolicy = 'EditProposalPolicy',
  RemoveProposalPolicy = 'RemoveProposalPolicy',
  EditAccessPolicy = 'EditAccessPolicy',
  ChangeCanister = 'ChangeCanister',
  Transfer = 'Transfer',
}
