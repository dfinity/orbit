import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type AccessControlUserSpecifier = CommonSpecifier;
export interface AccessPolicy {
  'id' : UUID,
  'resource' : ResourceSpecifier,
  'info' : AccessPolicyInfo,
  'user' : AccessControlUserSpecifier,
}
export interface AccessPolicyInfo {
  'can_delete' : boolean,
  'can_edit' : boolean,
}
export interface Account {
  'id' : UUID,
  'decimals' : number,
  'balance' : [] | [AccountBalanceInfo],
  'owners' : Array<UUID>,
  'metadata' : Array<AccountMetadata>,
  'name' : string,
  'blockchain' : string,
  'address' : string,
  'last_modification_timestamp' : TimestampRFC3339,
  'standard' : string,
  'symbol' : AssetSymbol,
  'policies' : AccountPolicies,
}
export interface AccountBalance {
  'account_id' : UUID,
  'decimals' : number,
  'balance' : bigint,
  'last_update_timestamp' : TimestampRFC3339,
}
export interface AccountBalanceInfo {
  'decimals' : number,
  'balance' : bigint,
  'last_update_timestamp' : TimestampRFC3339,
}
export interface AccountMetadata { 'key' : string, 'value' : string }
export interface AccountPolicies {
  'edit' : [] | [ProposalPolicyCriteria],
  'transfer' : [] | [ProposalPolicyCriteria],
}
export type AccountSpecifier = CommonSpecifier;
export interface AddAccessPolicyOperation {
  'input' : AddAccessPolicyOperationInput,
  'policy_id' : [] | [UUID],
}
export interface AddAccessPolicyOperationInput {
  'resource' : ResourceSpecifier,
  'user' : AccessControlUserSpecifier,
}
export interface AddAccountOperation {
  'account' : [] | [Account],
  'input' : AddAccountOperationInput,
}
export interface AddAccountOperationInput {
  'owners' : Array<UUID>,
  'metadata' : Array<AccountMetadata>,
  'name' : string,
  'blockchain' : string,
  'standard' : string,
  'policies' : AccountPolicies,
}
export interface AddAddressBookEntryOperation {
  'address_book_entry' : [] | [AddressBookEntry],
  'input' : AddAddressBookEntryOperationInput,
}
export interface AddAddressBookEntryOperationInput {
  'metadata' : Array<AddressBookMetadata>,
  'blockchain' : string,
  'address' : string,
  'address_owner' : string,
  'standard' : string,
}
export interface AddProposalPolicyOperation {
  'input' : AddProposalPolicyOperationInput,
  'policy' : [] | [ProposalPolicy],
}
export interface AddProposalPolicyOperationInput {
  'specifier' : ProposalSpecifier,
  'criteria' : ProposalPolicyCriteria,
}
export interface AddUserGroupOperation {
  'user_group' : [] | [UserGroup],
  'input' : AddUserGroupOperationInput,
}
export interface AddUserGroupOperationInput { 'name' : string }
export interface AddUserOperation {
  'user' : [] | [User],
  'input' : AddUserOperationInput,
}
export interface AddUserOperationInput {
  'status' : UserStatus,
  'groups' : Array<UUID>,
  'name' : [] | [string],
  'identities' : Array<Principal>,
}
export interface AddressBookEntry {
  'id' : UUID,
  'metadata' : Array<AddressBookMetadata>,
  'blockchain' : string,
  'address' : string,
  'last_modification_timestamp' : string,
  'address_owner' : string,
  'standard' : string,
}
export interface AddressBookMetadata { 'key' : string, 'value' : string }
export interface AssetMetadata { 'key' : string, 'value' : string }
export type AssetSymbol = string;
export interface BasicUser {
  'id' : UUID,
  'status' : UserStatus,
  'name' : string,
}
export type CanisterSettingsActionSpecifier = { 'Read' : null } |
  { 'ReadConfig' : null };
export type ChangeAddressBookMetadata = {
    'OverrideSpecifiedBy' : Array<AddressBookMetadata>
  } |
  { 'RemoveKeys' : Array<string> } |
  { 'ReplaceAllBy' : Array<AddressBookMetadata> };
