import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'id' : AccountId,
  'unconfirmed_identities' : Array<Principal>,
  'access_roles' : Array<AccountRole>,
  'last_modification_timestamp' : TimestampRFC3339,
  'identities' : Array<Principal>,
}
export type AccountId = string;
export type AccountRole = { 'Guest' : null } |
  { 'User' : null } |
  { 'Admin' : null };
export type ApprovalThresholdPolicy = { 'VariableThreshold' : number } |
  { 'FixedThreshold' : number };
export type AssetSymbol = string;
export interface BankAsset {
  'decimals' : number,
  'standards' : Array<string>,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'blockchain' : string,
  'symbol' : AssetSymbol,
}
export interface BankFeatures { 'supported_assets' : Array<BankAsset> }
export interface BankInit {
  'permissions' : [] | [Array<BankPermission>],
  'approval_threshold' : [] | [number],
  'owners' : [] | [Array<Principal>],
  'wallet_policies' : [] | [Array<WalletPolicy>],
}
export interface BankPermission {
  'access_roles' : Array<AccountRole>,
  'permission_id' : string,
}
export interface BankSettings {
  'permissions' : Array<BankPermission>,
  'approval_threshold' : number,
  'owners' : Array<Account>,
  'last_upgrade_timestamp' : TimestampRFC3339,
  'wallet_policies' : Array<WalletPolicy>,
}
export type BankSettingsResult = { 'Ok' : { 'settings' : BankSettings } } |
  { 'Err' : Error };
export interface ConfirmAccountInput { 'account_id' : AccountId }
export type ConfirmAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : Error };
export interface CreateWalletInput {
  'owners' : Array<{ 'Principal' : Principal } | { 'AccountID' : AccountId }>,
  'metadata' : [] | [Array<[string, string]>],
  'name' : [] | [string],
  'blockchain' : string,
  'standard' : string,
  'policies' : Array<WalletPolicy>,
}
export type CreateWalletResult = { 'Ok' : { 'wallet' : Wallet } } |
  { 'Err' : Error };
export interface EditAccountInput {
  'account_id' : AccountId,
  'identities' : [] | [Array<Principal>],
}
export type EditAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : Error };
export interface EditOperationInput {
  'read' : [] | [boolean],
  'approve' : [] | [boolean],
  'operation_id' : OperationId,
  'reason' : [] | [string],
}
export type EditOperationResult = { 'Ok' : { 'operation' : Operation } } |
  { 'Err' : Error };
export interface Error {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export interface GetAccountInput { 'account_id' : [] | [AccountId] }
export type GetAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : Error };
export type GetFeaturesResult = { 'Ok' : { 'features' : BankFeatures } } |
  { 'Err' : Error };
export interface GetOperationInput { 'operation_id' : OperationId }
export type GetOperationResult = { 'Ok' : { 'operation' : Operation } } |
  { 'Err' : Error };
export interface GetTransferInput { 'transfer_id' : TransferId }
export type GetTransferResult = { 'Ok' : { 'transfer' : Transfer } } |
  { 'Err' : Error };
export interface GetWalletBalanceInput { 'wallet_id' : WalletId }
export type GetWalletBalanceResult = { 'Ok' : { 'balance' : WalletBalance } } |
  { 'Err' : Error };
export interface GetWalletInput { 'wallet_id' : WalletId }
export type GetWalletResult = { 'Ok' : { 'wallet' : Wallet } } |
  { 'Err' : Error };
export interface ListOperationsInput {
  'status' : [] | [OperationStatus],
  'code' : [] | [string],
  'read' : [] | [boolean],
}
export type ListOperationsResult = {
    'Ok' : { 'operations' : Array<Operation> }
  } |
  { 'Err' : Error };
export interface ListWalletOperationsInput {
  'status' : [] | [OperationStatus],
  'code' : [] | [string],
  'read' : [] | [boolean],
  'wallet_id' : WalletId,
}
export type ListWalletOperationsResult = {
    'Ok' : { 'operations' : Array<Operation> }
  } |
  { 'Err' : Error };
export type ListWalletResult = {
    'Ok' : { 'wallets' : Array<WalletListItem> }
  } |
  { 'Err' : Error };
export interface ListWalletTransfersInput {
  'status' : [] | [string],
  'to_dt' : [] | [TimestampRFC3339],
  'from_dt' : [] | [TimestampRFC3339],
  'wallet_id' : WalletId,
}
export type ListWalletTransfersResult = {
    'Ok' : { 'transfers' : Array<TransferListItem> }
  } |
  { 'Err' : Error };
export interface Network { 'id' : NetworkId, 'name' : string }
export type NetworkId = string;
export interface Operation {
  'id' : OperationId,
  'status' : OperationStatus,
  'metadata' : Array<[string, string]>,
  'code' : string,
  'read' : boolean,
  'created_at' : TimestampRFC3339,
  'feedback_reason' : [] | [string],
  'account' : AccountId,
  'feedback_time_at' : [] | [TimestampRFC3339],
}
export type OperationId = string;
export type OperationStatus = { 'Rejected' : null } |
  { 'Abstained' : null } |
  { 'Adopted' : null } |
  { 'Pending' : null };
