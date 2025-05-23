import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'id' : UUID,
  'configs_request_policy' : [] | [RequestPolicyRule],
  'metadata' : Array<AccountMetadata>,
  'name' : string,
  'assets' : Array<AccountAsset>,
  'addresses' : Array<AccountAddress>,
  'transfer_request_policy' : [] | [RequestPolicyRule],
  'last_modification_timestamp' : TimestampRFC3339,
}
export interface AccountAddress { 'address' : string, 'format' : string }
export interface AccountAsset {
  'balance' : [] | [AccountBalance],
  'asset_id' : UUID,
}
export interface AccountBalance {
  'account_id' : UUID,
  'decimals' : number,
  'balance' : bigint,
  'last_update_timestamp' : TimestampRFC3339,
  'query_state' : string,
  'asset_id' : UUID,
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
export type AccountSeed = Uint8Array | number[];
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
  'assets' : Array<UUID>,
  'transfer_request_policy' : [] | [RequestPolicyRule],
  'transfer_permission' : Allow,
}
export interface AddAddressBookEntryOperation {
  'address_book_entry' : [] | [AddressBookEntry],
  'input' : AddAddressBookEntryOperationInput,
}
export interface AddAddressBookEntryOperationInput {
  'metadata' : Array<AddressBookMetadata>,
  'labels' : Array<string>,
  'blockchain' : string,
  'address' : string,
  'address_format' : string,
  'address_owner' : string,
}
export interface AddAssetOperation {
  'asset' : [] | [Asset],
  'input' : AddAssetOperationInput,
}
export interface AddAssetOperationInput {
  'decimals' : number,
  'standards' : Array<string>,
  'metadata' : Array<AssetMetadata>,
  'name' : string,
  'blockchain' : string,
  'symbol' : AssetSymbol,
}
export interface AddNamedRuleOperation {
  'named_rule' : [] | [NamedRule],
  'input' : AddNamedRuleOperationInput,
}
export interface AddNamedRuleOperationInput {
  'name' : string,
  'rule' : RequestPolicyRule,
  'description' : [] | [string],
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
  'labels' : Array<string>,
  'blockchain' : string,
  'address' : string,
  'last_modification_timestamp' : string,
  'address_format' : string,
  'address_owner' : string,
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
export interface Asset {
  'id' : UUID,
  'decimals' : number,
  'standards' : Array<string>,
  'metadata' : Array<AssetMetadata>,
  'name' : string,
  'blockchain' : string,
  'symbol' : AssetSymbol,
}
export interface AssetCallerPrivileges {
  'id' : UUID,
  'can_delete' : boolean,
  'can_edit' : boolean,
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
export interface CallExternalCanisterOperation {
  'arg' : [] | [Uint8Array | number[]],
  'execution_method' : CanisterMethod,
  'validation_method' : [] | [CanisterMethod],
  'arg_checksum' : [] | [Sha256Hash],
  'execution_method_cycles' : [] | [bigint],
  'arg_rendering' : [] | [string],
  'execution_method_reply' : [] | [Uint8Array | number[]],
}
export interface CallExternalCanisterOperationInput {
  'arg' : [] | [Uint8Array | number[]],
  'execution_method' : CanisterMethod,
  'validation_method' : [] | [CanisterMethod],
  'execution_method_cycles' : [] | [bigint],
}
export interface CallExternalCanisterResourceTarget {
  'execution_method' : ExecutionMethodResourceTarget,
  'validation_method' : ValidationMethodResourceTarget,
}
export interface CancelRequestInput {
  'request_id' : UUID,
  'reason' : [] | [string],
}
export type CancelRequestResult = { 'Ok' : { 'request' : Request } } |
  { 'Err' : Error };
export interface CanisterExecutionAndValidationMethodPair {
  'execution_method' : string,
  'validation_method' : ValidationMethodResourceTarget,
}
export type CanisterInstallMode = { 'reinstall' : null } |
  { 'upgrade' : null } |
  { 'install' : null };
export interface CanisterMethod {
  'canister_id' : Principal,
  'method_name' : string,
}
export interface CanisterSnapshotsInput { 'canister_id' : Principal }
export type CanisterSnapshotsResponse = Array<
  {
    'total_size' : bigint,
    'taken_at_timestamp' : TimestampRFC3339,
    'snapshot_id' : string,
  }
>;
export type CanisterSnapshotsResult = { 'Ok' : CanisterSnapshotsResponse } |
  { 'Err' : Error };
export interface CanisterStatusInput { 'canister_id' : Principal }
export interface CanisterStatusResponse {
  'status' : { 'stopped' : null } |
    { 'stopping' : null } |
    { 'running' : null },
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : DefiniteCanisterSettings,
  'query_stats' : {
    'response_payload_bytes_total' : bigint,
    'num_instructions_total' : bigint,
    'num_calls_total' : bigint,
    'request_payload_bytes_total' : bigint,
  },
  'idle_cycles_burned_per_day' : bigint,
  'module_hash' : [] | [Uint8Array | number[]],
  'reserved_cycles' : bigint,
}
export interface Capabilities {
  'name' : string,
  'version' : string,
  'supported_assets' : Array<Asset>,
  'supported_blockchains' : Array<SupportedBlockchain>,
}
export type CapabilitiesResult = { 'Ok' : { 'capabilities' : Capabilities } } |
  { 'Err' : Error };
export type ChangeAddressBookMetadata = {
    'OverrideSpecifiedBy' : Array<AddressBookMetadata>
  } |
  { 'RemoveKeys' : Array<string> } |
  { 'ReplaceAllBy' : Array<AddressBookMetadata> };
export type ChangeAssets = { 'ReplaceWith' : { 'assets' : Array<UUID> } } |
  { 'Change' : { 'add_assets' : Array<UUID>, 'remove_assets' : Array<UUID> } };
export type ChangeExternalCanisterMetadata = {
    'OverrideSpecifiedBy' : Array<ExternalCanisterMetadata>
  } |
  { 'RemoveKeys' : Array<string> } |
  { 'ReplaceAllBy' : Array<ExternalCanisterMetadata> };
export interface ChangeExternalCanisterOperation {
  'mode' : CanisterInstallMode,
  'canister_id' : Principal,
  'module_checksum' : Sha256Hash,
  'arg_checksum' : [] | [Sha256Hash],
}
export interface ChangeExternalCanisterOperationInput {
  'arg' : [] | [Uint8Array | number[]],
  'module_extra_chunks' : [] | [WasmModuleExtraChunks],
  'mode' : CanisterInstallMode,
  'canister_id' : Principal,
  'module' : Uint8Array | number[],
}
export type ChangeMetadata = { 'OverrideSpecifiedBy' : Array<AssetMetadata> } |
  { 'RemoveKeys' : Array<string> } |
  { 'ReplaceAllBy' : Array<AssetMetadata> };
export type ConfigureExternalCanisterOperation = ConfigureExternalCanisterOperationInput;
export interface ConfigureExternalCanisterOperationInput {
  'kind' : ConfigureExternalCanisterOperationKind,
  'canister_id' : Principal,
}
export type ConfigureExternalCanisterOperationKind = { 'SoftDelete' : null } |
  { 'Settings' : ConfigureExternalCanisterSettingsInput } |
  { 'Delete' : null } |
  { 'NativeSettings' : DefiniteCanisterSettingsInput };
export interface ConfigureExternalCanisterSettingsInput {
  'permissions' : [] | [ExternalCanisterPermissionsUpdateInput],
  'name' : [] | [string],
  'labels' : [] | [Array<string>],
  'description' : [] | [string],
  'request_policies' : [] | [ExternalCanisterRequestPoliciesUpdateInput],
  'state' : [] | [ExternalCanisterState],
  'change_metadata' : [] | [ChangeExternalCanisterMetadata],
}
export interface CreateExternalCanisterOperation {
  'canister_id' : [] | [Principal],
  'input' : CreateExternalCanisterOperationInput,
}
export interface CreateExternalCanisterOperationInput {
  'permissions' : ExternalCanisterPermissionsCreateInput,
  'metadata' : [] | [Array<ExternalCanisterMetadata>],
  'kind' : CreateExternalCanisterOperationKind,
  'name' : string,
  'labels' : [] | [Array<string>],
  'description' : [] | [string],
  'request_policies' : ExternalCanisterRequestPoliciesCreateInput,
}
export type CreateExternalCanisterOperationKind = {
    'AddExisting' : CreateExternalCanisterOperationKindAddExisting
  } |
  { 'CreateNew' : CreateExternalCanisterOperationKindCreateNew };
export interface CreateExternalCanisterOperationKindAddExisting {
  'canister_id' : Principal,
}
export interface CreateExternalCanisterOperationKindCreateNew {
  'initial_cycles' : [] | [bigint],
  'subnet_selection' : [] | [SubnetSelection],
}
export interface CreateRequestInput {
  'title' : [] | [string],
  'execution_plan' : [] | [RequestExecutionSchedule],
  'expiration_dt' : [] | [TimestampRFC3339],
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
export type CycleObtainStrategy = { 'Disabled' : null } |
  {
    'MintFromNativeToken' : {
      'account_id' : UUID,
      'account_name' : [] | [string],
    }
  } |
  {
    'WithdrawFromCyclesLedger' : {
      'account_id' : UUID,
      'account_name' : [] | [string],
    }
  };
export type CycleObtainStrategyInput = { 'Disabled' : null } |
  { 'MintFromNativeToken' : { 'account_id' : UUID } } |
  { 'WithdrawFromCyclesLedger' : { 'account_id' : UUID } };
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'reserved_cycles_limit' : bigint,
  'log_visibility' : LogVisibility,
  'wasm_memory_limit' : bigint,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export interface DefiniteCanisterSettingsInput {
  'freezing_threshold' : [] | [bigint],
  'controllers' : [] | [Array<Principal>],
  'reserved_cycles_limit' : [] | [bigint],
  'log_visibility' : [] | [LogVisibility],
  'wasm_memory_limit' : [] | [bigint],
  'memory_allocation' : [] | [bigint],
  'compute_allocation' : [] | [bigint],
}
export interface DisasterRecovery {
  'user_group_name' : [] | [string],
  'committee' : DisasterRecoveryCommittee,
}
export interface DisasterRecoveryCommittee {
  'user_group_id' : UUID,
  'quorum' : number,
}
export interface DisplayUser { 'id' : UUID, 'name' : string }
export interface EditAccountOperation { 'input' : EditAccountOperationInput }
export interface EditAccountOperationInput {
  'account_id' : UUID,
  'configs_request_policy' : [] | [RequestPolicyRuleInput],
  'read_permission' : [] | [Allow],
  'configs_permission' : [] | [Allow],
  'name' : [] | [string],
  'change_assets' : [] | [ChangeAssets],
  'transfer_request_policy' : [] | [RequestPolicyRuleInput],
  'transfer_permission' : [] | [Allow],
}
export interface EditAddressBookEntryOperation {
  'input' : EditAddressBookEntryOperationInput,
}
export interface EditAddressBookEntryOperationInput {
  'labels' : [] | [Array<string>],
  'change_metadata' : [] | [ChangeAddressBookMetadata],
  'address_book_entry_id' : UUID,
  'address_owner' : [] | [string],
}
export interface EditAssetOperation { 'input' : EditAssetOperationInput }
export interface EditAssetOperationInput {
  'standards' : [] | [Array<string>],
  'name' : [] | [string],
  'blockchain' : [] | [string],
  'change_metadata' : [] | [ChangeMetadata],
  'asset_id' : UUID,
  'symbol' : [] | [AssetSymbol],
}
export interface EditNamedRuleOperation {
  'input' : EditNamedRuleOperationInput,
}
export interface EditNamedRuleOperationInput {
  'name' : [] | [string],
  'rule' : [] | [RequestPolicyRule],
  'description' : [] | [[] | [string]],
  'named_rule_id' : UUID,
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
  'cancel_pending_requests' : [] | [boolean],
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
export type ExecutionMethodResourceTarget = { 'Any' : null } |
  { 'ExecutionMethod' : CanisterMethod };
export interface ExternalCanister {
  'id' : UUID,
  'permissions' : ExternalCanisterPermissions,
  'modified_at' : [] | [TimestampRFC3339],
  'metadata' : Array<ExternalCanisterMetadata>,
  'name' : string,
  'labels' : Array<string>,
  'canister_id' : Principal,
  'description' : [] | [string],
  'created_at' : TimestampRFC3339,
  'request_policies' : ExternalCanisterRequestPolicies,
  'state' : ExternalCanisterState,
  'monitoring' : [] | [MonitorExternalCanisterStartInput],
}
export interface ExternalCanisterCallPermission {
  'execution_method' : string,
  'allow' : Allow,
  'validation_method' : ValidationMethodResourceTarget,
}
export interface ExternalCanisterCallRequestPolicyRule {
  'execution_method' : string,
  'rule' : RequestPolicyRule,
  'validation_method' : ValidationMethodResourceTarget,
  'policy_id' : UUID,
}
export interface ExternalCanisterCallRequestPolicyRuleInput {
  'execution_method' : string,
  'rule' : RequestPolicyRule,
  'validation_method' : ValidationMethodResourceTarget,
  'policy_id' : [] | [UUID],
}
export interface ExternalCanisterCallerMethodsPrivileges {
  'execution_method' : string,
  'validation_method' : ValidationMethodResourceTarget,
}
export interface ExternalCanisterCallerPrivileges {
  'id' : UUID,
  'can_change' : boolean,
  'canister_id' : Principal,
  'can_call' : Array<ExternalCanisterCallerMethodsPrivileges>,
  'can_fund' : boolean,
}
export type ExternalCanisterChangeCallPermissionsInput = {
    'OverrideSpecifiedByExecutionMethods' : Array<
      {
        'execution_method' : string,
        'permissions' : Array<
          {
            'allow' : Allow,
            'validation_method' : ValidationMethodResourceTarget,
          }
        >,
      }
    >
  } |
  {
    'OverrideSpecifiedByExecutionValidationMethodPairs' : Array<
      {
        'allow' : [] | [Allow],
        'method_configuration' : CanisterExecutionAndValidationMethodPair,
      }
    >
  } |
  { 'ReplaceAllBy' : Array<ExternalCanisterCallPermission> };
export type ExternalCanisterChangeCallRequestPoliciesInput = {
    'RemoveByPolicyIds' : Array<UUID>
  } |
  {
    'OverrideSpecifiedByExecutionMethods' : Array<
      {
        'execution_method' : string,
        'policies' : Array<
          {
            'rule' : RequestPolicyRule,
            'validation_method' : ValidationMethodResourceTarget,
            'policy_id' : [] | [UUID],
          }
        >,
      }
    >
  } |
  {
    'OverrideSpecifiedByExecutionValidationMethodPairs' : Array<
      {
        'method_configuration' : CanisterExecutionAndValidationMethodPair,
        'policies' : Array<ExternalCanisterChangeRequestPolicyRuleInput>,
      }
    >
  } |
  { 'ReplaceAllBy' : Array<ExternalCanisterCallRequestPolicyRuleInput> };
export interface ExternalCanisterChangeRequestPolicyRule {
  'rule' : RequestPolicyRule,
  'policy_id' : UUID,
}
export interface ExternalCanisterChangeRequestPolicyRuleInput {
  'rule' : RequestPolicyRule,
  'policy_id' : [] | [UUID],
}
export type ExternalCanisterId = { 'Any' : null } |
  { 'Canister' : Principal };
export interface ExternalCanisterMetadata { 'key' : string, 'value' : string }
export interface ExternalCanisterPermissions {
  'calls' : Array<ExternalCanisterCallPermission>,
  'read' : Allow,
  'change' : Allow,
}
export type ExternalCanisterPermissionsCreateInput = ExternalCanisterPermissions;
export interface ExternalCanisterPermissionsUpdateInput {
  'calls' : [] | [ExternalCanisterChangeCallPermissionsInput],
  'read' : [] | [Allow],
  'change' : [] | [Allow],
}
export interface ExternalCanisterRequestPolicies {
  'calls' : Array<ExternalCanisterCallRequestPolicyRule>,
  'change' : Array<ExternalCanisterChangeRequestPolicyRule>,
}
export interface ExternalCanisterRequestPoliciesCreateInput {
  'calls' : Array<ExternalCanisterCallRequestPolicyRuleInput>,
  'change' : Array<ExternalCanisterChangeRequestPolicyRuleInput>,
}
export interface ExternalCanisterRequestPoliciesUpdateInput {
  'calls' : [] | [ExternalCanisterChangeCallRequestPoliciesInput],
  'change' : [] | [Array<ExternalCanisterChangeRequestPolicyRuleInput>],
}
export type ExternalCanisterResourceAction = {
    'Call' : CallExternalCanisterResourceTarget
  } |
  { 'Fund' : ExternalCanisterId } |
  { 'List' : null } |
  { 'Read' : ExternalCanisterId } |
  { 'Create' : null } |
  { 'Change' : ExternalCanisterId };
export type ExternalCanisterState = { 'Active' : null } |
  { 'Archived' : null };
export interface FetchAccountBalancesInput { 'account_ids' : Array<UUID> }
export type FetchAccountBalancesResult = {
    'Ok' : { 'balances' : Array<[] | [AccountBalance]> }
  } |
  { 'Err' : Error };
export type FundExternalCanisterOperation = FundExternalCanisterOperationInput;
export interface FundExternalCanisterOperationInput {
  'kind' : FundExternalCanisterOperationKind,
  'canister_id' : Principal,
}
export type FundExternalCanisterOperationKind = {
    'Send' : FundExternalCanisterSendCyclesInput
  };
export interface FundExternalCanisterSendCyclesInput { 'cycles' : bigint }
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
export interface GetAssetInput { 'asset_id' : UUID }
export type GetAssetResult = {
    'Ok' : { 'privileges' : AssetCallerPrivileges, 'asset' : Asset }
  } |
  { 'Err' : Error };
export interface GetExternalCanisterFiltersInput {
  'with_labels' : [] | [boolean],
  'with_name' : [] | [{ 'prefix' : [] | [string] }],
}
export type GetExternalCanisterFiltersResult = {
    'Ok' : {
      'labels' : [] | [Array<string>],
      'names' : [] | [Array<{ 'name' : string, 'canister_id' : Principal }>],
    }
  } |
  { 'Err' : Error };
export interface GetExternalCanisterInput { 'canister_id' : Principal }
export type GetExternalCanisterResult = {
    'Ok' : {
      'privileges' : ExternalCanisterCallerPrivileges,
      'canister' : ExternalCanister,
    }
  } |
  { 'Err' : Error };
export interface GetNamedRuleInput { 'named_rule_id' : UUID }
export type GetNamedRuleResult = {
    'Ok' : {
      'privileges' : NamedRuleCallerPrivileges,
      'named_rule' : NamedRule,
    }
  } |
  { 'Err' : Error };
export interface GetNextApprovableRequestInput {
  'sort_by' : [] | [ListRequestsSortBy],
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
export interface GetRequestInput {
  'request_id' : UUID,
  'with_full_info' : [] | [boolean],
}
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
export interface InitAccountInput {
  'id' : [] | [UUID],
  'metadata' : Array<AccountMetadata>,
  'name' : string,
  'assets' : Array<UUID>,
  'seed' : AccountSeed,
}
export interface InitAccountPermissionsInput {
  'configs_request_policy' : [] | [RequestPolicyRule],
  'read_permission' : Allow,
  'configs_permission' : Allow,
  'transfer_request_policy' : [] | [RequestPolicyRule],
  'transfer_permission' : Allow,
}
export interface InitAccountWithPermissionsInput {
  'permissions' : InitAccountPermissionsInput,
  'account_init' : InitAccountInput,
}
export interface InitAssetInput {
  'id' : [] | [UUID],
  'decimals' : number,
  'standards' : Array<string>,
  'metadata' : Array<AssetMetadata>,
  'name' : string,
  'blockchain' : string,
  'symbol' : string,
}
export interface InitNamedRuleInput {
  'id' : [] | [UUID],
  'name' : string,
  'rule' : RequestPolicyRule,
  'description' : [] | [string],
}
export interface InitPermissionInput { 'resource' : Resource, 'allow' : Allow }
export interface InitRequestPolicyInput {
  'id' : [] | [UUID],
  'rule' : RequestPolicyRule,
  'specifier' : RequestSpecifier,
}
export interface InitUserGroupInput { 'id' : [] | [UUID], 'name' : string }
export interface InitUserInput {
  'id' : [] | [UUID],
  'status' : UserStatus,
  'groups' : [] | [Array<UUID>],
  'name' : string,
  'identities' : Array<UserIdentityInput>,
}
export type InitialConfig = {
    'WithDefaultPolicies' : {
      'assets' : Array<InitAssetInput>,
      'admin_quorum' : number,
      'accounts' : Array<InitAccountInput>,
      'users' : Array<InitUserInput>,
      'operator_quorum' : number,
    }
  } |
  {
    'WithAllDefaults' : {
      'admin_quorum' : number,
      'users' : Array<InitUserInput>,
      'operator_quorum' : number,
    }
  } |
  {
    'Complete' : {
      'permissions' : Array<InitPermissionInput>,
      'assets' : Array<InitAssetInput>,
      'request_policies' : Array<InitRequestPolicyInput>,
      'user_groups' : Array<InitUserGroupInput>,
      'accounts' : Array<InitAccountWithPermissionsInput>,
      'disaster_recovery_committee' : [] | [DisasterRecoveryCommittee],
      'users' : Array<InitUserInput>,
      'named_rules' : Array<InitNamedRuleInput>,
    }
  };
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
  'address_formats' : [] | [Array<string>],
  'labels' : [] | [Array<string>],
  'blockchain' : [] | [string],
  'addresses' : [] | [Array<string>],
  'paginate' : [] | [PaginationInput],
  'search_term' : [] | [string],
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
export interface ListAssetsInput { 'paginate' : [] | [PaginationInput] }
export type ListAssetsResult = {
    'Ok' : {
      'total' : bigint,
      'privileges' : Array<AssetCallerPrivileges>,
      'assets' : Array<Asset>,
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
export interface ListExternalCanistersInput {
  'sort_by' : [] | [ListExternalCanistersSortInput],
  'states' : [] | [Array<ExternalCanisterState>],
  'canister_ids' : [] | [Array<Principal>],
  'labels' : [] | [Array<string>],
  'paginate' : [] | [PaginationInput],
}
export type ListExternalCanistersResult = {
    'Ok' : {
      'total' : bigint,
      'privileges' : Array<ExternalCanisterCallerPrivileges>,
      'canisters' : Array<ExternalCanister>,
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
export type ListExternalCanistersSortInput = { 'Name' : SortByDirection };
export interface ListNamedRulesInput { 'paginate' : [] | [PaginationInput] }
export type ListNamedRulesResult = {
    'Ok' : {
      'total' : bigint,
      'privileges' : Array<NamedRuleCallerPrivileges>,
      'named_rules' : Array<NamedRule>,
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
export type ListRequestsOperationType = { 'RemoveAsset' : null } |
  { 'AddUserGroup' : null } |
  { 'EditPermission' : null } |
  { 'SnapshotExternalCanister' : [] | [Principal] } |
  { 'PruneExternalCanister' : [] | [Principal] } |
  { 'EditNamedRule' : null } |
  { 'ConfigureExternalCanister' : [] | [Principal] } |
  { 'ChangeExternalCanister' : [] | [Principal] } |
  { 'MonitorExternalCanister' : [] | [Principal] } |
  { 'AddUser' : null } |
  { 'EditAsset' : null } |
  { 'EditUserGroup' : null } |
  { 'SetDisasterRecovery' : null } |
  { 'EditRequestPolicy' : null } |
  { 'RemoveRequestPolicy' : null } |
  { 'AddAsset' : null } |
  { 'SystemUpgrade' : null } |
  { 'RemoveAddressBookEntry' : null } |
  { 'SystemRestore' : null } |
  { 'CreateExternalCanister' : null } |
  { 'EditAddressBookEntry' : null } |
  { 'FundExternalCanister' : [] | [Principal] } |
  { 'EditUser' : null } |
  { 'ManageSystemInfo' : null } |
  { 'Transfer' : [] | [UUID] } |
  { 'EditAccount' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'AddRequestPolicy' : null } |
  { 'RemoveNamedRule' : null } |
  { 'RemoveUserGroup' : null } |
  { 'CallExternalCanister' : [] | [Principal] } |
  { 'AddNamedRule' : null } |
  { 'RestoreExternalCanister' : [] | [Principal] } |
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
  'groups' : [] | [Array<UUID>],
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
export type LogVisibility = { 'controllers' : null } |
  { 'public' : null } |
  { 'allowed_viewers' : Array<Principal> };
export interface ManageSystemInfoOperation {
  'input' : ManageSystemInfoOperationInput,
}
export interface ManageSystemInfoOperationInput {
  'name' : [] | [string],
  'cycle_obtain_strategy' : [] | [CycleObtainStrategyInput],
  'max_upgrader_backup_snapshots' : [] | [bigint],
  'max_station_backup_snapshots' : [] | [bigint],
}
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
export type MonitorExternalCanisterOperation = MonitorExternalCanisterOperationInput;
export interface MonitorExternalCanisterOperationInput {
  'kind' : MonitorExternalCanisterOperationKind,
  'canister_id' : Principal,
}
export type MonitorExternalCanisterOperationKind = {
    'Start' : MonitorExternalCanisterStartInput
  } |
  { 'Stop' : null };
export interface MonitorExternalCanisterStartInput {
  'cycle_obtain_strategy' : [] | [CycleObtainStrategyInput],
  'funding_strategy' : MonitorExternalCanisterStrategyInput,
}
export type MonitorExternalCanisterStrategyInput = { 'Always' : bigint } |
  { 'BelowThreshold' : MonitoringExternalCanisterCyclesThresholdInput } |
  { 'BelowEstimatedRuntime' : MonitoringExternalCanisterEstimatedRuntimeInput };
export interface MonitoringExternalCanisterCyclesThresholdInput {
  'fund_cycles' : bigint,
  'min_cycles' : bigint,
}
export interface MonitoringExternalCanisterEstimatedRuntimeInput {
  'fund_runtime_secs' : bigint,
  'fallback_min_cycles' : bigint,
  'min_runtime_secs' : bigint,
  'fallback_fund_cycles' : bigint,
  'max_runtime_cycles_fund' : bigint,
}
export interface NamedRule {
  'id' : UUID,
  'name' : string,
  'rule' : RequestPolicyRule,
  'description' : [] | [string],
}
export interface NamedRuleCallerPrivileges {
  'id' : UUID,
  'can_delete' : boolean,
  'can_edit' : boolean,
}
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
export type NotificationResourceAction = { 'List' : null } |
  { 'Update' : ResourceId };
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
export interface NotifyFailedStationUpgradeInput { 'reason' : string }
export type NotifyFailedStationUpgradeResult = { 'Ok' : null } |
  { 'Err' : Error };
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
export interface PruneExternalCanisterOperation {
  'input' : PruneExternalCanisterOperationInput,
}
export interface PruneExternalCanisterOperationInput {
  'canister_id' : Principal,
  'prune' : { 'snapshot' : string } |
    { 'state' : null } |
    { 'chunk_store' : null },
}
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
export interface RemoveAssetOperation { 'input' : RemoveAssetOperationInput }
export interface RemoveAssetOperationInput { 'asset_id' : UUID }
export interface RemoveNamedRuleOperation {
  'input' : RemoveNamedRuleOperationInput,
}
export interface RemoveNamedRuleOperationInput { 'named_rule_id' : UUID }
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
export type RequestOperation = { 'RemoveAsset' : RemoveAssetOperation } |
  { 'AddUserGroup' : AddUserGroupOperation } |
  { 'EditPermission' : EditPermissionOperation } |
  { 'SnapshotExternalCanister' : SnapshotExternalCanisterOperation } |
  { 'PruneExternalCanister' : PruneExternalCanisterOperation } |
  { 'EditNamedRule' : EditNamedRuleOperation } |
  { 'ConfigureExternalCanister' : ConfigureExternalCanisterOperation } |
  { 'ChangeExternalCanister' : ChangeExternalCanisterOperation } |
  { 'MonitorExternalCanister' : MonitorExternalCanisterOperation } |
  { 'AddUser' : AddUserOperation } |
  { 'EditAsset' : EditAssetOperation } |
  { 'EditUserGroup' : EditUserGroupOperation } |
  { 'SetDisasterRecovery' : SetDisasterRecoveryOperation } |
  { 'EditRequestPolicy' : EditRequestPolicyOperation } |
  { 'RemoveRequestPolicy' : RemoveRequestPolicyOperation } |
  { 'AddAsset' : AddAssetOperation } |
  { 'SystemUpgrade' : SystemUpgradeOperation } |
  { 'RemoveAddressBookEntry' : RemoveAddressBookEntryOperation } |
  { 'SystemRestore' : SystemRestoreOperation } |
  { 'CreateExternalCanister' : CreateExternalCanisterOperation } |
  { 'EditAddressBookEntry' : EditAddressBookEntryOperation } |
  { 'FundExternalCanister' : FundExternalCanisterOperation } |
  { 'EditUser' : EditUserOperation } |
  { 'ManageSystemInfo' : ManageSystemInfoOperation } |
  { 'Transfer' : TransferOperation } |
  { 'EditAccount' : EditAccountOperation } |
  { 'AddAddressBookEntry' : AddAddressBookEntryOperation } |
  { 'AddRequestPolicy' : AddRequestPolicyOperation } |
  { 'RemoveNamedRule' : RemoveNamedRuleOperation } |
  { 'RemoveUserGroup' : RemoveUserGroupOperation } |
  { 'CallExternalCanister' : CallExternalCanisterOperation } |
  { 'AddNamedRule' : AddNamedRuleOperation } |
  { 'RestoreExternalCanister' : RestoreExternalCanisterOperation } |
  { 'AddAccount' : AddAccountOperation };
export type RequestOperationInput = {
    'RemoveAsset' : RemoveAssetOperationInput
  } |
  { 'AddUserGroup' : AddUserGroupOperationInput } |
  { 'EditPermission' : EditPermissionOperationInput } |
  { 'SnapshotExternalCanister' : SnapshotExternalCanisterOperationInput } |
  { 'PruneExternalCanister' : PruneExternalCanisterOperationInput } |
  { 'EditNamedRule' : EditNamedRuleOperationInput } |
  { 'ConfigureExternalCanister' : ConfigureExternalCanisterOperationInput } |
  { 'ChangeExternalCanister' : ChangeExternalCanisterOperationInput } |
  { 'MonitorExternalCanister' : MonitorExternalCanisterOperationInput } |
  { 'AddUser' : AddUserOperationInput } |
  { 'EditAsset' : EditAssetOperationInput } |
  { 'EditUserGroup' : EditUserGroupOperationInput } |
  { 'SetDisasterRecovery' : SetDisasterRecoveryOperationInput } |
  { 'EditRequestPolicy' : EditRequestPolicyOperationInput } |
  { 'RemoveRequestPolicy' : RemoveRequestPolicyOperationInput } |
  { 'AddAsset' : AddAssetOperationInput } |
  { 'SystemUpgrade' : SystemUpgradeOperationInput } |
  { 'RemoveAddressBookEntry' : RemoveAddressBookEntryOperationInput } |
  { 'SystemRestore' : SystemRestoreOperationInput } |
  { 'CreateExternalCanister' : CreateExternalCanisterOperationInput } |
  { 'EditAddressBookEntry' : EditAddressBookEntryOperationInput } |
  { 'FundExternalCanister' : FundExternalCanisterOperationInput } |
  { 'EditUser' : EditUserOperationInput } |
  { 'ManageSystemInfo' : ManageSystemInfoOperationInput } |
  { 'Transfer' : TransferOperationInput } |
  { 'EditAccount' : EditAccountOperationInput } |
  { 'AddAddressBookEntry' : AddAddressBookEntryOperationInput } |
  { 'AddRequestPolicy' : AddRequestPolicyOperationInput } |
  { 'RemoveNamedRule' : RemoveNamedRuleOperationInput } |
  { 'RemoveUserGroup' : RemoveUserGroupOperationInput } |
  { 'CallExternalCanister' : CallExternalCanisterOperationInput } |
  { 'AddNamedRule' : AddNamedRuleOperationInput } |
  { 'RestoreExternalCanister' : RestoreExternalCanisterOperationInput } |
  { 'AddAccount' : AddAccountOperationInput };
export type RequestOperationType = { 'RemoveAsset' : null } |
  { 'AddUserGroup' : null } |
  { 'EditPermission' : null } |
  { 'SnapshotExternalCanister' : null } |
  { 'PruneExternalCanister' : null } |
  { 'EditNamedRule' : null } |
  { 'ConfigureExternalCanister' : null } |
  { 'ChangeExternalCanister' : null } |
  { 'MonitorExternalCanister' : null } |
  { 'AddUser' : null } |
  { 'EditAsset' : null } |
  { 'EditUserGroup' : null } |
  { 'SetDisasterRecovery' : null } |
  { 'EditRequestPolicy' : null } |
  { 'RemoveRequestPolicy' : null } |
  { 'AddAsset' : null } |
  { 'SystemUpgrade' : null } |
  { 'RemoveAddressBookEntry' : null } |
  { 'SystemRestore' : null } |
  { 'CreateExternalCanister' : null } |
  { 'EditAddressBookEntry' : null } |
  { 'FundExternalCanister' : null } |
  { 'EditUser' : null } |
  { 'ManageSystemInfo' : null } |
  { 'Transfer' : null } |
  { 'EditAccount' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'AddRequestPolicy' : null } |
  { 'RemoveNamedRule' : null } |
  { 'RemoveUserGroup' : null } |
  { 'CallExternalCanister' : null } |
  { 'AddNamedRule' : null } |
  { 'RestoreExternalCanister' : null } |
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
  { 'AllowListedByMetadata' : AddressBookMetadata } |
  { 'NamedRule' : UUID };
export type RequestPolicyRuleInput = { 'Set' : RequestPolicyRule } |
  { 'Remove' : null };
export interface RequestPolicyRuleResult {
  'status' : EvaluationStatus,
  'evaluated_rule' : EvaluatedRequestPolicyRule,
}
export type RequestResourceAction = { 'List' : null } |
  { 'Read' : ResourceId };
export type RequestSpecifier = { 'RemoveAsset' : ResourceIds } |
  { 'AddUserGroup' : null } |
  { 'EditPermission' : ResourceSpecifier } |
  { 'EditNamedRule' : ResourceIds } |
  { 'ChangeExternalCanister' : ExternalCanisterId } |
  { 'AddUser' : null } |
  { 'EditAsset' : ResourceIds } |
  { 'EditUserGroup' : ResourceIds } |
  { 'SetDisasterRecovery' : null } |
  { 'EditRequestPolicy' : ResourceIds } |
  { 'RemoveRequestPolicy' : ResourceIds } |
  { 'AddAsset' : null } |
  { 'SystemUpgrade' : null } |
  { 'RemoveAddressBookEntry' : ResourceIds } |
  { 'CreateExternalCanister' : null } |
  { 'EditAddressBookEntry' : ResourceIds } |
  { 'FundExternalCanister' : ExternalCanisterId } |
  { 'EditUser' : ResourceIds } |
  { 'ManageSystemInfo' : null } |
  { 'Transfer' : ResourceIds } |
  { 'EditAccount' : ResourceIds } |
  { 'AddAddressBookEntry' : null } |
  { 'AddRequestPolicy' : null } |
  { 'RemoveNamedRule' : ResourceIds } |
  { 'RemoveUserGroup' : ResourceIds } |
  { 'CallExternalCanister' : CallExternalCanisterResourceTarget } |
  { 'AddNamedRule' : null } |
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
  { 'Notification' : NotificationResourceAction } |
  { 'System' : SystemResourceAction } |
  { 'User' : UserResourceAction } |
  { 'ExternalCanister' : ExternalCanisterResourceAction } |
  { 'Account' : AccountResourceAction } |
  { 'AddressBook' : ResourceAction } |
  { 'Asset' : ResourceAction } |
  { 'NamedRule' : ResourceAction } |
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
export interface RestoreExternalCanisterOperation {
  'input' : RestoreExternalCanisterOperationInput,
}
export interface RestoreExternalCanisterOperationInput {
  'canister_id' : Principal,
  'snapshot_id' : string,
}
export interface SetDisasterRecoveryOperation {
  'committee' : [] | [DisasterRecoveryCommittee],
}
export interface SetDisasterRecoveryOperationInput {
  'committee' : [] | [DisasterRecoveryCommittee],
}
export type Sha256Hash = string;
export interface SnapshotExternalCanisterOperation {
  'input' : SnapshotExternalCanisterOperationInput,
  'snapshot_id' : [] | [string],
}
export interface SnapshotExternalCanisterOperationInput {
  'force' : boolean,
  'replace_snapshot' : [] | [string],
  'canister_id' : Principal,
}
export type SortByDirection = { 'Asc' : null } |
  { 'Desc' : null };
export interface StandardData {
  'supported_operations' : Array<string>,
  'supported_address_formats' : Array<string>,
  'required_metadata_fields' : Array<string>,
  'standard' : string,
}
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
export interface SubnetFilter { 'subnet_type' : [] | [string] }
export type SubnetSelection = { 'Filter' : SubnetFilter } |
  { 'Subnet' : { 'subnet' : Principal } };
export interface SupportedBlockchain {
  'blockchain' : string,
  'supported_standards' : Array<StandardData>,
}
export interface SystemInfo {
  'disaster_recovery' : [] | [DisasterRecovery],
  'upgrader_cycles' : [] | [bigint],
  'name' : string,
  'last_upgrade_timestamp' : TimestampRFC3339,
  'raw_rand_successful' : boolean,
  'version' : string,
  'cycles' : bigint,
  'upgrader_id' : Principal,
  'cycle_obtain_strategy' : CycleObtainStrategy,
  'max_upgrader_backup_snapshots' : bigint,
  'max_station_backup_snapshots' : bigint,
}
export type SystemInfoResult = { 'Ok' : { 'system' : SystemInfo } } |
  { 'Err' : Error };
export interface SystemInit {
  'name' : string,
  'initial_config' : InitialConfig,
  'fallback_controller' : [] | [Principal],
  'upgrader' : SystemUpgraderInput,
}
export type SystemInstall = { 'Upgrade' : SystemUpgrade } |
  { 'Init' : SystemInit };
export type SystemResourceAction = { 'Upgrade' : null } |
  { 'ManageSystemInfo' : null } |
  { 'SystemInfo' : null } |
  { 'Capabilities' : null };
export interface SystemRestoreOperation {
  'input' : SystemRestoreOperationInput,
}
export interface SystemRestoreOperationInput {
  'target' : SystemRestoreTarget,
  'snapshot_id' : string,
}
export type SystemRestoreTarget = { 'RestoreUpgrader' : null } |
  { 'RestoreStation' : null };
export interface SystemUpgrade { 'name' : [] | [string] }
export interface SystemUpgradeOperation {
  'take_backup_snapshot' : [] | [boolean],
  'module_checksum' : Sha256Hash,
  'target' : SystemUpgradeTarget,
  'arg_checksum' : [] | [Sha256Hash],
}
export interface SystemUpgradeOperationInput {
  'arg' : [] | [Uint8Array | number[]],
  'module_extra_chunks' : [] | [WasmModuleExtraChunks],
  'take_backup_snapshot' : [] | [boolean],
  'target' : SystemUpgradeTarget,
  'module' : Uint8Array | number[],
}
export type SystemUpgradeTarget = { 'UpgradeUpgrader' : null } |
  { 'UpgradeStation' : null };
export type SystemUpgraderInput = { 'Id' : Principal } |
  {
    'Deploy' : {
      'initial_cycles' : [] | [bigint],
      'wasm_module' : Uint8Array | number[],
    }
  };
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
  'fee' : [] | [bigint],
  'from_asset' : Asset,
  'network' : Network,
  'transfer_id' : [] | [UUID],
  'from_account' : [] | [Account],
  'input' : TransferOperationInput,
}
export interface TransferOperationInput {
  'to' : string,
  'fee' : [] | [bigint],
  'with_standard' : string,
  'from_account_id' : UUID,
  'metadata' : Array<TransferMetadata>,
  'network' : [] | [Network],
  'amount' : bigint,
  'from_asset_id' : UUID,
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
export interface UserIdentityInput { 'identity' : Principal }
export type UserPrivilege = { 'AddUserGroup' : null } |
  { 'ListRequestPolicies' : null } |
  { 'ListNamedRules' : null } |
  { 'ListPermissions' : null } |
  { 'ListUserGroups' : null } |
  { 'AddUser' : null } |
  { 'ListUsers' : null } |
  { 'AddAsset' : null } |
  { 'SystemUpgrade' : null } |
  { 'CreateExternalCanister' : null } |
  { 'ListAssets' : null } |
  { 'ManageSystemInfo' : null } |
  { 'AddAddressBookEntry' : null } |
  { 'ListAccounts' : null } |
  { 'AddRequestPolicy' : null } |
  { 'ListAddressBookEntries' : null } |
  { 'ListExternalCanisters' : null } |
  { 'ListRequests' : null } |
  { 'CallAnyExternalCanister' : null } |
  { 'SystemInfo' : null } |
  { 'AddNamedRule' : null } |
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
export type ValidationMethodResourceTarget = { 'No' : null } |
  { 'ValidationMethod' : CanisterMethod };
export interface WasmModuleExtraChunks {
  'wasm_module_hash' : Uint8Array | number[],
  'store_canister' : Principal,
  'extra_chunks_key' : string,
}
export interface _SERVICE {
  'cancel_request' : ActorMethod<[CancelRequestInput], CancelRequestResult>,
  'canister_snapshots' : ActorMethod<
    [CanisterSnapshotsInput],
    CanisterSnapshotsResult
  >,
  'canister_status' : ActorMethod<
    [CanisterStatusInput],
    CanisterStatusResponse
  >,
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
  'get_asset' : ActorMethod<[GetAssetInput], GetAssetResult>,
  'get_external_canister' : ActorMethod<
    [GetExternalCanisterInput],
    GetExternalCanisterResult
  >,
  'get_external_canister_filters' : ActorMethod<
    [GetExternalCanisterFiltersInput],
    GetExternalCanisterFiltersResult
  >,
  'get_named_rule' : ActorMethod<[GetNamedRuleInput], GetNamedRuleResult>,
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
  'list_assets' : ActorMethod<[ListAssetsInput], ListAssetsResult>,
  'list_external_canisters' : ActorMethod<
    [ListExternalCanistersInput],
    ListExternalCanistersResult
  >,
  'list_named_rules' : ActorMethod<[ListNamedRulesInput], ListNamedRulesResult>,
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
  'notify_failed_station_upgrade' : ActorMethod<
    [NotifyFailedStationUpgradeInput],
    NotifyFailedStationUpgradeResult
  >,
  'submit_request_approval' : ActorMethod<
    [SubmitRequestApprovalInput],
    SubmitRequestApprovalResult
  >,
  'system_info' : ActorMethod<[], SystemInfoResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