export type ChangeCanisterActionSpecifier = { 'Create' : null };
export interface ChangeCanisterOperation {
  'input' : ChangeCanisterOperationInput,
}
export interface ChangeCanisterOperationInput {
  'arg' : [] | [Uint8Array | number[]],
  'target' : ChangeCanisterTarget,
  'checksum' : Uint8Array | number[],
  'module' : Uint8Array | number[],
}
export type ChangeCanisterTarget = { 'UpgradeUpgrader' : null } |
  { 'UpgradeCanister' : Principal } |
  { 'UpgradeWallet' : null };
export type CommonActionSpecifier = { 'List' : null } |
  { 'Read' : CommonSpecifier } |
  { 'Delete' : CommonSpecifier } |
  { 'Create' : null } |
  { 'Update' : CommonSpecifier };
export type CommonSpecifier = { 'Id' : Array<UUID> } |
  { 'Any' : null } |
  { 'Group' : Array<UUID> };
export interface Config { 'supported_assets' : Array<WalletAsset> }
export interface CreateProposalInput {
  'title' : [] | [string],
  'execution_plan' : [] | [ProposalExecutionSchedule],
  'summary' : [] | [string],
  'operation' : ProposalOperationInput,
}
export type CreateProposalResult = { 'Ok' : { 'proposal' : Proposal } } |
  { 'Err' : Error };
export interface EditAccessPolicyOperation {
  'input' : EditAccessPolicyOperationInput,
}
export interface EditAccessPolicyOperationInput {
  'resource' : [] | [ResourceSpecifier],
  'user' : [] | [AccessControlUserSpecifier],
  'policy_id' : UUID,
}
export interface EditAccountOperation { 'input' : EditAccountOperationInput }
export interface EditAccountOperationInput {
  'account_id' : UUID,
  'owners' : [] | [Array<UUID>],
  'name' : [] | [string],
  'policies' : [] | [AccountPolicies],
}
export interface EditAddressBookEntryOperation {
  'input' : EditAddressBookEntryOperationInput,
}
export interface EditAddressBookEntryOperationInput {
  'change_metadata' : [] | [ChangeAddressBookMetadata],
  'address_book_entry_id' : UUID,
  'address_owner' : [] | [string],
}
export interface EditProposalPolicyOperation {
  'input' : EditProposalPolicyOperationInput,
}
export interface EditProposalPolicyOperationInput {
  'specifier' : [] | [ProposalSpecifier],
  'criteria' : [] | [ProposalPolicyCriteria],
  'policy_id' : UUID,
}
export interface EditUserGroupOperation {
  'input' : EditUserGroupOperationInput,
}
export interface EditUserGroupOperationInput {
  'name' : string,
  'user_group_id' : UUID,
}
export interface EditUserOperation { 'input' : EditUserOperationInput }
export interface EditUserOperationInput {
  'id' : UUID,
  'groups' : [] | [Array<UUID>],
  'name' : [] | [string],
  'identities' : [] | [Array<Principal>],
}
export interface Error {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export interface FetchAccountBalancesInput { 'account_ids' : Array<UUID> }
export type FetchAccountBalancesResult = {
    'Ok' : { 'balances' : Array<AccountBalance> }
  } |
  { 'Err' : Error };
export interface GetAccessPolicyInput { 'id' : UUID }
export type GetAccessPolicyResult = { 'Ok' : { 'policy' : AccessPolicy } } |
  { 'Err' : Error };
export interface GetAccountInput { 'account_id' : UUID }
export type GetAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : Error };
export interface GetAddressBookEntryInput { 'address_book_entry_id' : UUID }
export type GetAddressBookEntryResult = {
    'Ok' : { 'address_book_entry' : AddressBookEntry }
  } |
  { 'Err' : Error };
export type GetConfigResult = { 'Ok' : { 'config' : Config } } |
  { 'Err' : Error };
export interface GetProposalInput { 'proposal_id' : UUID }
export interface GetProposalPolicyInput { 'id' : UUID }
export type GetProposalPolicyResult = { 'Ok' : { 'policy' : ProposalPolicy } } |
  { 'Err' : Error };
export type GetProposalResult = { 'Ok' : { 'proposal' : Proposal } } |
  { 'Err' : Error };
export interface GetTransfersInput { 'transfer_ids' : Array<UUID> }
export type GetTransfersResult = { 'Ok' : { 'transfers' : Array<Transfer> } } |
  { 'Err' : Error };
