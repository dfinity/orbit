import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

/**
 * The input for adding a registry entry.
 */
export interface AddRegistryEntryInput { 'entry' : RegistryEntryInput }
/**
 * The response of adding a registry entry.
 */
export interface AddRegistryEntryResponse { 'entry' : RegistryEntry }
/**
 * The result of adding a registry entry.
 */
export type AddRegistryEntryResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : AddRegistryEntryResponse
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * Generic error type added to responses that can fail.
 */
export interface ApiError {
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
 * The artifact type.
 */
export interface Artifact {
  /**
   * The id of the artifact.
   */
  'id' : UUID,
  /**
   * The sha256 hash of the artifact.
   */
  'hash' : Sha256Hex,
  /**
   * The blob of the artifact.
   */
  'artifact' : Uint8Array | number[],
  /**
   * The size of the artifact in bytes.
   */
  'size' : bigint,
  /**
   * The timestamp when the artifact was created.
   */
  'created_at' : TimestampRFC3339,
}
/**
 * The successful result of checking if the caller can deploy a station canister.
 * Returns the remaining number of station canisters the caller can still deploy
 * or a reason why the caller cannot deploy a station canister
 * (bad subscription status or exceeded quota).
 */
export type CanDeployStationResponse = {
    'NotAllowed' : UserSubscriptionStatus
  } |
  { 'Allowed' : bigint } |
  { 'QuotaExceeded' : null };
/**
 * The result of checking if the caller can deploy a station canister.
 */
export type CanDeployStationResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : CanDeployStationResponse
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The input for deleting a registry entry.
 */
export interface DeleteRegistryEntryInput {
  /**
   * The id of the registry entry.
   */
  'id' : UUID,
}
/**
 * The response of deleting a registry entry.
 */
export interface DeleteRegistryEntryResponse {
  /**
   * The registry entry that was deleted.
   */
  'entry' : RegistryEntry,
}
/**
 * The result of deleting a registry entry.
 */
export type DeleteRegistryEntryResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : DeleteRegistryEntryResponse
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The input for deploying a station admin user.
 * 
 * Used to associate a user with a station canister as an admin when initializing the station.
 */
export interface DeployStationAdminUserInput {
  /**
   * The username to associate with the station canister as an admin.
   */
  'username' : string,
  /**
   * The principal of the user to associate with the station canister as an admin.
   */
  'identity' : Principal,
}
/**
 * The input for deploying a station canister.
 */
export interface DeployStationInput {
  /**
   * The station name to use.
   */
  'name' : string,
  /**
   * The subnet to which the station should be deployed.
   * 
   * By default, the station is deployed to the same subnet as the control panel.
   */
  'subnet_selection' : [] | [SubnetSelection],
  /**
   * The initial admin users for the station.
   */
  'admins' : Array<DeployStationAdminUserInput>,
  /**
   * Wether to associate the deployed station to the caller's stations.
   * 
   * By default, the station is only added to the caller's stations if the caller is in the provided admins list.
   */
  'associate_with_caller' : [] | [
    {
      /**
       * The labels to associate with the station for the caller.
       */
      'labels' : Array<string>,
    }
  ],
}
/**
 * The result of deploying a station canister for the caller.
 */
export type DeployStationResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : {
      /**
       * The station canister id.
       */
      'canister_id' : StationID,
    }
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The input for editing a registry entry.
 */
export interface EditRegistryEntryInput {
  /**
   * The id of the registry entry.
   */
  'id' : UUID,
  /**
   * The updated registry entry.
   */
  'entry' : RegistryEntryUpdateInput,
}
/**
 * The response of editing a registry entry.
 */
export interface EditRegistryEntryResponse { 'entry' : RegistryEntry }
/**
 * The result of editing a registry entry.
 */
export type EditRegistryEntryResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : EditRegistryEntryResponse
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The get artifact input.
 */
export interface GetArtifactInput {
  /**
   * The id of the artifact.
   */
  'artifact_id' : UUID,
}
/**
 * The get artifact response.
 */
export interface GetArtifactResponse { 'artifact' : Artifact }
/**
 * The get artifact result.
 */
export type GetArtifactResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : GetArtifactResponse
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The input for getting a registry entry.
 */
export interface GetRegistryEntryInput {
  /**
   * The id of the registry entry.
   */
  'id' : UUID,
}
/**
 * The get registry entry response.
 */
export interface GetRegistryEntryResponse {
  /**
   * The registry entry that matches the provided id.
   */
  'entry' : RegistryEntry,
}
/**
 * The get registry entry result.
 */
export type GetRegistryEntryResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : GetRegistryEntryResponse
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The result of getting the user information.
 */
export type GetUserResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : {
      /**
       * The caller user information.
       */
      'user' : User,
    }
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The HTTP request header field.
 */
export type HeaderField = [string, string];
/**
 * The HTTP request.
 */
export interface HttpRequest {
  /**
   * The URL of the request (e.g. https://example.com).
   */
  'url' : string,
  /**
   * The HTTP method of the request (e.g. GET, POST, PUT).
   */
  'method' : string,
  /**
   * The body of the request.
   */
  'body' : Uint8Array | number[],
  /**
   * The headers of the request.
   */
  'headers' : Array<HeaderField>,
}
/**
 * The HTTP response.
 */
export interface HttpResponse {
  /**
   * The body of the response.
   */
  'body' : Uint8Array | number[],
  /**
   * The headers of the response.
   */
  'headers' : Array<HeaderField>,
  /**
   * The status code of the response (e.g. 200, 404, 500).
   */
  'status_code' : number,
}
/**
 * The input for listing stations.
 */
export interface ListUserStationsInput {
  /**
   * The labels to filter the stations by.
   * 
   * All stations that have at least one of the provided labels will be returned, if no labels
   * are provided all stations will be returned.
   * 
   * The match is case insensitive.
   */
  'filter_by_labels' : [] | [Array<string>],
}
/**
 * The result of listing user stations.
 */
export type ListUserStationsResult = {
    /**
     * The list of stations.
     */
    'Ok' : {
      /**
       * The list of stations.
       */
      'stations' : Array<UserStation>,
    }
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The action to perform on the stations associated with the user.
 * 
 * This can be used to add, remove, update the stations associated with the user.
 */
export type ManageUserStationsInput = {
    /**
     * Add a the specified stations to the user, if the station already exists it will be updated.
     */
    'Add' : Array<UserStation>
  } |
  {
    /**
     * Remove the specified stations from the user, if the station does not exist it will be ignored.
     */
    'Remove' : Array<StationID>
  } |
  {
    /**
     * Update the specified stations associated with the user.
     */
    'Update' : Array<
      {
        /**
         * The stations to update, if the station does not exist it will be ignored.
         */
        'station' : UserStation,
        /**
         * The new index of the station, if the index is out of bounds it will set to the closest bound.
         */
        'index' : [] | [bigint],
      }
    >
  };
/**
 * The result of managing the user stations.
 */
export type ManageUserStationsResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : null
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * A metadata record that contains a key and a value.
 */
export interface Metadata { 'key' : string, 'value' : string }
/**
 * The input for finding the next wasm module version.
 */
export interface NextWasmModuleVersionInput {
  /**
   * The name of the registry entry.
   */
  'name' : string,
  /**
   * The current version of the registry entry.
   */
  'current_version' : string,
}
/**
 * The response of finding the next wasm module version.
 */
export interface NextWasmModuleVersionResponse {
  /**
   * The registry entry that is the next version of the provided entry.
   */
  'entry' : [] | [RegistryEntry],
}
/**
 * The result of finding the next wasm module version.
 */
export type NextWasmModuleVersionResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : NextWasmModuleVersionResponse
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
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
 * The input for registering an user.
 */
export interface RegisterUserInput {
  /**
   * A station canister to use for this user.
   */
  'station' : [] | [UserStation],
}
/**
 * The result of registering an user.
 */
export type RegisterUserResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : {
      /**
       * The caller newly created user information.
       */
      'user' : User,
    }
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * An entry record, which contains information and the value stored in the registry.
 */
export interface RegistryEntry {
  /**
   * The UUID that identifies the entry in the registry.
   */
  'id' : UUID,
  /**
   * The category is used to associate the entry with a specific category (e.g. "chain-fusion")
   * across all the entries in the registry.
   * 
   * Restrictions:
   * 
   * - Categories must be between 2 and 32 characters long.
   * - There can be up to 10 categories per entry.
   */
  'categories' : Array<string>,
  /**
   * The timestamp when the entry was last updated.
   */
  'updated_at' : [] | [TimestampRFC3339],
  /**
   * The content of the entry in the registry.
   */
  'value' : RegistryEntryValue,
  /**
   * The metadata of the entry in the registry.
   * 
   * This is a key-value map that can be used to store additional information about the entry,
   * such as the author, license, repository, docs, etc.
   * 
   * Restrictions:
   * 
   * - The key must be between 1 and 32 characters long.
   * - The value must be between 1 and 512 characters long.
   * - There can be up to 10 metadata entries per entry in the registry.
   */
  'metadata' : Array<Metadata>,
  /**
   * The name of the entry, which is used to identify it (e.g. station). Names that start with `@` are considered
   * to be namespaced, and the namespace is the part of the name that comes before the `/`. Within each namespace
   * the name should refer to the same type of entry, but many entries can exist with the same name.
   * 
   * e.g. if the namespace is "@orbit" and the name is "station", then all the entries will refer to a wasm module.
   * 
   * Restrictions:
   * 
   * - Names that start with `@` are considered namespaced.
   * - Names that start with `@` must have a namespace and a name separated by a `/`.
   * - Names must be between 2 and 48 characters long.
   * - Namespaces must be between 2 and 32 characters long.
   * - Names that are not namespaced, are put in the default namespace `@default`.
   */
  'name' : string,
  /**
   * The tags are used to tag the entry with specific search terms (e.g. "latest", "stable").
   * 
   * Tags are grouped within the same `namespace/name` (e.g. "@orbit/station").
   * 
   * Restrictions:
   * 
   * - Tags must be between 2 and 32 characters long.
   * - There can be up to 10 tags per entry.
   */
  'tags' : Array<string>,
  /**
   * The description of the entry, which is a human-readable description of the entry.
   * 
   * Restrictions:
   * 
   * - Descriptions must be between 24 and 512 characters long.
   */
  'description' : string,
  /**
   * The timestamp when the entry was created.
   */
  'created_at' : TimestampRFC3339,
}
/**
 * The registry entry input.
 */
export interface RegistryEntryInput {
  /**
   * The category is used to associate the entry with a specific category (e.g. "chain-fusion")
   * across all the entries in the registry.
   */
  'categories' : Array<string>,
  /**
   * The content of the entry in the registry.
   */
  'value' : RegistryEntryValueInput,
  /**
   * The metadata of the entry in the registry.
   */
  'metadata' : Array<Metadata>,
  /**
   * The name of the entry, which is used to identify it (e.g. station).
   */
  'name' : string,
  /**
   * The tags are used to tag the entry with specific search terms (e.g. "latest", "stable").
   */
  'tags' : Array<string>,
  /**
   * The description of the entry, which is a human-readable description of the entry.
   */
  'description' : string,
}
/**
 * The registry entry sort options.
 */
export type RegistryEntrySortBy = {
    /**
     * Sort by the version of the registry entry, if applicable.
     */
    'Version' : SortDirection
  } |
  {
    /**
     * Sort by the timestamp when the registry entry was created.
     */
    'CreatedAt' : SortDirection
  };
/**
 * The registry entry update input.
 */
export interface RegistryEntryUpdateInput {
  /**
   * The category is used to associate the entry with a specific category (e.g. "chain-fusion")
   * across all the entries in the registry.
   */
  'categories' : [] | [Array<string>],
  /**
   * The content of the entry in the registry.
   */
  'value' : [] | [RegistryEntryValueInput],
  /**
   * The metadata of the entry in the registry.
   */
  'metadata' : [] | [Array<Metadata>],
  /**
   * The tags are used to tag the entry with specific search terms (e.g. "latest", "stable").
   */
  'tags' : [] | [Array<string>],
  /**
   * The description of the entry, which is a human-readable description of the entry.
   */
  'description' : [] | [string],
}
/**
 * The registry entry value.
 */
export type RegistryEntryValue = {
    'WasmModule' : WasmModuleRegistryEntryValue
  };
/**
 * The registry entry value input.
 */
export type RegistryEntryValueInput = {
    'WasmModule' : WasmModuleRegistryEntryValueInput
  };
/**
 * The registry entry value kind.
 */
export type RegistryEntryValueKind = { 'WasmModule' : null };
/**
 * The result of removing the user associated with the caller.
 */
export type RemoveUserResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : {
      /**
       * The caller user that was removed.
       */
      'user' : User,
    }
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The search registry filter options.
 */
export type SearchRegistryFilterKind = {
    /**
     * The kind of registry entry to find.
     */
    'Kind' : RegistryEntryValueKind
  } |
  {
    /**
     * The name of the registry entry to find, if the namespace is not provided the default namespace is used.
     */
    'Name' : string
  } |
  {
    /**
     * The namespace of the registry entry to find.
     */
    'Namespace' : string
  };
/**
 * The input for searching the registry.
 */
export interface SearchRegistryInput {
  /**
   * Sort the entries in the registry.
   */
  'sort_by' : [] | [RegistryEntrySortBy],
  /**
   * The pagination options to use for the search.
   */
  'pagination' : [] | [PaginationInput],
  /**
   * Filters are used sequentially and are combined with an AND operation to filter the entries in the registry.
   */
  'filter_by' : Array<SearchRegistryFilterKind>,
}
/**
 * The search registry response.
 */
export interface SearchRegistryResponse {
  /**
   * The total number of entries that match the search criteria.
   */
  'total' : bigint,
  /**
   * The list of registry entries that match the search criteria.
   */
  'entries' : Array<RegistryEntry>,
  /**
   * The next offset to use for pagination.
   */
  'next_offset' : [] | [bigint],
}
/**
 * The search registry result.
 */
export type SearchRegistryResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : SearchRegistryResponse
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The result of setting the user active.
 */
export type SetUserActiveResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : null
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The sha256 hash.
 */
export type Sha256Hex = string;
export type SortDirection = { 'Asc' : null } |
  { 'Desc' : null };
/**
 * The canister id of a station.
 */
export type StationID = Principal;
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
 * The timestamp type used in the canister.
 */
export type TimestampRFC3339 = string;
/**
 * A UUID used to identify items.
 */
export type UUID = string;
/**
 * The canister modules required for the control panel.
 */
export interface UploadCanisterModulesInput {
  /**
   * Optional extra chunks of the station canister wasm module.
   */
  'station_wasm_module_extra_chunks' : [] | [[] | [WasmModuleExtraChunks]],
  /**
   * The station wasm module to use.
   */
  'station_wasm_module' : [] | [Uint8Array | number[]],
  /**
   * The upgrader wasm module to use for the station canister.
   */
  'upgrader_wasm_module' : [] | [Uint8Array | number[]],
}
/**
 * The result of uploading canister modules.
 */
export type UploadUploadCanisterModulesInputResult = {
    /**
     * Successfull operation result.
     */
    'Ok' : null
  } |
  {
    /**
     * The error that occurred during the operation.
     */
    'Err' : ApiError
  };
/**
 * The user user information.
 */
export interface User {
  /**
   * The last time the user was active in the system.
   */
  'last_active' : TimestampRFC3339,
  /**
   * The waiting list subscription status.
   */
  'subscription_status' : UserSubscriptionStatus,
  /**
   * The identity associated with the user.
   */
  'identity' : Principal,
}
/**
 * The id of an user.
 */
export type UserId = UUID;
/**
 * The principal that is associated with an user.
 */
export type UserIdentityID = Principal;
/**
 * The station information associated with the user.
 */
export interface UserStation {
  /**
   * The name of the station.
   */
  'name' : string,
  /**
   * The labels associated with the station.
   * 
   * This can be used to store user preferences or other information related to the station such as the station type.
   * 
   * Maximum of 10 labels per station and 64 characters per label.
   */
  'labels' : Array<string>,
  /**
   * The id associated with the station.
   */
  'canister_id' : StationID,
}
export type UserSubscriptionStatus = { 'Unsubscribed' : null } |
  { 'Approved' : null } |
  { 'Denylisted' : null } |
  { 'Pending' : null };
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
 * The dependency of a wasm module.
 */
export interface WasmModuleRegistryEntryDependency {
  /**
   * The name of the dependency.
   */
  'name' : string,
  /**
   * The version of the wasm module.
   */
  'version' : string,
}
/**
 * The wasm module registry value, which is the content of the wasm module and its version.
 */
export interface WasmModuleRegistryEntryValue {
  /**
   * Optional extra chunks of the wasm module that are stored in an asset canister referenced in the record `WasmModuleExtraChunks`.
   */
  'module_extra_chunks' : [] | [WasmModuleExtraChunks],
  /**
   * The version of the wasm module.
   * 
   * Restrictions:
   * 
   * - Versions must be between 1 and 32 characters long.
   */
  'version' : string,
  /**
   * The dependencies of the wasm module, which are other wasm modules that this wasm module depends on.
   * 
   * This registry ids should only reference registry entries that are of type `WasmModule`, others will be ignored.
   */
  'dependencies' : Array<WasmModuleRegistryEntryDependency>,
  /**
   * The id of the wasm module that is stored in the artifact repository.
   */
  'wasm_artifact_id' : UUID,
}
/**
 * The wasm module registry value input, which is the content of the wasm module and its version.
 */
export interface WasmModuleRegistryEntryValueInput {
  /**
   * The wasm module that should be stored in the artifact repository.
   */
  'wasm_module' : Uint8Array | number[],
  /**
   * Optional extra chunks of the wasm module that are stored in an asset canister referenced in the record `WasmModuleExtraChunks`.
   */
  'module_extra_chunks' : [] | [WasmModuleExtraChunks],
  /**
   * The version of the wasm module.
   */
  'version' : string,
  /**
   * The dependencies of the wasm module, which are other wasm modules that this wasm module depends on.
   */
  'dependencies' : Array<WasmModuleRegistryEntryDependency>,
}
/**
 * The control panel service definition.
 */
export interface _SERVICE {
  /**
   * Add a new entry to the registry.
   * 
   * The caller must have the necessary permissions to add an entry to the registry.
   */
  'add_registry_entry' : ActorMethod<
    [AddRegistryEntryInput],
    AddRegistryEntryResult
  >,
  /**
   * Checks if the caller can deploy a new station canister.
   */
  'can_deploy_station' : ActorMethod<[], CanDeployStationResult>,
  /**
   * Deletes an existing entry from the registry.
   */
  'delete_registry_entry' : ActorMethod<
    [DeleteRegistryEntryInput],
    DeleteRegistryEntryResult
  >,
  /**
   * Delete user associated with the caller.
   */
  'delete_user' : ActorMethod<[], RemoveUserResult>,
  /**
   * Deploys a new station canister for the caller.
   */
  'deploy_station' : ActorMethod<[DeployStationInput], DeployStationResult>,
  /**
   * Edit an existing entry in the registry.
   * A registry entry can only be edited if it references the same kind of value.
   * 
   * The caller must have the necessary permissions to edit an entry in the registry.
   */
  'edit_registry_entry' : ActorMethod<
    [EditRegistryEntryInput],
    EditRegistryEntryResult
  >,
  /**
   * Enables the caller to get an artifact by its id.
   */
  'get_artifact' : ActorMethod<[GetArtifactInput], GetArtifactResult>,
  /**
   * Get the registry entry by its id.
   */
  'get_registry_entry' : ActorMethod<
    [GetRegistryEntryInput],
    GetRegistryEntryResult
  >,
  /**
   * Get the user information for the caller.
   */
  'get_user' : ActorMethod<[], GetUserResult>,
  /**
   * HTTP Protocol interface.
   */
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  /**
   * List all the stations associated with the caller.
   */
  'list_user_stations' : ActorMethod<
    [ListUserStationsInput],
    ListUserStationsResult
  >,
  /**
   * Manage the stations associated with the caller.
   */
  'manage_user_stations' : ActorMethod<
    [ManageUserStationsInput],
    ManageUserStationsResult
  >,
  /**
   * Find the next wasm module version for the provided module name and current version.
   * 
   * If no next version is found, the result will be `None`.
   */
  'next_wasm_module_version' : ActorMethod<
    [NextWasmModuleVersionInput],
    NextWasmModuleVersionResult
  >,
  /**
   * Create a new user for the caller.
   */
  'register_user' : ActorMethod<[RegisterUserInput], RegisterUserResult>,
  /**
   * Search the registry for entries.
   */
  'search_registry' : ActorMethod<[SearchRegistryInput], SearchRegistryResult>,
  /**
   * Set the last active time for the user associated with the caller.
   */
  'set_user_active' : ActorMethod<[], SetUserActiveResult>,
  /**
   * Uploads the canister modules for the station and upgrader canisters.
   */
  'upload_canister_modules' : ActorMethod<
    [UploadCanisterModulesInput],
    UploadUploadCanisterModulesInputResult
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
