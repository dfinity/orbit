export const idlFactory = ({ IDL }) => {
  const UserRole = IDL.Variant({
    'Guest' : IDL.Null,
    'User' : IDL.Null,
    'Admin' : IDL.Null,
  });
  const WalletPermission = IDL.Record({
    'access_roles' : IDL.Vec(UserRole),
    'permission_id' : IDL.Text,
  });
  const WalletInit = IDL.Record({
    'permissions' : IDL.Opt(IDL.Vec(WalletPermission)),
    'approval_threshold' : IDL.Opt(IDL.Nat8),
    'owners' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  const UserId = IDL.Text;
  const ConfirmUserIdentityInput = IDL.Record({ 'user_id' : UserId });
  const TimestampRFC3339 = IDL.Text;
  const User = IDL.Record({
    'id' : UserId,
    'unconfirmed_identities' : IDL.Vec(IDL.Principal),
    'access_roles' : IDL.Vec(UserRole),
    'last_modification_timestamp' : TimestampRFC3339,
    'identities' : IDL.Vec(IDL.Principal),
  });
  const Error = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const ConfirmUserIdentityResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : Error,
  });
  const ProposalExecutionSchedule = IDL.Variant({
    'Immediate' : IDL.Null,
    'Scheduled' : IDL.Record({ 'execution_time' : TimestampRFC3339 }),
  });
  const AccountId = IDL.Text;
  const TransferMetadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const NetworkId = IDL.Text;
  const Network = IDL.Record({ 'id' : NetworkId, 'name' : IDL.Text });
  const TransferOperationInput = IDL.Record({
    'to' : IDL.Text,
    'fee' : IDL.Opt(IDL.Nat),
    'from_account_id' : AccountId,
    'metadata' : IDL.Opt(IDL.Vec(TransferMetadata)),
    'network' : IDL.Opt(Network),
    'amount' : IDL.Nat,
  });
  const ApprovalThresholdPolicy = IDL.Variant({
    'VariableThreshold' : IDL.Nat8,
    'FixedThreshold' : IDL.Nat8,
  });
  const Policy = IDL.Variant({
    'approval_threshold' : ApprovalThresholdPolicy,
  });
  const EditAccountOperationInput = IDL.Record({
    'account_id' : AccountId,
    'owners' : IDL.Opt(IDL.Vec(UserId)),
    'name' : IDL.Opt(IDL.Text),
    'policies' : IDL.Opt(IDL.Vec(Policy)),
  });
  const AddAccountOperationInput = IDL.Record({
    'owners' : IDL.Vec(UserId),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'standard' : IDL.Text,
    'policies' : IDL.Vec(Policy),
  });
  const ProposalOperationInput = IDL.Variant({
    'Transfer' : TransferOperationInput,
    'EditAccount' : EditAccountOperationInput,
    'AddAccount' : AddAccountOperationInput,
  });
  const CreateProposalInput = IDL.Record({
    'title' : IDL.Opt(IDL.Text),
    'execution_plan' : IDL.Opt(ProposalExecutionSchedule),
    'summary' : IDL.Opt(IDL.Text),
    'operation' : ProposalOperationInput,
  });
  const ProposalId = IDL.Text;
  const ProposalStatus = IDL.Variant({
    'Failed' : IDL.Record({ 'reason' : IDL.Opt(IDL.Text) }),
    'Rejected' : IDL.Null,
    'Scheduled' : IDL.Record({ 'scheduled_at' : TimestampRFC3339 }),
    'Adopted' : IDL.Null,
    'Cancelled' : IDL.Record({ 'reason' : IDL.Opt(IDL.Text) }),
    'Processing' : IDL.Record({ 'started_at' : TimestampRFC3339 }),
    'Created' : IDL.Null,
    'Completed' : IDL.Record({ 'completed_at' : TimestampRFC3339 }),
  });
  const ProposalVoteStatus = IDL.Variant({
    'Rejected' : IDL.Null,
    'Accepted' : IDL.Null,
  });
  const ProposalVote = IDL.Record({
    'status' : ProposalVoteStatus,
    'user_id' : UserId,
    'status_reason' : IDL.Opt(IDL.Text),
    'decided_at' : TimestampRFC3339,
  });
  const AccountBalanceInfo = IDL.Record({
    'decimals' : IDL.Nat32,
    'balance' : IDL.Nat,
    'last_update_timestamp' : TimestampRFC3339,
  });
  const AssetSymbol = IDL.Text;
  const Account = IDL.Record({
    'id' : AccountId,
    'decimals' : IDL.Nat32,
    'balance' : IDL.Opt(AccountBalanceInfo),
    'owners' : IDL.Vec(UserId),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'last_modification_timestamp' : TimestampRFC3339,
    'standard' : IDL.Text,
    'symbol' : AssetSymbol,
    'policies' : IDL.Vec(Policy),
  });
  const TransferOperation = IDL.Record({
    'to' : IDL.Text,
    'fee' : IDL.Opt(IDL.Nat),
    'metadata' : IDL.Vec(TransferMetadata),
    'network' : Network,
    'from_account' : Account,
    'amount' : IDL.Nat,
  });
  const EditAccountOperation = EditAccountOperationInput;
  const AddAccountOperation = IDL.Record({
    'owners' : IDL.Vec(UserId),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'account' : IDL.Opt(Account),
    'standard' : IDL.Text,
    'policies' : IDL.Vec(Policy),
  });
  const ProposalOperation = IDL.Variant({
    'Transfer' : TransferOperation,
    'EditAccount' : EditAccountOperation,
    'AddAccount' : AddAccountOperation,
  });
  const Proposal = IDL.Record({
    'id' : ProposalId,
    'status' : ProposalStatus,
    'title' : IDL.Text,
    'execution_plan' : ProposalExecutionSchedule,
    'expiration_dt' : TimestampRFC3339,
    'votes' : IDL.Vec(ProposalVote),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'created_at' : TimestampRFC3339,
    'summary' : IDL.Opt(IDL.Text),
    'operation' : ProposalOperation,
    'proposed_by' : IDL.Opt(UserId),
  });
  const CreateProposalResult = IDL.Variant({
    'Ok' : IDL.Record({ 'proposal' : Proposal }),
    'Err' : Error,
  });
  const EditUserInput = IDL.Record({
    'user_id' : UserId,
    'identities' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  const EditUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : Error,
  });
  const WalletAsset = IDL.Record({
    'standards' : IDL.Vec(IDL.Text),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'symbol' : AssetSymbol,
  });
  const WalletFeatures = IDL.Record({
    'supported_assets' : IDL.Vec(WalletAsset),
  });
  const GetFeaturesResult = IDL.Variant({
    'Ok' : IDL.Record({ 'features' : WalletFeatures }),
    'Err' : Error,
  });
  const FetchAccountBalancesInput = IDL.Record({
    'account_ids' : IDL.Vec(AccountId),
  });
  const AccountBalance = IDL.Record({
    'account_id' : AccountId,
    'decimals' : IDL.Nat32,
    'balance' : IDL.Nat,
    'last_update_timestamp' : TimestampRFC3339,
  });
  const FetchAccountBalancesResult = IDL.Variant({
    'Ok' : IDL.Record({ 'balances' : IDL.Vec(AccountBalance) }),
    'Err' : Error,
  });
  const GetAccountInput = IDL.Record({ 'account_id' : AccountId });
  const GetAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : Error,
  });
  const GetProposalInput = IDL.Record({ 'proposal_id' : ProposalId });
  const GetProposalResult = IDL.Variant({
    'Ok' : IDL.Record({ 'proposal' : Proposal }),
    'Err' : Error,
  });
  const TransferId = IDL.Text;
  const GetTransferInput = IDL.Record({ 'transfer_id' : TransferId });
  const TransferStatus = IDL.Variant({
    'Failed' : IDL.Record({ 'reason' : IDL.Text }),
    'Cancelled' : IDL.Record({ 'reason' : IDL.Opt(IDL.Text) }),
    'Processing' : IDL.Record({ 'started_at' : TimestampRFC3339 }),
    'Created' : IDL.Null,
    'Completed' : IDL.Record({
      'signature' : IDL.Opt(IDL.Text),
      'hash' : IDL.Opt(IDL.Text),
      'completed_at' : TimestampRFC3339,
    }),
  });
  const Transfer = IDL.Record({
    'id' : TransferId,
    'to' : IDL.Text,
    'fee' : IDL.Nat,
    'status' : TransferStatus,
    'from_account_id' : AccountId,
    'metadata' : IDL.Vec(TransferMetadata),
    'network' : Network,
    'amount' : IDL.Nat,
  });
  const GetTransferResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfer' : Transfer }),
    'Err' : Error,
  });
  const GetTransfersInput = IDL.Record({
    'transfer_ids' : IDL.Vec(TransferId),
  });
  const GetTransfersResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfers' : IDL.Vec(Transfer) }),
    'Err' : Error,
  });
  const GetUserInput = IDL.Record({ 'user_id' : IDL.Opt(UserId) });
  const GetUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : Error,
  });
  const ProposalStatusCode = IDL.Variant({
    'Failed' : IDL.Null,
    'Rejected' : IDL.Null,
    'Scheduled' : IDL.Null,
    'Adopted' : IDL.Null,
    'Cancelled' : IDL.Null,
    'Processing' : IDL.Null,
    'Created' : IDL.Null,
    'Completed' : IDL.Null,
  });
  const ProposalOperationType = IDL.Variant({
    'Transfer' : IDL.Null,
    'EditAccount' : IDL.Null,
    'AddAccount' : IDL.Null,
  });
  const ListAccountProposalsInput = IDL.Record({
    'account_id' : AccountId,
    'status' : IDL.Opt(IDL.Vec(ProposalStatusCode)),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'operation_type' : IDL.Opt(ProposalOperationType),
    'from_dt' : IDL.Opt(TimestampRFC3339),
  });
  const ListAccountProposalsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'proposals' : IDL.Vec(Proposal) }),
    'Err' : Error,
  });
  const ListAccountTransfersInput = IDL.Record({
    'account_id' : AccountId,
    'status' : IDL.Opt(IDL.Text),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'from_dt' : IDL.Opt(TimestampRFC3339),
  });
  const TransferListItem = IDL.Record({
    'to' : IDL.Text,
    'status' : TransferStatus,
    'created_at' : TimestampRFC3339,
    'transfer_id' : TransferId,
    'amount' : IDL.Nat,
  });
  const ListAccountTransfersResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfers' : IDL.Vec(TransferListItem) }),
    'Err' : Error,
  });
  const ListAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'accounts' : IDL.Vec(Account) }),
    'Err' : Error,
  });
  const NotificationStatus = IDL.Variant({
    'Read' : IDL.Null,
    'Sent' : IDL.Null,
  });
  const NotificationTypeInput = IDL.Variant({
    'ProposalCreated' : IDL.Null,
    'SystemMessage' : IDL.Null,
    'TransferProposalCreated' : IDL.Null,
    'AccountProposalCreated' : IDL.Null,
  });
  const ListNotificationsInput = IDL.Record({
    'status' : IDL.Opt(NotificationStatus),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'from_dt' : IDL.Opt(TimestampRFC3339),
    'notification_type' : IDL.Opt(NotificationTypeInput),
  });
  const NotificationId = IDL.Text;
  const UUID = IDL.Text;
  const NotificationType = IDL.Variant({
    'ProposalCreated' : IDL.Record({ 'proposal_id' : UUID }),
    'SystemMessage' : IDL.Null,
    'TransferProposalCreated' : IDL.Record({
      'account_id' : UUID,
      'proposal_id' : UUID,
    }),
    'AccountProposalCreated' : IDL.Record({
      'account_id' : UUID,
      'proposal_id' : UUID,
    }),
  });
  const Notification = IDL.Record({
    'id' : NotificationId,
    'status' : NotificationStatus,
    'title' : IDL.Record({ 'locale_key' : IDL.Text, 'body' : IDL.Text }),
    'created_at' : TimestampRFC3339,
    'notification_type' : NotificationType,
    'message' : IDL.Record({ 'locale_key' : IDL.Text, 'body' : IDL.Text }),
    'target_user_id' : UserId,
  });
  const ListNotificationsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'notifications' : IDL.Vec(Notification) }),
    'Err' : Error,
  });
  const ListProposalsInput = IDL.Record({
    'status' : IDL.Opt(IDL.Vec(ProposalStatusCode)),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'operation_type' : IDL.Opt(ProposalOperationType),
    'from_dt' : IDL.Opt(TimestampRFC3339),
  });
  const ListProposalsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'proposals' : IDL.Vec(Proposal) }),
    'Err' : Error,
  });
  const MarkNotificationsReadInput = IDL.Record({
    'notification_ids' : IDL.Vec(NotificationId),
    'read' : IDL.Bool,
  });
  const MarkNotificationReadResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : Error,
  });
  const RegisterUserInput = IDL.Record({
    'identities' : IDL.Vec(IDL.Principal),
  });
  const RegisterUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : Error,
  });
  const VoteOnProposalInput = IDL.Record({
    'approve' : IDL.Bool,
    'proposal_id' : ProposalId,
    'reason' : IDL.Opt(IDL.Text),
  });
  const VoteOnProposalResult = IDL.Variant({
    'Ok' : IDL.Record({ 'proposal' : Proposal }),
    'Err' : Error,
  });
  const WalletSettings = IDL.Record({
    'permissions' : IDL.Vec(WalletPermission),
    'approval_threshold' : IDL.Nat8,
    'owners' : IDL.Vec(User),
    'last_upgrade_timestamp' : TimestampRFC3339,
  });
  const WalletSettingsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'settings' : WalletSettings }),
    'Err' : Error,
  });
  return IDL.Service({
    'confirm_user_identity' : IDL.Func(
        [ConfirmUserIdentityInput],
        [ConfirmUserIdentityResult],
        [],
      ),
    'create_proposal' : IDL.Func(
        [CreateProposalInput],
        [CreateProposalResult],
        [],
      ),
    'edit_user' : IDL.Func([EditUserInput], [EditUserResult], []),
    'features' : IDL.Func([], [GetFeaturesResult], ['query']),
    'fetch_account_balances' : IDL.Func(
        [FetchAccountBalancesInput],
        [FetchAccountBalancesResult],
        [],
      ),
    'get_account' : IDL.Func([GetAccountInput], [GetAccountResult], ['query']),
    'get_proposal' : IDL.Func(
        [GetProposalInput],
        [GetProposalResult],
        ['query'],
      ),
    'get_transfer' : IDL.Func(
        [GetTransferInput],
        [GetTransferResult],
        ['query'],
      ),
    'get_transfers' : IDL.Func(
        [GetTransfersInput],
        [GetTransfersResult],
        ['query'],
      ),
    'get_user' : IDL.Func([GetUserInput], [GetUserResult], ['query']),
    'list_account_proposals' : IDL.Func(
        [ListAccountProposalsInput],
        [ListAccountProposalsResult],
        ['query'],
      ),
    'list_account_transfers' : IDL.Func(
        [ListAccountTransfersInput],
        [ListAccountTransfersResult],
        ['query'],
      ),
    'list_accounts' : IDL.Func([], [ListAccountResult], ['query']),
    'list_notifications' : IDL.Func(
        [ListNotificationsInput],
        [ListNotificationsResult],
        ['query'],
      ),
    'list_proposals' : IDL.Func(
        [ListProposalsInput],
        [ListProposalsResult],
        ['query'],
      ),
    'mark_notifications_read' : IDL.Func(
        [MarkNotificationsReadInput],
        [MarkNotificationReadResult],
        [],
      ),
    'register_user' : IDL.Func([RegisterUserInput], [RegisterUserResult], []),
    'vote_on_proposal' : IDL.Func(
        [VoteOnProposalInput],
        [VoteOnProposalResult],
        [],
      ),
    'wallet_settings' : IDL.Func([], [WalletSettingsResult], ['query']),
  });
};
export const init = ({ IDL }) => {
  const UserRole = IDL.Variant({
    'Guest' : IDL.Null,
    'User' : IDL.Null,
    'Admin' : IDL.Null,
  });
  const WalletPermission = IDL.Record({
    'access_roles' : IDL.Vec(UserRole),
    'permission_id' : IDL.Text,
  });
  const WalletInit = IDL.Record({
    'permissions' : IDL.Opt(IDL.Vec(WalletPermission)),
    'approval_threshold' : IDL.Opt(IDL.Nat8),
    'owners' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  return [IDL.Opt(WalletInit)];
};
