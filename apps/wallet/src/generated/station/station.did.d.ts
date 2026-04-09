import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

/**
 * A record type that can be used to represent a account in the canister.
 */
export interface Account {
  /**
   * The internal account id.
   */
  'id' : UUID,
  /**
   * The configs approval policy for the account.
   * 
   * The configs approval policy defines the rule that must be met for the account to have its configs updated.
   */
  'configs_request_policy' : [] | [RequestPolicyRule],
  /**
   * Metadata associated with the account (e.g. `{"contract": "0x1234", "symbol": "ANY"}`).
   */
  'metadata' : Array<AccountMetadata>,
  /**
   * A friendly name for the account.
   */
  'name' : string,
  /**
   * The list of assets supported by this account.
   */
  'assets' : Array<AccountAsset>,
  /**
   * The list of addresses associated with the account.
   */
  'addresses' : Array<AccountAddress>,
  /**
   * The transfer approval policy for the account.
   * 
   * The transfer approval policy defines the rule that must be met for a transfer to be approved.
   */
  'transfer_request_policy' : [] | [RequestPolicyRule],
  /**
   * The time at which the account was created or last modified (e.g. "2021-01-01T00:00:00Z").
   */
  'last_modification_timestamp' : TimestampRFC3339,
}
/**
 * Record type to describe an address of an account.
 */
export interface AccountAddress {
  /**
   * The address.
   */
  'address' : string,
  /**
   * The format of the address, eg. icp_account_identifier.
   */
  'format' : string,
}
/**
 * Record type to describe an asset of an account.
 */
export interface AccountAsset {
  /**
   * The balance of the asset.
   */
  'balance' : [] | [AccountBalance],
  /**
   * The asset id.
   */
  'asset_id' : UUID,
}
export interface AccountBalance {
  /**
   * The account id.
   */
  'account_id' : UUID,
  /**
   * The number of decimals used by the asset (e.g. `8` for `BTC`, `18` for `ETH`, etc.).
   */
  'decimals' : number,
  /**
   * The balance of the account.
   */
  'balance' : bigint,
  /**
   * The time at which the balance was last updated.
   */
  'last_update_timestamp' : TimestampRFC3339,
  /**
   * The state of balance query:
   * - `fresh`: The balance was recently updated and is considered fresh.
   * - `stale`: The balance may be out of date.
   * - `stale_refreshing`: The balance may be out of date but it is being refreshed in the background.
   */
  'query_state' : string,
  /**
   * The asset id.
   */
  'asset_id' : UUID,
}
/**
 * A record type that can be used to represent a account balance.
 */
export interface AccountBalanceInfo {
  /**
   * The number of decimals used by the asset (e.g. `8` for `BTC`, `18` for `ETH`, etc.).
   */
  'decimals' : number,
  /**
   * Balance of the account.
   */
  'balance' : bigint,
  /**
   * The time at which the balance was last updated.
   */
  'last_update_timestamp' : TimestampRFC3339,
}
/**
 * A record type that can be used to represent the privileges of a caller for a given account.
 */
export interface AccountCallerPrivileges {
  /**
   * The account id that the caller has privileges for.
   */
  'id' : UUID,
  /**
   * Whether or not the caller can request transfers from the account.
   */
  'can_transfer' : boolean,
  /**
   * Whether or not the caller can edit the account.
   */
  'can_edit' : boolean,
}
/**
 * Account can have additional information attached to them,
 * this type can be used to represent the additional info.
 */
export interface AccountMetadata {
  /**
   * The key of the additional info (e.g. "contract")
   */
  'key' : string,
  /**
   * The value of the additional info (e.g. "0x1234")
   */
  'value' : string,
}
/**
 * The actions that are available for accounts.
 */
export type AccountResourceAction = { 'List' : null } |
  { 'Read' : ResourceId } |
  { 'Create' : null } |
  { 'Transfer' : ResourceId } |
  { 'Update' : ResourceId };
/**
 * The seed used to derive the addresses of the account.
 */
export type AccountSeed = Uint8Array | number[];
export interface AddAccountOperation {
  /**
   * The account, only available after the request is executed.
   */
  'account' : [] | [Account],
  /**
   * The input to the request to add the account.
   */
  'input' : AddAccountOperationInput,
}
/**
 * Input type for adding an account through a request.
 */
export interface AddAccountOperationInput {
  /**
   * The approval policy for updates to the account.
   */
  'configs_request_policy' : [] | [RequestPolicyRule],
  /**
   * Who can read the account information.
   */
  'read_permission' : Allow,
  /**
   * Who can request updates to the account.
   */
  'configs_permission' : Allow,
  /**
   * Metadata associated with the account (e.g. `{"contract": "0x1234", "symbol": "ANY"}`).
   */
  'metadata' : Array<AccountMetadata>,
  /**
   * A friendly name for the account (e.g. "My Account").
   */
  'name' : string,
  /**
   * The assets to add to the account.
   */
  'assets' : Array<UUID>,
  /**
   * The approval policy for transfers from the account.
   */
  'transfer_request_policy' : [] | [RequestPolicyRule],
  /**
   * Who can request transfers from the account.
   */
  'transfer_permission' : Allow,
}
export interface AddAddressBookEntryOperation {
  /**
   * The address book entry, only available after the request is executed.
   */
  'address_book_entry' : [] | [AddressBookEntry],
  /**
   * The input to the request to add the address book entry.
   */
  'input' : AddAddressBookEntryOperationInput,
}
/**
 * Input type for creating a new address book entry through a request.
 */
export interface AddAddressBookEntryOperationInput {
  /**
   * Metadata associated with the address book entry (e.g. `{"kyc": "true"}`).
   */
  'metadata' : Array<AddressBookMetadata>,
  /**
   * The labels associated with the address book entry (e.g. `["exchange", "kyc"]`).
   */
  'labels' : Array<string>,
  /**
   * The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
   */
  'blockchain' : string,
  /**
   * The actual address.
   */
  'address' : string,
  /**
   * The format of the address, eg. icp_account_identifier
   */
  'address_format' : string,
  /**
   * The owner of the address.
   */
  'address_owner' : string,
}
export interface AddAssetOperation {
  /**
   * The result of adding an asset.
   */
  'asset' : [] | [Asset],
  /**
   * The input to the request to add an asset.
   */
  'input' : AddAssetOperationInput,
}
/**
 * The input type for adding an asset.
 */
export interface AddAssetOperationInput {
  /**
   * The number of decimals used by the asset (e.g. `8` for `BTC`, `18` for `ETH`, etc.).
   */
  'decimals' : number,
  /**
   * The asset standard that is supported (e.g. `erc20`, etc.), canonically represented as a lowercase string
   * with spaces replaced with underscores.
   */
  'standards' : Array<string>,
  /**
   * The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`).
   */
  'metadata' : Array<AssetMetadata>,
  /**
   * The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
   */
  'name' : string,
  /**
   * The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
   */
  'blockchain' : string,
  /**
   * The asset symbol, e.g. "ICP" or "BTC".
   */
  'symbol' : AssetSymbol,
}
/**
 * The operation type for adding a new named rule.
 */
export interface AddNamedRuleOperation {
  /**
   * The result of adding a named rule.
   */
  'named_rule' : [] | [NamedRule],
  /**
   * The input to the request to add a named rule.
   */
  'input' : AddNamedRuleOperationInput,
}
/**
 * The input type for creating a named rule.
 */
export interface AddNamedRuleOperationInput {
  /**
   * The rule name.
   */
  'name' : string,
  /**
   * The rule value.
   */
  'rule' : RequestPolicyRule,
  /**
   * The rule description.
   */
  'description' : [] | [string],
}
export interface AddRequestPolicyOperation {
  /**
   * The input to the request to add a request policy.
   */
  'input' : AddRequestPolicyOperationInput,
  /**
   * The request policy that was created by the request (only available after the request is executed).
   */
  'policy_id' : [] | [UUID],
}
export interface AddRequestPolicyOperationInput {
  /**
   * The rule to use for the request evaluation.
   */
  'rule' : RequestPolicyRule,
  /**
   * The request specifier that identifies the request to add a policy for.
   */
  'specifier' : RequestSpecifier,
}
export interface AddUserGroupOperation {
  /**
   * The user group that was added, only available after the request is executed.
   */
  'user_group' : [] | [UserGroup],
  /**
   * The input to the request to add the user group.
   */
  'input' : AddUserGroupOperationInput,
}
export interface AddUserGroupOperationInput {
  /**
   * The name of the group.
   */
  'name' : string,
}
export interface AddUserOperation {
  /**
   * The user that was added, only available after the request is executed.
   */
  'user' : [] | [User],
  /**
   * The input to the request to add the user.
   */
  'input' : AddUserOperationInput,
}
export interface AddUserOperationInput {
  /**
   * The status of the user (e.g. `Active`).
   * 
   * The user must be active to be able to practically use the station.
   */
  'status' : UserStatus,
  /**
   * The list of groups the user belongs to.
   * 
   * Users can be tagged with groups that can be used to control access to the station
   * (e.g. the UUID of the finance group).
   */
  'groups' : Array<UUID>,
  /**
   * The user name (e.g. "John Doe").
   */
  'name' : string,
  /**
   * The principals associated with the user.
   */
  'identities' : Array<Principal>,
}
/**
 * A record type that can be used to represent an address book entry in the canister.
 */
export interface AddressBookEntry {
  /**
   * The internal address book entry id.
   */
  'id' : UUID,
  /**
   * Metadata associated with the address book entry (e.g. `{"kyc": "true"}`).
   */
  'metadata' : Array<AddressBookMetadata>,
  /**
   * The list of labels associated with the address book entry (e.g. `["kyc", "approved"]`).
   */
  'labels' : Array<string>,
  /**
   * The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
   */
  'blockchain' : string,
  /**
   * The actual address.
   */
  'address' : string,
  /**
   * The time at which the address book entry was created or last modified (e.g. "2021-01-01T00:00:00Z").
   */
  'last_modification_timestamp' : string,
  /**
   * The address format (e.g. "icp_account_identifier").
   */
  'address_format' : string,
  /**
   * The address owner.
   */
  'address_owner' : string,
}
/**
 * A record type that can be used to represent the privileges of a caller for a given address book entry.
 */
export interface AddressBookEntryCallerPrivileges {
  /**
   * The address book entry id.
   */
  'id' : UUID,
  /**
   * Whether or not the caller can delete the address book entry.
   */
  'can_delete' : boolean,
  /**
   * Whether or not the caller can edit the address book entry.
   */
  'can_edit' : boolean,
}
/**
 * Address book entries can have additional information attached to them,
 * this type can be used to represent the additional info.
 */
export interface AddressBookMetadata {
  /**
   * The key of the additional info (e.g. "kyc")
   */
  'key' : string,
  /**
   * The value of the additional info (e.g. "true")
   */
  'value' : string,
}
/**
 * The allow rules for who can access the resource.
 */
export interface Allow {
  /**
   * Only the specified user groups can access the resource.
   */
  'user_groups' : Array<UUID>,
  /**
   * Required authentication level for accessing the resource.
   */
  'auth_scope' : AuthScope,
  /**
   * Only the specified users can access the resource.
   */
  'users' : Array<UUID>,
}
/**
 * A record type that can be used to represent an asset in the station.
 */
export interface Asset {
  /**
   * The internal asset id.
   */
  'id' : UUID,
  /**
   * The number of decimals used by the asset (e.g. `8` for `BTC`, `18` for `ETH`, etc.).
   */
  'decimals' : number,
  /**
   * The asset standard that is supported (e.g. `erc20`, etc.), canonically represented as a lowercase string
   * with spaces replaced with underscores.
   */
  'standards' : Array<string>,
  /**
   * The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`).
   */
  'metadata' : Array<AssetMetadata>,
  /**
   * The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
   */
  'name' : string,
  /**
   * The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
   */
  'blockchain' : string,
  /**
   * The asset symbol, e.g. "ICP" or "BTC".
   */
  'symbol' : AssetSymbol,
}
export interface AssetCallerPrivileges {
  'id' : UUID,
  'can_delete' : boolean,
  'can_edit' : boolean,
}
/**
 * Assets can have additional information attached to them,
 * this type can be used to represent the additional info.
 */
export interface AssetMetadata {
  /**
   * The key of the additional info (e.g. "logo")
   */
  'key' : string,
  /**
   * The value of the additional info (e.g. "https://example.com/logo.png")
   */
  'value' : string,
}
/**
 * The asset symbol, e.g. "ICP" or "BTC".
 */
export type AssetSymbol = string;
/**
 * The authorization scope the caller must have, used to specify the required scope for accessing a resource.
 */
export type AuthScope = {
    /**
     * Requires to be an authenticated user to access the resource.
     */
    'Authenticated' : null
  } |
  {
    /**
     * Allows access to the resource without requiring any authentication.
     */
    'Public' : null
  } |
  {
    /**
     * Requires the caller to have direct access to the resource through user groups or user ids.
     */
    'Restricted' : null
  };
/**
 * A basic user record that can be used to represent a user in the station.
 */
export interface BasicUser {
  /**
   * The UUID of the user (e.g. "d0cf5b3f-7017-4cb8-9dcf-52619c42a7b0").
   */
  'id' : UUID,
  /**
   * The status of the user (e.g. `Active`).
   */
  'status' : UserStatus,
  /**
   * The user name (e.g. "John Doe").
   */
  'name' : string,
}
export interface CallExternalCanisterOperation {
  /**
   * This field is not populated in list responses, only when using `get_request` and
   * setting `with_full_info` to `opt true` to avoid going over the response size limit.
   */
  'arg' : [] | [Uint8Array | number[]],
  /**
   * see `CallExternalCanisterOperationInput`
   */
  'execution_method' : CanisterMethod,
  /**
   * see `CallExternalCanisterOperationInput`
   */
  'validation_method' : [] | [CanisterMethod],
  /**
   * The checksum of the argument blob passed to both the validation and execution method.
   * Defaults to `null` if no argument blob is provided.
   */
  'arg_checksum' : [] | [Sha256Hash],
  /**
   * The amount of cycles attached to the call of the execution method.
   */
  'execution_method_cycles' : [] | [bigint],
  /**
   * A human-readable rendering of the argument blob procuded by the validation method.
   */
  'arg_rendering' : [] | [string],
  /**
   * The reply blob produced by a successful call of the execution method,
   * i.e., when the request is `Completed`.
   */
  'execution_method_reply' : [] | [Uint8Array | number[]],
}
export interface CallExternalCanisterOperationInput {
  /**
   * The argument blob passed to both the validation and execution method.
   * Defaults to the candid encoding of '()' if omitted.
   */
  'arg' : [] | [Uint8Array | number[]],
  /**
   * The canister method that is called after the request becomes `Approved`
   * passing the validated argument blob.
   */
  'execution_method' : CanisterMethod,
  /**
   * The canister method validating the argument blob:
   * - on validation success, returns a human-readable rendering of the argument blob
   * and then the request becomes `Created`;
   * - on validation error, returns a textual diagnostic message
   * and then the request creation fails with a validation error
   * containing the textual diagnostic message.
   * Formally, the return type of the validation method must be
   * ```
   * variant {
   * Ok : text;
   * Err : text;
   * }
   * ```
   * If omitted (`validation_method = null`), no validation of the argument blob is performed
   * and no human-readable rendering of the argument blob is provided.
   */
  'validation_method' : [] | [CanisterMethod],
  /**
   * The amount of cycles attached to the call of the execution method.
   */
  'execution_method_cycles' : [] | [bigint],
}
/**
 * The validation and execution method targets of a `CallExternalCanister` request.
 */
export interface CallExternalCanisterResourceTarget {
  'execution_method' : ExecutionMethodResourceTarget,
  'validation_method' : ValidationMethodResourceTarget,
}
/**
 * The input type for canceling a request.
 */
export interface CancelRequestInput {
  /**
   * The request id to cancel.
   */
  'request_id' : UUID,
  /**
   * The reason for canceling the request.
   */
  'reason' : [] | [string],
}
/**
 * The result type for canceling a request.
 */
export type CancelRequestResult = {
    'Ok' : {
      /**
       * The request that was canceled.
       */
      'request' : Request,
    }
  } |
  { 'Err' : Error };
/**
 * The pair that is used to represent the execution and validation method.
 */
