import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'id' : UUID,
  'unconfirmed_identities' : Array<AccountIdentityID>,
  'name' : [] | [string],
  'main_bank' : [] | [BankID],
  'banks' : Array<BankID>,
  'identities' : Array<AccountIdentityID>,
}
export interface AccountBank { 'name' : [] | [string], 'canister_id' : BankID }
export interface AccountDetails {
  'id' : UUID,
  'unconfirmed_identities' : Array<AccountIdentityID>,
  'name' : [] | [string],
  'main_bank' : [] | [BankID],
  'banks' : Array<AccountBank>,
  'identities' : Array<AccountIdentity>,
}
export type AccountDetailsResult = {
    'Ok' : { 'account_details' : [] | [AccountDetails] }
  } |
  { 'Err' : Error };
export interface AccountIdentity {
  'name' : [] | [string],
  'identity' : AccountIdentityID,
}
export type AccountIdentityID = Principal;
export interface AssociateIdentityWithAccountInput { 'account_id' : UUID }
export type AssociateIdentityWithAccountResult = {
    'Ok' : { 'account' : Account }
  } |
  { 'Err' : Error };
export type BankID = Principal;
export type BankListItem = AccountBank;
export interface CanisterInit { 'default_bank' : DefaultBankInit }
export type DefaultBankInit = { 'InitSharedBankCanister' : null } |
  { 'SpecifiedBankCanister' : BankID };
export interface Error {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export type GetMainBankResult = { 'Ok' : { 'bank' : [] | [AccountBank] } } |
  { 'Err' : Error };
export type ListBanksResult = { 'Ok' : { 'banks' : Array<BankListItem> } } |
  { 'Err' : Error };
export interface ManageAccountInput {
  'unconfirmed_identities' : [] | [Array<AccountIdentityID>],
  'name' : [] | [string],
  'main_bank' : [] | [BankID],
  'banks' : [] | [Array<AccountBank>],
  'identities' : [] | [Array<AccountIdentity>],
}
export type ManageAccountResult = {
    'Ok' : { 'account_details' : AccountDetails }
  } |
  { 'Err' : Error };
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
  { 'Err' : Error };
export type RemoveAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : Error };
export type UUID = string;
export interface _SERVICE {
  'account_details' : ActorMethod<[], AccountDetailsResult>,
  'associate_identity_with_account' : ActorMethod<
    [AssociateIdentityWithAccountInput],
    AssociateIdentityWithAccountResult
  >,
  'delete_account' : ActorMethod<[], RemoveAccountResult>,
  'get_main_bank' : ActorMethod<[], GetMainBankResult>,
  'list_banks' : ActorMethod<[], ListBanksResult>,
  'manage_account' : ActorMethod<[ManageAccountInput], ManageAccountResult>,
  'register_account' : ActorMethod<
    [RegisterAccountInput],
    RegisterAccountResult
  >,
}