export interface GetUserGroupInput { 'user_group_id' : UUID }
export type GetUserGroupResult = { 'Ok' : { 'user_group' : UserGroup } } |
  { 'Err' : Error };
export interface GetUserInput { 'user_id' : UUID }
export type GetUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : Error };
export type HeaderField = [string, string];
export type HealthStatus = { 'Healthy' : null } |
  { 'Uninitialized' : null };
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<HeaderField>,
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<HeaderField>,
  'status_code' : number,
}
export type ListAccessPoliciesInput = PaginationInput;
export type ListAccessPoliciesResult = {
    'Ok' : {
      'total' : bigint,
      'user_groups' : Array<UserGroup>,
      'users' : Array<BasicUser>,
      'next_offset' : [] | [bigint],
      'policies' : Array<AccessPolicy>,
    }
  } |
  { 'Err' : Error };
export interface ListAccountTransfersInput {
  'account_id' : UUID,
  'status' : [] | [TransferStatusType],
  'to_dt' : [] | [TimestampRFC3339],
  'from_dt' : [] | [TimestampRFC3339],
}
export type ListAccountTransfersResult = {
    'Ok' : { 'transfers' : Array<TransferListItem> }
  } |
  { 'Err' : Error };
export interface ListAccountsInput {
  'paginate' : [] | [PaginationInput],
  'search_term' : [] | [string],
}
export type ListAccountsResult = {
    'Ok' : {
      'total' : bigint,
      'accounts' : Array<Account>,
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
export interface ListAddressBookEntriesInput {
  'blockchain' : string,
  'paginate' : PaginationInput,
  'standard' : string,
}
export type ListAddressBookEntriesResult = {
    'Ok' : {
      'total' : bigint,
      'address_book_entries' : Array<AddressBookEntry>,
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
export interface ListNotificationsInput {
  'status' : [] | [NotificationStatus],
  'to_dt' : [] | [TimestampRFC3339],
  'from_dt' : [] | [TimestampRFC3339],
  'notification_type' : [] | [NotificationTypeInput],
}
export type ListNotificationsResult = {
    'Ok' : { 'notifications' : Array<Notification> }
  } |
  { 'Err' : Error };
export type ListProposalPoliciesInput = PaginationInput;
export type ListProposalPoliciesResult = {
    'Ok' : {
      'total' : bigint,
      'next_offset' : [] | [bigint],
      'policies' : Array<ProposalPolicy>,
    }
  } |
  { 'Err' : Error };
export interface ListProposalsInput {
  'sort_by' : [] | [ListProposalsSortBy],
  'voter_ids' : [] | [Array<UUID>],
  'expiration_from_dt' : [] | [TimestampRFC3339],
  'created_to_dt' : [] | [TimestampRFC3339],
  'statuses' : [] | [Array<ProposalStatusCode>],
  'proposer_ids' : [] | [Array<UUID>],
  'expiration_to_dt' : [] | [TimestampRFC3339],
  'paginate' : [] | [PaginationInput],
  'operation_types' : [] | [Array<ListProposalsOperationType>],
  'created_from_dt' : [] | [TimestampRFC3339],
}
export type ListProposalsOperationType = { 'EditAccessPolicy' : null } |
  { 'AddUserGroup' : null } |
  { 'RemoveProposalPolicy' : null } |
  { 'AddUser' : null } |
  { 'EditUserGroup' : null } |
  { 'RemoveAddressBookEntry' : null } |
  { 'EditAddressBookEntry' : null } |
  { 'AddProposalPolicy' : null } |
  { 'ChangeCanister' : null } |
  { 'EditProposalPolicy' : null } |
  { 'EditUser' : null } |
  { 'Transfer' : [] | [UUID] } |
  { 'EditAccount' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'AddAccessPolicy' : null } |
  { 'RemoveAccessPolicy' : null } |
  { 'RemoveUserGroup' : null } |
  { 'AddAccount' : null };
export type ListProposalsResult = {
    'Ok' : {
      'total' : bigint,
      'proposals' : Array<Proposal>,
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
export type ListProposalsSortBy = { 'ExpirationDt' : SortByDirection } |
  { 'LastModificationDt' : SortByDirection } |
  { 'CreatedAt' : SortByDirection };
export interface ListUserGroupsInput {
  'paginate' : [] | [PaginationInput],
  'search_term' : [] | [string],
}
export type ListUserGroupsResult = {
    'Ok' : {
      'total' : bigint,
      'user_groups' : Array<UserGroup>,
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
export interface ListUsersInput {
  'statuses' : [] | [Array<UserStatus>],
  'paginate' : [] | [PaginationInput],
  'search_term' : [] | [string],
}
export type ListUsersResult = {
    'Ok' : {
      'total' : bigint,
      'users' : Array<User>,
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
export type MarkNotificationReadResult = { 'Ok' : null } |
  { 'Err' : Error };
export interface MarkNotificationsReadInput {
  'notification_ids' : Array<UUID>,
  'read' : boolean,
}
export type MeResult = {
    'Ok' : { 'me' : User, 'privileges' : Array<UserPrivilege> }
  } |
  { 'Err' : Error };
export interface Network { 'id' : NetworkId, 'name' : string }
export type NetworkId = string;
export interface Notification {
  'id' : UUID,
  'status' : NotificationStatus,
  'title' : string,
  'created_at' : TimestampRFC3339,
  'notification_type' : NotificationType,
  'message' : [] | [string],
  'target_user_id' : UUID,
}
export type NotificationStatus = { 'Read' : null } |
  { 'Sent' : null };
export type NotificationType = {
    'ProposalCreated' : {
      'account_id' : [] | [UUID],
      'operation_type' : ProposalOperationType,
      'user_id' : [] | [UUID],
      'proposal_id' : UUID,
    }
  } |
  { 'SystemMessage' : null };
export type NotificationTypeInput = { 'ProposalCreated' : null } |
  { 'SystemMessage' : null };
export interface PaginationInput {
  'offset' : [] | [bigint],
  'limit' : [] | [number],
}
export interface Proposal {
  'id' : UUID,
  'status' : ProposalStatus,
  'title' : string,
  'execution_plan' : ProposalExecutionSchedule,
  'expiration_dt' : TimestampRFC3339,
  'votes' : Array<ProposalVote>,
  'info' : ProposalInfo,
  'created_at' : TimestampRFC3339,
  'summary' : [] | [string],
  'operation' : ProposalOperation,
  'proposed_by' : UUID,
}
export type ProposalActionSpecifier = { 'List' : null } |
  { 'Read' : CommonSpecifier };
export type ProposalExecutionSchedule = { 'Immediate' : null } |
  { 'Scheduled' : { 'execution_time' : TimestampRFC3339 } };
export interface ProposalInfo {
  'can_vote' : boolean,
  'proposer_name' : [] | [string],
}
export type ProposalOperation = {
    'EditAccessPolicy' : EditAccessPolicyOperation
  } |
  { 'AddUserGroup' : AddUserGroupOperation } |
  { 'RemoveProposalPolicy' : RemoveProposalPolicyOperation } |
  { 'AddUser' : AddUserOperation } |
  { 'EditUserGroup' : EditUserGroupOperation } |
  { 'RemoveAddressBookEntry' : RemoveAddressBookEntryOperation } |
  { 'EditAddressBookEntry' : EditAddressBookEntryOperation } |
  { 'AddProposalPolicy' : AddProposalPolicyOperation } |
  { 'ChangeCanister' : ChangeCanisterOperation } |
  { 'EditProposalPolicy' : EditProposalPolicyOperation } |
  { 'EditUser' : EditUserOperation } |
  { 'Transfer' : TransferOperation } |
  { 'EditAccount' : EditAccountOperation } |
  { 'AddAddressBookEntry' : AddAddressBookEntryOperation } |
  { 'AddAccessPolicy' : AddAccessPolicyOperation } |
  { 'RemoveAccessPolicy' : RemoveAccessPolicyOperation } |
  { 'RemoveUserGroup' : RemoveUserGroupOperation } |
  { 'AddAccount' : AddAccountOperation };
export type ProposalOperationInput = {
    'EditAccessPolicy' : EditAccessPolicyOperationInput
  } |
  { 'AddUserGroup' : AddUserGroupOperationInput } |
  { 'RemoveProposalPolicy' : RemoveProposalPolicyOperationInput } |
  { 'AddUser' : AddUserOperationInput } |
  { 'EditUserGroup' : EditUserGroupOperationInput } |
  { 'RemoveAddressBookEntry' : RemoveAddressBookEntryOperationInput } |
  { 'EditAddressBookEntry' : EditAddressBookEntryOperationInput } |
  { 'AddProposalPolicy' : AddProposalPolicyOperationInput } |
  { 'ChangeCanister' : ChangeCanisterOperationInput } |
  { 'EditProposalPolicy' : EditProposalPolicyOperationInput } |
  { 'EditUser' : EditUserOperationInput } |
  { 'Transfer' : TransferOperationInput } |
  { 'EditAccount' : EditAccountOperationInput } |
  { 'AddAddressBookEntry' : AddAddressBookEntryOperationInput } |
  { 'AddAccessPolicy' : AddAccessPolicyOperationInput } |
  { 'RemoveAccessPolicy' : RemoveAccessPolicyOperationInput } |
  { 'RemoveUserGroup' : RemoveUserGroupOperationInput } |
  { 'AddAccount' : AddAccountOperationInput };
export type ProposalOperationType = { 'EditAccessPolicy' : null } |
  { 'AddUserGroup' : null } |
  { 'RemoveProposalPolicy' : null } |
  { 'AddUser' : null } |
  { 'EditUserGroup' : null } |
  { 'RemoveAddressBookEntry' : null } |
  { 'EditAddressBookEntry' : null } |
  { 'AddProposalPolicy' : null } |
  { 'ChangeCanister' : null } |
  { 'EditProposalPolicy' : null } |
  { 'EditUser' : null } |
  { 'Transfer' : null } |
  { 'EditAccount' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'AddAccessPolicy' : null } |
  { 'RemoveAccessPolicy' : null } |
  { 'RemoveUserGroup' : null } |
  { 'AddAccount' : null };
export interface ProposalPolicy {
  'id' : UUID,
  'specifier' : ProposalSpecifier,
  'criteria' : ProposalPolicyCriteria,
}
export type ProposalPolicyCriteria = { 'Or' : Array<ProposalPolicyCriteria> } |
  { 'And' : Array<ProposalPolicyCriteria> } |
  { 'Not' : ProposalPolicyCriteria } |
  { 'HasAddressBookMetadata' : AddressBookMetadata } |
  { 'MinimumVotes' : [UserSpecifier, number] } |
  { 'ApprovalThreshold' : [UserSpecifier, number] } |
  { 'AutoAdopted' : null };
export type ProposalSpecifier = { 'EditAccessPolicy' : CommonSpecifier } |
  { 'AddUserGroup' : null } |
  { 'RemoveProposalPolicy' : CommonSpecifier } |
  { 'AddUser' : null } |
  { 'EditUserGroup' : CommonSpecifier } |
  { 'RemoveAddressBookEntry' : CommonSpecifier } |
  { 'EditAddressBookEntry' : CommonSpecifier } |
  { 'AddProposalPolicy' : null } |
  { 'ChangeCanister' : null } |
  { 'EditProposalPolicy' : CommonSpecifier } |
  { 'EditUser' : UserSpecifier } |
  { 'Transfer' : TransferSpecifier } |
  { 'EditAccount' : AccountSpecifier } |
  { 'AddAddressBookEntry' : null } |
  { 'AddAccessPolicy' : null } |
  { 'RemoveAccessPolicy' : CommonSpecifier } |
  { 'RemoveUserGroup' : CommonSpecifier } |
  { 'AddAccount' : null };
export type ProposalStatus = { 'Failed' : { 'reason' : [] | [string] } } |
  { 'Rejected' : null } |
  { 'Scheduled' : { 'scheduled_at' : TimestampRFC3339 } } |
  { 'Adopted' : null } |
  { 'Cancelled' : { 'reason' : [] | [string] } } |
  { 'Processing' : { 'started_at' : TimestampRFC3339 } } |
  { 'Created' : null } |
  { 'Completed' : { 'completed_at' : TimestampRFC3339 } };
export type ProposalStatusCode = { 'Failed' : null } |
  { 'Rejected' : null } |
  { 'Scheduled' : null } |
  { 'Adopted' : null } |
  { 'Cancelled' : null } |
  { 'Processing' : null } |
  { 'Created' : null } |
  { 'Completed' : null };
export interface ProposalVote {
  'status' : ProposalVoteStatus,
  'user_id' : UUID,
  'status_reason' : [] | [string],
  'decided_at' : TimestampRFC3339,
}
export type ProposalVoteStatus = { 'Rejected' : null } |
  { 'Accepted' : null };
export interface RemoveAccessPolicyOperation {
  'input' : RemoveAccessPolicyOperationInput,
}
export interface RemoveAccessPolicyOperationInput { 'policy_id' : UUID }
export interface RemoveAddressBookEntryOperation {
  'input' : RemoveAddressBookEntryOperationInput,
}
export interface RemoveAddressBookEntryOperationInput {
  'address_book_entry_id' : UUID,
}
export interface RemoveProposalPolicyOperation {
  'input' : RemoveProposalPolicyOperationInput,
}
export interface RemoveProposalPolicyOperationInput { 'policy_id' : UUID }
export interface RemoveUserGroupOperation {
  'input' : RemoveUserGroupOperationInput,
}
export interface RemoveUserGroupOperationInput { 'user_group_id' : UUID }
export type ResourceSpecifier = { 'User' : CommonActionSpecifier } |
  { 'ProposalPolicy' : CommonActionSpecifier } |
  { 'Account' : CommonActionSpecifier } |
  { 'AddressBook' : CommonActionSpecifier } |
  { 'Proposal' : ProposalActionSpecifier } |
  { 'ChangeCanister' : ChangeCanisterActionSpecifier } |
  { 'AccessPolicy' : CommonActionSpecifier } |
  { 'Transfer' : TransferActionSpecifier } |
  { 'UserGroup' : CommonActionSpecifier } |
  { 'CanisterSettings' : CanisterSettingsActionSpecifier };
export type SortByDirection = { 'Asc' : null } |
  { 'Desc' : null };
export type TimestampRFC3339 = string;
export interface Transfer {
  'id' : UUID,
  'to' : string,
  'fee' : bigint,
  'status' : TransferStatus,
  'from_account_id' : UUID,
  'metadata' : Array<TransferMetadata>,
  'network' : Network,
  'amount' : bigint,
}
export type TransferActionSpecifier = { 'Read' : TransferSpecifier } |
  { 'Delete' : TransferSpecifier } |
  { 'Create' : TransferSpecifier };
export interface TransferListItem {
  'to' : string,
  'status' : TransferStatus,
  'created_at' : TimestampRFC3339,
  'transfer_id' : UUID,
  'amount' : bigint,
}
export interface TransferMetadata { 'key' : string, 'value' : string }
export interface TransferOperation {
  'network' : Network,
  'from_account' : [] | [Account],
  'input' : TransferOperationInput,
}
export interface TransferOperationInput {
  'to' : string,
  'fee' : [] | [bigint],
  'from_account_id' : UUID,
  'metadata' : Array<TransferMetadata>,
  'network' : [] | [Network],
  'amount' : bigint,
}
export interface TransferSpecifier { 'account' : CommonSpecifier }
export type TransferStatus = { 'Failed' : { 'reason' : string } } |
  { 'Processing' : { 'started_at' : TimestampRFC3339 } } |
  { 'Created' : null } |
  {
    'Completed' : {
      'signature' : [] | [string],
      'hash' : [] | [string],
      'completed_at' : TimestampRFC3339,
    }
  };
export type TransferStatusType = { 'Failed' : null } |
  { 'Processing' : null } |
  { 'Created' : null } |
  { 'Completed' : null };
export type UUID = string;
export interface User {
  'id' : UUID,
  'status' : UserStatus,
  'groups' : Array<UserGroup>,
  'name' : [] | [string],
  'last_modification_timestamp' : TimestampRFC3339,
  'identities' : Array<Principal>,
}
export interface UserGroup { 'id' : UserGroupId, 'name' : string }
export type UserGroupId = UUID;
export type UserPrivilege = { 'AddUserGroup' : null } |
  { 'ListProposals' : null } |
  { 'ListUserGroups' : null } |
  { 'AddUser' : null } |
  { 'ListUsers' : null } |
  { 'AddProposalPolicy' : null } |
  { 'UpgradeCanister' : null } |
  { 'ListProposalPolicies' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'ListAccounts' : null } |
  { 'ListAccessPolicies' : null } |
  { 'AddAccessPolicy' : null } |
  { 'ListAddressBookEntries' : null } |
  { 'AddAccount' : null };
export type UserSpecifier = { 'Id' : Array<UUID> } |
  { 'Any' : null } |
  { 'Group' : Array<UserGroupId> } |
  { 'Proposer' : null } |
  { 'Owner' : null };
export type UserStatus = { 'Inactive' : null } |
  { 'Active' : null };
export interface VoteOnProposalInput {
  'approve' : boolean,
  'proposal_id' : UUID,
  'reason' : [] | [string],
}
export type VoteOnProposalResult = { 'Ok' : { 'proposal' : Proposal } } |
  { 'Err' : Error };
export interface WalletAsset {
  'standards' : Array<string>,
  'metadata' : Array<AssetMetadata>,
  'name' : string,
  'blockchain' : string,
  'symbol' : AssetSymbol,
}
export interface WalletInit {
  'owners' : [] | [Array<Principal>],
  'upgrader_wasm_module' : Uint8Array | number[],
}
export type WalletInstall = { 'Upgrade' : WalletUpgrade } |
  { 'Init' : WalletInit };
export interface WalletSettings {
  'owners' : Array<User>,
  'last_upgrade_timestamp' : TimestampRFC3339,
}
export type WalletSettingsResult = { 'Ok' : { 'settings' : WalletSettings } } |
  { 'Err' : Error };
export interface WalletUpgrade { 'owners' : [] | [Array<Principal>] }
export interface _SERVICE {
  'config' : ActorMethod<[], GetConfigResult>,
  'create_proposal' : ActorMethod<[CreateProposalInput], CreateProposalResult>,
  'fetch_account_balances' : ActorMethod<
    [FetchAccountBalancesInput],
    FetchAccountBalancesResult
  >,
  'get_access_policy' : ActorMethod<
    [GetAccessPolicyInput],
    GetAccessPolicyResult
  >,
  'get_account' : ActorMethod<[GetAccountInput], GetAccountResult>,
  'get_address_book_entry' : ActorMethod<
    [GetAddressBookEntryInput],
    GetAddressBookEntryResult
  >,
  'get_proposal' : ActorMethod<[GetProposalInput], GetProposalResult>,
  'get_proposal_policy' : ActorMethod<
    [GetProposalPolicyInput],
    GetProposalPolicyResult
  >,
  'get_transfers' : ActorMethod<[GetTransfersInput], GetTransfersResult>,
  'get_user' : ActorMethod<[GetUserInput], GetUserResult>,
  'get_user_group' : ActorMethod<[GetUserGroupInput], GetUserGroupResult>,
  'health_status' : ActorMethod<[], HealthStatus>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'list_access_policies' : ActorMethod<
    [ListAccessPoliciesInput],
    ListAccessPoliciesResult
  >,
  'list_account_transfers' : ActorMethod<
    [ListAccountTransfersInput],
    ListAccountTransfersResult
  >,
  'list_accounts' : ActorMethod<[ListAccountsInput], ListAccountsResult>,
  'list_address_book_entries' : ActorMethod<
    [ListAddressBookEntriesInput],
    ListAddressBookEntriesResult
  >,
  'list_notifications' : ActorMethod<
    [ListNotificationsInput],
    ListNotificationsResult
  >,
  'list_proposal_policies' : ActorMethod<
    [ListProposalPoliciesInput],
    ListProposalPoliciesResult
  >,
  'list_proposals' : ActorMethod<[ListProposalsInput], ListProposalsResult>,
  'list_user_groups' : ActorMethod<[ListUserGroupsInput], ListUserGroupsResult>,
  'list_users' : ActorMethod<[ListUsersInput], ListUsersResult>,
  'mark_notifications_read' : ActorMethod<
    [MarkNotificationsReadInput],
    MarkNotificationReadResult
  >,
  'me' : ActorMethod<[], MeResult>,
  'vote_on_proposal' : ActorMethod<[VoteOnProposalInput], VoteOnProposalResult>,
  'wallet_settings' : ActorMethod<[], WalletSettingsResult>,
}
