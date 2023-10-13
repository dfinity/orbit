import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'id' : AccountId,
  'unconfirmed_identities' : Array<AccountIdentity>,
  'name' : [] | [string],
  'main_bank' : [] | [BankID],
  'banks' : Array<AccountBank>,
  'identities' : Array<AccountIdentity>,
}
export interface AccountBank { 'name' : [] | [string], 'canister_id' : BankID }
export type AccountId = UUID;
export interface AccountIdentity {
  'name' : [] | [string],
  'identity' : AccountIdentityID,
}
export type AccountIdentityID = Principal;
export interface ApiError {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export interface AssociateIdentityWithAccountInput { 'account_id' : AccountId }
export type AssociateIdentityWithAccountResult = {
    'Ok' : { 'account' : Account }
  } |
  { 'Err' : ApiError };
export type BankID = Principal;
export interface CanisterInit { 'default_bank' : DefaultBankInit }
export type DefaultBankInit = { 'InitSharedBankCanister' : null } |
  { 'SpecifiedBankCanister' : BankID };
export type GetAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : ApiError };
export type GetMainBankResult = { 'Ok' : { 'bank' : [] | [AccountBank] } } |
  { 'Err' : ApiError };
export type ListBanksResult = { 'Ok' : { 'banks' : Array<AccountBank> } } |
  { 'Err' : ApiError };
export interface ManageAccountInput {
  'name' : [] | [string],
  'main_bank' : [] | [BankID],
  'banks' : [] | [Array<AccountBank>],
  'identities' : [] | [Array<AccountIdentity>],
}
export type ManageAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : ApiError };
export type RegisterAccountBankInput = {
    'PrivateBank' : {
      'id' : BankID,
      'use_shared_bank' : [] | [{ 'is_main' : boolean }],
    }
  } |
  { 'SharedBank' : null };
export interface RegisterAccountInput {
  'bank' : RegisterAccountBankInput,
  'name' : [] | [string],
}
export type RegisterAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : ApiError };
export type RemoveAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : ApiError };
export type UUID = string;
export interface _SERVICE {
  'associate_identity_with_account' : ActorMethod<
    [AssociateIdentityWithAccountInput],
    AssociateIdentityWithAccountResult
  >,
  'delete_account' : ActorMethod<[], RemoveAccountResult>,
  'get_account' : ActorMethod<[], GetAccountResult>,
  'get_main_bank' : ActorMethod<[], GetMainBankResult>,
  'list_banks' : ActorMethod<[], ListBanksResult>,
  'manage_account' : ActorMethod<[ManageAccountInput], ManageAccountResult>,
  'register_account' : ActorMethod<
    [RegisterAccountInput],
    RegisterAccountResult
  >,
}
