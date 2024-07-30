import {
  DisplayUser,
  RequestEvaluationResult,
  ListRequestsOperationType,
  RequestStatusCode,
  UUID,
} from '~/generated/station/station.did';

export enum AccountTransferStatus {
  Created = 'created',
  Failed = 'failed',
  Processing = 'processing',
  Completed = 'completed',
  Unknown = 'unknown',
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
  System = 'system',
}

export interface ListAccountsArgs {
  limit?: number;
  offset?: number;
  searchTerm?: string;
}

export enum ChangeCanisterTargetType {
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
  ChangeCanister = 'ChangeCanister',
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
  SetDisasterRecovery = 'SetDisasterRecovery',
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
  standard?: string;
  ids?: UUID[];
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
  ChangeCanister = 'ChangeCanister',
  Transfer = 'Transfer',
  ManageSystemInfo = 'ManageSystemInfo',
  ChangeExternalCanister = 'ChangeExternalCanister',
  CreateExternalCanister = 'CreateExternalCanister',
  CallExternalCanister = 'CallExternalCanister',
  ConfigureExternalCanister = 'ConfigureExternalCanister',
  SetDisasterRecovery = 'SetDisasterRecovery',
}
