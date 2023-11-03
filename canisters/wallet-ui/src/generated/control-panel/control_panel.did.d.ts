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
export type BankID = Principal;
export interface CanisterInit { 'default_bank' : DefaultBankInit }
export type DefaultBankInit = { 'InitSharedBankCanister' : null } |
  { 'SpecifiedBankCanister' : BankID };
export type GetMainBankResult = { 'Ok' : { 'bank' : [] | [UserBank] } } |
  { 'Err' : ApiError };
export type GetUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type ListBanksResult = { 'Ok' : { 'banks' : Array<UserBank> } } |
  { 'Err' : ApiError };
export interface ManageUserInput {
  'name' : [] | [string],
  'main_bank' : [] | [BankID],
  'banks' : [] | [Array<UserBank>],
  'identities' : [] | [Array<UserIdentity>],
}
export type ManageUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type RegisterUserBankInput = {
    'PrivateBank' : {
      'id' : BankID,
      'use_shared_bank' : [] | [{ 'is_main' : boolean }],
    }
  } |
  { 'SharedBank' : null };
export interface RegisterUserInput {
  'bank' : RegisterUserBankInput,
  'name' : [] | [string],
}
export type RegisterUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type RemoveUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : ApiError };
export type UUID = string;
export interface User {
  'id' : UserId,
  'unconfirmed_identities' : Array<UserIdentity>,
  'name' : [] | [string],
  'main_bank' : [] | [BankID],
  'banks' : Array<UserBank>,
  'identities' : Array<UserIdentity>,
}
export interface UserBank { 'name' : [] | [string], 'canister_id' : BankID }
export type UserId = UUID;
export interface UserIdentity {
  'name' : [] | [string],
  'identity' : UserIdentityID,
}
export type UserIdentityID = Principal;
export interface _SERVICE {
  'associate_identity_with_user' : ActorMethod<
    [AssociateIdentityWithUserInput],
    AssociateIdentityWithUserResult
  >,
  'delete_user' : ActorMethod<[], RemoveUserResult>,
  'get_main_bank' : ActorMethod<[], GetMainBankResult>,
  'get_user' : ActorMethod<[], GetUserResult>,
  'list_banks' : ActorMethod<[], ListBanksResult>,
  'manage_user' : ActorMethod<[ManageUserInput], ManageUserResult>,
  'register_user' : ActorMethod<[RegisterUserInput], RegisterUserResult>,
}
