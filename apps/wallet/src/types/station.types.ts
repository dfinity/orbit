import { Principal } from '@dfinity/principal';
import {
  DisplayUser,
  RequestEvaluationResult,
  ListRequestsOperationType,
  RequestStatusCode,
  UUID,
  ExternalCanisterState,
  ListExternalCanistersSortInput,
} from '~/generated/station/station.did';

export enum AccountTransferStatus {
  Created = 'created',
  Failed = 'failed',
  Processing = 'processing',
  Completed = 'completed',
  Unknown = 'unknown',
}

export enum ExternalCanisterStateEnum {
  Active = 'Active',
  Archived = 'Archived',
}

export enum RequestStatusEnum {
  Created = 'Created',
  Approved = 'Approved',
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

export interface ListRequestsArgs {
  limit?: number;
  offset?: number;
  statuses?: RequestStatusCode[];
  types?: ListRequestsOperationType[];
  created_dt?: DateRange;
  expiration_dt?: DateRange;
  approverIds?: UUID[];
  requesterIds?: UUID[];
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
  onlyApprovable?: boolean;
}

export interface GetNextApprovableRequestArgs {
  types?: ListRequestsOperationType[];
  excludedRequestIds?: UUID[];
}

export enum RequestDomains {
  All = 'all',
  Accounts = 'accounts',
  AddressBook = 'address_book',
  Transfers = 'transfers',
  Users = 'users',
  ExternalCanisters = 'external_canisters',
  System = 'system',
  Assets = 'assets',
}

export interface ListAccountsArgs {
  limit?: number;
  offset?: number;
  searchTerm?: string;
}

export enum SystemUpgradeTargetType {
  UpgradeStation = 'UpgradeStation',
  UpgradeUpgrader = 'UpgradeUpgrader',
}

export enum RequestSpecifierEnum {
  EditPermission = 'EditPermission',
  AddUserGroup = 'AddUserGroup',
  RemoveRequestPolicy = 'RemoveRequestPolicy',
  AddUser = 'AddUser',
  EditUserGroup = 'EditUserGroup',
  RemoveAddressBookEntry = 'RemoveAddressBookEntry',
  EditAddressBookEntry = 'EditAddressBookEntry',
  AddRequestPolicy = 'AddRequestPolicy',
  SystemUpgrade = 'SystemUpgrade',
  EditRequestPolicy = 'EditRequestPolicy',
  EditUser = 'EditUser',
  Transfer = 'Transfer',
  EditAccount = 'EditAccount',
  AddAddressBookEntry = 'AddAddressBookEntry',
  RemoveUserGroup = 'RemoveUserGroup',
  AddAccount = 'AddAccount',
  ManageSystemInfo = 'ManageSystemInfo',
  ChangeExternalCanister = 'ChangeExternalCanister',
  CreateExternalCanister = 'CreateExternalCanister',
  CallExternalCanister = 'CallExternalCanister',
  FundExternalCanister = 'FundExternalCanister',
  SetDisasterRecovery = 'SetDisasterRecovery',
  AddAsset = 'AddAsset',
  EditAsset = 'EditAsset',
  RemoveAsset = 'RemoveAsset',
}

export enum RequestPolicyRuleEnum {
  AutoApproved = 'AutoApproved',
  AllowListedByMetadata = 'AllowListedByMetadata',
  AllowListed = 'AllowListed',
  Quorum = 'Quorum',
  QuorumPercentage = 'QuorumPercentage',
  AllOf = 'AllOf',
  AnyOf = 'AnyOf',
  Not = 'Not',
}

export enum RequestPolicyRuleUserSpecifierEnum {
  Any = 'Any',
  Group = 'Group',
  Id = 'Id',
}

export interface ListAddressBookEntriesArgs {
  limit?: number;
  offset?: number;
  addresses?: string[];
  blockchain?: string;
  labels?: [];
  ids?: UUID[];
  address_formats?: string[];
}

export interface ListAssetsArgs {
  limit?: number;
  offset?: number;
}

export interface ListExternalCanistersArgs {
  limit?: number;
  offset?: number;
  canisterIds?: Principal[];
  labels?: string[];
  states?: ExternalCanisterState[];
  sortBy?: ListExternalCanistersSortInput;
}

export type MetadataItem = { key: string; value: string };

export interface RequestDetails {
  can_approve: boolean;
  requester_name: string;
  approvers: DisplayUser[];
  evaluationResult?: RequestEvaluationResult;
}

export enum RequestOperationEnum {
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
  AddRequestPolicy = 'AddRequestPolicy',
  EditRequestPolicy = 'EditRequestPolicy',
  RemoveRequestPolicy = 'RemoveRequestPolicy',
  EditPermission = 'EditPermission',
  SystemUpgrade = 'SystemUpgrade',
  Transfer = 'Transfer',
  ManageSystemInfo = 'ManageSystemInfo',
  ChangeExternalCanister = 'ChangeExternalCanister',
  CreateExternalCanister = 'CreateExternalCanister',
  CallExternalCanister = 'CallExternalCanister',
  ConfigureExternalCanister = 'ConfigureExternalCanister',
  FundExternalCanister = 'FundExternalCanister',
  SnapshotExternalCanister = 'SnapshotExternalCanister',
  PruneExternalCanister = 'PruneExternalCanister',
  RestoreExternalCanister = 'RestoreExternalCanister',
  SetDisasterRecovery = 'SetDisasterRecovery',
  AddAsset = 'AddAsset',
  EditAsset = 'EditAsset',
  RemoveAsset = 'RemoveAsset',
}
