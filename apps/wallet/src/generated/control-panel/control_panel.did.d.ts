import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface ApiError {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export type CanDeployStationResponse = {
    'NotAllowed' : UserSubscriptionStatus
  } |
  { 'Allowed' : bigint } |
  { 'QuotaExceeded' : null };
export type CanDeployStationResult = { 'Ok' : CanDeployStationResponse } |
  { 'Err' : ApiError };
export interface DeployStationInput {
  'admin_name' : string,
  'station_name' : string,
}
export type DeployStationResult = { 'Ok' : { 'canister_id' : StationID } } |
  { 'Err' : ApiError };
export type GetMainStationResult = {
    'Ok' : { 'station' : [] | [UserStation] }
  } |
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
export type ListStationsResult = {
    'Ok' : { 'stations' : Array<UserStation> }
  } |
  { 'Err' : ApiError };
export interface ManageUserInput {
  'stations' : [] | [Array<UserStation>],
  'main_station' : [] | [StationID],
}
export type ManageUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export interface RegisterUserInput { 'station' : [] | [UserStation] }
export type RegisterUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type RemoveUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type SetUserActiveResult = { 'Ok' : null } |
  { 'Err' : ApiError };
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
  'stations' : Array<UserStation>,
  'last_active' : TimestampRFC3339,
  'subscription_status' : UserSubscriptionStatus,
  'identity' : Principal,
  'main_station' : [] | [StationID],
}
export type UserId = UUID;
export type UserIdentityID = Principal;
export interface UserStation { 'name' : string, 'canister_id' : StationID }
export type UserSubscriptionStatus = { 'Unsubscribed' : null } |
  { 'Approved' : null } |
  { 'Denylisted' : null } |
  { 'Pending' : null };
export interface _SERVICE {
  'can_deploy_station' : ActorMethod<[], CanDeployStationResult>,
  'delete_user' : ActorMethod<[], RemoveUserResult>,
  'deploy_station' : ActorMethod<[DeployStationInput], DeployStationResult>,
  'get_main_station' : ActorMethod<[], GetMainStationResult>,
  'get_user' : ActorMethod<[], GetUserResult>,
  'get_waiting_list' : ActorMethod<[], GetWaitingListResult>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'list_stations' : ActorMethod<[], ListStationsResult>,
  'manage_user' : ActorMethod<[ManageUserInput], ManageUserResult>,
  'register_user' : ActorMethod<[RegisterUserInput], RegisterUserResult>,
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
