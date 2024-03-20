import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type AccessControlUserSpecifier = CommonSpecifier;
export interface AccessPolicy { 'resource' : Resource, 'allow' : Allow }
export interface AccessPolicyCallerPrivileges {
  'resource' : Resource,
  'can_edit' : boolean,
}
export type AccessPolicyResourceAction = { 'Read' : null } |
  { 'Update' : null };
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
export interface AccountCallerPrivileges {
  'id' : UUID,
  'can_transfer' : boolean,
  'can_edit' : boolean,
}
export interface AccountMetadata { 'key' : string, 'value' : string }
export interface AccountPolicies {
  'edit' : [] | [ProposalPolicyCriteria],
  'transfer' : [] | [ProposalPolicyCriteria],
}
export type AccountResourceAction = { 'List' : null } |
  { 'Read' : ResourceId } |
  { 'Create' : null } |
  { 'Transfer' : ResourceId } |
  { 'Update' : ResourceId };
export type AccountSpecifier = CommonSpecifier;
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
  'policy_id' : [] | [UUID],
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
export interface AddressBookEntryCallerPrivileges {
  'id' : UUID,
  'can_delete' : boolean,
  'can_edit' : boolean,
}
export interface AddressBookMetadata { 'key' : string, 'value' : string }
export interface Allow {
  'user_groups' : Array<UUID>,
  'auth_scope' : AuthScope,
  'users' : Array<UUID>,
}
export interface ApprovalThreshold {
  'threshold' : number,
  'voters' : UserSpecifier,
}
export interface AssetMetadata { 'key' : string, 'value' : string }
export type AssetSymbol = string;
export type AuthScope = { 'Authenticated' : null } |
  { 'Public' : null } |
  { 'Restricted' : null };
export interface BasicUser {
  'id' : UUID,
  'status' : UserStatus,
  'name' : string,
}
export type ChangeAddressBookMetadata = {
    'OverrideSpecifiedBy' : Array<AddressBookMetadata>
  } |
  { 'RemoveKeys' : Array<string> } |
  { 'ReplaceAllBy' : Array<AddressBookMetadata> };
export interface ChangeCanisterOperation {
  'module_checksum' : Sha256Hash,
  'target' : ChangeCanisterTarget,
  'arg_checksum' : [] | [Sha256Hash],
}
export interface ChangeCanisterOperationInput {
  'arg' : [] | [Uint8Array | number[]],
  'target' : ChangeCanisterTarget,
  'module' : Uint8Array | number[],
}
export type ChangeCanisterResourceAction = { 'Create' : null };
export type ChangeCanisterTarget = { 'UpgradeUpgrader' : null } |
  { 'UpgradeCanister' : Principal } |
  { 'UpgradeWallet' : null };
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
export type CreateProposalResult = {
    'Ok' : {
      'privileges' : ProposalCallerPrivileges,
      'proposal' : Proposal,
      'additional_info' : ProposalAdditionalInfo,
    }
  } |
  { 'Err' : Error };
