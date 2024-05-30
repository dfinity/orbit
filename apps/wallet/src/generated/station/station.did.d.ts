import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'id' : UUID,
  'configs_request_policy' : [] | [RequestPolicyRule],
  'decimals' : number,
  'balance' : [] | [AccountBalanceInfo],
  'metadata' : Array<AccountMetadata>,
  'name' : string,
  'blockchain' : string,
  'address' : string,
  'transfer_request_policy' : [] | [RequestPolicyRule],
  'last_modification_timestamp' : TimestampRFC3339,
  'standard' : string,
  'symbol' : AssetSymbol,
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
export type AccountResourceAction = { 'List' : null } |
  { 'Read' : ResourceId } |
  { 'Create' : null } |
  { 'Transfer' : ResourceId } |
  { 'Update' : ResourceId };
export interface AddAccountOperation {
  'account' : [] | [Account],
  'input' : AddAccountOperationInput,
}
export interface AddAccountOperationInput {
  'configs_request_policy' : [] | [RequestPolicyRule],
  'read_permission' : Allow,
  'configs_permission' : Allow,
  'metadata' : Array<AccountMetadata>,
  'name' : string,
  'blockchain' : string,
  'transfer_request_policy' : [] | [RequestPolicyRule],
  'transfer_permission' : Allow,
  'standard' : string,
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
export interface AddRequestPolicyOperation {
  'input' : AddRequestPolicyOperationInput,
  'policy_id' : [] | [UUID],
}
export interface AddRequestPolicyOperationInput {
  'rule' : RequestPolicyRule,
  'specifier' : RequestSpecifier,
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
  'name' : string,
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
export interface AdminInitInput { 'name' : string, 'identity' : Principal }
export interface Allow {
  'user_groups' : Array<UUID>,
  'auth_scope' : AuthScope,
  'users' : Array<UUID>,
}
export interface Asset {
  'metadata' : Array<AssetMetadata>,
  'name' : string,
  'blockchain' : string,
  'standard' : string,
  'symbol' : AssetSymbol,
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
export type CanisterInstallMode = { 'reinstall' : null } |
  { 'upgrade' : null } |
  { 'install' : null };
export interface Capabilities {
  'name' : string,
  'version' : string,
  'supported_assets' : Array<Asset>,
}
export type CapabilitiesResult = { 'Ok' : { 'capabilities' : Capabilities } } |
  { 'Err' : Error };
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
export type ChangeCanisterResourceAction = {
    'Create' : ChangeCanisterResourceTarget
  };
export type ChangeCanisterResourceTarget = { 'Any' : null } |
  { 'Canister' : Principal };
export type ChangeCanisterTarget = { 'UpgradeUpgrader' : null } |
  { 'InstallCanister' : InstallCanisterInput } |
  { 'UpgradeStation' : null };
export interface CreateRequestInput {
  'title' : [] | [string],
  'execution_plan' : [] | [RequestExecutionSchedule],
  'summary' : [] | [string],
  'operation' : RequestOperationInput,
}
export type CreateRequestResult = {
    'Ok' : {
      'privileges' : RequestCallerPrivileges,
      'request' : Request,
      'additional_info' : RequestAdditionalInfo,
    }
  } |
  { 'Err' : Error };
export interface DisplayUser { 'id' : UUID, 'name' : string }
export interface EditAccountOperation { 'input' : EditAccountOperationInput }
export interface EditAccountOperationInput {
  'account_id' : UUID,
  'configs_request_policy' : [] | [RequestPolicyRuleInput],
  'read_permission' : [] | [Allow],
  'configs_permission' : [] | [Allow],
  'name' : [] | [string],
  'transfer_request_policy' : [] | [RequestPolicyRuleInput],
  'transfer_permission' : [] | [Allow],
}
export interface EditAddressBookEntryOperation {
  'input' : EditAddressBookEntryOperationInput,
}
export interface EditAddressBookEntryOperationInput {
  'change_metadata' : [] | [ChangeAddressBookMetadata],
  'address_book_entry_id' : UUID,
  'address_owner' : [] | [string],
}
export interface EditPermissionOperation {
  'input' : EditPermissionOperationInput,
}
export interface EditPermissionOperationInput {
  'resource' : Resource,
  'user_groups' : [] | [Array<UUID>],
  'auth_scope' : [] | [AuthScope],
  'users' : [] | [Array<UUID>],
}
export interface EditRequestPolicyOperation {
  'input' : EditRequestPolicyOperationInput,
}
export interface EditRequestPolicyOperationInput {
  'rule' : [] | [RequestPolicyRule],
  'specifier' : [] | [RequestSpecifier],
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
export type EvaluatedRequestPolicyRule = { 'Not' : RequestPolicyRuleResult } |
  {
    'Quorum' : {
      'total_possible_approvers' : bigint,
      'min_approved' : bigint,
      'approvers' : Array<UUID>,
    }
  } |
  { 'AllowListed' : null } |
  {
    'QuorumPercentage' : {
      'total_possible_approvers' : bigint,
      'min_approved' : bigint,
      'approvers' : Array<UUID>,
    }
  } |
  { 'AutoApproved' : null } |
  { 'AllOf' : Array<RequestPolicyRuleResult> } |
  { 'AnyOf' : Array<RequestPolicyRuleResult> } |
  { 'AllowListedByMetadata' : { 'metadata' : AddressBookMetadata } };
export type EvaluationStatus = { 'Approved' : null } |
  { 'Rejected' : null } |
  { 'Pending' : null };
export type EvaluationSummaryReason = { 'AllowList' : null } |
  { 'AllowListMetadata' : null } |
  { 'AutoApproved' : null } |
  { 'ApprovalQuorum' : null };
export interface FetchAccountBalancesInput { 'account_ids' : Array<UUID> }
export type FetchAccountBalancesResult = {
    'Ok' : { 'balances' : Array<AccountBalance> }
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
export interface GetNextApprovableRequestInput {
  'excluded_request_ids' : Array<UUID>,
  'operation_types' : [] | [Array<ListRequestsOperationType>],
}
export type GetNextApprovableRequestResult = {
    'Ok' : [] | [GetRequestResultData]
  } |
  { 'Err' : Error };
export interface GetPermissionInput { 'resource' : Resource }
export type GetPermissionResult = {
    'Ok' : {
      'permission' : Permission,
      'privileges' : PermissionCallerPrivileges,
    }
  } |
  { 'Err' : Error };
export interface GetRequestInput { 'request_id' : UUID }
export interface GetRequestPolicyInput { 'id' : UUID }
export type GetRequestPolicyResult = {
    'Ok' : {
      'privileges' : RequestPolicyCallerPrivileges,
      'policy' : RequestPolicy,
    }
  } |
  { 'Err' : Error };
export type GetRequestResult = { 'Ok' : GetRequestResultData } |
  { 'Err' : Error };
export interface GetRequestResultData {
  'privileges' : RequestCallerPrivileges,
  'request' : Request,
  'additional_info' : RequestAdditionalInfo,
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
export interface InstallCanisterInput {
  'mode' : CanisterInstallMode,
  'canister_id' : Principal,
}
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
export interface ListPermissionsInput {
  'resources' : [] | [Array<Resource>],
  'paginate' : [] | [PaginationInput],
}
export type ListPermissionsResult = {
    'Ok' : {
      'permissions' : Array<Permission>,
      'total' : bigint,
      'privileges' : Array<PermissionCallerPrivileges>,
      'user_groups' : Array<UserGroup>,
      'users' : Array<BasicUser>,
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
export type ListRequestPoliciesInput = PaginationInput;
export type ListRequestPoliciesResult = {
    'Ok' : {
      'total' : bigint,
      'privileges' : Array<RequestPolicyCallerPrivileges>,
      'next_offset' : [] | [bigint],
      'policies' : Array<RequestPolicy>,
    }
  } |
  { 'Err' : Error };
export interface ListRequestsInput {
  'sort_by' : [] | [ListRequestsSortBy],
  'with_evaluation_results' : boolean,
  'expiration_from_dt' : [] | [TimestampRFC3339],
  'created_to_dt' : [] | [TimestampRFC3339],
  'statuses' : [] | [Array<RequestStatusCode>],
  'approver_ids' : [] | [Array<UUID>],
  'expiration_to_dt' : [] | [TimestampRFC3339],
  'paginate' : [] | [PaginationInput],
  'requester_ids' : [] | [Array<UUID>],
  'operation_types' : [] | [Array<ListRequestsOperationType>],
  'only_approvable' : boolean,
  'created_from_dt' : [] | [TimestampRFC3339],
}
export type ListRequestsOperationType = { 'AddUserGroup' : null } |
  { 'EditPermission' : null } |
  { 'AddUser' : null } |
  { 'EditUserGroup' : null } |
  { 'EditRequestPolicy' : null } |
  { 'RemoveRequestPolicy' : null } |
  { 'RemoveAddressBookEntry' : null } |
  { 'EditAddressBookEntry' : null } |
  { 'ChangeCanister' : null } |
  { 'EditUser' : null } |
  { 'ManageSystemInfo' : null } |
  { 'Transfer' : [] | [UUID] } |
  { 'EditAccount' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'AddRequestPolicy' : null } |
  { 'RemoveUserGroup' : null } |
  { 'AddAccount' : null };
export type ListRequestsResult = {
    'Ok' : {
      'total' : bigint,
      'privileges' : Array<RequestCallerPrivileges>,
      'requests' : Array<Request>,
      'next_offset' : [] | [bigint],
      'additional_info' : Array<RequestAdditionalInfo>,
    }
  } |
  { 'Err' : Error };
export type ListRequestsSortBy = { 'ExpirationDt' : SortByDirection } |
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
export interface ManageSystemInfoOperation {
  'input' : ManageSystemInfoOperationInput,
}
export interface ManageSystemInfoOperationInput { 'name' : [] | [string] }
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
    'RequestCreated' : {
      'account_id' : [] | [UUID],
      'request_id' : UUID,
      'operation_type' : RequestOperationType,
      'user_id' : [] | [UUID],
    }
  } |
  {
    'RequestRejected' : {
      'request_id' : UUID,
      'reasons' : [] | [Array<EvaluationSummaryReason>],
      'operation_type' : RequestOperationType,
    }
  } |
  { 'SystemMessage' : null } |
  {
    'RequestFailed' : {
      'request_id' : UUID,
      'operation_type' : RequestOperationType,
      'reason' : [] | [string],
    }
  };
export type NotificationTypeInput = { 'RequestCreated' : null } |
  { 'SystemMessage' : null };
export interface PaginationInput {
  'offset' : [] | [bigint],
  'limit' : [] | [number],
}
export interface Permission { 'resource' : Resource, 'allow' : Allow }
export interface PermissionCallerPrivileges {
  'resource' : Resource,
  'can_edit' : boolean,
}
export type PermissionResourceAction = { 'Read' : null } |
  { 'Update' : null };
export interface Quorum { 'min_approved' : number, 'approvers' : UserSpecifier }
export interface QuorumPercentage {
  'min_approved' : number,
  'approvers' : UserSpecifier,
}
export interface RemoveAddressBookEntryOperation {
  'input' : RemoveAddressBookEntryOperationInput,
}
export interface RemoveAddressBookEntryOperationInput {
  'address_book_entry_id' : UUID,
}
export interface RemoveRequestPolicyOperation {
  'input' : RemoveRequestPolicyOperationInput,
}
export interface RemoveRequestPolicyOperationInput { 'policy_id' : UUID }
export interface RemoveUserGroupOperation {
  'input' : RemoveUserGroupOperationInput,
}
export interface RemoveUserGroupOperationInput { 'user_group_id' : UUID }
export interface Request {
  'id' : UUID,
  'status' : RequestStatus,
  'title' : string,
  'execution_plan' : RequestExecutionSchedule,
  'expiration_dt' : TimestampRFC3339,
  'created_at' : TimestampRFC3339,
  'requested_by' : UUID,
  'summary' : [] | [string],
  'operation' : RequestOperation,
  'approvals' : Array<RequestApproval>,
}
export interface RequestAdditionalInfo {
  'id' : UUID,
  'evaluation_result' : [] | [RequestEvaluationResult],
  'requester_name' : string,
  'approvers' : Array<DisplayUser>,
}
export interface RequestApproval {
  'status' : RequestApprovalStatus,
  'approver_id' : UUID,
  'status_reason' : [] | [string],
  'decided_at' : TimestampRFC3339,
}
export type RequestApprovalStatus = { 'Approved' : null } |
  { 'Rejected' : null };
export interface RequestCallerPrivileges {
  'id' : UUID,
  'can_approve' : boolean,
}
export interface RequestEvaluationResult {
  'request_id' : UUID,
  'status' : EvaluationStatus,
  'result_reasons' : [] | [Array<EvaluationSummaryReason>],
  'policy_results' : Array<RequestPolicyRuleResult>,
}
export type RequestExecutionSchedule = { 'Immediate' : null } |
  { 'Scheduled' : { 'execution_time' : TimestampRFC3339 } };
export type RequestOperation = { 'AddUserGroup' : AddUserGroupOperation } |
  { 'EditPermission' : EditPermissionOperation } |
  { 'AddUser' : AddUserOperation } |
  { 'EditUserGroup' : EditUserGroupOperation } |
  { 'EditRequestPolicy' : EditRequestPolicyOperation } |
  { 'RemoveRequestPolicy' : RemoveRequestPolicyOperation } |
  { 'RemoveAddressBookEntry' : RemoveAddressBookEntryOperation } |
  { 'EditAddressBookEntry' : EditAddressBookEntryOperation } |
  { 'ChangeCanister' : ChangeCanisterOperation } |
  { 'EditUser' : EditUserOperation } |
  { 'ManageSystemInfo' : ManageSystemInfoOperation } |
  { 'Transfer' : TransferOperation } |
  { 'EditAccount' : EditAccountOperation } |
  { 'AddAddressBookEntry' : AddAddressBookEntryOperation } |
  { 'AddRequestPolicy' : AddRequestPolicyOperation } |
  { 'RemoveUserGroup' : RemoveUserGroupOperation } |
  { 'AddAccount' : AddAccountOperation };
export type RequestOperationInput = {
    'AddUserGroup' : AddUserGroupOperationInput
  } |
  { 'EditPermission' : EditPermissionOperationInput } |
  { 'AddUser' : AddUserOperationInput } |
  { 'EditUserGroup' : EditUserGroupOperationInput } |
  { 'EditRequestPolicy' : EditRequestPolicyOperationInput } |
  { 'RemoveRequestPolicy' : RemoveRequestPolicyOperationInput } |
  { 'RemoveAddressBookEntry' : RemoveAddressBookEntryOperationInput } |
  { 'EditAddressBookEntry' : EditAddressBookEntryOperationInput } |
  { 'ChangeCanister' : ChangeCanisterOperationInput } |
  { 'EditUser' : EditUserOperationInput } |
  { 'ManageSystemInfo' : ManageSystemInfoOperationInput } |
  { 'Transfer' : TransferOperationInput } |
  { 'EditAccount' : EditAccountOperationInput } |
  { 'AddAddressBookEntry' : AddAddressBookEntryOperationInput } |
  { 'AddRequestPolicy' : AddRequestPolicyOperationInput } |
  { 'RemoveUserGroup' : RemoveUserGroupOperationInput } |
  { 'AddAccount' : AddAccountOperationInput };
export type RequestOperationType = { 'AddUserGroup' : null } |
  { 'EditPermission' : null } |
  { 'AddUser' : null } |
  { 'EditUserGroup' : null } |
  { 'EditRequestPolicy' : null } |
  { 'RemoveRequestPolicy' : null } |
  { 'RemoveAddressBookEntry' : null } |
  { 'EditAddressBookEntry' : null } |
  { 'ChangeCanister' : null } |
  { 'EditUser' : null } |
  { 'ManageSystemInfo' : null } |
  { 'Transfer' : null } |
  { 'EditAccount' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'AddRequestPolicy' : null } |
  { 'RemoveUserGroup' : null } |
  { 'AddAccount' : null };
export interface RequestPolicy {
  'id' : UUID,
  'rule' : RequestPolicyRule,
  'specifier' : RequestSpecifier,
}
export interface RequestPolicyCallerPrivileges {
  'id' : UUID,
  'can_delete' : boolean,
  'can_edit' : boolean,
}
export type RequestPolicyRule = { 'Not' : RequestPolicyRule } |
  { 'Quorum' : Quorum } |
  { 'AllowListed' : null } |
  { 'QuorumPercentage' : QuorumPercentage } |
  { 'AutoApproved' : null } |
  { 'AllOf' : Array<RequestPolicyRule> } |
  { 'AnyOf' : Array<RequestPolicyRule> } |
  { 'AllowListedByMetadata' : AddressBookMetadata };
export type RequestPolicyRuleInput = { 'Set' : RequestPolicyRule } |
  { 'Remove' : null };
export interface RequestPolicyRuleResult {
  'status' : EvaluationStatus,
  'evaluated_rule' : EvaluatedRequestPolicyRule,
}
export type RequestResourceAction = { 'List' : null } |
  { 'Read' : ResourceId };
export type RequestSpecifier = { 'AddUserGroup' : null } |
  { 'EditPermission' : ResourceSpecifier } |
  { 'AddUser' : null } |
  { 'EditUserGroup' : ResourceIds } |
  { 'EditRequestPolicy' : ResourceIds } |
  { 'RemoveRequestPolicy' : ResourceIds } |
  { 'RemoveAddressBookEntry' : ResourceIds } |
  { 'EditAddressBookEntry' : ResourceIds } |
  { 'ChangeCanister' : ChangeCanisterResourceTarget } |
  { 'EditUser' : ResourceIds } |
  { 'ManageSystemInfo' : null } |
  { 'Transfer' : ResourceIds } |
  { 'EditAccount' : ResourceIds } |
  { 'AddAddressBookEntry' : null } |
  { 'AddRequestPolicy' : null } |
  { 'RemoveUserGroup' : ResourceIds } |
  { 'AddAccount' : null };
export type RequestStatus = { 'Failed' : { 'reason' : [] | [string] } } |
  { 'Approved' : null } |
  { 'Rejected' : null } |
  { 'Scheduled' : { 'scheduled_at' : TimestampRFC3339 } } |
  { 'Cancelled' : { 'reason' : [] | [string] } } |
  { 'Processing' : { 'started_at' : TimestampRFC3339 } } |
  { 'Created' : null } |
  { 'Completed' : { 'completed_at' : TimestampRFC3339 } };
export type RequestStatusCode = { 'Failed' : null } |
  { 'Approved' : null } |
  { 'Rejected' : null } |
  { 'Scheduled' : null } |
  { 'Cancelled' : null } |
  { 'Processing' : null } |
  { 'Created' : null } |
  { 'Completed' : null };
export type Resource = { 'Request' : RequestResourceAction } |
  { 'System' : SystemResourceAction } |
  { 'User' : UserResourceAction } |
  { 'Account' : AccountResourceAction } |
  { 'AddressBook' : ResourceAction } |
  { 'ChangeCanister' : ChangeCanisterResourceAction } |
  { 'UserGroup' : ResourceAction } |
  { 'Permission' : PermissionResourceAction } |
  { 'RequestPolicy' : ResourceAction };
export type ResourceAction = { 'List' : null } |
  { 'Read' : ResourceId } |
  { 'Delete' : ResourceId } |
  { 'Create' : null } |
  { 'Update' : ResourceId };
export type ResourceId = { 'Id' : UUID } |
  { 'Any' : null };
export type ResourceIds = { 'Any' : null } |
  { 'Ids' : Array<UUID> };
export type ResourceSpecifier = { 'Any' : null } |
  { 'Resource' : Resource };
export type Sha256Hash = string;
export type SortByDirection = { 'Asc' : null } |
  { 'Desc' : null };
export interface SubmitRequestApprovalInput {
  'request_id' : UUID,
  'decision' : RequestApprovalStatus,
  'reason' : [] | [string],
}
export type SubmitRequestApprovalResult = {
    'Ok' : {
      'privileges' : RequestCallerPrivileges,
      'request' : Request,
      'additional_info' : RequestAdditionalInfo,
    }
  } |
  { 'Err' : Error };
export interface SystemInfo {
  'name' : string,
  'last_upgrade_timestamp' : TimestampRFC3339,
  'raw_rand_successful' : boolean,
  'version' : string,
  'cycles' : bigint,
  'upgrader_id' : Principal,
}
export type SystemInfoResult = { 'Ok' : { 'system' : SystemInfo } } |
  { 'Err' : Error };
export interface SystemInit {
  'name' : string,
  'admins' : Array<AdminInitInput>,
  'upgrader_wasm_module' : Uint8Array | number[],
}
export type SystemInstall = { 'Upgrade' : SystemUpgrade } |
  { 'Init' : SystemInit };
export type SystemResourceAction = { 'ManageSystemInfo' : null } |
  { 'SystemInfo' : null } |
  { 'Capabilities' : null };
export interface SystemUpgrade { 'name' : [] | [string] }
export type TimestampRFC3339 = string;
export interface Transfer {
  'id' : UUID,
  'to' : string,
  'fee' : bigint,
  'request_id' : UUID,
  'status' : TransferStatus,
  'from_account_id' : UUID,
  'metadata' : Array<TransferMetadata>,
  'network' : Network,
  'amount' : bigint,
}
export interface TransferListItem {
  'to' : string,
  'request_id' : UUID,
  'status' : TransferStatus,
  'created_at' : TimestampRFC3339,
  'transfer_id' : UUID,
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
  'name' : string,
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
  { 'ListRequestPolicies' : null } |
  { 'ListPermissions' : null } |
  { 'ListUserGroups' : null } |
  { 'AddUser' : null } |
  { 'ListUsers' : null } |
  { 'ChangeCanister' : null } |
  { 'ManageSystemInfo' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'ListAccounts' : null } |
  { 'AddRequestPolicy' : null } |
  { 'ListAddressBookEntries' : null } |
  { 'ListRequests' : null } |
  { 'SystemInfo' : null } |
  { 'Capabilities' : null } |
  { 'AddAccount' : null };
export type UserResourceAction = { 'List' : null } |
  { 'Read' : ResourceId } |
  { 'Create' : null } |
  { 'Update' : ResourceId };
export type UserSpecifier = { 'Id' : Array<UUID> } |
  { 'Any' : null } |
  { 'Group' : Array<UUID> };
export type UserStatus = { 'Inactive' : null } |
  { 'Active' : null };
export interface _SERVICE {
  'capabilities' : ActorMethod<[], CapabilitiesResult>,
  'create_request' : ActorMethod<[CreateRequestInput], CreateRequestResult>,
  'fetch_account_balances' : ActorMethod<
    [FetchAccountBalancesInput],
    FetchAccountBalancesResult
  >,
  'get_account' : ActorMethod<[GetAccountInput], GetAccountResult>,
  'get_address_book_entry' : ActorMethod<
    [GetAddressBookEntryInput],
    GetAddressBookEntryResult
  >,
  'get_next_approvable_request' : ActorMethod<
    [GetNextApprovableRequestInput],
    GetNextApprovableRequestResult
  >,
  'get_permission' : ActorMethod<[GetPermissionInput], GetPermissionResult>,
  'get_request' : ActorMethod<[GetRequestInput], GetRequestResult>,
  'get_request_policy' : ActorMethod<
    [GetRequestPolicyInput],
    GetRequestPolicyResult
  >,
  'get_transfers' : ActorMethod<[GetTransfersInput], GetTransfersResult>,
  'get_user' : ActorMethod<[GetUserInput], GetUserResult>,
  'get_user_group' : ActorMethod<[GetUserGroupInput], GetUserGroupResult>,
  'health_status' : ActorMethod<[], HealthStatus>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
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
  'list_permissions' : ActorMethod<
    [ListPermissionsInput],
    ListPermissionsResult
  >,
  'list_request_policies' : ActorMethod<
    [ListRequestPoliciesInput],
    ListRequestPoliciesResult
  >,
  'list_requests' : ActorMethod<[ListRequestsInput], ListRequestsResult>,
  'list_user_groups' : ActorMethod<[ListUserGroupsInput], ListUserGroupsResult>,
  'list_users' : ActorMethod<[ListUsersInput], ListUsersResult>,
  'mark_notifications_read' : ActorMethod<
    [MarkNotificationsReadInput],
    MarkNotificationReadResult
  >,
  'me' : ActorMethod<[], MeResult>,
  'submit_request_approval' : ActorMethod<
    [SubmitRequestApprovalInput],
    SubmitRequestApprovalResult
  >,
  'system_info' : ActorMethod<[], SystemInfoResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
