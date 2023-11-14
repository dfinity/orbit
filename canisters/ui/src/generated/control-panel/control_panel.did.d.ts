import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ApiError {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export interface AssociateIdentityWithUserInput { 'user_id' : UserId }
export type AssociateIdentityWithUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export interface CanisterInit { 'default_wallet' : DefaultWalletInit }
export type DefaultWalletInit = { 'SpecifiedWalletCanister' : WalletID } |
  { 'InitSharedWalletCanister' : null };
export type GetMainWalletResult = { 'Ok' : { 'wallet' : [] | [UserWallet] } } |
  { 'Err' : ApiError };
export type GetUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type ListWalletsResult = { 'Ok' : { 'wallets' : Array<UserWallet> } } |
  { 'Err' : ApiError };
export interface ManageUserInput {
  'name' : [] | [string],
  'wallets' : [] | [Array<UserWallet>],
  'identities' : [] | [Array<UserIdentity>],
  'main_wallet' : [] | [WalletID],
}
export type ManageUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export interface RegisterUserInput {
  'name' : [] | [string],
  'wallet' : RegisterUserWalletInput,
}
export type RegisterUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type RegisterUserWalletInput = { 'SharedWallet' : null } |
  {
    'PrivateWallet' : {
      'id' : WalletID,
      'use_shared_wallet' : [] | [{ 'is_main' : boolean }],
    }
  };
export type RemoveUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type UUID = string;
export interface User {
  'id' : UserId,
  'unconfirmed_identities' : Array<UserIdentity>,
  'name' : [] | [string],
  'wallets' : Array<UserWallet>,
  'identities' : Array<UserIdentity>,
  'main_wallet' : [] | [WalletID],
}
export type UserId = UUID;
export interface UserIdentity {
  'name' : [] | [string],
  'identity' : UserIdentityID,
}
export type UserIdentityID = Principal;
export interface UserWallet { 'name' : [] | [string], 'canister_id' : WalletID }
export type WalletID = Principal;
export interface _SERVICE {
  'associate_identity_with_user' : ActorMethod<
    [AssociateIdentityWithUserInput],
    AssociateIdentityWithUserResult
  >,
  'delete_user' : ActorMethod<[], RemoveUserResult>,
  'get_main_wallet' : ActorMethod<[], GetMainWalletResult>,
  'get_user' : ActorMethod<[], GetUserResult>,
  'list_wallets' : ActorMethod<[], ListWalletsResult>,
  'manage_user' : ActorMethod<[ManageUserInput], ManageUserResult>,
  'register_user' : ActorMethod<[RegisterUserInput], RegisterUserResult>,
}
