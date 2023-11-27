import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'id' : AccountId,
  'decimals' : number,
  'balance' : [] | [AccountBalanceInfo],
  'owners' : Array<UserId>,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'blockchain' : string,
  'address' : string,
  'last_modification_timestamp' : TimestampRFC3339,
  'standard' : string,
  'symbol' : AssetSymbol,
  'policies' : Array<Policy>,
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
export interface AddAccountOperation {
  'owners' : Array<UserId>,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'blockchain' : string,
  'account' : [] | [Account],
  'standard' : string,
  'policies' : Array<Policy>,
}
export interface AddAccountOperationInput {
  'owners' : Array<UserId>,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'blockchain' : string,
  'standard' : string,
  'policies' : Array<Policy>,
}
export type ApprovalThresholdPolicy = { 'VariableThreshold' : number } |
  { 'FixedThreshold' : number };
export type AssetSymbol = string;
export interface ConfirmUserIdentityInput { 'user_id' : UserId }
export type ConfirmUserIdentityResult = { 'Ok' : { 'user' : User } } |
  { 'Err' : Error };
export interface CreateProposalInput {
  'title' : [] | [string],
  'execution_plan' : [] | [ProposalExecutionSchedule],
  'summary' : [] | [string],
  'operation' : ProposalOperationInput,
}
export type CreateProposalResult = { 'Ok' : { 'proposal' : Proposal } } |
  { 'Err' : Error };
export type EditAccountOperation = EditAccountOperationInput;
export interface EditAccountOperationInput {
  'account_id' : AccountId,
  'owners' : [] | [Array<UserId>],
  'name' : [] | [string],
  'policies' : [] | [Array<Policy>],
}
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
export type GetFeaturesResult = { 'Ok' : { 'features' : WalletFeatures } } |
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
  'status' : [] | [Array<ProposalStatusCode>],
  'to_dt' : [] | [TimestampRFC3339],
  'operation_type' : [] | [ProposalOperationType],
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
export interface ListNotificationsInput {
  'status' : [] | [NotificationStatus],
  'to_dt' : [] | [TimestampRFC3339],
  'from_dt' : [] | [TimestampRFC3339],
  'notification_type' : [] | [NotificationTypeInput],
}
export type ListNotificationsResult = {
    'Ok' : { 'notifications' : Array<Notification> }
  } |
  { 'Err' : Error };
export interface ListProposalsInput {
  'status' : [] | [Array<ProposalStatusCode>],
  'to_dt' : [] | [TimestampRFC3339],
  'operation_type' : [] | [ProposalOperationType],
  'from_dt' : [] | [TimestampRFC3339],
}
export type ListProposalsResult = { 'Ok' : { 'proposals' : Array<Proposal> } } |
  { 'Err' : Error };
export type MarkNotificationReadResult = { 'Ok' : null } |
  { 'Err' : Error };
export interface MarkNotificationsReadInput {
  'notification_ids' : Array<NotificationId>,
  'read' : boolean,
}
export interface Network { 'id' : NetworkId, 'name' : string }
export type NetworkId = string;
export interface Notification {
  'id' : NotificationId,
  'status' : NotificationStatus,
  'title' : { 'locale_key' : string, 'body' : string },
  'created_at' : TimestampRFC3339,
  'notification_type' : NotificationType,
  'message' : { 'locale_key' : string, 'body' : string },
  'target_user_id' : UserId,
}
export type NotificationId = string;
export type NotificationStatus = { 'Read' : null } |
  { 'Sent' : null };
export type NotificationType = {
    'ProposalCreated' : { 'proposal_id' : UUID }
  } |
  { 'SystemMessage' : null } |
  {
    'TransferProposalCreated' : { 'account_id' : UUID, 'proposal_id' : UUID }
  } |
  { 'AccountProposalCreated' : { 'account_id' : UUID, 'proposal_id' : UUID } };
export type NotificationTypeInput = { 'ProposalCreated' : null } |
  { 'SystemMessage' : null } |
  { 'TransferProposalCreated' : null } |
  { 'AccountProposalCreated' : null };
export type Policy = { 'approval_threshold' : ApprovalThresholdPolicy };
export interface Proposal {
  'id' : ProposalId,
  'status' : ProposalStatus,
  'title' : string,
  'execution_plan' : ProposalExecutionSchedule,
  'expiration_dt' : TimestampRFC3339,
  'votes' : Array<ProposalVote>,
  'metadata' : Array<[string, string]>,
  'created_at' : TimestampRFC3339,
  'summary' : [] | [string],
  'operation' : ProposalOperation,
  'proposed_by' : [] | [UserId],
}
export type ProposalExecutionSchedule = { 'Immediate' : null } |
  { 'Scheduled' : { 'execution_time' : TimestampRFC3339 } };
