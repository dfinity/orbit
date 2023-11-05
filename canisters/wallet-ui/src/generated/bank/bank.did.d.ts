import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'id' : AccountId,
  'decimals' : number,
  'balance' : [] | [AccountBalanceInfo],
  'owners' : Array<UserId>,
  'metadata' : Array<[string, string]>,
  'name' : [] | [string],
  'blockchain' : string,
  'address' : string,
  'last_modification_timestamp' : TimestampRFC3339,
  'standard' : string,
  'symbol' : AssetSymbol,
  'policies' : Array<AccountPolicy>,
}
export interface AccountBalance {
  'account_id' : AccountId,
  'decimals' : number,
  'balance' : bigint,
  'last_update_timestamp' : TimestampRFC3339,
}
export interface AccountBalanceInfo {
  'decimals' : number,
  'balance' : bigint,
  'last_update_timestamp' : TimestampRFC3339,
}
export type AccountId = string;
export type AccountPolicy = { 'approval_threshold' : ApprovalThresholdPolicy };
export type ApprovalThresholdPolicy = { 'VariableThreshold' : number } |
  { 'FixedThreshold' : number };
export type AssetSymbol = string;
export interface BankAsset {
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
}
export interface BankPermission {
  'access_roles' : Array<UserRole>,
  'permission_id' : string,
}
export interface BankSettings {
  'permissions' : Array<BankPermission>,
  'approval_threshold' : number,
  'owners' : Array<User>,
  'last_upgrade_timestamp' : TimestampRFC3339,
}
export type BankSettingsResult = { 'Ok' : { 'settings' : BankSettings } } |
  { 'Err' : Error };
export interface ConfirmUserIdentityInput { 'user_id' : UserId }
export type ConfirmUserIdentityResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : Error };
export interface CreateAccountInput {
  'owners' : Array<UserId>,
  'metadata' : [] | [Array<[string, string]>],
  'name' : [] | [string],
  'blockchain' : string,
  'standard' : string,
  'policies' : Array<AccountPolicy>,
}
export type CreateAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : Error };
export interface EditUserInput {
  'user_id' : UserId,
  'identities' : [] | [Array<Principal>],
}
export type EditUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : Error };
export interface Error {
  'code' : string,
  'message' : [] | [string],
  'details' : [] | [Array<[string, string]>],
}
export interface FetchAccountBalancesInput { 'account_ids' : Array<AccountId> }
export type FetchAccountBalancesResult = {
    'Ok' : { 'balances' : Array<AccountBalance> }
  } |
  { 'Err' : Error };
export interface GetAccountInput { 'account_id' : AccountId }
export type GetAccountResult = { 'Ok' : { 'account' : Account } } |
  { 'Err' : Error };
export type GetFeaturesResult = { 'Ok' : { 'features' : BankFeatures } } |
  { 'Err' : Error };
export interface GetProposalInput { 'proposal_id' : ProposalId }
export type GetProposalResult = { 'Ok' : { 'proposal' : Proposal } } |
  { 'Err' : Error };
export interface GetTransferInput { 'transfer_id' : TransferId }
export type GetTransferResult = { 'Ok' : { 'transfer' : Transfer } } |
  { 'Err' : Error };
export interface GetTransfersInput { 'transfer_ids' : Array<TransferId> }
export type GetTransfersResult = { 'Ok' : { 'transfers' : Array<Transfer> } } |
  { 'Err' : Error };
export interface GetUserInput { 'user_id' : [] | [UserId] }
export type GetUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : Error };
export interface ListAccountProposalsInput {
  'account_id' : AccountId,
  'status' : [] | [ProposalStatus],
  'to_dt' : [] | [TimestampRFC3339],
  'operation_type' : [] | [ProposalOperationType],
  'read' : [] | [boolean],
  'from_dt' : [] | [TimestampRFC3339],
}
export type ListAccountProposalsResult = {
    'Ok' : { 'proposals' : Array<Proposal> }
  } |
  { 'Err' : Error };
export type ListAccountResult = { 'Ok' : { 'accounts' : Array<Account> } } |
  { 'Err' : Error };
export interface ListAccountTransfersInput {
  'account_id' : AccountId,
  'status' : [] | [string],
  'to_dt' : [] | [TimestampRFC3339],
  'from_dt' : [] | [TimestampRFC3339],
}
export type ListAccountTransfersResult = {
    'Ok' : { 'transfers' : Array<TransferListItem> }
  } |
  { 'Err' : Error };
export interface ListProposalsInput {
  'status' : [] | [ProposalStatus],
  'to_dt' : [] | [TimestampRFC3339],
  'operation_type' : [] | [ProposalOperationType],
  'read' : [] | [boolean],
  'from_dt' : [] | [TimestampRFC3339],
}
export type ListProposalsResult = { 'Ok' : { 'proposals' : Array<Proposal> } } |
  { 'Err' : Error };