export interface DisplayUser { 'id' : UUID, 'name' : [] | [string] }
export interface EditAccessPolicyOperation {
  'input' : EditAccessPolicyOperationInput,
}
export interface EditAccessPolicyOperationInput {
  'resource' : Resource,
  'user_groups' : [] | [Array<UUID>],
  'auth_scope' : [] | [AuthScope],
  'users' : [] | [Array<UUID>],
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
  'status' : [] | [UserStatus],
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
export interface GetAccessPolicyInput { 'resource' : Resource }
export type GetAccessPolicyResult = {
    'Ok' : {
      'privileges' : AccessPolicyCallerPrivileges,
      'policy' : AccessPolicy,
    }
  } |
  { 'Err' : Error };
export interface GetAccountInput { 'account_id' : UUID }
export type GetAccountResult = {
    'Ok' : { 'privileges' : AccountCallerPrivileges, 'account' : Account }
  } |
  { 'Err' : Error };
export interface GetAddressBookEntryInput { 'address_book_entry_id' : UUID }
export type GetAddressBookEntryResult = {
    'Ok' : {
      'privileges' : AddressBookEntryCallerPrivileges,
      'address_book_entry' : AddressBookEntry,
    }
  } |
  { 'Err' : Error };
export type GetConfigResult = { 'Ok' : { 'config' : Config } } |
  { 'Err' : Error };
export interface GetNextVotableProposalInput {
  'excluded_proposal_ids' : Array<UUID>,
  'operation_types' : [] | [Array<ListProposalsOperationType>],
}
export type GetNextVotableProposalResponse = {
    'Ok' : [] | [GetProposalResultData]
  } |
  { 'Err' : Error };
export interface GetProposalInput { 'proposal_id' : UUID }
export interface GetProposalPolicyInput { 'id' : UUID }
export type GetProposalPolicyResult = {
    'Ok' : {
      'privileges' : ProposalPolicyCallerPrivileges,
      'policy' : ProposalPolicy,
    }
  } |
  { 'Err' : Error };
export type GetProposalResult = { 'Ok' : GetProposalResultData } |
  { 'Err' : Error };
export interface GetProposalResultData {
  'privileges' : ProposalCallerPrivileges,
  'proposal' : Proposal,
  'additional_info' : ProposalAdditionalInfo,
}
export interface GetTransfersInput { 'transfer_ids' : Array<UUID> }
export type GetTransfersResult = { 'Ok' : { 'transfers' : Array<Transfer> } } |
  { 'Err' : Error };
export interface GetUserGroupInput { 'user_group_id' : UUID }
export type GetUserGroupResult = {
    'Ok' : {
      'privileges' : UserGroupCallerPrivileges,
      'user_group' : UserGroup,
    }
  } |
  { 'Err' : Error };
export interface GetUserInput { 'user_id' : UUID }
export type GetUserResult = {
    'Ok' : { 'privileges' : UserCallerPrivileges, 'user' : User }
  } |
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
export interface ListAccessPoliciesInput {
  'resources' : [] | [Array<Resource>],
  'paginate' : [] | [PaginationInput],
}
export type ListAccessPoliciesResult = {
    'Ok' : {
      'total' : bigint,
      'privileges' : Array<AccessPolicyCallerPrivileges>,
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
      'privileges' : Array<AccountCallerPrivileges>,
      'accounts' : Array<Account>,
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
export interface ListAddressBookEntriesInput {
  'ids' : [] | [Array<UUID>],
  'addresses' : [] | [Array<string>],
  'paginate' : [] | [PaginationInput],
  'address_chain' : [] | [{ 'blockchain' : string, 'standard' : string }],
}
export type ListAddressBookEntriesResult = {
    'Ok' : {
      'total' : bigint,
      'privileges' : Array<AddressBookEntryCallerPrivileges>,
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
      'privileges' : Array<ProposalPolicyCallerPrivileges>,
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
  'only_votable' : boolean,
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
  { 'RemoveUserGroup' : null } |
  { 'AddAccount' : null };
export type ListProposalsResult = {
    'Ok' : {
      'total' : bigint,
      'privileges' : Array<ProposalCallerPrivileges>,
      'proposals' : Array<Proposal>,
      'next_offset' : [] | [bigint],
      'additional_info' : Array<ProposalAdditionalInfo>,
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
      'privileges' : Array<UserGroupCallerPrivileges>,
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
      'privileges' : Array<UserCallerPrivileges>,
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
export interface MinimumVotes { 'minimum' : number, 'voters' : UserSpecifier }
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
  'created_at' : TimestampRFC3339,
  'summary' : [] | [string],
  'operation' : ProposalOperation,
  'proposed_by' : UUID,
}
export interface ProposalAdditionalInfo {
  'id' : UUID,
  'voters' : Array<DisplayUser>,
  'proposer_name' : [] | [string],
}
export interface ProposalCallerPrivileges { 'id' : UUID, 'can_vote' : boolean }
export type ProposalExecutionSchedule = { 'Immediate' : null } |
  { 'Scheduled' : { 'execution_time' : TimestampRFC3339 } };
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
  { 'RemoveUserGroup' : null } |
  { 'AddAccount' : null };
export interface ProposalPolicy {
  'id' : UUID,
  'specifier' : ProposalSpecifier,
  'criteria' : ProposalPolicyCriteria,
}
export interface ProposalPolicyCallerPrivileges {
  'id' : UUID,
  'can_delete' : boolean,
  'can_edit' : boolean,
}
export type ProposalPolicyCriteria = { 'Or' : Array<ProposalPolicyCriteria> } |
  { 'And' : Array<ProposalPolicyCriteria> } |
  { 'Not' : ProposalPolicyCriteria } |
  { 'HasAddressBookMetadata' : AddressBookMetadata } |
  { 'MinimumVotes' : MinimumVotes } |
  { 'ApprovalThreshold' : ApprovalThreshold } |
  { 'AutoAdopted' : null };
export type ProposalResourceAction = { 'List' : null } |
  { 'Read' : ResourceId };
export type ProposalSpecifier = { 'EditAccessPolicy' : ResourceSpecifier } |
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
export type Resource = { 'User' : UserResourceAction } |
  { 'ProposalPolicy' : ResourceAction } |
  { 'Settings' : SettingsResourceAction } |
  { 'Account' : AccountResourceAction } |
  { 'AddressBook' : ResourceAction } |
  { 'Proposal' : ProposalResourceAction } |
  { 'ChangeCanister' : ChangeCanisterResourceAction } |
  { 'AccessPolicy' : AccessPolicyResourceAction } |
  { 'UserGroup' : ResourceAction };
export type ResourceAction = { 'List' : null } |
  { 'Read' : ResourceId } |
  { 'Delete' : ResourceId } |
  { 'Create' : null } |
  { 'Update' : ResourceId };
export type ResourceId = { 'Id' : UUID } |
  { 'Any' : null };
export type ResourceSpecifier = { 'Any' : null } |
  { 'Resource' : Resource };
export type SettingsResourceAction = { 'Read' : null } |
  { 'ReadConfig' : null };
export type Sha256Hash = string;
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
  'proposal_id' : UUID,
  'amount' : bigint,
}
export interface TransferListItem {
  'to' : string,
  'status' : TransferStatus,
  'created_at' : TimestampRFC3339,
  'transfer_id' : UUID,
  'proposal_id' : UUID,
  'amount' : bigint,
}
export interface TransferMetadata { 'key' : string, 'value' : string }
export interface TransferOperation {
  'network' : Network,
  'transfer_id' : [] | [UUID],
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
export interface UserCallerPrivileges { 'id' : UUID, 'can_edit' : boolean }
export interface UserGroup { 'id' : UUID, 'name' : string }
export interface UserGroupCallerPrivileges {
  'id' : UUID,
  'can_delete' : boolean,
  'can_edit' : boolean,
}
export type UserPrivilege = { 'AddUserGroup' : null } |
  { 'ListProposals' : null } |
  { 'ListUserGroups' : null } |
  { 'AddUser' : null } |
  { 'ListUsers' : null } |
  { 'AddProposalPolicy' : null } |
  { 'ChangeCanister' : null } |
  { 'ListProposalPolicies' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'ListAccounts' : null } |
  { 'ListAccessPolicies' : null } |
  { 'ListAddressBookEntries' : null } |
  { 'AddAccount' : null };
export type UserResourceAction = { 'List' : null } |
  { 'Read' : ResourceId } |
  { 'Create' : null } |
  { 'Update' : ResourceId };
export type UserSpecifier = { 'Id' : Array<UUID> } |
  { 'Any' : null } |
  { 'Group' : Array<UUID> } |
  { 'Proposer' : null } |
  { 'Owner' : null };
export type UserStatus = { 'Inactive' : null } |
  { 'Active' : null };
export interface VoteOnProposalInput {
  'approve' : boolean,
  'proposal_id' : UUID,
  'reason' : [] | [string],
}
export type VoteOnProposalResult = {
    'Ok' : {
      'privileges' : ProposalCallerPrivileges,
      'proposal' : Proposal,
      'additional_info' : ProposalAdditionalInfo,
    }
  } |
  { 'Err' : Error };
export interface WalletAsset {
  'metadata' : Array<AssetMetadata>,
  'name' : string,
  'blockchain' : string,
  'standard' : string,
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
  'get_next_votable_proposal' : ActorMethod<
    [GetNextVotableProposalInput],
    GetNextVotableProposalResponse
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
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
