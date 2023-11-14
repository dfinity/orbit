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
  const ApprovalThresholdPolicy = IDL.Variant({
    'VariableThreshold' : IDL.Nat8,
    'FixedThreshold' : IDL.Nat8,
  });
  const AccountPolicy = IDL.Variant({
    'approval_threshold' : ApprovalThresholdPolicy,
  });
  const CreateAccountInput = IDL.Record({
    'owners' : IDL.Vec(UserId),
    'metadata' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
    'name' : IDL.Opt(IDL.Text),
    'blockchain' : IDL.Text,
    'standard' : IDL.Text,
    'policies' : IDL.Vec(AccountPolicy),
  });
  const AccountId = IDL.Text;
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
    'name' : IDL.Opt(IDL.Text),
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'last_modification_timestamp' : TimestampRFC3339,
    'standard' : IDL.Text,
    'symbol' : AssetSymbol,
    'policies' : IDL.Vec(AccountPolicy),
  });
  const CreateAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
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
  const ProposalId = IDL.Text;
  const GetProposalInput = IDL.Record({ 'proposal_id' : ProposalId });
  const ProposalStatus = IDL.Variant({
    'Rejected' : IDL.Null,
    'Adopted' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const ProposalVoteStatus = IDL.Variant({
    'Rejected' : IDL.Null,
    'Adopted' : IDL.Null,
    'NotRequired' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const ProposalVote = IDL.Record({
    'status' : ProposalVoteStatus,
    'user_id' : UserId,
    'status_reason' : IDL.Opt(IDL.Text),
    'decided_at' : IDL.Opt(TimestampRFC3339),
  });
  const TransferId = IDL.Text;
  const TransferStatus = IDL.Variant({
    'Failed' : IDL.Record({ 'reason' : IDL.Text }),
    'Approved' : IDL.Null,
    'Rejected' : IDL.Record({ 'reason' : IDL.Text }),
    'Cancelled' : IDL.Record({ 'reason' : IDL.Opt(IDL.Text) }),
    'Submitted' : IDL.Null,
    'Processing' : IDL.Record({ 'started_at' : TimestampRFC3339 }),
    'Completed' : IDL.Record({
      'signature' : IDL.Opt(IDL.Text),
      'hash' : IDL.Opt(IDL.Text),
      'completed_at' : TimestampRFC3339,
    }),
    'Pending' : IDL.Null,
  });
  const TransferExecutionSchedule = IDL.Variant({
    'Immediate' : IDL.Null,
    'Scheduled' : IDL.Record({ 'execution_time' : TimestampRFC3339 }),
  });
  const TransferMetadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const NetworkId = IDL.Text;
  const Network = IDL.Record({ 'id' : NetworkId, 'name' : IDL.Text });
  const Transfer = IDL.Record({
    'id' : TransferId,
    'to' : IDL.Text,
    'fee' : IDL.Nat,
    'status' : TransferStatus,
    'from_account_id' : AccountId,
    'execution_plan' : TransferExecutionSchedule,
    'expiration_dt' : TimestampRFC3339,
    'metadata' : IDL.Vec(TransferMetadata),
    'network' : Network,
    'amount' : IDL.Nat,
  });
  const ProposalOperation = IDL.Variant({
    'Transfer' : IDL.Record({ 'account' : Account, 'transfer' : Transfer }),
  });
  const Proposal = IDL.Record({
    'id' : ProposalId,
    'status' : ProposalStatus,
    'votes' : IDL.Vec(ProposalVote),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'created_at' : TimestampRFC3339,
    'operation' : ProposalOperation,
    'proposed_by' : IDL.Opt(UserId),
  });
  const GetProposalResult = IDL.Variant({
    'Ok' : IDL.Record({ 'proposal' : Proposal }),
    'Err' : Error,
  });
  const GetTransferInput = IDL.Record({ 'transfer_id' : TransferId });
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
  const ProposalOperationType = IDL.Variant({ 'Transfer' : IDL.Null });
  const ListAccountProposalsInput = IDL.Record({
    'account_id' : AccountId,
    'status' : IDL.Opt(ProposalStatus),
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
  });
  const ListNotificationsInput = IDL.Record({
    'status' : IDL.Opt(NotificationStatus),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'from_dt' : IDL.Opt(TimestampRFC3339),
    'notification_type' : IDL.Opt(NotificationTypeInput),
  });
  const NotificationId = IDL.Text;
  const NotificationType = IDL.Variant({
    'ProposalCreated' : IDL.Record({ 'proposal_id' : ProposalId }),
    'SystemMessage' : IDL.Null,
    'TransferProposalCreated' : IDL.Record({
      'account_id' : AccountId,
      'transfer_id' : TransferId,
      'proposal_id' : ProposalId,
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
    'status' : IDL.Opt(ProposalStatus),
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
  const TransferInput = IDL.Record({
    'to' : IDL.Text,
    'fee' : IDL.Opt(IDL.Nat),
    'from_account_id' : AccountId,
    'execution_plan' : IDL.Opt(TransferExecutionSchedule),
    'expiration_dt' : IDL.Opt(TimestampRFC3339),
    'metadata' : IDL.Opt(IDL.Vec(TransferMetadata)),
    'network' : IDL.Opt(Network),
    'amount' : IDL.Nat,
  });
  const TransferResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfer' : Transfer }),
    'Err' : Error,
  });
  const VoteOnProposalInput = IDL.Record({
    'approve' : IDL.Opt(IDL.Bool),
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
    'create_account' : IDL.Func(
        [CreateAccountInput],
        [CreateAccountResult],
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
    'transfer' : IDL.Func([TransferInput], [TransferResult], []),
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
