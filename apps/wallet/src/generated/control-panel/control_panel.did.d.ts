import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AddRegistryEntryInput { 'entry' : RegistryEntryInput }
export interface AddRegistryEntryResponse { 'entry' : RegistryEntry }
export type AddRegistryEntryResult = { 'Ok' : AddRegistryEntryResponse } |
  { 'Err' : ApiError };
export interface ApiError {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export interface Artifact {
  'id' : UUID,
  'hash' : Sha256Hex,
  'artifact' : Uint8Array | number[],
  'size' : bigint,
  'created_at' : TimestampRFC3339,
}
export type CanDeployStationResponse = {
    'NotAllowed' : UserSubscriptionStatus
  } |
  { 'Allowed' : bigint } |
  { 'QuotaExceeded' : null };
export type CanDeployStationResult = { 'Ok' : CanDeployStationResponse } |
  { 'Err' : ApiError };
export interface DeleteRegistryEntryInput { 'id' : UUID }
export interface DeleteRegistryEntryResponse { 'entry' : RegistryEntry }
export type DeleteRegistryEntryResult = { 'Ok' : DeleteRegistryEntryResponse } |
  { 'Err' : ApiError };
export interface DeployStationAdminUserInput {
  'username' : string,
  'identity' : Principal,
}
export interface DeployStationInput {
  'name' : string,
  'admins' : Array<DeployStationAdminUserInput>,
  'associate_with_caller' : [] | [{ 'labels' : Array<string> }],
}
export type DeployStationResult = { 'Ok' : { 'canister_id' : StationID } } |
  { 'Err' : ApiError };
export interface EditRegistryEntryInput {
  'id' : UUID,
  'entry' : RegistryEntryUpdateInput,
}
export interface EditRegistryEntryResponse { 'entry' : RegistryEntry }
export type EditRegistryEntryResult = { 'Ok' : EditRegistryEntryResponse } |
  { 'Err' : ApiError };
export interface GetArtifactInput { 'artifact_id' : UUID }
export interface GetArtifactResponse { 'artifact' : Artifact }
export type GetArtifactResult = { 'Ok' : GetArtifactResponse } |
  { 'Err' : ApiError };
export interface GetRegistryEntryInput { 'id' : UUID }
export interface GetRegistryEntryResponse { 'entry' : RegistryEntry }
export type GetRegistryEntryResult = { 'Ok' : GetRegistryEntryResponse } |
  { 'Err' : ApiError };
export type GetUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export interface GetWaitingListResponse {
  'subscribed_users' : Array<SubscribedUser>,
}
export type GetWaitingListResult = { 'Ok' : GetWaitingListResponse } |
  { 'Err' : ApiError };
export type HeaderField = [string, string];
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
export interface ListUserStationsInput {
  'filter_by_labels' : [] | [Array<string>],
}
export type ListUserStationsResult = {
    'Ok' : { 'stations' : Array<UserStation> }
  } |
  { 'Err' : ApiError };
export type ManageUserStationsInput = { 'Add' : Array<UserStation> } |
  { 'Remove' : Array<StationID> } |
  { 'Update' : Array<{ 'station' : UserStation, 'index' : [] | [bigint] }> };
export type ManageUserStationsResult = { 'Ok' : null } |
  { 'Err' : ApiError };
export interface Metadata { 'key' : string, 'value' : string }
export interface NextModuleVersionInput {
  'name' : string,
  'current_version' : string,
}
export interface NextModuleVersionResponse { 'entry' : [] | [RegistryEntry] }
export type NextModuleVersionResult = { 'Ok' : NextModuleVersionResponse } |
  { 'Err' : ApiError };
export interface PaginationInput {
  'offset' : [] | [bigint],
  'limit' : [] | [number],
}
export interface RegisterUserInput { 'station' : [] | [UserStation] }
export type RegisterUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export interface RegistryEntry {
  'id' : UUID,
  'categories' : Array<string>,
  'updated_at' : [] | [TimestampRFC3339],
  'value' : RegistryEntryValue,
  'metadata' : Array<Metadata>,
  'name' : string,
  'tags' : Array<string>,
  'description' : string,
  'created_at' : TimestampRFC3339,
}
export interface RegistryEntryInput {
  'categories' : Array<string>,
  'value' : RegistryEntryValueInput,
  'metadata' : Array<Metadata>,
  'name' : string,
  'tags' : Array<string>,
  'description' : string,
}
export type RegistryEntrySortBy = { 'Version' : SortDirection } |
  { 'CreatedAt' : SortDirection };
export interface RegistryEntryUpdateInput {
  'categories' : [] | [Array<string>],
  'value' : [] | [RegistryEntryValueInput],
  'metadata' : [] | [Array<Metadata>],
  'tags' : [] | [Array<string>],
  'description' : [] | [string],
}
export type RegistryEntryValue = {
    'WasmModule' : WasmModuleRegistryEntryValue
  };
export type RegistryEntryValueInput = {
    'WasmModule' : WasmModuleRegistryEntryValueInput
  };
export type RegistryEntryValueKind = { 'WasmModule' : null };
export type RemoveUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type SearchRegistryFilterKind = { 'Kind' : RegistryEntryValueKind } |
  { 'Name' : string } |
  { 'Namespace' : string };
export interface SearchRegistryInput {
  'sort_by' : [] | [RegistryEntrySortBy],
  'pagination' : [] | [PaginationInput],
  'filter_by' : Array<SearchRegistryFilterKind>,
}
export interface SearchRegistryResponse {
  'total' : bigint,
  'entries' : Array<RegistryEntry>,
  'next_offset' : [] | [bigint],
}
export type SearchRegistryResult = { 'Ok' : SearchRegistryResponse } |
  { 'Err' : ApiError };
export type SetUserActiveResult = { 'Ok' : null } |
  { 'Err' : ApiError };
export type Sha256Hex = string;
export type SortDirection = { 'Asc' : null } |
  { 'Desc' : null };
export type StationID = Principal;
export type SubscribeToWaitingListResult = { 'Ok' : null } |
  { 'Err' : ApiError };
export interface SubscribedUser {
  'user_principal' : Principal,
  'email' : string,
}
export type TimestampRFC3339 = string;
export type UUID = string;
export interface UpdateWaitingListInput {
  'users' : Array<Principal>,
  'new_status' : UserSubscriptionStatus,
}
export type UpdateWaitingListResult = { 'Ok' : null } |
  { 'Err' : ApiError };
export interface UploadCanisterModulesInput {
  'station_wasm_module' : Uint8Array | number[],
  'upgrader_wasm_module' : Uint8Array | number[],
}
export type UploadUploadCanisterModulesInputResult = { 'Ok' : null } |
  { 'Err' : ApiError };
export interface User {
  'last_active' : TimestampRFC3339,
  'subscription_status' : UserSubscriptionStatus,
  'identity' : Principal,
}
export type UserId = UUID;
export type UserIdentityID = Principal;
export interface UserStation {
  'name' : string,
  'labels' : Array<string>,
  'canister_id' : StationID,
}
export type UserSubscriptionStatus = { 'Unsubscribed' : null } |
  { 'Approved' : null } |
  { 'Denylisted' : null } |
  { 'Pending' : null };
export interface WasmModuleRegistryEntryDependency {
  'name' : string,
  'version' : string,
}
export interface WasmModuleRegistryEntryValue {
  'version' : string,
  'dependencies' : Array<WasmModuleRegistryEntryDependency>,
  'wasm_artifact_id' : UUID,
}
export interface WasmModuleRegistryEntryValueInput {
  'wasm_module' : Uint8Array | number[],
  'version' : string,
  'dependencies' : Array<WasmModuleRegistryEntryDependency>,
}
export interface _SERVICE {
  'add_registry_entry' : ActorMethod<
    [AddRegistryEntryInput],
    AddRegistryEntryResult
  >,
  'can_deploy_station' : ActorMethod<[], CanDeployStationResult>,
  'delete_registry_entry' : ActorMethod<
    [DeleteRegistryEntryInput],
    DeleteRegistryEntryResult
  >,
  'delete_user' : ActorMethod<[], RemoveUserResult>,
  'deploy_station' : ActorMethod<[DeployStationInput], DeployStationResult>,
  'edit_registry_entry' : ActorMethod<
    [EditRegistryEntryInput],
    EditRegistryEntryResult
  >,
  'get_artifact' : ActorMethod<[GetArtifactInput], GetArtifactResult>,
  'get_registry_entry' : ActorMethod<
    [GetRegistryEntryInput],
    GetRegistryEntryResult
  >,
  'get_user' : ActorMethod<[], GetUserResult>,
  'get_waiting_list' : ActorMethod<[], GetWaitingListResult>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'list_user_stations' : ActorMethod<
    [ListUserStationsInput],
    ListUserStationsResult
  >,
  'manage_user_stations' : ActorMethod<
    [ManageUserStationsInput],
    ManageUserStationsResult
  >,
  'next_module_version' : ActorMethod<
    [NextModuleVersionInput],
    NextModuleVersionResult
  >,
  'register_user' : ActorMethod<[RegisterUserInput], RegisterUserResult>,
  'search_registry' : ActorMethod<[SearchRegistryInput], SearchRegistryResult>,
  'set_user_active' : ActorMethod<[], SetUserActiveResult>,
  'subscribe_to_waiting_list' : ActorMethod<
    [string],
    SubscribeToWaitingListResult
  >,
  'update_waiting_list' : ActorMethod<
    [UpdateWaitingListInput],
    UpdateWaitingListResult
  >,
  'upload_canister_modules' : ActorMethod<
    [UploadCanisterModulesInput],
    UploadUploadCanisterModulesInputResult
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