export interface Network { 'id' : NetworkId, 'name' : string }
export type NetworkId = string;
export interface Proposal {
  'id' : ProposalId,
  'status' : ProposalStatus,
  'votes' : Array<ProposalVote>,
  'metadata' : Array<[string, string]>,
  'created_at' : TimestampRFC3339,
  'operation' : ProposalOperation,
  'proposed_by' : [] | [UserId],
}
export type ProposalId = string;
export type ProposalOperation = {
    'Transfer' : { 'account' : Account, 'transfer' : Transfer }
  };
export type ProposalOperationType = { 'Transfer' : null };
export type ProposalStatus = { 'Rejected' : null } |
  { 'Adopted' : null } |
  { 'Pending' : null };
export interface ProposalVote {
  'status' : ProposalVoteStatus,
  'read' : boolean,
  'user_id' : UserId,
  'status_reason' : [] | [string],
  'decided_at' : [] | [TimestampRFC3339],
}
export type ProposalVoteStatus = { 'Rejected' : null } |
  { 'Adopted' : null } |
  { 'NotRequired' : null } |
  { 'Pending' : null };
export interface RegisterUserInput { 'identities' : Array<Principal> }
export type RegisterUserResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : Error };
export type TimestampRFC3339 = string;
export interface Transfer {
  'id' : TransferId,
  'to' : string,
  'fee' : bigint,
  'status' : TransferStatus,
  'from_account_id' : AccountId,
  'execution_plan' : TransferExecutionSchedule,
  'expiration_dt' : TimestampRFC3339,
  'metadata' : Array<TransferMetadata>,
  'network' : Network,
  'amount' : bigint,
}
export type TransferExecutionSchedule = { 'Immediate' : null } |
  { 'Scheduled' : { 'execution_time' : TimestampRFC3339 } };
export type TransferId = string;
export interface TransferInput {
  'to' : string,
  'fee' : [] | [bigint],
  'from_account_id' : AccountId,
  'execution_plan' : [] | [TransferExecutionSchedule],
  'expiration_dt' : [] | [TimestampRFC3339],
  'metadata' : [] | [Array<TransferMetadata>],
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
export type TransferStatus = { 'Failed' : { 'reason' : string } } |
  { 'Approved' : null } |
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
export interface User {
  'id' : UserId,
  'unconfirmed_identities' : Array<Principal>,
  'access_roles' : Array<UserRole>,
  'last_modification_timestamp' : TimestampRFC3339,
  'identities' : Array<Principal>,
}
export type UserId = string;
export type UserRole = { 'Guest' : null } |
  { 'User' : null } |
  { 'Admin' : null };
export interface VoteOnProposalInput {
  'read' : [] | [boolean],
  'approve' : [] | [boolean],
  'proposal_id' : ProposalId,
  'reason' : [] | [string],
}
export type VoteOnProposalResult = { 'Ok' : { 'proposal' : Proposal } } |
  { 'Err' : Error };
export interface _SERVICE {
  'bank_settings' : ActorMethod<[], BankSettingsResult>,
  'confirm_user_identity' : ActorMethod<
    [ConfirmUserIdentityInput],
    ConfirmUserIdentityResult
  >,
  'create_account' : ActorMethod<[CreateAccountInput], CreateAccountResult>,
  'edit_user' : ActorMethod<[EditUserInput], EditUserResult>,
  'features' : ActorMethod<[], GetFeaturesResult>,
  'fetch_account_balances' : ActorMethod<
    [FetchAccountBalancesInput],
    FetchAccountBalancesResult
  >,
  'get_account' : ActorMethod<[GetAccountInput], GetAccountResult>,
  'get_proposal' : ActorMethod<[GetProposalInput], GetProposalResult>,
  'get_transfer' : ActorMethod<[GetTransferInput], GetTransferResult>,
  'get_transfers' : ActorMethod<[GetTransfersInput], GetTransfersResult>,
  'get_user' : ActorMethod<[GetUserInput], GetUserResult>,
  'list_account_proposals' : ActorMethod<
    [ListAccountProposalsInput],
    ListAccountProposalsResult
  >,
  'list_account_transfers' : ActorMethod<
    [ListAccountTransfersInput],
    ListAccountTransfersResult
  >,
  'list_accounts' : ActorMethod<[], ListAccountResult>,
  'list_proposals' : ActorMethod<[ListProposalsInput], ListProposalsResult>,
  'register_user' : ActorMethod<[RegisterUserInput], RegisterUserResult>,
  'transfer' : ActorMethod<[TransferInput], TransferResult>,
  'vote_on_proposal' : ActorMethod<[VoteOnProposalInput], VoteOnProposalResult>,
}