export type ProposalId = string;
export type ProposalOperation = { 'Transfer' : TransferOperation } |
  { 'EditAccount' : EditAccountOperation } |
  { 'AddAccount' : AddAccountOperation };
export type ProposalOperationInput = { 'Transfer' : TransferOperationInput } |
  { 'EditAccount' : EditAccountOperationInput } |
  { 'AddAccount' : AddAccountOperationInput };
export type ProposalOperationType = { 'Transfer' : null } |
  { 'EditAccount' : null } |
  { 'AddAccount' : null };
export type ProposalStatus = { 'Failed' : { 'reason' : [] | [string] } } |
  { 'Rejected' : null } |
  { 'Scheduled' : { 'scheduled_at' : TimestampRFC3339 } } |
  { 'Adopted' : null } |
  { 'Cancelled' : { 'reason' : [] | [string] } } |
  { 'Processing' : { 'started_at' : TimestampRFC3339 } } |
  { 'Created' : null } |
  { 'Completed' : { 'completed_at' : TimestampRFC3339 } };
export type ProposalStatusCode = { 'Failed' : null } |
  { 'Rejected' : null } |
  { 'Scheduled' : null } |
  { 'Adopted' : null } |
  { 'Cancelled' : null } |
  { 'Processing' : null } |
  { 'Created' : null } |
  { 'Completed' : null };
export interface ProposalVote {
  'status' : ProposalVoteStatus,
  'user_id' : UserId,
  'status_reason' : [] | [string],
  'decided_at' : TimestampRFC3339,
}
export type ProposalVoteStatus = { 'Rejected' : null } |
  { 'Accepted' : null };
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
  'metadata' : Array<TransferMetadata>,
  'network' : Network,
  'amount' : bigint,
}
export type TransferId = string;
export interface TransferListItem {
  'to' : string,
  'status' : TransferStatus,
  'created_at' : TimestampRFC3339,
  'transfer_id' : TransferId,
  'amount' : bigint,
}
export interface TransferMetadata { 'key' : string, 'value' : string }
export interface TransferOperation {
  'to' : string,
  'fee' : [] | [bigint],
  'metadata' : Array<TransferMetadata>,
  'network' : Network,
  'from_account' : Account,
  'amount' : bigint,
}
export interface TransferOperationInput {
  'to' : string,
  'fee' : [] | [bigint],
  'from_account_id' : AccountId,
  'metadata' : [] | [Array<TransferMetadata>],
  'network' : [] | [Network],
  'amount' : bigint,
}
export type TransferStatus = { 'Failed' : { 'reason' : string } } |
  { 'Cancelled' : { 'reason' : [] | [string] } } |
  { 'Processing' : { 'started_at' : TimestampRFC3339 } } |
  { 'Created' : null } |
  {
    'Completed' : {
      'signature' : [] | [string],
      'hash' : [] | [string],
      'completed_at' : TimestampRFC3339,
    }
  };
export type UUID = string;
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
  'approve' : boolean,
  'proposal_id' : ProposalId,
  'reason' : [] | [string],
}
export type VoteOnProposalResult = { 'Ok' : { 'proposal' : Proposal } } |
  { 'Err' : Error };
export interface WalletAsset {
  'standards' : Array<string>,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'blockchain' : string,
  'symbol' : AssetSymbol,
}
export interface WalletFeatures { 'supported_assets' : Array<WalletAsset> }
export interface WalletInit {
  'permissions' : [] | [Array<WalletPermission>],
  'approval_threshold' : [] | [number],
  'owners' : [] | [Array<Principal>],
}
export interface WalletPermission {
  'access_roles' : Array<UserRole>,
  'permission_id' : string,
}
export interface WalletSettings {
  'permissions' : Array<WalletPermission>,
  'approval_threshold' : number,
  'owners' : Array<User>,
  'last_upgrade_timestamp' : TimestampRFC3339,
}
export type WalletSettingsResult = { 'Ok' : { 'settings' : WalletSettings } } |
  { 'Err' : Error };
export interface _SERVICE {
  'confirm_user_identity' : ActorMethod<
    [ConfirmUserIdentityInput],
    ConfirmUserIdentityResult
  >,
  'create_proposal' : ActorMethod<[CreateProposalInput], CreateProposalResult>,
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
  'list_notifications' : ActorMethod<
    [ListNotificationsInput],
    ListNotificationsResult
  >,
  'list_proposals' : ActorMethod<[ListProposalsInput], ListProposalsResult>,
  'mark_notifications_read' : ActorMethod<
    [MarkNotificationsReadInput],
    MarkNotificationReadResult
  >,
  'register_user' : ActorMethod<[RegisterUserInput], RegisterUserResult>,
  'vote_on_proposal' : ActorMethod<[VoteOnProposalInput], VoteOnProposalResult>,
  'wallet_settings' : ActorMethod<[], WalletSettingsResult>,
}
