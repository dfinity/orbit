import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface ApiError {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export interface CanisterInit {
  'upgrader_wasm_module' : Uint8Array | number[],
  'wallet_wasm_module' : Uint8Array | number[],
}
export type CanisterInstall = { 'Upgrade' : CanisterUpgrade } |
  { 'Init' : CanisterInit };
export interface CanisterUpgrade {
  'upgrader_wasm_module' : [] | [Uint8Array | number[]],
  'wallet_wasm_module' : [] | [Uint8Array | number[]],
}
export type DeployWalletResult = { 'Ok' : { 'canister_id' : WalletID } } |
  { 'Err' : ApiError };
export type GetMainWalletResult = { 'Ok' : { 'wallet' : [] | [UserWallet] } } |
  { 'Err' : ApiError };
export type GetUserResult = { 'Ok' : { 'user' : User } } |
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
export type ListWalletsResult = { 'Ok' : { 'wallets' : Array<UserWallet> } } |
  { 'Err' : ApiError };
export interface ManageUserInput {
  'wallets' : [] | [Array<UserWallet>],
  'main_wallet' : [] | [WalletID],
}
export type ManageUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export interface RegisterUserInput { 'wallet_id' : [] | [Principal] }
export type RegisterUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type RemoveUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type UUID = string;
export interface User {
  'id' : Principal,
  'wallets' : Array<UserWallet>,
  'main_wallet' : [] | [WalletID],
}
export type UserId = UUID;
export type UserIdentityID = Principal;
export interface UserWallet { 'name' : [] | [string], 'canister_id' : WalletID }
export type WalletID = Principal;
export interface _SERVICE {
  'delete_user' : ActorMethod<[], RemoveUserResult>,
  'deploy_wallet' : ActorMethod<[], DeployWalletResult>,
  'get_main_wallet' : ActorMethod<[], GetMainWalletResult>,
  'get_user' : ActorMethod<[], GetUserResult>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'list_wallets' : ActorMethod<[], ListWalletsResult>,
  'manage_user' : ActorMethod<[ManageUserInput], ManageUserResult>,
  'register_user' : ActorMethod<[RegisterUserInput], RegisterUserResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