export interface RegisterAccountInput { 'identities' : Array<Principal> }
export type RegisterAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : Error };
export type TimestampRFC3339 = string;
export interface Transfer {
  'id' : TransferId,
  'to' : string,
  'fee' : bigint,
  'status' : TransferStatus,
  'execution_plan' : TransferExecutionSchedule,
  'expiration_dt' : TimestampRFC3339,
  'metadata' : Array<TransferMetadata>,
  'from_wallet_id' : WalletId,
  'network' : Network,
  'amount' : bigint,
}
export type TransferExecutionSchedule = { 'Immediate' : null } |
  { 'Scheduled' : { 'execution_time' : TimestampRFC3339 } };
export type TransferId = string;
export interface TransferInput {
  'to' : string,
  'fee' : [] | [bigint],
  'execution_plan' : [] | [TransferExecutionSchedule],
  'expiration_dt' : [] | [TimestampRFC3339],
  'metadata' : [] | [Array<TransferMetadata>],
  'from_wallet_id' : WalletId,
  'network' : [] | [Network],
  'amount' : bigint,
}
export interface TransferListItem {
  'to' : string,
  'status' : TransferStatus,
  'created_at' : TimestampRFC3339,
  'transfer_id' : TransferId,
  'amount' : bigint,
}
export interface TransferMetadata { 'key' : string, 'value' : string }
export type TransferResult = { 'Ok' : { 'transfer' : Transfer } } |
  { 'Err' : Error };
export type TransferStatus = { 'Approved' : null } |
  { 'Rejected' : { 'reason' : string } } |
  { 'Cancelled' : { 'reason' : [] | [string] } } |
  { 'Submitted' : null } |
  { 'Processing' : { 'started_at' : TimestampRFC3339 } } |
  {
    'Completed' : {
      'signature' : [] | [string],
      'hash' : [] | [string],
      'completed_at' : TimestampRFC3339,
    }
  } |
  { 'Pending' : null };
export interface Wallet {
  'id' : WalletId,
  'decimals' : number,
  'balance' : [] | [WalletBalanceInfo],
  'owners' : Array<AccountId>,
  'metadata' : Array<[string, string]>,
  'name' : [] | [string],
  'blockchain' : string,
  'address' : string,
  'last_modification_timestamp' : TimestampRFC3339,
  'standard' : string,
  'symbol' : AssetSymbol,
  'policies' : Array<WalletPolicy>,
}
export interface WalletBalance {
  'decimals' : number,
  'balance' : bigint,
  'last_update_timestamp' : TimestampRFC3339,
  'wallet_id' : WalletId,
}
export interface WalletBalanceInfo {
  'decimals' : number,
  'balance' : bigint,
  'last_update_timestamp' : TimestampRFC3339,
}
export type WalletId = string;
export interface WalletListItem {
  'id' : WalletId,
  'decimals' : number,
  'asset_name' : [] | [string],
  'balance' : [] | [WalletBalanceInfo],
  'name' : [] | [string],
  'address' : string,
  'asset_symbol' : AssetSymbol,
  'nr_owners' : number,
}
export type WalletPolicy = { 'approval_threshold' : ApprovalThresholdPolicy };
export interface _SERVICE {
  'bank_settings' : ActorMethod<[], BankSettingsResult>,
  'confirm_account' : ActorMethod<[ConfirmAccountInput], ConfirmAccountResult>,
  'create_wallet' : ActorMethod<[CreateWalletInput], CreateWalletResult>,
  'edit_account' : ActorMethod<[EditAccountInput], EditAccountResult>,
  'edit_operation' : ActorMethod<[EditOperationInput], EditOperationResult>,
  'features' : ActorMethod<[], GetFeaturesResult>,
  'get_account' : ActorMethod<[GetAccountInput], GetAccountResult>,
  'get_operation' : ActorMethod<[GetOperationInput], GetOperationResult>,
  'get_transfer' : ActorMethod<[GetTransferInput], GetTransferResult>,
  'get_wallet' : ActorMethod<[GetWalletInput], GetWalletResult>,
  'get_wallet_balance' : ActorMethod<
    [GetWalletBalanceInput],
    GetWalletBalanceResult
  >,
  'list_operations' : ActorMethod<[ListOperationsInput], ListOperationsResult>,
  'list_wallet_operations' : ActorMethod<
    [ListWalletOperationsInput],
    ListWalletOperationsResult
  >,
  'list_wallet_transfers' : ActorMethod<
    [ListWalletTransfersInput],
    ListWalletTransfersResult
  >,
  'list_wallets' : ActorMethod<[], ListWalletResult>,
  'register_account' : ActorMethod<
    [RegisterAccountInput],
    RegisterAccountResult
  >,
  'transfer' : ActorMethod<[TransferInput], TransferResult>,
}