export interface CanisterExecutionAndValidationMethodPair {
  /**
   * The method that the caller can call on the external canister.
   * 
   * The `*` method name is used to represent that the caller can call any method on the canister.
   */
  'execution_method' : string,
  /**
   * The validation method that is used to validate the request and
   * render the argument.
   */
  'validation_method' : ValidationMethodResourceTarget,
}
export type CanisterInstallMode = { 'reinstall' : null } |
  { 'upgrade' : null } |
  { 'install' : null };
export interface CanisterMethod {
  /**
   * The canister to call.
   */
  'canister_id' : Principal,
  /**
   * The method to call on the canister.
   */
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
/**
 * A record type that is used to show the current capabilities of the station.
 */
export interface Capabilities {
  /**
   * The name of the station.
   */
  'name' : string,
  /**
   * Version of the station.
   */
  'version' : string,
  /**
   * The list of supported assets.
   */
  'supported_assets' : Array<Asset>,
  /**
   * The list of supported blockchains and standards.
   */
  'supported_blockchains' : Array<SupportedBlockchain>,
}
/**
 * Result type for getting the current config.
 */
export type CapabilitiesResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The config.
       */
      'capabilities' : Capabilities,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Type for instructions to update the address book entry's metadata.
 */
export type ChangeAddressBookMetadata = {
    /**
     * Override values of existing metadata with the specified keys
     * and add new metadata if no metadata can be found with the specified keys.
     */
    'OverrideSpecifiedBy' : Array<AddressBookMetadata>
  } |
  {
    /**
     * Remove metadata with the specified keys.
     */
    'RemoveKeys' : Array<string>
  } |
  {
    /**
     * Replace all existing metadata by the specified metadata.
     */
    'ReplaceAllBy' : Array<AddressBookMetadata>
  };
/**
 * Mutate the list of assets.
 */
export type ChangeAssets = {
    /**
     * Replace all current assets with the specified list.
     */
    'ReplaceWith' : { 'assets' : Array<UUID> }
  } |
  {
    /**
     * Change the list of assets by adding and removing assets.
     */
    'Change' : { 'add_assets' : Array<UUID>, 'remove_assets' : Array<UUID> }
  };
/**
 * Type for instructions to update the external canister's metadata.
 */
export type ChangeExternalCanisterMetadata = {
    /**
     * Override values of existing metadata with the specified keys
     * and add new metadata if no metadata can be found with the specified keys.
     */
    'OverrideSpecifiedBy' : Array<ExternalCanisterMetadata>
  } |
  {
    /**
     * Remove metadata with the specified keys.
     */
    'RemoveKeys' : Array<string>
  } |
  {
    /**
     * Replace all existing metadata by the specified metadata.
     */
    'ReplaceAllBy' : Array<ExternalCanisterMetadata>
  };
export interface ChangeExternalCanisterOperation {
  /**
   * The canister installation mode.
   */
  'mode' : CanisterInstallMode,
  /**
   * The canister to install.
   */
  'canister_id' : Principal,
  /**
   * The checksum of the wasm module.
   */
  'module_checksum' : Sha256Hash,
  /**
   * The checksum of the arg blob.
   */
  'arg_checksum' : [] | [Sha256Hash],
}
export interface ChangeExternalCanisterOperationInput {
  /**
   * The initial argument passed to the new wasm module.
   */
  'arg' : [] | [Uint8Array | number[]],
  /**
   * Additional wasm module chunks to append to the wasm module.
   */
  'module_extra_chunks' : [] | [WasmModuleExtraChunks],
  /**
   * The canister installation mode.
   */
  'mode' : CanisterInstallMode,
  /**
   * The canister to install.
   */
  'canister_id' : Principal,
  /**
   * The wasm module to install.
   */
  'module' : Uint8Array | number[],
}
/**
 * Type for instructions to update the address book entry's metadata.
 */
export type ChangeMetadata = {
    /**
     * Override values of existing metadata with the specified keys
     * and add new metadata if no metadata can be found with the specified keys.
     */
    'OverrideSpecifiedBy' : Array<AssetMetadata>
  } |
  {
    /**
     * Remove metadata with the specified keys.
     */
    'RemoveKeys' : Array<string>
  } |
  {
    /**
     * Replace all existing metadata by the specified metadata.
     */
    'ReplaceAllBy' : Array<AssetMetadata>
  };
export type ConfigureExternalCanisterOperation = ConfigureExternalCanisterOperationInput;
export interface ConfigureExternalCanisterOperationInput {
  /**
   * The kind of operation to perform.
   */
  'kind' : ConfigureExternalCanisterOperationKind,
  /**
   * The canister to update.
   */
  'canister_id' : Principal,
}
/**
 * The input type for configuring an external canister in the station.
 */
export type ConfigureExternalCanisterOperationKind = {
    /**
     * Remove the canister from the Station only.
     */
    'SoftDelete' : null
  } |
  {
    /**
     * The settings to configure for the external canister.
     */
    'Settings' : ConfigureExternalCanisterSettingsInput
  } |
  {
    /**
     * Remove the canister from the Station and the IC.
     * 
     * Caution: This operation is irreversible.
     */
    'Delete' : null
  } |
  {
    /**
     * The Internet Computer canister settings to configure for the external canister.
     */
    'NativeSettings' : DefiniteCanisterSettingsInput
  };
export interface ConfigureExternalCanisterSettingsInput {
  /**
   * What operations are allowed on the canister.
   */
  'permissions' : [] | [ExternalCanisterPermissionsUpdateInput],
  'name' : [] | [string],
  /**
   * The labels of the external canister.
   */
  'labels' : [] | [Array<string>],
  /**
   * The description of the external canister.
   */
  'description' : [] | [string],
  /**
   * The request policies for the canister.
   */
  'request_policies' : [] | [ExternalCanisterRequestPoliciesUpdateInput],
  /**
   * The state of the external canister.
   */
  'state' : [] | [ExternalCanisterState],
  /**
   * The metadata of the external canister.
   */
  'change_metadata' : [] | [ChangeExternalCanisterMetadata],
}
export interface CreateExternalCanisterOperation {
  'canister_id' : [] | [Principal],
  'input' : CreateExternalCanisterOperationInput,
}
export interface CreateExternalCanisterOperationInput {
  /**
   * What operations are allowed on the canister.
   */
  'permissions' : ExternalCanisterPermissionsCreateInput,
  /**
   * The metadata of the external canister.
   */
  'metadata' : [] | [Array<ExternalCanisterMetadata>],
  /**
   * The kind of create operation to perform.
   */
  'kind' : CreateExternalCanisterOperationKind,
  /**
   * The name of the external canister.
   */
  'name' : string,
  /**
   * The labels of the external canister.
   */
  'labels' : [] | [Array<string>],
  /**
   * The description of the external canister.
   */
  'description' : [] | [string],
  /**
   * The request policies for the canister.
   */
  'request_policies' : ExternalCanisterRequestPoliciesCreateInput,
}
export type CreateExternalCanisterOperationKind = {
    /**
     * An existing canister is added to the station.
     */
    'AddExisting' : CreateExternalCanisterOperationKindAddExisting
  } |
  {
    /**
     * A new canister is created.
     */
    'CreateNew' : CreateExternalCanisterOperationKindCreateNew
  };
export interface CreateExternalCanisterOperationKindAddExisting {
  /**
   * The canister id to use.
   */
  'canister_id' : Principal,
}
export interface CreateExternalCanisterOperationKindCreateNew {
  /**
   * The initial cycles to allocate to the canister.
   * 
   * If not set, only the minimal amount of cycles required to create the
   * canister will be allocated.
   */
  'initial_cycles' : [] | [bigint],
  /**
   * The subnet on which the canister should be created.
   * 
   * By default, the canister is created on the same subnet as the station.
   */
  'subnet_selection' : [] | [SubnetSelection],
}
/**
 * The input type for creating a request.
 */
export interface CreateRequestInput {
  /**
   * The request title (e.g. "Payment to John").
   */
  'title' : [] | [string],
  /**
   * The time at which the request will execute if approved.
   */
  'execution_plan' : [] | [RequestExecutionSchedule],
  /**
   * The time at which the request will expire if still pending.
   */
  'expiration_dt' : [] | [TimestampRFC3339],
  /**
   * The optional deduplication key used to ensure request uniqueness.
   */
  'deduplication_key' : [] | [string],
  /**
   * The list of tags for the request.
   */
  'tags' : [] | [Array<string>],
  /**
   * The request summary (e.g. "This request will transfer 100 ICP to the account 0x1234").
   */
  'summary' : [] | [string],
  /**
   * The operation that was requested.
   */
  'operation' : RequestOperationInput,
}
/**
 * The result type for creating a request.
 */
export type CreateRequestResult = {
    'Ok' : {
      /**
       * The privileges of the caller.
       */
      'privileges' : RequestCallerPrivileges,
      /**
       * The request that was created.
       */
      'request' : Request,
      /**
       * The additional info about the request.
       */
      'additional_info' : RequestAdditionalInfo,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Strategy defining how the station canister tops up its own cycles.
 */
export type CycleObtainStrategy = {
    /**
     * Do not obtain cycles for Orbit.
     */
    'Disabled' : null
  } |
  {
    /**
     * Use the CMC to mint cycles from ICP held in an Orbit account.
     */
    'MintFromNativeToken' : {
      /**
       * The Orbit account ID to use for minting cycles.
       */
      'account_id' : UUID,
      /**
       * The Orbit account name.
       */
      'account_name' : [] | [string],
    }
  } |
  {
    /**
     * Use the Cycles Ledger balance to obtain cycles.
     */
    'WithdrawFromCyclesLedger' : {
      /**
       * The Orbit account ID to use for obtaining cycles.
       */
      'account_id' : UUID,
      /**
       * The Orbit account name.
       */
      'account_name' : [] | [string],
    }
  };
/**
 * Strategy defining how the station canister tops up its own cycles.
 */
export type CycleObtainStrategyInput = {
    /**
     * Do not obtain cycles for Orbit.
     */
    'Disabled' : null
  } |
  {
    /**
     * Use the CMC to mint cycles from ICP held in an Orbit account.
     */
    'MintFromNativeToken' : {
      /**
       * The Orbit account ID to use for minting cycles.
       */
      'account_id' : UUID,
    }
  } |
  {
    /**
     * Use the Cycles Ledger balance to obtain cycles.
     */
    'WithdrawFromCyclesLedger' : {
      /**
       * The Orbit account ID to use for obtaining cycles.
       */
      'account_id' : UUID,
    }
  };
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
/**
 * The disaster recovery committee extended with the user group name.
 */
export interface DisasterRecovery {
  /**
   * The name of the disaster recovery committee user group.
   */
  'user_group_name' : [] | [string],
  /**
   * The disaster recovery committee.
   */
  'committee' : DisasterRecoveryCommittee,
}
export interface DisasterRecoveryCommittee {
  /**
   * The user group id of the committee.
   */
  'user_group_id' : UUID,
  /**
   * The quorum required for the committee to approve a disaster recovery operation.
   */
  'quorum' : number,
}
/**
 * A minimal user record that is meant to aid displaying users on the client.
 */
export interface DisplayUser {
  /**
   * The UUID of the user (e.g. "d0cf5b3f-7017-4cb8-9dcf-52619c42a7b0").
   */
  'id' : UUID,
  /**
   * The user name (e.g. "John Doe").
   */
  'name' : string,
}
export interface EditAccountOperation {
  /**
   * The input to the request to edit the account.
   */
  'input' : EditAccountOperationInput,
}
/**
 * Input type for editing an account through a request.
 */
export interface EditAccountOperationInput {
  /**
   * The account id that will be edited.
   */
  'account_id' : UUID,
  /**
   * The request policy for what it takes to execute a configuration change.
   */
  'configs_request_policy' : [] | [RequestPolicyRuleInput],
  /**
   * Who can read the account information.
   */
  'read_permission' : [] | [Allow],
  /**
   * Who can request configuration changes to the account.
   */
  'configs_permission' : [] | [Allow],
  /**
   * A friendly name for the account (e.g. "My Account").
   */
  'name' : [] | [string],
  /**
   * Mutate the list of assets.
   */
  'change_assets' : [] | [ChangeAssets],
  /**
   * The request policy for what it takes to execute a transfer.
   */
  'transfer_request_policy' : [] | [RequestPolicyRuleInput],
  /**
   * Who can request transfers from the account.
   */
  'transfer_permission' : [] | [Allow],
}
export interface EditAddressBookEntryOperation {
  /**
   * The input to the request to edit the address book entry.
   */
  'input' : EditAddressBookEntryOperationInput,
}
/**
 * Input type for updating an address book entry through a request.
 */
export interface EditAddressBookEntryOperationInput {
  /**
   * The updated list of labels associated with the address book entry.
   */
  'labels' : [] | [Array<string>],
  /**
   * Instructions to update the address book entry's metadata.
   */
  'change_metadata' : [] | [ChangeAddressBookMetadata],
  /**
   * The id of the address book entry.
   */
  'address_book_entry_id' : UUID,
  /**
   * The new owner of the address.
   */
  'address_owner' : [] | [string],
}
export interface EditAssetOperation {
  /**
   * The input to the request to edit an asset.
   */
  'input' : EditAssetOperationInput,
}
/**
 * The input type for editing an asset.
 */
export interface EditAssetOperationInput {
  /**
   * The asset standard that is supported (e.g. `erc20`, etc.), canonically represented as a lowercase string
   * with spaces replaced with underscores.
   */
  'standards' : [] | [Array<string>],
  /**
   * The name of the asset.
   */
  'name' : [] | [string],
  /**
   * The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
   */
  'blockchain' : [] | [string],
  /**
   * The metadata to change.
   */
  'change_metadata' : [] | [ChangeMetadata],
  /**
   * The asset id to edit.
   */
  'asset_id' : UUID,
  /**
   * The asset symbol, e.g. "ICP" or "BTC".
   */
  'symbol' : [] | [AssetSymbol],
}
/**
 * The operation type for editing an existing named rule.
 */
export interface EditNamedRuleOperation {
  /**
   * The input to the request to edit a named rule.
   */
  'input' : EditNamedRuleOperationInput,
}
/**
 * The input type for editing a named rule.
 */
export interface EditNamedRuleOperationInput {
  /**
   * The rule name.
   */
  'name' : [] | [string],
  /**
   * The rule value.
   */
  'rule' : [] | [RequestPolicyRule],
  /**
   * The optional rule description.
   */
  'description' : [] | [[] | [string]],
  /**
   * The named rule id to edit.
   */
  'named_rule_id' : UUID,
}
export interface EditPermissionOperation {
  /**
   * The input to the request to edit an permission.
   */
  'input' : EditPermissionOperationInput,
}
export interface EditPermissionOperationInput {
  /**
   * The updated resource that this policy will apply to.
   */
  'resource' : Resource,
  /**
   * The updated list of user groups that have access to the resource.
   */
  'user_groups' : [] | [Array<UUID>],
  /**
   * The updated authorization scope for the resource.
   */
  'auth_scope' : [] | [AuthScope],
  /**
   * The updated list of users that have access to the resource.
   */
  'users' : [] | [Array<UUID>],
}
export interface EditRequestPolicyOperation {
  /**
   * The input to the request to edit a request policy.
   */
  'input' : EditRequestPolicyOperationInput,
}
export interface EditRequestPolicyOperationInput {
  /**
   * The updated rule to use for the request evaluation.
   */
  'rule' : [] | [RequestPolicyRule],
  /**
   * The updated request specifier that identifies the request to add a policy for.
   */
  'specifier' : [] | [RequestSpecifier],
  /**
   * The request policy id that will be edited.
   */
  'policy_id' : UUID,
}
export interface EditUserGroupOperation {
  /**
   * The input to the request to edit the user group.
   */
  'input' : EditUserGroupOperationInput,
}
export interface EditUserGroupOperationInput {
  /**
   * The name of the group.
   */
  'name' : string,
  /**
   * The id of the group to edit.
   */
  'user_group_id' : UUID,
}
export interface EditUserOperation {
  /**
   * The input to the request to edit the user.
   */
  'input' : EditUserOperationInput,
}
export interface EditUserOperationInput {
  /**
   * The id of the user to edit.
   */
  'id' : UUID,
  /**
   * The status of the user (e.g. `Active`).
   */
  'status' : [] | [UserStatus],
  /**
   * The list of groups the user belongs to.
   * 
   * Users can be tagged with groups that can be used to control access to the station
   * (e.g. "UUID of the finance group").
   */
  'groups' : [] | [Array<UUID>],
  /**
   * Cancel all pending (request status `Created`) requests for this user.
   */
  'cancel_pending_requests' : [] | [boolean],
  /**
   * The user name (e.g. "John Doe").
   */
  'name' : [] | [string],
  /**
   * The principals associated with the user.
   */
  'identities' : [] | [Array<Principal>],
}
/**
 * Generic error type added to responses that can fail.
 */
export interface Error {
  /**
   * Error code, added as a string to allow for custom error codes.
   */
  'code' : string,
  /**
   * Error message to be displayed to the user.
   */
  'message' : [] | [string],
  /**
   * Error details to be displayed to the user.
   */
  'details' : [] | [Array<[string, string]>],
}
/**
 * Defines the evaluation data of a request policy rule.
 */
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
/**
 * Defines the high level result of evaluating a request policy rule.
 */
export type EvaluationStatus = { 'Approved' : null } |
  { 'Rejected' : null } |
  { 'Pending' : null };
/**
 * List of reasons why a request can be approved or rejected.
 */
export type EvaluationSummaryReason = { 'AllowList' : null } |
  { 'AllowListMetadata' : null } |
  { 'AutoApproved' : null } |
  { 'ApprovalQuorum' : null };
/**
 * The execution method targets of a `CallExternalCanister` request.
 */
export type ExecutionMethodResourceTarget = { 'Any' : null } |
  { 'ExecutionMethod' : CanisterMethod };
/**
 * An external canister that the station can interact with.
 */
export interface ExternalCanister {
  /**
   * The id of the resource in the station.
   */
  'id' : UUID,
  /**
   * The permissions that are set for who can interact with the canister.
   */
  'permissions' : ExternalCanisterPermissions,
  /**
   * The time at which the canister was last modified, if available.
   */
  'modified_at' : [] | [TimestampRFC3339],
  /**
   * The metadata that is associated with the canister.
   */
  'metadata' : Array<ExternalCanisterMetadata>,
  /**
   * The name of the canister.
   */
  'name' : string,
  /**
   * The labels that can be used to categorize the canister.
   */
  'labels' : Array<string>,
  /**
   * The principal id of the canister.
   */
  'canister_id' : Principal,
  /**
   * A description that can be used to describe the canister.
   */
  'description' : [] | [string],
  /**
   * The time at which the canister was created.
   */
  'created_at' : TimestampRFC3339,
  /**
   * The request policies that are associated with the canister.
   */
  'request_policies' : ExternalCanisterRequestPolicies,
  /**
   * The current state of the record (e.g. `Active`).
   */
  'state' : ExternalCanisterState,
  /**
   * Monitoring configuration for the canister.
   */
  'monitoring' : [] | [MonitorExternalCanisterStartInput],
}
/**
 * The permission for making calls to a specific or any external canister method.
 */
export interface ExternalCanisterCallPermission {
  /**
   * The execution method that the caller can use,
   * if `*` is used the caller can use any method.
   */
  'execution_method' : string,
  /**
   * Allowed users and user groups for the operation.
   */
  'allow' : Allow,
  /**
   * The validation method that is used to validate the request and
   * render the argument.
   */
  'validation_method' : ValidationMethodResourceTarget,
}
/**
 * The request policy rule for the canister call operation.
 */
export interface ExternalCanisterCallRequestPolicyRule {
  /**
   * The method name that the rule is for,
   * if `*` is used the rule applies to all methods.
   */
  'execution_method' : string,
  /**
   * The request policy rule for the canister call operation.
   */
  'rule' : RequestPolicyRule,
  /**
   * The validation method that is used to match the policy against
   * the permission of the resource.
   */
  'validation_method' : ValidationMethodResourceTarget,
  /**
   * The id of the request policy rule.
   */
  'policy_id' : UUID,
}
/**
 * The request policy rule for the canister call operation.
 */
export interface ExternalCanisterCallRequestPolicyRuleInput {
  /**
   * The method name that the rule is for,
   * if `*` is used the rule applies to all methods.
   */
  'execution_method' : string,
  /**
   * The request policy rule for the canister call operation.
   */
  'rule' : RequestPolicyRule,
  /**
   * The validation method that is used to match the policy against
   * the permission of the resource.
   */
  'validation_method' : ValidationMethodResourceTarget,
  /**
   * The id of the request policy rule.
   * 
   * If not provided a new entry will be created.
   */
  'policy_id' : [] | [UUID],
}
/**
 * The caller privileges for the external canister methods.
 */
export interface ExternalCanisterCallerMethodsPrivileges {
  /**
   * The method that the caller can call on the external canister.
   * 
   * The `*` method name is used to represent that the caller can
   * call any method on the canister.
   */
  'execution_method' : string,
  /**
   * The validation method that is used to validate the request and
   * render the argument.
   */
  'validation_method' : ValidationMethodResourceTarget,
}
/**
 * The caller privileges for the external canister.
 */
export interface ExternalCanisterCallerPrivileges {
  /**
   * The external canister entry id.
   */
  'id' : UUID,
  /**
   * Whether or not the caller can edit the external canister.
   */
  'can_change' : boolean,
  /**
   * The canister id.
   */
  'canister_id' : Principal,
  /**
   * The list of methods that the caller can call on the external canister.
   */
  'can_call' : Array<ExternalCanisterCallerMethodsPrivileges>,
  /**
   * Whether or not the caller can fund the external canister.
   */
  'can_fund' : boolean,
}
/**
 * The input type for setting call permissions of an existing external canister.
 */
export type ExternalCanisterChangeCallPermissionsInput = {
    /**
     * Override the call permissions from the specified execution methods.
     */
    'OverrideSpecifiedByExecutionMethods' : Array<
      {
        /**
         * The method that the caller can call on the external canister.
         * 
         * The `*` method name is used to represent that the caller can call any method on the canister.
         */
        'execution_method' : string,
        /**
         * The permissions associated with the execution method, if the list is empty all call permissions of the
         * execution method will be removed.
         */
        'permissions' : Array<
          {
            /**
             * Allowed users and user groups for the operation.
             */
            'allow' : Allow,
            /**
             * The validation method that is used to validate the request and render the argument.
             */
            'validation_method' : ValidationMethodResourceTarget,
          }
        >,
      }
    >
  } |
  {
    /**
     * Override the permissions for the specified execution and validation method pairs.
     */
    'OverrideSpecifiedByExecutionValidationMethodPairs' : Array<
      {
        /**
         * If allow is not provided the call permission will be removed for the specified execution
         * and validation method pair.
         */
        'allow' : [] | [Allow],
        /**
         * The method configuration that is used to represent the execution and validation method pair.
         */
        'method_configuration' : CanisterExecutionAndValidationMethodPair,
      }
    >
  } |
  {
    /**
     * Replaces all the call permissions with the provided list, if the list is empty
     * all the call permissions will be removed.
     */
    'ReplaceAllBy' : Array<ExternalCanisterCallPermission>
  };
export type ExternalCanisterChangeCallRequestPoliciesInput = {
    /**
     * Remove call request policies by the provided ids.
     */
    'RemoveByPolicyIds' : Array<UUID>
  } |
  {
    /**
     * Override the request policies for the specified execution methods.
     */
    'OverrideSpecifiedByExecutionMethods' : Array<
      {
        /**
         * The method that the caller can call on the external canister.
         * 
         * The `*` method name is used to represent that the caller can call any method on the canister.
         */
        'execution_method' : string,
        /**
         * The request policies associated with the execution method, if the list is empty all the policies of
         * the execution method will be removed.
         */
        'policies' : Array<
          {
            /**
             * The request policy rule for the canister call operation.
             */
            'rule' : RequestPolicyRule,
            /**
             * The validation method that is used to match the policy against
             * the permission of the resource.
             */
            'validation_method' : ValidationMethodResourceTarget,
            /**
             * The id of the request policy rule.
             * 
             * If not provided a new entry will be created.
             */
            'policy_id' : [] | [UUID],
          }
        >,
      }
    >
  } |
  {
    /**
     * Override the request policies for the specified execution and validation method pairs.
     */
    'OverrideSpecifiedByExecutionValidationMethodPairs' : Array<
      {
        /**
         * The method configuration that is used to represent the execution and validation method pair.
         */
        'method_configuration' : CanisterExecutionAndValidationMethodPair,
        /**
         * The request policies to use for the method configuration, if the list is empty all the policies of
         * the execution and validation method pair will be removed.
         */
        'policies' : Array<ExternalCanisterChangeRequestPolicyRuleInput>,
      }
    >
  } |
  {
    /**
     * Replaces all the call request policies with the provided list.
     */
    'ReplaceAllBy' : Array<ExternalCanisterCallRequestPolicyRuleInput>
  };
/**
 * The request policy rule for the canister change operation.
 */
export interface ExternalCanisterChangeRequestPolicyRule {
  /**
   * The request policy rule for the canister change operation.
   */
  'rule' : RequestPolicyRule,
  /**
   * The id of the request policy rule.
   */
  'policy_id' : UUID,
}
/**
 * The request policy rule for the canister change operation.
 */
export interface ExternalCanisterChangeRequestPolicyRuleInput {
  /**
   * The request policy rule for the canister change operation.
   */
  'rule' : RequestPolicyRule,
  /**
   * The id of the request policy rule.
   * 
   * If not provided a new entry will be created.
   */
  'policy_id' : [] | [UUID],
}
/**
 * The target canister to interact with.
 */
export type ExternalCanisterId = { 'Any' : null } |
  { 'Canister' : Principal };
/**
 * ExternalCanister can have additional information attached to them,
 * this type can be used to represent the additional info.
 */
export interface ExternalCanisterMetadata {
  /**
   * The key of the additional info (e.g. "app_id")
   */
  'key' : string,
  /**
   * The value of the additional info (e.g. "2ec270f1-7663-4d51-b70f-9339486b6d6d")
   */
  'value' : string,
}
/**
 * The permissions set for the external canister.
 */
export interface ExternalCanisterPermissions {
  /**
   * The permissions for the calling methods on the canister.
   */
  'calls' : Array<ExternalCanisterCallPermission>,
  /**
   * Who can read information about the canister (e.g. canister status),
   * changes to this permission can be made by the `change` permission.
   */
  'read' : Allow,
  /**
   * Who can make changes to the canister, includes:
   * - changing the permissions
   * - install operations
   */
  'change' : Allow,
}
/**
 * The create input type for setting the permissions for the external canister.
 */
export type ExternalCanisterPermissionsCreateInput = ExternalCanisterPermissions;
/**
 * The input type for setting the permissions for the external canister.
 */
export interface ExternalCanisterPermissionsUpdateInput {
  /**
   * The permissions for calling methods on the canister.
   */
  'calls' : [] | [ExternalCanisterChangeCallPermissionsInput],
  /**
   * Who can read information about the canister (e.g. canister status),
   * changes to this permission can be made by the `change` permission.
   */
  'read' : [] | [Allow],
  /**
   * Who can make changes to the canister, includes:
   * - changing the permissions
   * - install operations
   */
  'change' : [] | [Allow],
}
/**
 * The request policy rules for the external canister.
 */
export interface ExternalCanisterRequestPolicies {
  /**
   * The request policy rules for the calling methods on the canister.
   */
  'calls' : Array<ExternalCanisterCallRequestPolicyRule>,
  /**
   * The request policy rules for the canister change operation.
   */
  'change' : Array<ExternalCanisterChangeRequestPolicyRule>,
}
/**
 * The input type for setting the request policies for a new external canister.
 */
export interface ExternalCanisterRequestPoliciesCreateInput {
  /**
   * The request policy rules for the calling methods on the canister.
   */
  'calls' : Array<ExternalCanisterCallRequestPolicyRuleInput>,
  /**
   * The request policy rules for the canister change operation.
   */
  'change' : Array<ExternalCanisterChangeRequestPolicyRuleInput>,
}
/**
 * The input type for setting the request policies for an existing external canister.
 */
export interface ExternalCanisterRequestPoliciesUpdateInput {
  /**
   * The request policy rules for the calling methods on the canister.
   */
  'calls' : [] | [ExternalCanisterChangeCallRequestPoliciesInput],
  /**
   * The request policy rules for the canister change operation.
   */
  'change' : [] | [Array<ExternalCanisterChangeRequestPolicyRuleInput>],
}
/**
 * The actions that are available for external canisters.
 */
export type ExternalCanisterResourceAction = {
    'Call' : CallExternalCanisterResourceTarget
  } |
  { 'Fund' : ExternalCanisterId } |
  { 'List' : null } |
  { 'Read' : ExternalCanisterId } |
  { 'Create' : null } |
  { 'Change' : ExternalCanisterId };
/**
 * The state of the external canister.
 */
export type ExternalCanisterState = {
    /**
     * The record is active and can be interacted with.
     */
    'Active' : null
  } |
  {
    /**
     * The record is archived and can no longer be interacted with.
     */
    'Archived' : null
  };
/**
 * Input type for getting a account balance.
 */
export interface FetchAccountBalancesInput {
  /**
   * The account ids to retrieve.
   */
  'account_ids' : Array<UUID>,
}
/**
 * Result type for getting a account.
 */
export type FetchAccountBalancesResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The account balance that was retrieved.
       */
      'balances' : Array<[] | [AccountBalance]>,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * The request operation for funding an external canister from the station.
 */
export type FundExternalCanisterOperation = FundExternalCanisterOperationInput;
/**
 * The input type for funding an external canister in the station.
 */
export interface FundExternalCanisterOperationInput {
  /**
   * The kind of funding operation to perform.
   */
  'kind' : FundExternalCanisterOperationKind,
  /**
   * The external canister to fund.
   */
  'canister_id' : Principal,
}
/**
 * The operation kind for funding an external canister in the station.
 */
export type FundExternalCanisterOperationKind = {
    /**
     * The amount of cycles to send to the canister.
     */
    'Send' : FundExternalCanisterSendCyclesInput
  };
/**
 * The input type for specifying the cycles to send to an external canister.
 */
export interface FundExternalCanisterSendCyclesInput {
  /**
   * The amount of cycles to send to the canister.
   */
  'cycles' : bigint,
}
/**
 * Input type for getting a account.
 */
export interface GetAccountInput {
  /**
   * The account id to retrieve.
   */
  'account_id' : UUID,
}
/**
 * Result type for getting a account.
 */
export type GetAccountResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The privileges of the caller for the account.
       */
      'privileges' : AccountCallerPrivileges,
      /**
       * The account that was retrieved.
       */
      'account' : Account,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Input type for getting a single address book entry.
 */
export interface GetAddressBookEntryInput {
  /**
   * The address book entry id to retrieve.
   */
  'address_book_entry_id' : UUID,
}
/**
 * Result type for getting an address book entry.
 */
export type GetAddressBookEntryResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The privileges of the caller for the address book entry.
       */
      'privileges' : AddressBookEntryCallerPrivileges,
      /**
       * The address book entry that was retrieved.
       */
      'address_book_entry' : AddressBookEntry,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * The input type for getting an asset.
 */
export interface GetAssetInput {
  /**
   * The asset id to retrieve.
   */
  'asset_id' : UUID,
}
/**
 * The result type for getting an asset.
 */
export type GetAssetResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The caller privileges for the asset.
       */
      'privileges' : AssetCallerPrivileges,
      /**
       * The asset that was retrieved.
       */
      'asset' : Asset,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * The input type for getting the available filters for the external canisters.
 */
export interface GetExternalCanisterFiltersInput {
  /**
   * Include the labels of the external canisters in the result.
   */
  'with_labels' : [] | [boolean],
  /**
   * Include the names of the external canisters in the result.
   */
  'with_name' : [] | [
    {
      /**
       * The prefix to use for filtering the names.
       * 
       * If the prefix is not provided, any name will be returned.
       */
      'prefix' : [] | [string],
    }
  ],
}
/**
 * The result type for the filtering of external canisters.
 */
export type GetExternalCanisterFiltersResult = {
    'Ok' : {
      /**
       * The list of labels that are used by the external canisters.
       */
      'labels' : [] | [Array<string>],
      /**
       * The list of names that are used by the external canisters
       * and their canister id.
       */
      'names' : [] | [Array<{ 'name' : string, 'canister_id' : Principal }>],
    }
  } |
  { 'Err' : Error };
/**
 * Input type for getting a external canister.
 */
export interface GetExternalCanisterInput {
  /**
   * The principal id of the external canister.
   */
  'canister_id' : Principal,
}
export type GetExternalCanisterResult = {
    'Ok' : {
      /**
       * The caller privileges for the external canister.
       */
      'privileges' : ExternalCanisterCallerPrivileges,
      /**
       * The external canister that was retrieved.
       */
      'canister' : ExternalCanister,
    }
  } |
  { 'Err' : Error };
/**
 * Input type for getting a named rule.
 */
export interface GetNamedRuleInput {
  /**
   * The named rule to retrieve by the id.
   */
  'named_rule_id' : UUID,
}
/**
 * Result type for retrieving a named rule.
 */
export type GetNamedRuleResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The privileges of the caller for the named rule.
       */
      'privileges' : NamedRuleCallerPrivileges,
      /**
       * The named rule that was retrieved.
       */
      'named_rule' : NamedRule,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * The input type for getting the list of requests based on the given filters.
 */
export interface GetNextApprovableRequestInput {
  /**
   * Get the next request from a list sorted by the given field.
   */
  'sort_by' : [] | [ListRequestsSortBy],
  /**
   * Exclude requests the user indicated to skip.
   */
  'excluded_request_ids' : Array<UUID>,
  /**
   * The type of the request (e.g. "transfer").
   */
  'operation_types' : [] | [Array<ListRequestsOperationType>],
}
/**
 * Result type for retrieving a request.
 */
export type GetNextApprovableRequestResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : [] | [GetRequestResultData]
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
export interface GetPermissionInput {
  /**
   * The resource to retrieve the permission for.
   */
  'resource' : Resource,
}
export type GetPermissionResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The permission that was retrieved.
       */
      'permission' : Permission,
      /**
       * The privileges of the caller for the permission.
       */
      'privileges' : PermissionCallerPrivileges,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Input type for getting a request.
 */
export interface GetRequestInput {
  /**
   * The request id to retrieve.
   */
  'request_id' : UUID,
  /**
   * Fill in all the additional info about the request operation, request types such as `CallExternalCanisterOperation`
   * will include the request argument, this can be a large amount of data and could potentially exceed the response
   * size limit.
   * 
   * If not provided, this field defaults to `false` and the additional info is not included in the response.
   */
  'with_full_info' : [] | [boolean],
}
export interface GetRequestPolicyInput {
  /**
   * The id of the request policy to retrieve.
   */
  'id' : UUID,
}
export type GetRequestPolicyResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The privileges of the caller for the request policy.
       */
      'privileges' : RequestPolicyCallerPrivileges,
      /**
       * The request policy that was retrieved.
       */
      'policy' : RequestPolicy,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Result type for retrieving a request.
 */
export type GetRequestResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : GetRequestResultData
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
export interface GetRequestResultData {
  /**
   * The privileges of the caller.
   */
  'privileges' : RequestCallerPrivileges,
  /**
   * The request that was requested.
   */
  'request' : Request,
  /**
   * The additional info about the request.
   */
  'additional_info' : RequestAdditionalInfo,
}
export interface GetTransfersInput {
  /**
   * The list of transfer ids to retrieve.
   */
  'transfer_ids' : Array<UUID>,
}
export type GetTransfersResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The transfer that was retrieved.
       */
      'transfers' : Array<Transfer>,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Input type for getting a user group.
 */
export interface GetUserGroupInput {
  /**
   * The group id to retrieve.
   */
  'user_group_id' : UUID,
}
/**
 * Result type for getting a user group.
 */
export type GetUserGroupResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The caller privileges for the user group.
       */
      'privileges' : UserGroupCallerPrivileges,
      /**
       * The group that was retrieved.
       */
      'user_group' : UserGroup,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Input type for retrieving a user.
 */
export interface GetUserInput {
  /**
   * The user id to retrieve (e.g. "d0cf5b3f-7017-4cb8-9dcf-52619c42a7b0").
   */
  'user_id' : UUID,
}
/**
 * Result type for retrieving a user.
 */
export type GetUserResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The caller privileges for the user.
       */
      'privileges' : UserCallerPrivileges,
      /**
       * The user that was retrieved.
       */
      'user' : User,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
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
/**
 * The initial accounts to create when initializing the canister for the first time, e.g., after disaster recovery.
 */
export interface InitAccountInput {
  /**
   * The UUID of the account, if not provided a new UUID will be generated.
   */
  'id' : [] | [UUID],
  /**
   * Metadata associated with the account (e.g. `{"contract": "0x1234", "symbol": "ANY"}`).
   */
  'metadata' : Array<AccountMetadata>,
  /**
   * A friendly name for the account (e.g. "My Account").
   */
  'name' : string,
  /**
   * The asset standard for this account (e.g. `native`, `erc20`, etc.).
   */
  'assets' : Array<UUID>,
  /**
   * The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
   */
  'seed' : AccountSeed,
}
/**
 * The permissions for the account.
 */
export interface InitAccountPermissionsInput {
  /**
   * The approval policy for updates to the account.
   */
  'configs_request_policy' : [] | [RequestPolicyRule],
  /**
   * Who can read the account information.
   */
  'read_permission' : Allow,
  /**
   * Who can request updates to the account.
   */
  'configs_permission' : Allow,
  /**
   * The approval policy for transfers from the account.
   */
  'transfer_request_policy' : [] | [RequestPolicyRule],
  /**
   * Who can request transfers from the account.
   */
  'transfer_permission' : Allow,
}
/**
 * The initial account to create when initializing the canister for the first time.
 */
export interface InitAccountWithPermissionsInput {
  /**
   * The permissions for the account.
   */
  'permissions' : InitAccountPermissionsInput,
  /**
   * The initial account to create.
   */
  'account_init' : InitAccountInput,
}
/**
 * The initial assets to create when initializing the canister for the first time, e.g., after disaster recovery.
 */
export interface InitAssetInput {
  /**
   * The UUID of the asset, if not provided a new UUID will be generated.
   */
  'id' : [] | [UUID],
  /**
   * The number of decimals used to format the asset balance.
   */
  'decimals' : number,
  /**
   * The standards this asset supports.
   */
  'standards' : Array<string>,
  /**
   * Metadata associated with the asset.
   */
  'metadata' : Array<AssetMetadata>,
  /**
   * The name of the asset.
   */
  'name' : string,
  /**
   * The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
   */
  'blockchain' : string,
  /**
   * The asset symbol, e.g. "ICP" or "BTC".
   */
  'symbol' : string,
}
/**
 * The init type for adding a named rule when initializing the canister for the first time.
 */
export interface InitNamedRuleInput {
  /**
   * The id of the named rule, if not provided a new UUID will be generated.
   */
  'id' : [] | [UUID],
  /**
   * The name of the named rule.
   */
  'name' : string,
  /**
   * The rule to use for the named rule.
   */
  'rule' : RequestPolicyRule,
  /**
   * The description of the named rule.
   */
  'description' : [] | [string],
}
/**
 * The init type for initializing the permissions when first creating the canister.
 */
export interface InitPermissionInput {
  /**
   * The resource that the permission is for.
   */
  'resource' : Resource,
  /**
   * The allow rules for who can access the resource.
   */
  'allow' : Allow,
}
/**
 * The init type for adding a request approval policy when initializing the canister for the first time.
 */
export interface InitRequestPolicyInput {
  /**
   * The id of the request policy, if not provided a new UUID will be generated.
   */
  'id' : [] | [UUID],
  /**
   * The rule to use for the request approval evaluation (e.g. "quorum").
   */
  'rule' : RequestPolicyRule,
  /**
   * The request specifier that identifies what operation this policy is for (e.g. "transfer").
   */
  'specifier' : RequestSpecifier,
}
/**
 * The input type for creating a user group when initializing the canister for the first time.
 */
export interface InitUserGroupInput {
  /**
   * The id of the user group, if not provided a new UUID will be generated.
   */
  'id' : [] | [UUID],
  /**
   * The name of the user group, must be unique.
   */
  'name' : string,
}
/**
 * The users to create when initializing the canister for the first time.
 */
export interface InitUserInput {
  /**
   * The id of the user, if not provided a new UUID will be generated.
   */
  'id' : [] | [UUID],
  /**
   * The status of the user (e.g. `Active`).
   */
  'status' : UserStatus,
  /**
   * The user groups to associate with the user (optional).
   * If not provided it defaults to the [`Admin`,`Operator`] groups if default user groups are created,
   * i.e., when the field `initial_config` in `SystemInit` has the form of `WithAllDefaults` or `WithDefaultPolicies`.
   */
  'groups' : [] | [Array<UUID>],
  /**
   * The name of the user.
   */
  'name' : string,
  /**
   * The identities of the user.
   */
  'identities' : Array<UserIdentityInput>,
}
/**
 * The initial configuration for the station.
 * 
 * Unless the `Complete` variant is used, the station will be initialized with default user
 * groups, named rules (aka. approval rules), request policies, permissions, and assets.
 * 
 * The default user groups for the station will be:
 * - `Admin` with the UUID "00000000-0000-4000-8000-000000000000"
 * - `Operator` with the UUID "00000000-0000-4000-8000-000000000001"
 * 
 * The default named rules for the station will be:
 * - `Admin approval` with a specified admin quorum
 * - `Operator approval` with a specified operator and admin quorum
 * 
 */
export type InitialConfig = {
    /**
     * Initialize the station with default user groups, named rules, policies, permissions.
     */
    'WithDefaultPolicies' : {
      /**
       * The initial assets to create.
       */
      'assets' : Array<InitAssetInput>,
      /**
       * The initial admin quorum in the admin level approval rule.
       */
      'admin_quorum' : number,
      /**
       * The initial accounts to create.
       */
      'accounts' : Array<InitAccountInput>,
      /**
       * The initial users to create.
       */
      'users' : Array<InitUserInput>,
      /**
       * The initial operator quorum in the operator level approval rule.
       */
      'operator_quorum' : number,
    }
  } |
  {
    /**
     * Initialize the station with default user groups, named rules, policies, permissions, and assets.
     * This does not create an initial account.
     */
    'WithAllDefaults' : {
      /**
       * The initial admin quorum in the admin level approval rule.
       */
      'admin_quorum' : number,
      /**
       * The initial users to create.
       */
      'users' : Array<InitUserInput>,
      /**
       * The initial operator quorum in the operator level approval rule.
       */
      'operator_quorum' : number,
    }
  } |
  {
    /**
     * Initialize the station with all custom entries.
     */
    'Complete' : {
      /**
       * The initial permissions to create.
       */
      'permissions' : Array<InitPermissionInput>,
      /**
       * The initial assets to create.
       */
      'assets' : Array<InitAssetInput>,
      /**
       * The initial request policies to create.
       */
      'request_policies' : Array<InitRequestPolicyInput>,
      /**
       * The initial user groups to create.
       */
      'user_groups' : Array<InitUserGroupInput>,
      /**
       * The initial accounts to create.
       */
      'accounts' : Array<InitAccountWithPermissionsInput>,
      /**
       * The initial disaster recovery committee to create.
       */
      'disaster_recovery_committee' : [] | [DisasterRecoveryCommittee],
      /**
       * The initial users to create.
       */
      'users' : Array<InitUserInput>,
      /**
       * The initial named rules to create.
       */
      'named_rules' : Array<InitNamedRuleInput>,
    }
  };
export interface ListAccountTransfersInput {
  /**
   * The account id to retrieve.
   */
  'account_id' : UUID,
  /**
   * The transfer status in text format (e.g. "pending", "approved", etc.).
   */
  'status' : [] | [TransferStatusType],
  /**
   * Until which date to retrieve the transfers.
   */
  'to_dt' : [] | [TimestampRFC3339],
  /**
   * From which date to retrieve the transfers.
   */
  'from_dt' : [] | [TimestampRFC3339],
}
export type ListAccountTransfersResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The list of transfers.
       */
      'transfers' : Array<TransferListItem>,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * A record type that can be used search for accounts.
 */
export interface ListAccountsInput {
  /**
   * The pagination parameters.
   */
  'paginate' : [] | [PaginationInput],
  /**
   * The name of the account to search for.
   */
  'search_term' : [] | [string],
}
/**
 * Result type for listing all accounts.
 */
export type ListAccountsResult = {
    'Ok' : {
      /**
       * The total number of users.
       */
      'total' : bigint,
      /**
       * The privileges of the caller.
       */
      'privileges' : Array<AccountCallerPrivileges>,
      /**
       * The list of accounts.
       */
      'accounts' : Array<Account>,
      /**
       * The offset to use for the next page.
       */
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
/**
 * Input type for listing address book entries for a given blockchain standard.
 */
export interface ListAddressBookEntriesInput {
  /**
   * The address boo entry ids to retrieve.
   */
  'ids' : [] | [Array<UUID>],
  /**
   * The address formats to search for.
   */
  'address_formats' : [] | [Array<string>],
  /**
   * The labels to search for, if provided only address book entries with the given labels will be returned.
   */
  'labels' : [] | [Array<string>],
  /**
   * The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
   */
  'blockchain' : [] | [string],
  /**
   * The address to search for.
   */
  'addresses' : [] | [Array<string>],
  /**
   * The pagination parameters.
   */
  'paginate' : [] | [PaginationInput],
  /**
   * The term to use for filtering the address book entries.
   */
  'search_term' : [] | [string],
}
/**
 * Result type for listing address book entries for a given blockchain standard.
 */
export type ListAddressBookEntriesResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The total number of address book entries for the given blockchain standard.
       */
      'total' : bigint,
      /**
       * The privileges of the caller for the address book entries.
       */
      'privileges' : Array<AddressBookEntryCallerPrivileges>,
      /**
       * The list of retrieved address book entries.
       */
      'address_book_entries' : Array<AddressBookEntry>,
      /**
       * The offset to use for the next page.
       */
      'next_offset' : [] | [bigint],
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * The input type for listing assets.
 */
export interface ListAssetsInput {
  /**
   * The pagination parameters.
   */
  'paginate' : [] | [PaginationInput],
}
/**
 * The result type for listing assets.
 */
export type ListAssetsResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The total number of assets.
       */
      'total' : bigint,
      /**
       * The caller privileges for the assets.
       */
      'privileges' : Array<AssetCallerPrivileges>,
      /**
       * The list of assets.
       */
      'assets' : Array<Asset>,
      /**
       * The offset to use for the next page.
       */
      'next_offset' : [] | [bigint],
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Input type for listing external canisters with the given filters.
 */
export interface ListExternalCanistersInput {
  /**
   * The sort parameters.
   */
  'sort_by' : [] | [ListExternalCanistersSortInput],
  /**
   * The current state of the external canisters to use for filtering (e.g. `Active`, `Archived`).
   */
  'states' : [] | [Array<ExternalCanisterState>],
  /**
   * The principal id of the external canister to search for.
   */
  'canister_ids' : [] | [Array<Principal>],
  /**
   * The labels to use for filtering the external canisters.
   */
  'labels' : [] | [Array<string>],
  /**
   * The pagination parameters.
   */
  'paginate' : [] | [PaginationInput],
}
/**
 * Result type for listing external canisters.
 */
export type ListExternalCanistersResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The total number of external canisters.
       */
      'total' : bigint,
      /**
       * The caller privileges for the external canisters.
       */
      'privileges' : Array<ExternalCanisterCallerPrivileges>,
      /**
       * The list of external canisters.
       */
      'canisters' : Array<ExternalCanister>,
      /**
       * The offset to use for the next page.
       */
      'next_offset' : [] | [bigint],
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * The input type for sorting the results of listing external canisters.
 */
export type ListExternalCanistersSortInput = {
    /**
     * Sort by the name of the external canister.
     */
    'Name' : SortByDirection
  };
/**
 * Input type for listing named rules.
 */
export interface ListNamedRulesInput {
  /**
   * The pagination parameters.
   */
  'paginate' : [] | [PaginationInput],
}
/**
 * Result type for listing named rules.
 */
export type ListNamedRulesResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The total number of named rules.
       */
      'total' : bigint,
      /**
       * The privileges of the caller.
       */
      'privileges' : Array<NamedRuleCallerPrivileges>,
      /**
       * The list of named rules.
       */
      'named_rules' : Array<NamedRule>,
      /**
       * The offset to use for the next page.
       */
      'next_offset' : [] | [bigint],
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * The input type for getting the list of notifications associated with the caller.
 */
export interface ListNotificationsInput {
  /**
   * Show only notifications with the given status.
   */
  'status' : [] | [NotificationStatus],
  /**
   * Until which created time to retrieve the notifications.
   */
  'to_dt' : [] | [TimestampRFC3339],
  /**
   * From which created time to retrieve the notifications.
   */
  'from_dt' : [] | [TimestampRFC3339],
  /**
   * The type of the notification (e.g. "system-message").
   */
  'notification_type' : [] | [NotificationTypeInput],
}
/**
 * The result type for getting the list of notifications.
 */
export type ListNotificationsResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The list of notifications ordered by creation time (newest first).
       */
      'notifications' : Array<Notification>,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Input type for listing permissions with the given pagination parameters.
 */
export interface ListPermissionsInput {
  /**
   * The resources to retrieve the permissions for.
   */
  'resources' : [] | [Array<Resource>],
  /**
   * The pagination parameters.
   */
  'paginate' : [] | [PaginationInput],
}
/**
 * Result type for listing permissions.
 */
export type ListPermissionsResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The list of permissions.
       */
      'permissions' : Array<Permission>,
      /**
       * The total number of permissions.
       */
      'total' : bigint,
      /**
       * The caller privileges for the permissions.
       */
      'privileges' : Array<PermissionCallerPrivileges>,
      /**
       * The user groups that are associated with returned permissions.
       */
      'user_groups' : Array<UserGroup>,
      /**
       * The users that are associated with returned permissions.
       */
      'users' : Array<BasicUser>,
      /**
       * The offset to use for the next page.
       */
      'next_offset' : [] | [bigint],
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * Input type for listing request policies with the given pagination parameters.
 */
export type ListRequestPoliciesInput = PaginationInput;
/**
 * Result type for listing request policies.
 */
export type ListRequestPoliciesResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The total number of request policies.
       */
      'total' : bigint,
      /**
       * The caller privileges for the request policies.
       */
      'privileges' : Array<RequestPolicyCallerPrivileges>,
      /**
       * The offset to use for the next page.
       */
      'next_offset' : [] | [bigint],
      /**
       * The list of request policies.
       */
      'policies' : Array<RequestPolicy>,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * The input type for getting the list of requests based on the given filters.
 */
export interface ListRequestsInput {
  /**
   * The sorting parameters.
   */
  'sort_by' : [] | [ListRequestsSortBy],
  /**
   * Return only requests with one of these deduplication keys.
   */
  'deduplication_keys' : [] | [Array<string>],
  /**
   * Return the full evaluation results for the requests.
   */
  'with_evaluation_results' : boolean,
  /**
   * From which expiration time to retrieve the requests.
   */
  'expiration_from_dt' : [] | [TimestampRFC3339],
  /**
   * The tags to search. Return only requests which have at least one matching tag.
   */
  'tags' : [] | [Array<string>],
  /**
   * Until which created time to retrieve the requests.
   */
  'created_to_dt' : [] | [TimestampRFC3339],
  /**
   * Show only requests with the given status.
   */
  'statuses' : [] | [Array<RequestStatusCode>],
  /**
   * Show only requests that the specified users have submitted an approval decision for.
   */
  'approver_ids' : [] | [Array<UUID>],
  /**
   * Until which expiration time to retrieve the requests.
   */
  'expiration_to_dt' : [] | [TimestampRFC3339],
  /**
   * The pagination parameters.
   */
  'paginate' : [] | [PaginationInput],
  /**
   * Show only requests made by the given users.
   */
  'requester_ids' : [] | [Array<UUID>],
  /**
   * The type of the request (e.g. "transfer").
   */
  'operation_types' : [] | [Array<ListRequestsOperationType>],
  /**
   * Return only requests the the user can submit an approval decision for.
   */
  'only_approvable' : boolean,
  /**
   * From which created time to retrieve the requests.
   */
  'created_from_dt' : [] | [TimestampRFC3339],
}
export type ListRequestsOperationType = {
    /**
     * An operation for removing an asset.
     */
    'RemoveAsset' : null
  } |
  {
    /**
     * An operation for adding a new user group.
     */
    'AddUserGroup' : null
  } |
  {
    /**
     * An operation for editing an permission.
     */
    'EditPermission' : null
  } |
  {
    /**
     * An operation for snapshotting an external canister.
     */
    'SnapshotExternalCanister' : [] | [Principal]
  } |
  {
    /**
     * An operation for pruning an external canister.
     */
    'PruneExternalCanister' : [] | [Principal]
  } |
  {
    /**
     * An operation for editing a named rule.
     */
    'EditNamedRule' : null
  } |
  {
    /**
     * An operation for configuring an external canister.
     */
    'ConfigureExternalCanister' : [] | [Principal]
  } |
  {
    /**
     * An operation for changing a external canister with an optionally specified canister ID.
     */
    'ChangeExternalCanister' : [] | [Principal]
  } |
  {
    /**
     * An operation for monitoring cycles of an external canister.
     */
    'MonitorExternalCanister' : [] | [Principal]
  } |
  {
    /**
     * An operation for adding a new user.
     */
    'AddUser' : null
  } |
  {
    /**
     * An operation for editing an asset.
     */
    'EditAsset' : null
  } |
  {
    /**
     * An operation for editing an existing user group.
     */
    'EditUserGroup' : null
  } |
  {
    /**
     * An operation for setting disaster recovery config.
     */
    'SetDisasterRecovery' : null
  } |
  {
    /**
     * An operation for editing a request policy.
     */
    'EditRequestPolicy' : null
  } |
  {
    /**
     * An operation for removing a request policy.
     */
    'RemoveRequestPolicy' : null
  } |
  {
    /**
     * An operation for adding an asset.
     */
    'AddAsset' : null
  } |
  {
    /**
     * An operation for performing a system upgrade on the station or upgrader.
     */
    'SystemUpgrade' : null
  } |
  {
    /**
     * An operation for removing an address book entry.
     */
    'RemoveAddressBookEntry' : null
  } |
  {
    /**
     * An operation for restoring the station or upgrader from a snapshot.
     */
    'SystemRestore' : null
  } |
  {
    /**
     * An operation for creating a external canister.
     */
    'CreateExternalCanister' : null
  } |
  {
    /**
     * An operation for updating an address book entry.
     */
    'EditAddressBookEntry' : null
  } |
  {
    /**
     * An operation for sending cycles to an external canister.
     */
    'FundExternalCanister' : [] | [Principal]
  } |
  {
    /**
     * An operation for editing an existing user.
     */
    'EditUser' : null
  } |
  {
    /**
     * An operation for managing system info.
     */
    'ManageSystemInfo' : null
  } |
  {
    /**
     * A new transfer of funds from a given account.
     */
    'Transfer' : [] | [UUID]
  } |
  {
    /**
     * An operation for updating information of an account.
     */
    'EditAccount' : null
  } |
  {
    /**
     * An operation for creating a new address book entry.
     */
    'AddAddressBookEntry' : null
  } |
  {
    /**
     * An operation for adding a request policy.
     */
    'AddRequestPolicy' : null
  } |
  {
    /**
     * An operation for removing a named rule.
     */
    'RemoveNamedRule' : null
  } |
  {
    /**
     * An operation for removing an existing user group.
     */
    'RemoveUserGroup' : null
  } |
  {
    /**
     * An operation for calling an external canister with an optionally specified canister ID.
     */
    'CallExternalCanister' : [] | [Principal]
  } |
  {
    /**
     * An operation for adding a named rule.
     */
    'AddNamedRule' : null
  } |
  {
    /**
     * An operation for restoring an external canister from a snapshot.
     */
    'RestoreExternalCanister' : [] | [Principal]
  } |
  {
    /**
     * An operation for creating a new account.
     */
    'AddAccount' : null
  };
/**
 * The result type for getting the list of requests.
 */
export type ListRequestsResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The total number of requests.
       */
      'total' : bigint,
      /**
       * The privileges of the caller.
       */
      'privileges' : Array<RequestCallerPrivileges>,
      /**
       * The list of requests.
       */
      'requests' : Array<Request>,
      /**
       * The next offset to use for pagination.
       */
      'next_offset' : [] | [bigint],
      /**
       * The additional info about the requests.
       */
      'additional_info' : Array<RequestAdditionalInfo>,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
/**
 * The input type that can be used to sort the list of requests by a given field.
 */
export type ListRequestsSortBy = {
    /**
     * Sort by the request expiration time.
     */
    'ExpirationDt' : SortByDirection
  } |
  {
    /**
     * Sort by the request last modification time.
     */
    'LastModificationDt' : SortByDirection
  } |
  {
    /**
     * Sort by the request creation time.
     */
    'CreatedAt' : SortByDirection
  };
export interface ListUserGroupsInput {
  /**
   * The pagination parameters.
   */
  'paginate' : [] | [PaginationInput],
  /**
   * The term to use for filtering the user groups.
   */
  'search_term' : [] | [string],
}
/**
 * Result type for listing all user groups.
 */
export type ListUserGroupsResult = {
    'Ok' : {
      /**
       * The total number of user groups.
       */
      'total' : bigint,
      /**
       * The caller privileges for the user groups.
       */
      'privileges' : Array<UserGroupCallerPrivileges>,
      /**
       * The list of groups.
       */
      'user_groups' : Array<UserGroup>,
      /**
       * The offset to use for the next page.
       */
      'next_offset' : [] | [bigint],
    }
  } |
  { 'Err' : Error };
/**
 * Input type for listing users of the station.
 */
export interface ListUsersInput {
  /**
   * The groups to use for filtering the users.
   */
  'groups' : [] | [Array<UUID>],
  /**
   * The statuses to use for filtering the users.
   */
  'statuses' : [] | [Array<UserStatus>],
  /**
   * The pagination parameters.
   */
  'paginate' : [] | [PaginationInput],
  /**
   * The search term to use for filtering the users.
   */
  'search_term' : [] | [string],
}
/**
 * Result type for listing users of the station.
 */
export type ListUsersResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The total number of users.
       */
      'total' : bigint,
      /**
       * The privileges of the caller.
       */
      'privileges' : Array<UserCallerPrivileges>,
      /**
       * The list of users.
       */
      'users' : Array<User>,
      /**
       * The offset to use for the next page.
       */
      'next_offset' : [] | [bigint],
    }
  } |
  {
    /**
     * The error that occurred (e.g. the user does not have the necessary permissions).
     */
    'Err' : Error
  };
export type LogVisibility = { 'controllers' : null } |
  { 'public' : null } |
  { 'allowed_viewers' : Array<Principal> };
/**
 * An operation for managing the system information.
 */
export interface ManageSystemInfoOperation {
  /**
   * The input to the request to manage the system information.
   */
  'input' : ManageSystemInfoOperationInput,
}
/**
 * Input type for managing the system information.
 */
export interface ManageSystemInfoOperationInput {
  /**
   * The name of the station.
   */
  'name' : [] | [string],
  /**
   * The strategy to use to for the station to top itself up with cycles.
   */
  'cycle_obtain_strategy' : [] | [CycleObtainStrategyInput],
  /**
   * The maximum number of upgrader backup snapshots to keep.
   */
  'max_upgrader_backup_snapshots' : [] | [bigint],
  /**
   * The maximum number of station backup snapshots to keep.
   */
  'max_station_backup_snapshots' : [] | [bigint],
}
export type MarkNotificationReadResult = { 'Ok' : null } |
  { 'Err' : Error };
export interface MarkNotificationsReadInput {
  /**
   * The notifications to mark as read.
   */
  'notification_ids' : Array<UUID>,
  /**
   * If true, all notifications will be marked as read.
   */
  'read' : boolean,
}
export type MeResult = {
    'Ok' : {
      /**
       * The user that is associated with the caller.
       */
      'me' : User,
      /**
       * The list of privileges associated with the user.
       */
      'privileges' : Array<UserPrivilege>,
    }
  } |
  { 'Err' : Error };
/**
 * The request operation for monitoring an external canister from the station.
 */
export type MonitorExternalCanisterOperation = MonitorExternalCanisterOperationInput;
/**
 * The input type for monitoring an external canister in the station.
 */
export interface MonitorExternalCanisterOperationInput {
  /**
   * The kind of funding operation to perform.
   */
  'kind' : MonitorExternalCanisterOperationKind,
  /**
   * The external canister to monitor.
   */
  'canister_id' : Principal,
}
/**
 * The operation kind for monitoring an external canister in the station.
 */
export type MonitorExternalCanisterOperationKind = {
    'Start' : MonitorExternalCanisterStartInput
  } |
  { 'Stop' : null };
/**
 * The input type for specifying the strategy for monitoring an external canister.
 */
export interface MonitorExternalCanisterStartInput {
  /**
   * The strategy for obtaining cycles for the funding operation.
   */
  'cycle_obtain_strategy' : [] | [CycleObtainStrategyInput],
  /**
   * The strategy for funding the canister.
   */
  'funding_strategy' : MonitorExternalCanisterStrategyInput,
}
/**
 * The input type for specifying the strategy for monitoring an external canister.
 */
export type MonitorExternalCanisterStrategyInput = {
    /**
     * Fund the canister at a fixed interval with the specified amount of cycles.
     */
    'Always' : bigint
  } |
  {
    /**
     * Fund the canister when the balance is below the threshold.
     */
    'BelowThreshold' : MonitoringExternalCanisterCyclesThresholdInput
  } |
  {
    /**
     * Fund the canister based on the estimated run time in seconds.
     */
    'BelowEstimatedRuntime' : MonitoringExternalCanisterEstimatedRuntimeInput
  };
export interface MonitoringExternalCanisterCyclesThresholdInput {
  /**
   * / The cycles to fund the canister with when the threshold is triggered.
   */
  'fund_cycles' : bigint,
  /**
   * / The min cycles threshold to trigger the funding operation.
   */
  'min_cycles' : bigint,
}
export interface MonitoringExternalCanisterEstimatedRuntimeInput {
  /**
   * / The runtime seconds to add to the estimated runtime.
   */
  'fund_runtime_secs' : bigint,
  /**
   * / The fallback min cycles to trigger the funding operation when the estimated runtime is not available,
   * / or the cycles balance is below the threshold.
   */
  'fallback_min_cycles' : bigint,
  /**
   * / The estimated min runtime in seconds to trigger the funding operation.
   */
  'min_runtime_secs' : bigint,
  /**
   * / The fallback cycles to fund the canister with when the estimated runtime is not available,
   * / or the cycles balance is below the threshold.
   */
  'fallback_fund_cycles' : bigint,
  /**
   * / The maximum cycles to fund the canister with, only used when the estimated runtime is available.
   */
  'max_runtime_cycles_fund' : bigint,
}
/**
 * The named rule type.
 * 
 * A named rule is a reusable configuration that can be applied to many approval policies.
 */
export interface NamedRule {
  /**
   * The rule id.
   */
  'id' : UUID,
  /**
   * The rule name.
   */
  'name' : string,
  /**
   * The rule value.
   */
  'rule' : RequestPolicyRule,
  /**
   * The rule description.
   */
  'description' : [] | [string],
}
/**
 * A record type that can be used to represent the caller privileges for a given named rule.
 */
export interface NamedRuleCallerPrivileges {
  /**
   * The named rule id.
   */
  'id' : UUID,
  /**
   * Whether or not the caller can delete the resource.
   */
  'can_delete' : boolean,
  /**
   * Whether or not the caller can edit the resource.
   */
  'can_edit' : boolean,
}
/**
 * The blockchain network to used in a transaction.
 */
export interface Network {
  /**
   * The network id, represented by the blockchain symbol and network name (e.g. "icp:mainnet").
   */
  'id' : NetworkId,
  /**
   * The name of the network (e.g. "Mainnet").
   */
  'name' : string,
}
/**
 * The network id, represented by the blockchain symbol and network name (e.g. "icp:mainnet").
 */
export type NetworkId = string;
/**
 * A record type that can be used to represent a notification.
 */
export interface Notification {
  /**
   * The notification id which is a UUID (e.g. "d0cf5b3f-7017-4cb8-9dcf-52619c42a7b0").
   */
  'id' : UUID,
  /**
   * The notification status.
   */
  'status' : NotificationStatus,
  /**
   * The notification title in a single locale.
   */
  'title' : string,
  /**
   * The time at which the notification was created.
   */
  'created_at' : TimestampRFC3339,
  /**
   * The type of the notification.
   */
  'notification_type' : NotificationType,
  /**
   * The notification message in a single locale.
   */
  'message' : [] | [string],
  /**
   * The user that the notification is for.
   */
  'target_user_id' : UUID,
}
/**
 * The actions that are available for notifications.
 */
export type NotificationResourceAction = { 'List' : null } |
  { 'Update' : ResourceId };
/**
 * Defines the various states that a notification can be in.
 */
export type NotificationStatus = {
    /**
     * The notification has been read by the user.
     */
    'Read' : null
  } |
  {
    /**
     * The notification has been sent.
     */
    'Sent' : null
  };
/**
 * Represents the different types of notifications within the system.
 */
export type NotificationType = {
    /**
     * Notification for the creation of a new request.
     * This should be used to alert users when a new request that requires their attention has been created.
     */
    'RequestCreated' : {
      /**
       * Account id is available for relevant request types.
       */
      'account_id' : [] | [UUID],
      /**
       * The request id that was created.
       */
      'request_id' : UUID,
      /**
       * The type of the request (e.g. "transfer").
       */
      'operation_type' : RequestOperationType,
      /**
       * User id is available for relevant request types.
       */
      'user_id' : [] | [UUID],
    }
  } |
  {
    /**
     * Notification for the rejection of a request.
     * This should be used to alert the requester when a request has been rejected.
     */
    'RequestRejected' : {
      /**
       * The request id that was created.
       */
      'request_id' : UUID,
      /**
       * List of reasons why the request was rejected.
       */
      'reasons' : [] | [Array<EvaluationSummaryReason>],
      /**
       * The type of the request (e.g. "transfer").
       */
      'operation_type' : RequestOperationType,
    }
  } |
  {
    /**
     * Notification for system-wide messages.
     * This can be used for announcements, scheduled maintenance reminders, or other important system messages.
     */
    'SystemMessage' : null
  } |
  {
    /**
     * Notification for the failure of a request.
     * This should be used to alert the requester when a request has failed to be executed.
     */
    'RequestFailed' : {
      /**
       * The request id that was created.
       */
      'request_id' : UUID,
      /**
       * The type of the request (e.g. "transfer").
       */
      'operation_type' : RequestOperationType,
      /**
       * Details about the failure.
       */
      'reason' : [] | [string],
    }
  };
export type NotificationTypeInput = { 'RequestCreated' : null } |
  { 'SystemMessage' : null };
export interface NotifyFailedStationUpgradeInput {
  /**
   * the failure reason
   */
  'reason' : string,
}
export type NotifyFailedStationUpgradeResult = { 'Ok' : null } |
  { 'Err' : Error };
export interface PaginationInput {
  /**
   * The offset to use for pagination.
   */
  'offset' : [] | [bigint],
  /**
   * The maximum number of items to retrieve.
   * 
   * If not set, the default limit will be used.
   */
  'limit' : [] | [number],
}
/**
 * The permission, used to specify the rules for users when interacting with resources.
 */
export interface Permission {
  /**
   * The resource that the permission is for.
   */
  'resource' : Resource,
  /**
   * The allowed users and user groups for the resource.
   */
  'allow' : Allow,
}
/**
 * A record type that can be used to represent the caller privileges for a given permission.
 */
export interface PermissionCallerPrivileges {
  /**
   * The resource that the caller has privileges for.
   */
  'resource' : Resource,
  /**
   * Whether or not the caller can edit the resource.
   */
  'can_edit' : boolean,
}
/**
 * The actions that are available for permissions.
 */
export type PermissionResourceAction = { 'Read' : null } |
  { 'Update' : null };
export interface PruneExternalCanisterOperation {
  'input' : PruneExternalCanisterOperationInput,
}
export interface PruneExternalCanisterOperationInput {
  /**
   * The canister to prune.
   */
  'canister_id' : Principal,
  /**
   * The resource to prune.
   */
  'prune' : { 'snapshot' : string } |
    { 'state' : null } |
    { 'chunk_store' : null },
}
/**
 * A record type that can be used to represent the minimum quorum of users that are required to approve a rule.
 */
export interface Quorum {
  /**
   * The minimum number of user approvals required for the rule to be approved.
   */
  'min_approved' : number,
  /**
   * The users that can approve the request.
   */
  'approvers' : UserSpecifier,
}
/**
 * A record type that can be used to represent a percentage of users that are required to approve a rule.
 */
export interface QuorumPercentage {
  /**
   * The required user approvals for the rule to be approved.
   */
  'min_approved' : number,
  /**
   * The users that are required to approve the request.
   */
  'approvers' : UserSpecifier,
}
export interface RemoveAddressBookEntryOperation {
  /**
   * The input to the request to remove the address book entry.
   */
  'input' : RemoveAddressBookEntryOperationInput,
}
/**
 * Input type for removing an address book entry through a request.
 */
export interface RemoveAddressBookEntryOperationInput {
  /**
   * The id of the address book entry.
   */
  'address_book_entry_id' : UUID,
}
export interface RemoveAssetOperation {
  /**
   * The input to the request to remove an asset.
   */
  'input' : RemoveAssetOperationInput,
}
/**
 * The input type for removing an asset.
 */
export interface RemoveAssetOperationInput {
  /**
   * The asset id to remove.
   */
  'asset_id' : UUID,
}
/**
 * The operation type for removing a named rule.
 */
export interface RemoveNamedRuleOperation {
  /**
   * The input to the request to remove a named rule.
   */
  'input' : RemoveNamedRuleOperationInput,
}
/**
 * The input type for deleting a named rule.
 */
export interface RemoveNamedRuleOperationInput {
  /**
   * The named rule id to remove.
   */
  'named_rule_id' : UUID,
}
export interface RemoveRequestPolicyOperation {
  /**
   * The input to the request to remove a request policy.
   */
  'input' : RemoveRequestPolicyOperationInput,
}
export interface RemoveRequestPolicyOperationInput {
  /**
   * The request policy id that will be removed.
   */
  'policy_id' : UUID,
}
export interface RemoveUserGroupOperation {
  /**
   * The input to the request to remove the user group.
   */
  'input' : RemoveUserGroupOperationInput,
}
export interface RemoveUserGroupOperationInput {
  /**
   * The id of the group to remove.
   */
  'user_group_id' : UUID,
}
/**
 * A record type that can be used to represent a requested operation in the station.
 */
export interface Request {
  /**
   * The request id which is a UUID (e.g. "d0cf5b3f-7017-4cb8-9dcf-52619c42a7b0").
   */
  'id' : UUID,
  /**
   * The request approval status.
   */
  'status' : RequestStatus,
  /**
   * The request title.
   */
  'title' : string,
  /**
   * The time at which the request should be executed if approved.
   */
  'execution_plan' : RequestExecutionSchedule,
  /**
   * The time at which the request will expire.
   */
  'expiration_dt' : TimestampRFC3339,
  /**
   * The optional deduplication key used to ensure request uniqueness.
   */
  'deduplication_key' : [] | [string],
  /**
   * The tags that were provided during request creation.
   */
  'tags' : Array<string>,
  /**
   * The time at which the request was created.
   */
  'created_at' : TimestampRFC3339,
  /**
   * The user that created the request.
   */
  'requested_by' : UUID,
  /**
   * The request summary (e.g. "This request will transfer 100 ICP to the account 0x1234").
   */
  'summary' : [] | [string],
  /**
   * The operation that was requested.
   */
  'operation' : RequestOperation,
  /**
   * The list of user approvals on the request.
   */
  'approvals' : Array<RequestApproval>,
}
/**
 * A record type that can be used to represent additional information about a request.
 */
export interface RequestAdditionalInfo {
  /**
   * The request id.
   */
  'id' : UUID,
  /**
   * The evaluation result of all matching policies for the request.
   */
  'evaluation_result' : [] | [RequestEvaluationResult],
  /**
   * The requester name (e.g. "John Doe").
   */
  'requester_name' : string,
  /**
   * Display information for the approvers.
   */
  'approvers' : Array<DisplayUser>,
}
/**
 * A record type that is used to represent a user approval decision on a request.
 */
export interface RequestApproval {
  /**
   * The user has added to the request, once provided it cannot be changed.
   */
  'status' : RequestApprovalStatus,
  /**
   * The user that has recorded the approval decision.
   */
  'approver_id' : UUID,
  /**
   * Optional reason for the decision.
   */
  'status_reason' : [] | [string],
  /**
   * The time at which the decision was made.
   */
  'decided_at' : TimestampRFC3339,
}
/**
 * The status of a request.
 */
export type RequestApprovalStatus = { 'Approved' : null } |
  { 'Rejected' : null };
/**
 * A record type that can be used to represent the caller privileges for a given request.
 */
export interface RequestCallerPrivileges {
  /**
   * The request id.
   */
  'id' : UUID,
  /**
   * Whether or not the caller can submit an approval decision.
   */
  'can_approve' : boolean,
}
/**
 * A record type representing the full evaluation result of all matching policies for a request.
 */
export interface RequestEvaluationResult {
  /**
   * The request id that was evaluated.
   */
  'request_id' : UUID,
  /**
   * The final evaluation status of the request.
   */
  'status' : EvaluationStatus,
  /**
   * The reasons why the request was approved or rejected.
   */
  'result_reasons' : [] | [Array<EvaluationSummaryReason>],
  /**
   * The evaluation results of all matching policies.
   */
  'policy_results' : Array<RequestPolicyRuleResult>,
}
/**
 * The schedule for executing a transaction of a given transfer.
 */
export type RequestExecutionSchedule = {
    /**
     * The transaction will be executed immediately.
     */
    'Immediate' : null
  } |
  {
    /**
     * The transaction will be executed at a given time.
     */
    'Scheduled' : {
      /**
       * The time at which the transaction will be executed,
       * it must be in the future.
       */
      'execution_time' : TimestampRFC3339,
    }
  };
export type RequestOperation = {
    /**
     * An operation for removing an existing asset.
     */
    'RemoveAsset' : RemoveAssetOperation
  } |
  {
    /**
     * An operation for adding a new user group.
     */
    'AddUserGroup' : AddUserGroupOperation
  } |
  {
    /**
     * An operation for editing an permission.
     */
    'EditPermission' : EditPermissionOperation
  } |
  {
    /**
     * An operation for snapshotting an external canister.
     */
    'SnapshotExternalCanister' : SnapshotExternalCanisterOperation
  } |
  {
    /**
     * An operation for pruning an external canister.
     */
    'PruneExternalCanister' : PruneExternalCanisterOperation
  } |
  {
    /**
     * An operation for editing an existing named rule.
     */
    'EditNamedRule' : EditNamedRuleOperation
  } |
  {
    /**
     * An operation for configuring an external canister.
     */
    'ConfigureExternalCanister' : ConfigureExternalCanisterOperation
  } |
  {
    /**
     * An operation for changing a external canister.
     */
    'ChangeExternalCanister' : ChangeExternalCanisterOperation
  } |
  {
    /**
     * An operation for monitoring an external canister.
     */
    'MonitorExternalCanister' : MonitorExternalCanisterOperation
  } |
  {
    /**
     * An operation for adding a new user.
     */
    'AddUser' : AddUserOperation
  } |
  {
    /**
     * An operation for editing an existing asset.
     */
    'EditAsset' : EditAssetOperation
  } |
  {
    /**
     * An operation for editing an existing user group.
     */
    'EditUserGroup' : EditUserGroupOperation
  } |
  {
    /**
     * An operation for setting disaster recovery.
     */
    'SetDisasterRecovery' : SetDisasterRecoveryOperation
  } |
  {
    /**
     * An operation for editing a request policy.
     */
    'EditRequestPolicy' : EditRequestPolicyOperation
  } |
  {
    /**
     * An operation for removing a request policy.
     */
    'RemoveRequestPolicy' : RemoveRequestPolicyOperation
  } |
  {
    /**
     * An operation for adding a new asset.
     */
    'AddAsset' : AddAssetOperation
  } |
  {
    /**
     * An operation for performing a system upgrade on the station or upgrader.
     */
    'SystemUpgrade' : SystemUpgradeOperation
  } |
  {
    /**
     * An operation for removing an existing address book entry.
     */
    'RemoveAddressBookEntry' : RemoveAddressBookEntryOperation
  } |
  {
    /**
     * An operation for restoring the station or upgrader from a snapshot.
     */
    'SystemRestore' : SystemRestoreOperation
  } |
  {
    /**
     * An operation for creating a external canister.
     */
    'CreateExternalCanister' : CreateExternalCanisterOperation
  } |
  {
    /**
     * An operation for updating an existing address book entry.
     */
    'EditAddressBookEntry' : EditAddressBookEntryOperation
  } |
  {
    /**
     * An operation for funding an external canister.
     */
    'FundExternalCanister' : FundExternalCanisterOperation
  } |
  {
    /**
     * An operation for editing an existing user.
     */
    'EditUser' : EditUserOperation
  } |
  {
    /**
     * An operation for managing system info.
     */
    'ManageSystemInfo' : ManageSystemInfoOperation
  } |
  {
    /**
     * A new transfer of funds from a given account.
     */
    'Transfer' : TransferOperation
  } |
  {
    /**
     * An operation for updating information of an account.
     */
    'EditAccount' : EditAccountOperation
  } |
  {
    /**
     * An operation for creating a new address book entry.
     */
    'AddAddressBookEntry' : AddAddressBookEntryOperation
  } |
  {
    /**
     * An operation for adding a request policy.
     */
    'AddRequestPolicy' : AddRequestPolicyOperation
  } |
  {
    /**
     * An operation for removing an existing named rule.
     */
    'RemoveNamedRule' : RemoveNamedRuleOperation
  } |
  {
    /**
     * An operation for removing an existing user group.
     */
    'RemoveUserGroup' : RemoveUserGroupOperation
  } |
  {
    /**
     * An operation for calling an external canister.
     */
    'CallExternalCanister' : CallExternalCanisterOperation
  } |
  {
    /**
     * An operation for adding a new named rule.
     */
    'AddNamedRule' : AddNamedRuleOperation
  } |
  {
    /**
     * An operation for restoring an external canister from a snapshot.
     */
    'RestoreExternalCanister' : RestoreExternalCanisterOperation
  } |
  {
    /**
     * An operation for creating a new account.
     */
    'AddAccount' : AddAccountOperation
  };
export type RequestOperationInput = {
    /**
     * An operation for removing an existing asset.
     */
    'RemoveAsset' : RemoveAssetOperationInput
  } |
  {
    /**
     * An operation for adding a new user group.
     */
    'AddUserGroup' : AddUserGroupOperationInput
  } |
  {
    /**
     * An operation for editing an permission.
     */
    'EditPermission' : EditPermissionOperationInput
  } |
  {
    /**
     * An operation for snapshotting an external canister.
     */
    'SnapshotExternalCanister' : SnapshotExternalCanisterOperationInput
  } |
  {
    /**
     * An operation for pruning an external canister.
     */
    'PruneExternalCanister' : PruneExternalCanisterOperationInput
  } |
  {
    /**
     * An operation for editing an existing named rule.
     */
    'EditNamedRule' : EditNamedRuleOperationInput
  } |
  {
    /**
     * An operation for configuring an external canister.
     */
    'ConfigureExternalCanister' : ConfigureExternalCanisterOperationInput
  } |
  {
    /**
     * An operation for changing a external canister.
     */
    'ChangeExternalCanister' : ChangeExternalCanisterOperationInput
  } |
  {
    /**
     * An operation for monitoring an external canister.
     */
    'MonitorExternalCanister' : MonitorExternalCanisterOperationInput
  } |
  {
    /**
     * An operation for adding a new user.
     */
    'AddUser' : AddUserOperationInput
  } |
  {
    /**
     * An operation for editing an existing asset.
     */
    'EditAsset' : EditAssetOperationInput
  } |
  {
    /**
     * An operation for editing an existing user group.
     */
    'EditUserGroup' : EditUserGroupOperationInput
  } |
  {
    /**
     * An operation for setting disaster recovery.
     */
    'SetDisasterRecovery' : SetDisasterRecoveryOperationInput
  } |
  {
    /**
     * An operation for editing a request policy.
     */
    'EditRequestPolicy' : EditRequestPolicyOperationInput
  } |
  {
    /**
     * An operation for removing a request policy.
     */
    'RemoveRequestPolicy' : RemoveRequestPolicyOperationInput
  } |
  {
    /**
     * An operation for adding a new asset.
     */
    'AddAsset' : AddAssetOperationInput
  } |
  {
    /**
     * An operation for performing a system upgrade on the station or upgrader.
     */
    'SystemUpgrade' : SystemUpgradeOperationInput
  } |
  {
    /**
     * An operation for removing an address book entry.
     */
    'RemoveAddressBookEntry' : RemoveAddressBookEntryOperationInput
  } |
  {
    /**
     * An operation for restoring the station or upgrader from a snapshot.
     */
    'SystemRestore' : SystemRestoreOperationInput
  } |
  {
    /**
     * An operation for creating a external canister.
     */
    'CreateExternalCanister' : CreateExternalCanisterOperationInput
  } |
  {
    /**
     * An operation for updating an address book entry.
     */
    'EditAddressBookEntry' : EditAddressBookEntryOperationInput
  } |
  {
    /**
     * An operation for funding an external canister.
     */
    'FundExternalCanister' : FundExternalCanisterOperationInput
  } |
  {
    /**
     * An operation for editing an existing user.
     */
    'EditUser' : EditUserOperationInput
  } |
  {
    /**
     * An operation for managing system info.
     */
    'ManageSystemInfo' : ManageSystemInfoOperationInput
  } |
  {
    /**
     * A new transfer of funds from a given account.
     */
    'Transfer' : TransferOperationInput
  } |
  {
    /**
     * An operation for updating information of an account.
     */
    'EditAccount' : EditAccountOperationInput
  } |
  {
    /**
     * An operation for creating a new address book entry.
     */
    'AddAddressBookEntry' : AddAddressBookEntryOperationInput
  } |
  {
    /**
     * An operation for adding a request policy.
     */
    'AddRequestPolicy' : AddRequestPolicyOperationInput
  } |
  {
    /**
     * An operation for removing an existing named rule.
     */
    'RemoveNamedRule' : RemoveNamedRuleOperationInput
  } |
  {
    /**
     * An operation for removing an existing user group.
     */
    'RemoveUserGroup' : RemoveUserGroupOperationInput
  } |
  {
    /**
     * An operation for calling an external canister.
     */
    'CallExternalCanister' : CallExternalCanisterOperationInput
  } |
  {
    /**
     * An operation for adding a new named rule.
     */
    'AddNamedRule' : AddNamedRuleOperationInput
  } |
  {
    /**
     * An operation for restoring an external canister from a snapshot.
     */
    'RestoreExternalCanister' : RestoreExternalCanisterOperationInput
  } |
  {
    /**
     * An operation for adding a new account.
     */
    'AddAccount' : AddAccountOperationInput
  };
export type RequestOperationType = {
    /**
     * An operation for removing an existing asset.
     */
    'RemoveAsset' : null
  } |
  {
    /**
     * An operation for adding a new user group.
     */
    'AddUserGroup' : null
  } |
  {
    /**
     * An operation for editing an permission.
     */
    'EditPermission' : null
  } |
  {
    /**
     * An operation for snapshotting an external canister.
     */
    'SnapshotExternalCanister' : null
  } |
  {
    /**
     * An operation for pruning an external canister.
     */
    'PruneExternalCanister' : null
  } |
  {
    /**
     * An operation for editing an existing named rule.
     */
    'EditNamedRule' : null
  } |
  {
    /**
     * An operation for creating a external canister.
     */
    'ConfigureExternalCanister' : null
  } |
  {
    /**
     * An operation for changing a external canister.
     */
    'ChangeExternalCanister' : null
  } |
  {
    /**
     * An operation for monitoring cycles of an external canister.
     */
    'MonitorExternalCanister' : null
  } |
  {
    /**
     * An operation for adding a new user.
     */
    'AddUser' : null
  } |
  {
    /**
     * An operation for editing an existing asset.
     */
    'EditAsset' : null
  } |
  {
    /**
     * An operation for editing an existing user group.
     */
    'EditUserGroup' : null
  } |
  {
    /**
     * An operation for setting disaster recovery for a canister.
     */
    'SetDisasterRecovery' : null
  } |
  {
    /**
     * An operation for editing a request policy.
     */
    'EditRequestPolicy' : null
  } |
  {
    /**
     * An operation for removing a request policy.
     */
    'RemoveRequestPolicy' : null
  } |
  {
    /**
     * An operation for adding a new asset.
     */
    'AddAsset' : null
  } |
  {
    /**
     * An operation for performing a system upgrade on the station or upgrader.
     */
    'SystemUpgrade' : null
  } |
  {
    /**
     * An operation for removing an address book entry.
     */
    'RemoveAddressBookEntry' : null
  } |
  {
    /**
     * An operation for restoring the station or upgrader from a snapshot.
     */
    'SystemRestore' : null
  } |
  {
    /**
     * An operation for creating a external canister.
     */
    'CreateExternalCanister' : null
  } |
  {
    /**
     * An operation for updating an address book entry.
     */
    'EditAddressBookEntry' : null
  } |
  {
    /**
     * An operation for sending cycles to an external canister.
     */
    'FundExternalCanister' : null
  } |
  {
    /**
     * An operation for editing an existing user.
     */
    'EditUser' : null
  } |
  {
    /**
     * And operation for managing system info.
     */
    'ManageSystemInfo' : null
  } |
  {
    /**
     * A new transfer of funds from a given account.
     */
    'Transfer' : null
  } |
  {
    /**
     * An operation for updating information of an account.
     */
    'EditAccount' : null
  } |
  {
    /**
     * An operation for creating a new address book entry.
     */
    'AddAddressBookEntry' : null
  } |
  {
    /**
     * An operation for adding a request policy.
     */
    'AddRequestPolicy' : null
  } |
  {
    /**
     * An operation for removing an existing named rule.
     */
    'RemoveNamedRule' : null
  } |
  {
    /**
     * An operation for removing an existing user group.
     */
    'RemoveUserGroup' : null
  } |
  {
    /**
     * An operation for calling an external canister.
     */
    'CallExternalCanister' : null
  } |
  {
    /**
     * An operation for adding a new named rule.
     */
    'AddNamedRule' : null
  } |
  {
    /**
     * An operation for restoring an external canister from a snapshot.
     */
    'RestoreExternalCanister' : null
  } |
  {
    /**
     * An operation for creating a new account.
     */
    'AddAccount' : null
  };
/**
 * Represents a request policy with the associated rule.
 */
export interface RequestPolicy {
  'id' : UUID,
  'rule' : RequestPolicyRule,
  'specifier' : RequestSpecifier,
}
/**
 * A record type that can be used to represent the caller privileges for a given request policy.
 */
export interface RequestPolicyCallerPrivileges {
  /**
   * The request policy id.
   */
  'id' : UUID,
  /**
   * Whether or not the caller can delete the resource.
   */
  'can_delete' : boolean,
  /**
   * Whether or not the caller can edit the resource.
   */
  'can_edit' : boolean,
}
/**
 * Defines the various types rules that can be used in a request evaluation.
 */
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
/**
 * A record type representing the full evaluation result of a request policy rule.
 */
export interface RequestPolicyRuleResult {
  /**
   * The final evaluation status of the rule.
   */
  'status' : EvaluationStatus,
  /**
   * The result of the evaluation of the rule and all its sub-rules.
   */
  'evaluated_rule' : EvaluatedRequestPolicyRule,
}
/**
 * The actions that are available for requests.
 */
export type RequestResourceAction = { 'List' : null } |
  { 'Read' : ResourceId };
/**
 * Defines the various types of requests that can be created.
 */
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
/**
 * The status of a request.
 */
export type RequestStatus = { 'Failed' : { 'reason' : [] | [string] } } |
  { 'Approved' : null } |
  { 'Rejected' : null } |
  { 'Scheduled' : { 'scheduled_at' : TimestampRFC3339 } } |
  { 'Cancelled' : { 'reason' : [] | [string] } } |
  { 'Processing' : { 'started_at' : TimestampRFC3339 } } |
  { 'Created' : null } |
  { 'Completed' : { 'completed_at' : TimestampRFC3339 } };
/**
 * The status code of a request.
 */
export type RequestStatusCode = { 'Failed' : null } |
  { 'Approved' : null } |
  { 'Rejected' : null } |
  { 'Scheduled' : null } |
  { 'Cancelled' : null } |
  { 'Processing' : null } |
  { 'Created' : null } |
  { 'Completed' : null };
/**
 * The Resource is used to specify what is being accessed.
 */
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
/**
 * The resource actions, used to specify the action that is performed on a resource.
 */
export type ResourceAction = { 'List' : null } |
  { 'Read' : ResourceId } |
  { 'Delete' : ResourceId } |
  { 'Create' : null } |
  { 'Update' : ResourceId };
/**
 * The record id of a resource, used to specify the resource that is being accessed.
 */
export type ResourceId = { 'Id' : UUID } |
  { 'Any' : null };
/**
 * The record ids of a resource, used to specify the resources that are being accessed.
 */
export type ResourceIds = { 'Any' : null } |
  { 'Ids' : Array<UUID> };
export type ResourceSpecifier = { 'Any' : null } |
  { 'Resource' : Resource };
export interface RestoreExternalCanisterOperation {
  'input' : RestoreExternalCanisterOperationInput,
}
export interface RestoreExternalCanisterOperationInput {
  /**
   * The canister to restore from a snapshot.
   */
  'canister_id' : Principal,
  /**
   * A snapshot to be restored.
   */
  'snapshot_id' : string,
}
export interface SetDisasterRecoveryOperation {
  /**
   * The disaster recovery committee.
   */
  'committee' : [] | [DisasterRecoveryCommittee],
}
export interface SetDisasterRecoveryOperationInput {
  /**
   * The disaster recovery committee.
   */
  'committee' : [] | [DisasterRecoveryCommittee],
}
/**
 * The hash string representation for sha256.
 */
export type Sha256Hash = string;
export interface SnapshotExternalCanisterOperation {
  'input' : SnapshotExternalCanisterOperationInput,
  /**
   * The snapshot id of the new snapshot.
   */
  'snapshot_id' : [] | [string],
}
export interface SnapshotExternalCanisterOperationInput {
  /**
   * Should a snapshot be taken if the external canister fails to stop.
   */
  'force' : boolean,
  /**
   * A snapshot to be replaced.
   */
  'replace_snapshot' : [] | [string],
  /**
   * The canister to snapshot.
   */
  'canister_id' : Principal,
}
/**
 * The direction to use for sorting.
 */
export type SortByDirection = {
    /**
     * Sort in ascending order.
     */
    'Asc' : null
  } |
  {
    /**
     * Sort in descending order.
     */
    'Desc' : null
  };
/**
 * Describes a standard suported by a blockchain.
 */
export interface StandardData {
  /**
   * Supported operations for the standard (e.g. `["transfer", "list_transfers", "balance"]`).
   */
  'supported_operations' : Array<string>,
  /**
   * Supported address formats of the standard.
   */
  'supported_address_formats' : Array<string>,
  /**
   * Required metadata fields for the standard (e.g. `["ledger_canister_id"]`).
   */
  'required_metadata_fields' : Array<string>,
  /**
   * The standard name.
   */
  'standard' : string,
}
/**
 * Input type for submitting an approval decision on a request.
 */
export interface SubmitRequestApprovalInput {
  /**
   * The request id to interact with.
   */
  'request_id' : UUID,
  /**
   * The decision to submit.
   */
  'decision' : RequestApprovalStatus,
  /**
   * The reason for the approval or rejection.
   */
  'reason' : [] | [string],
}
/**
 * Result type for submitting an approval decision on a request.
 */
export type SubmitRequestApprovalResult = {
    'Ok' : {
      /**
       * The privileges of the caller.
       */
      'privileges' : RequestCallerPrivileges,
      /**
       * The request that the decision was submitted for.
       */
      'request' : Request,
      /**
       * The additional info about the request.
       */
      'additional_info' : RequestAdditionalInfo,
    }
  } |
  { 'Err' : Error };
export interface SubnetFilter { 'subnet_type' : [] | [string] }
export type SubnetSelection = {
    /**
     * Choose a random subnet that fulfills the specified properties
     */
    'Filter' : SubnetFilter
  } |
  {
    /**
     * Choose a specific subnet
     */
    'Subnet' : { 'subnet' : Principal }
  };
/**
 * Describes a blockchain and its standards supported by the station.
 */
export interface SupportedBlockchain {
  /**
   * The blockchain name.
   */
  'blockchain' : string,
  /**
   * The supported standards for the blockchain.
   */
  'supported_standards' : Array<StandardData>,
}
/**
 * The system information.
 */
export interface SystemInfo {
  /**
   * The disaster recovery configuration.
   */
  'disaster_recovery' : [] | [DisasterRecovery],
  /**
   * Cycle balance of the canister.
   */
  'upgrader_cycles' : [] | [bigint],
  /**
   * The name of the station.
   */
  'name' : string,
  /**
   * The time at which the canister was last upgraded.
   */
  'last_upgrade_timestamp' : TimestampRFC3339,
  /**
   * Did the canister successfully fetched randomness from the management canister.
   */
  'raw_rand_successful' : boolean,
  /**
   * The station version.
   */
  'version' : string,
  /**
   * Cycle balance of the station.
   */
  'cycles' : bigint,
  /**
   * The upgrader principal id.
   */
  'upgrader_id' : Principal,
  /**
   * Strategy defining how the station canister tops up its own cycles.
   */
  'cycle_obtain_strategy' : CycleObtainStrategy,
  /**
   * The maximum number of upgrader backup snapshots to keep.
   */
  'max_upgrader_backup_snapshots' : bigint,
  /**
   * The maximum number of station backup snapshots to keep.
   */
  'max_station_backup_snapshots' : bigint,
}
/**
 * Result type for getting the canister system information.
 */
export type SystemInfoResult = {
    /**
     * The result data for a successful execution.
     */
    'Ok' : {
      /**
       * The system information.
       */
      'system' : SystemInfo,
    }
  } |
  {
    /**
     * The error that occurred (e.g. the caller does not have sufficient privileges).
     */
    'Err' : Error
  };
export interface SystemInit {
  /**
   * The name of the station.
   */
  'name' : string,
  /**
   * The initial configuration to apply.
   */
  'initial_config' : InitialConfig,
  /**
   * An additional controller of the station and upgrader canisters (optional).
   */
  'fallback_controller' : [] | [Principal],
  /**
   * The upgrader configuration.
   */
  'upgrader' : SystemUpgraderInput,
}
/**
 * The input type for the canister install method (e.g. init or upgrade).
 */
export type SystemInstall = {
    /**
     * The configuration to use when upgrading the canister.
     */
    'Upgrade' : SystemUpgrade
  } |
  {
    /**
     * The configuration to use when initializing the canister.
     */
    'Init' : SystemInit
  };
/**
 * The actions that are available for the system.
 */
export type SystemResourceAction = { 'Upgrade' : null } |
  { 'ManageSystemInfo' : null } |
  { 'SystemInfo' : null } |
  { 'Capabilities' : null };
export interface SystemRestoreOperation {
  'input' : SystemRestoreOperationInput,
}
export interface SystemRestoreOperationInput {
  /**
   * The target to restore from a snapshot.
   */
  'target' : SystemRestoreTarget,
  /**
   * A snapshot to be restored.
   */
  'snapshot_id' : string,
}
export type SystemRestoreTarget = { 'RestoreUpgrader' : null } |
  { 'RestoreStation' : null };
/**
 * The upgrade configuration for the canister.
 */
export interface SystemUpgrade {
  /**
   * The updated name of the station.
   */
  'name' : [] | [string],
}
export interface SystemUpgradeOperation {
  /**
   * Determines whether a backup snapshot should be taken (before the upgrade).
   * If so and the maximum number of backup snapshots is reached,
   * then the oldest backup snapshot is atomically replaced
   * by the new backup snapshot.
   */
  'take_backup_snapshot' : [] | [boolean],
  /**
   * The checksum of the wasm module.
   */
  'module_checksum' : Sha256Hash,
  /**
   * The target to change.
   */
  'target' : SystemUpgradeTarget,
  /**
   * The checksum of the arg blob.
   */
  'arg_checksum' : [] | [Sha256Hash],
}
export interface SystemUpgradeOperationInput {
  /**
   * The initial argument passed to the new wasm module.
   */
  'arg' : [] | [Uint8Array | number[]],
  /**
   * Additional wasm module chunks to append to the wasm module.
   */
  'module_extra_chunks' : [] | [WasmModuleExtraChunks],
  /**
   * Determines whether a backup snapshot should be taken (before the upgrade).
   * If so and the maximum number of backup snapshots is reached,
   * then the oldest backup snapshot is atomically replaced
   * by the new backup snapshot.
   */
  'take_backup_snapshot' : [] | [boolean],
  /**
   * The target to change.
   */
  'target' : SystemUpgradeTarget,
  /**
   * The wasm module to install.
   */
  'module' : Uint8Array | number[],
}
export type SystemUpgradeTarget = { 'UpgradeUpgrader' : null } |
  { 'UpgradeStation' : null };
/**
 * An input type for configuring the upgrader canister.
 */
export type SystemUpgraderInput = {
    /**
     * An existing upgrader canister.
     */
    'Id' : Principal
  } |
  {
    /**
     * Creates and deploys a new canister.
     */
    'Deploy' : {
      /**
       * The initial cycles to allocate to the canister.
       * 
       * If not set, only the minimal amount of cycles required to create
       * and deploy the canister will be allocated.
       */
      'initial_cycles' : [] | [bigint],
      'wasm_module' : Uint8Array | number[],
    }
  };
/**
 * The timestamp type used in the canister.
 */
export type TimestampRFC3339 = string;
/**
 * A record type that can be used to represent a transfer in a given account.
 */
export interface Transfer {
  /**
   * The internal transfer id, this a unique identifier for the transfer.
   */
  'id' : UUID,
  /**
   * The destination address of the transaction (e.g. "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2").
   */
  'to' : string,
  /**
   * The fee to pay for the transaction, if applicable.
   */
  'fee' : bigint,
  /**
   * The id of the request that created the transfer.
   */
  'request_id' : UUID,
  /**
   * The status of the transfer.
   */
  'status' : TransferStatus,
  /**
   * The account id to use for the transfer.
   */
  'from_account_id' : UUID,
  /**
   * Transfers can be tagged with optional additional info (e.g. a `nonce` for Ethereum transactions).
   */
  'metadata' : Array<TransferMetadata>,
  /**
   * The network used when submitting the transaction to the blockchain.
   */
  'network' : Network,
  /**
   * The amount to transfer.
   */
  'amount' : bigint,
}
export interface TransferListItem {
  /**
   * The destination address of the transaction (e.g. "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2").
   */
  'to' : string,
  /**
   * The id of the request that created the transfer.
   */
  'request_id' : UUID,
  /**
   * The status of the transfer.
   */
  'status' : TransferStatus,
  /**
   * The time at which the transfer was created.
   */
  'created_at' : TimestampRFC3339,
  /**
   * The transfer id.
   */
  'transfer_id' : UUID,
  /**
   * The amount to transfer.
   */
  'amount' : bigint,
}
/**
 * Transfers can have additional information attached to them,
 * this type can be used to represent the additional info.
 */
export interface TransferMetadata {
  /**
   * The key of the additional info (e.g. "nonce",  "tag", "memo", etc...)
   */
  'key' : string,
  /**
   * The value of the additional info (e.g. "0x1234" or "my-tag")
   */
  'value' : string,
}
/**
 * Input type for transferring funds.
 */
export interface TransferOperation {
  /**
   * The fee paid for the transaction.
   */
  'fee' : [] | [bigint],
  /**
   * The asset to use for the transaction.
   */
  'from_asset' : Asset,
  /**
   * The network to use for the transaction.
   */
  'network' : Network,
  /**
   * The id of the executed transfer.
   */
  'transfer_id' : [] | [UUID],
  /**
   * The account to use for the transaction.
   */
  'from_account' : [] | [Account],
  /**
   * The input to the request to transfer funds.
   */
  'input' : TransferOperationInput,
}
/**
 * Input type for transferring funds.
 */
export interface TransferOperationInput {
  /**
   * The destination address of the transaction (e.g. "1BvBMSE...").
   */
  'to' : string,
  /**
   * The fee to pay for the transaction, if applicable.
   * 
   * If not set, the default fee will be used.
   */
  'fee' : [] | [bigint],
  /**
   * The standard to use for the transfer.
   */
  'with_standard' : string,
  /**
   * The account id to use for the transaction.
   */
  'from_account_id' : UUID,
  /**
   * Transactions can be tagged with an optional additional info
   * (e.g. a nonce in the case of an Ethereum transaction)
   */
  'metadata' : Array<TransferMetadata>,
  /**
   * The network to use for the transaction, if not the
   * default network of the account will be used.
   */
  'network' : [] | [Network],
  /**
   * The amount to transfer.
   */
  'amount' : bigint,
  /**
   * The asset id to transfer.
   */
  'from_asset_id' : UUID,
}
/**
 * The status of a transfer.
 */
export type TransferStatus = {
    /**
     * The transfer has been failed.
     */
    'Failed' : {
      /**
       * The failure reason.
       */
      'reason' : string,
    }
  } |
  {
    /**
     * The transfer is being processed.
     */
    'Processing' : {
      /**
       * The time at which the transfer started being processed.
       */
      'started_at' : TimestampRFC3339,
    }
  } |
  {
    /**
     * The transfer is created for processing.
     */
    'Created' : null
  } |
  {
    /**
     * The transfer has been completed.
     * 
     * For natively supported tokens this means that the transaction has
     * submitted to the blockchain. For non natively supported tokens this means
     * that the transaction has been signed and can be submitted by the client.
     */
    'Completed' : {
      /**
       * The base64 encoded value of the signed transaction, if available.
       */
      'signature' : [] | [string],
      /**
       * The transaction hash, if available.
       */
      'hash' : [] | [string],
      /**
       * Time at which the transaction was completed.
       */
      'completed_at' : TimestampRFC3339,
    }
  };
/**
 * Transfer status type for filtering on the transfer status.
 */
export type TransferStatusType = { 'Failed' : null } |
  { 'Processing' : null } |
  { 'Created' : null } |
  { 'Completed' : null };
/**
 * Most ids under the station canister are in the UUID format (e.g. "d0cf5b3f-7017-4cb8-9dcf-52619c42a7b0").
 */
export type UUID = string;
/**
 * A record type that can be used to represent a user in the station.
 */
export interface User {
  /**
   * The UUID of the user (e.g. "d0cf5b3f-7017-4cb8-9dcf-52619c42a7b0").
   */
  'id' : UUID,
  /**
   * The status of the user (e.g. `Active`).
   */
  'status' : UserStatus,
  /**
   * The list of groups the user belongs to.
   * 
   * Users can be tagged with groups that can be used to control access to resources.
   */
  'groups' : Array<UserGroup>,
  /**
   * The user name (e.g. "John Doe").
   */
  'name' : string,
  /**
   * The time at which the user was created or last modified (e.g. "2021-01-01T00:00:00Z").
   */
  'last_modification_timestamp' : TimestampRFC3339,
  /**
   * The principals associated with the user.
   */
  'identities' : Array<Principal>,
}
/**
 * A record type that can be used to represent the privileges of a caller for a given user.
 */
export interface UserCallerPrivileges {
  /**
   * The user id.
   */
  'id' : UUID,
  /**
   * Whether or not the caller can edit the user.
   */
  'can_edit' : boolean,
}
/**
 * A record type that can be used to represent a user group in the station.
 */
export interface UserGroup {
  /**
   * The UUID of the group (e.g. "d0cf5b3f-7017-4cb8-9dcf-52619c42a7b0").
   */
  'id' : UUID,
  /**
   * The name of the group (e.g. "Finance").
   */
  'name' : string,
}
/**
 * A record type that can be used to represent the privileges of a caller for a given user group.
 */
export interface UserGroupCallerPrivileges {
  /**
   * The user id.
   */
  'id' : UUID,
  /**
   * Whether or not the caller can delete the user group.
   */
  'can_delete' : boolean,
  /**
   * Whether or not the caller can edit the user group.
   */
  'can_edit' : boolean,
}
/**
 * The input type for adding identities to a user.
 */
export interface UserIdentityInput {
  /**
   * The identity of the user.
   */
  'identity' : Principal,
}
/**
 * The top level privileges that the user has when making calls to the canister.
 */
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
/**
 * The actions that are available for users.
 */
export type UserResourceAction = { 'List' : null } |
  { 'Read' : ResourceId } |
  { 'Create' : null } |
  { 'Update' : ResourceId };
/**
 * Defines a user in the context of a request.
 */
export type UserSpecifier = { 'Id' : Array<UUID> } |
  { 'Any' : null } |
  { 'Group' : Array<UUID> };
export type UserStatus = {
    /**
     * The user is inactive.
     */
    'Inactive' : null
  } |
  {
    /**
     * The user is active.
     */
    'Active' : null
  };
/**
 * The validation method targets of a `CallExternalCanister` request.
 */
export type ValidationMethodResourceTarget = { 'No' : null } |
  { 'ValidationMethod' : CanisterMethod };
export interface WasmModuleExtraChunks {
  /**
   * The hash of the assembled wasm module.
   */
  'wasm_module_hash' : Uint8Array | number[],
  /**
   * The asset canister from which the chunks are to be retrieved.
   */
  'store_canister' : Principal,
  /**
   * The name of the asset containing extra chunks in the asset canister.
   */
  'extra_chunks_key' : string,
}
/**
 * The Station service definition.
 */
export interface _SERVICE {
  /**
   * Cancel a request if the request is in a cancelable state.
   * 
   * Cancelable conditions:
   * 
   * - The request is in the `Created` state.
   * - The caller is the requester of the request.
   */
  'cancel_request' : ActorMethod<[CancelRequestInput], CancelRequestResult>,
  /**
   * Get snapshots of a canister controlled by the station.
   */
  'canister_snapshots' : ActorMethod<
    [CanisterSnapshotsInput],
    CanisterSnapshotsResult
  >,
  /**
   * Get canister status of a canister controlled by the station.
   */
  'canister_status' : ActorMethod<
    [CanisterStatusInput],
    CanisterStatusResponse
  >,
  /**
   * This method exposes the supported assets and other capabilities of the canister.
   * 
   * By default can be accessed by any active user.
   */
  'capabilities' : ActorMethod<[], CapabilitiesResult>,
  /**
   * Create a new request.
   * 
   * The request will be created and the caller will be added as the requester.
   */
  'create_request' : ActorMethod<[CreateRequestInput], CreateRequestResult>,
  /**
   * Get the account balance.
   * 
   * If the caller does not have access to the account, an error will be returned.
   */
  'fetch_account_balances' : ActorMethod<
    [FetchAccountBalancesInput],
    FetchAccountBalancesResult
  >,
  /**
   * Get a account by id.
   * 
   * If the caller does not have access to the account, an error will be returned.
   */
  'get_account' : ActorMethod<[GetAccountInput], GetAccountResult>,
  /**
   * If the caller does not have access to the address book entry, an error will be returned.
   */
  'get_address_book_entry' : ActorMethod<
    [GetAddressBookEntryInput],
    GetAddressBookEntryResult
  >,
  /**
   * Get an asset by id.
   */
  'get_asset' : ActorMethod<[GetAssetInput], GetAssetResult>,
  /**
   * Get the external canister by its canister id.
   */
  'get_external_canister' : ActorMethod<
    [GetExternalCanisterInput],
    GetExternalCanisterResult
  >,
  /**
   * Get the available filters for the external canisters.
   */
  'get_external_canister_filters' : ActorMethod<
    [GetExternalCanisterFiltersInput],
    GetExternalCanisterFiltersResult
  >,
  /**
   * Get a named rule by id.
   */
  'get_named_rule' : ActorMethod<[GetNamedRuleInput], GetNamedRuleResult>,
  /**
   * Finds the next aprovable request for the caller.
   */
  'get_next_approvable_request' : ActorMethod<
    [GetNextApprovableRequestInput],
    GetNextApprovableRequestResult
  >,
  /**
   * Get the permission for the resource provided.
   */
  'get_permission' : ActorMethod<[GetPermissionInput], GetPermissionResult>,
  /**
   * Get the request by id.
   */
  'get_request' : ActorMethod<[GetRequestInput], GetRequestResult>,
  /**
   * Get request policy by id.
   */
  'get_request_policy' : ActorMethod<
    [GetRequestPolicyInput],
    GetRequestPolicyResult
  >,
  /**
   * Get transfers by their ids.
   */
  'get_transfers' : ActorMethod<[GetTransfersInput], GetTransfersResult>,
  /**
   * Get the user associated with the user id provided.
   */
  'get_user' : ActorMethod<[GetUserInput], GetUserResult>,
  /**
   * Get a user group by id.
   * 
   * If the caller does not have access to the user group, an error will be returned.
   */
  'get_user_group' : ActorMethod<[GetUserGroupInput], GetUserGroupResult>,
  /**
   * Check if the station is healthy and ready to be used.
   */
  'health_status' : ActorMethod<[], HealthStatus>,
  /**
   * HTTP Protocol interface.
   */
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  /**
   * List all transfers from the requested account.
   */
  'list_account_transfers' : ActorMethod<
    [ListAccountTransfersInput],
    ListAccountTransfersResult
  >,
  /**
   * List all accounts that the caller has access to.
   * 
   * If the caller is not the owner of any account, an error will be returned.
   */
  'list_accounts' : ActorMethod<[ListAccountsInput], ListAccountsResult>,
  /**
   * List all address book entries for a given blockchain standard.
   */
  'list_address_book_entries' : ActorMethod<
    [ListAddressBookEntriesInput],
    ListAddressBookEntriesResult
  >,
  /**
   * List all assets that the caller has access to.
   */
  'list_assets' : ActorMethod<[ListAssetsInput], ListAssetsResult>,
  /**
   * List all external canisters that the caller has access to.
   */
  'list_external_canisters' : ActorMethod<
    [ListExternalCanistersInput],
    ListExternalCanistersResult
  >,
  /**
   * List named rules that the caller has access to.
   */
  'list_named_rules' : ActorMethod<[ListNamedRulesInput], ListNamedRulesResult>,
  /**
   * Get the list of notifications associated with the caller.
   */
  'list_notifications' : ActorMethod<
    [ListNotificationsInput],
    ListNotificationsResult
  >,
  /**
   * List all permissions.
   */
  'list_permissions' : ActorMethod<
    [ListPermissionsInput],
    ListPermissionsResult
  >,
  /**
   * List add request policies.
   */
  'list_request_policies' : ActorMethod<
    [ListRequestPoliciesInput],
    ListRequestPoliciesResult
  >,
  /**
   * Get the list of requests.
   * 
   * Only requests that the caller has access to will be returned.
   */
  'list_requests' : ActorMethod<[ListRequestsInput], ListRequestsResult>,
  /**
   * List all user groups of the station.
   */
  'list_user_groups' : ActorMethod<[ListUserGroupsInput], ListUserGroupsResult>,
  /**
   * List all users of the station.
   */
  'list_users' : ActorMethod<[ListUsersInput], ListUsersResult>,
  /**
   * Mark the notifications as read.
   */
  'mark_notifications_read' : ActorMethod<
    [MarkNotificationsReadInput],
    MarkNotificationReadResult
  >,
  /**
   * Get the authenticated user and its privileges from the caller.
   */
  'me' : ActorMethod<[], MeResult>,
  /**
   * Internal endpoint used by the upgrader canister to notify the station about a failed station upgrade request.
   */
  'notify_failed_station_upgrade' : ActorMethod<
    [NotifyFailedStationUpgradeInput],
    NotifyFailedStationUpgradeResult
  >,
  /**
   * Submits the user approval decision for a request.
   */
  'submit_request_approval' : ActorMethod<
    [SubmitRequestApprovalInput],
    SubmitRequestApprovalResult
  >,
  /**
   * Get the system information of the canister (e.g. version, cycles, etc.).
   * 
   * This method contains sensitive information and is up to the canister owner to
   * decide who can access it (e.g. only admins).
   */
  'system_info' : ActorMethod<[], SystemInfoResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
