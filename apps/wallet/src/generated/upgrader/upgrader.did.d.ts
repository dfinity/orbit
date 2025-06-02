import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'id' : string,
  'decimals' : number,
  'metadata' : Array<Metadata>,
  'name' : string,
  'blockchain' : string,
  'address' : string,
  'standard' : string,
  'symbol' : string,
}
export interface AdminUser {
  'id' : string,
  'name' : string,
  'identities' : Array<Principal>,
}
export interface Asset {
  'id' : string,
  'decimals' : number,
  'standards' : Array<string>,
  'metadata' : Array<Metadata>,
  'name' : string,
  'blockchain' : string,
  'symbol' : string,
}
export interface DisasterRecoveryCommittee {
  'users' : Array<AdminUser>,
  'quorum' : number,
}
export interface Error {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export interface GetDisasterRecoveryAccountsAndAssetsResponse {
  'assets' : Array<Asset>,
  'accounts' : Array<MultiAssetAccount>,
}
export type GetDisasterRecoveryAccountsAndAssetsResult = {
    'Ok' : GetDisasterRecoveryAccountsAndAssetsResponse
  } |
  { 'Err' : Error };
export interface GetDisasterRecoveryAccountsResponse {
  'accounts' : Array<Account>,
}
export type GetDisasterRecoveryAccountsResult = {
    'Ok' : GetDisasterRecoveryAccountsResponse
  } |
  { 'Err' : Error };
export interface GetDisasterRecoveryCommitteeResponse {
  'committee' : [] | [DisasterRecoveryCommittee],
}
export type GetDisasterRecoveryCommitteeResult = {
    'Ok' : GetDisasterRecoveryCommitteeResponse
  } |
  { 'Err' : Error };
export interface GetDisasterRecoveryStateResponse {
  'recovery_requests' : Array<StationRecoveryRequest>,
  'assets' : Array<Asset>,
  'recovery_status' : RecoveryStatus,
  'committee' : [] | [DisasterRecoveryCommittee],
  'multi_asset_accounts' : Array<MultiAssetAccount>,
  'accounts' : Array<Account>,
  'last_recovery_result' : [] | [RecoveryResult],
}
export type GetDisasterRecoveryStateResult = {
    'Ok' : GetDisasterRecoveryStateResponse
  } |
  { 'Err' : Error };
export interface GetLogsInput { 'pagination' : [] | [GetLogsInputPagination] }
export interface GetLogsInputPagination {
  'offset' : [] | [bigint],
  'limit' : [] | [bigint],
}
export interface GetLogsResponse {
  'total' : bigint,
  'logs' : Array<LogEntry>,
  'next_offset' : [] | [bigint],
}
export type GetLogsResult = { 'Ok' : GetLogsResponse } |
  { 'Err' : Error };
export interface InitArg { 'target_canister' : Principal }
export type InstallMode = { 'Upgrade' : null } |
  { 'Install' : null } |
  { 'Reinstall' : null };
export interface IsCommitteeMemberResponse { 'is_committee_member' : boolean }
export type IsCommitteeMemberResult = { 'Ok' : IsCommitteeMemberResponse } |
  { 'Err' : Error };
export interface LogEntry {
  'time' : string,
  'entry_type' : string,
  'message' : string,
  'data_json' : string,
}
export interface Metadata { 'key' : string, 'value' : string }
export interface MultiAssetAccount {
  'id' : string,
  'metadata' : Array<Metadata>,
  'name' : string,
  'assets' : Array<string>,
  'seed' : Uint8Array | number[],
}
export interface RecoveryFailure { 'reason' : string }
export type RecoveryResult = { 'Success' : null } |
  { 'Failure' : RecoveryFailure };
export type RecoveryStatus = { 'Idle' : null } |
  { 'InProgress' : { 'since' : string } };
export type RequestDisasterRecoveryInput = {
    'InstallCode' : RequestDisasterRecoveryInstallCodeInput
  };
export interface RequestDisasterRecoveryInstallCodeInput {
  'arg' : Uint8Array | number[],
  'module_extra_chunks' : [] | [WasmModuleExtraChunks],
  'install_mode' : InstallMode,
  'module' : Uint8Array | number[],
}
export type RequestDisasterRecoveryResult = { 'Ok' : null } |
  { 'Err' : Error };
export interface SetDisasterRecoveryAccountsAndAssetsInput {
  'assets' : Array<Asset>,
  'accounts' : Array<MultiAssetAccount>,
}
export interface SetDisasterRecoveryAccountsInput {
  'accounts' : Array<Account>,
}
export interface SetDisasterRecoveryCommitteeInput {
  'committee' : DisasterRecoveryCommittee,
}
export type SetDisasterRecoveryResult = { 'Ok' : null } |
  { 'Err' : Error };
export interface StationRecoveryRequest {
  'user_id' : string,
  'operation' : StationRecoveryRequestOperation,
  'submitted_at' : string,
}
export interface StationRecoveryRequestInstallCodeOperation {
  'arg' : Uint8Array | number[],
  'wasm_sha256' : Uint8Array | number[],
  'install_mode' : InstallMode,
}
export type StationRecoveryRequestOperation = {
    'InstallCode' : StationRecoveryRequestInstallCodeOperation
  };
export type TriggerUpgradeError = { 'NotController' : null } |
  { 'Unauthorized' : null } |
  { 'UnexpectedError' : string };
export type TriggerUpgradeResponse = { 'Ok' : null } |
  { 'Err' : TriggerUpgradeError };
export interface UpgradeParams {
  'arg' : Uint8Array | number[],
  'module_extra_chunks' : [] | [WasmModuleExtraChunks],
  'module' : Uint8Array | number[],
}
export interface WasmModuleExtraChunks {
  'wasm_module_hash' : Uint8Array | number[],
  'store_canister' : Principal,
  'extra_chunks_key' : string,
}
export interface _SERVICE {
  'get_disaster_recovery_accounts' : ActorMethod<
    [],
    GetDisasterRecoveryAccountsResult
  >,
  'get_disaster_recovery_accounts_and_assets' : ActorMethod<
    [],
    GetDisasterRecoveryAccountsAndAssetsResult
  >,
  'get_disaster_recovery_committee' : ActorMethod<
    [],
    GetDisasterRecoveryCommitteeResult
  >,
  'get_disaster_recovery_state' : ActorMethod<
    [],
    GetDisasterRecoveryStateResult
  >,
  'get_logs' : ActorMethod<[GetLogsInput], GetLogsResult>,
  'is_committee_member' : ActorMethod<[], IsCommitteeMemberResult>,
  'request_disaster_recovery' : ActorMethod<
    [RequestDisasterRecoveryInput],
    RequestDisasterRecoveryResult
  >,
  'set_disaster_recovery_accounts' : ActorMethod<
    [SetDisasterRecoveryAccountsInput],
    SetDisasterRecoveryResult
  >,
  'set_disaster_recovery_accounts_and_assets' : ActorMethod<
    [SetDisasterRecoveryAccountsAndAssetsInput],
    SetDisasterRecoveryResult
  >,
  'set_disaster_recovery_committee' : ActorMethod<
    [SetDisasterRecoveryCommitteeInput],
    SetDisasterRecoveryResult
  >,
  'trigger_upgrade' : ActorMethod<[UpgradeParams], TriggerUpgradeResponse>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
