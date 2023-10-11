export const idlFactory = ({ IDL }) => {
  const AccountRole = IDL.Variant({
    'Guest' : IDL.Null,
    'User' : IDL.Null,
    'Admin' : IDL.Null,
  });
  const BankPermission = IDL.Record({
    'access_roles' : IDL.Vec(AccountRole),
    'permission_id' : IDL.Text,
  });
  const ApprovalThresholdPolicy = IDL.Variant({
    'VariableThreshold' : IDL.Nat8,
    'FixedThreshold' : IDL.Nat8,
  });
  const WalletPolicy = IDL.Variant({
    'approval_threshold' : ApprovalThresholdPolicy,
  });
  const BankInit = IDL.Record({
    'permissions' : IDL.Opt(IDL.Vec(BankPermission)),
    'approval_threshold' : IDL.Opt(IDL.Nat8),
    'owners' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'wallet_policies' : IDL.Opt(IDL.Vec(WalletPolicy)),
  });
  const AccountId = IDL.Text;
  const TimestampRFC3339 = IDL.Text;
  const Account = IDL.Record({
    'id' : AccountId,
    'unconfirmed_identities' : IDL.Vec(IDL.Principal),
    'access_roles' : IDL.Vec(AccountRole),
    'last_modification_timestamp' : TimestampRFC3339,
    'identities' : IDL.Vec(IDL.Principal),
  });
  const BankSettings = IDL.Record({
    'permissions' : IDL.Vec(BankPermission),
    'approval_threshold' : IDL.Nat8,
    'owners' : IDL.Vec(Account),
    'last_upgrade_timestamp' : TimestampRFC3339,
    'wallet_policies' : IDL.Vec(WalletPolicy),
  });
  const Error = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const BankSettingsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'settings' : BankSettings }),
    'Err' : Error,
  });
  const ConfirmAccountInput = IDL.Record({ 'account_id' : AccountId });
  const ConfirmAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : Error,
  });
  const CreateWalletInput = IDL.Record({
    'owners' : IDL.Vec(
      IDL.Variant({ 'Principal' : IDL.Principal, 'AccountID' : AccountId })
    ),
    'metadata' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
    'name' : IDL.Opt(IDL.Text),
    'blockchain' : IDL.Text,
    'standard' : IDL.Text,
    'policies' : IDL.Vec(WalletPolicy),
  });
  const WalletId = IDL.Text;
  const WalletBalanceInfo = IDL.Record({
    'decimals' : IDL.Nat32,
    'balance' : IDL.Nat,
    'last_update_timestamp' : TimestampRFC3339,
  });
  const AssetSymbol = IDL.Text;
  const Wallet = IDL.Record({
    'id' : WalletId,
    'decimals' : IDL.Nat32,
    'balance' : IDL.Opt(WalletBalanceInfo),
    'owners' : IDL.Vec(AccountId),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Opt(IDL.Text),
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'last_modification_timestamp' : TimestampRFC3339,
    'standard' : IDL.Text,
    'symbol' : AssetSymbol,
    'policies' : IDL.Vec(WalletPolicy),
  });
  const CreateWalletResult = IDL.Variant({
    'Ok' : IDL.Record({ 'wallet' : Wallet }),
    'Err' : Error,
  });
  const EditAccountInput = IDL.Record({
    'account_id' : AccountId,
    'identities' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  const EditAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : Error,
  });
  const OperationId = IDL.Text;
  const EditOperationInput = IDL.Record({
    'read' : IDL.Opt(IDL.Bool),
    'approve' : IDL.Opt(IDL.Bool),
    'operation_id' : OperationId,
    'reason' : IDL.Opt(IDL.Text),
  });
  const OperationStatus = IDL.Variant({
    'Rejected' : IDL.Null,
    'Adopted' : IDL.Null,
    'NotRequired' : IDL.Null,
    'Pending' : IDL.Null,
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
    'execution_plan' : TransferExecutionSchedule,
    'expiration_dt' : TimestampRFC3339,
    'metadata' : IDL.Vec(TransferMetadata),
    'from_wallet_id' : WalletId,
    'network' : Network,
    'amount' : IDL.Nat,
  });
  const OperationContext = IDL.Record({
    'wallet' : IDL.Opt(Wallet),
    'transfer' : IDL.Opt(Transfer),
  });
  const OperationDecision = IDL.Record({
    'account_id' : AccountId,
    'status' : OperationStatus,
    'read' : IDL.Bool,
    'status_reason' : IDL.Opt(IDL.Text),
    'decided_at' : IDL.Opt(TimestampRFC3339),
  });
  const Operation = IDL.Record({
    'id' : OperationId,
    'status' : OperationStatus,
    'context' : OperationContext,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'code' : IDL.Text,
    'created_at' : TimestampRFC3339,
    'decisions' : IDL.Vec(OperationDecision),
    'originator_account_id' : IDL.Opt(AccountId),
  });
  const EditOperationResult = IDL.Variant({
    'Ok' : IDL.Record({ 'operation' : Operation }),
    'Err' : Error,
  });
  const BankAsset = IDL.Record({
    'decimals' : IDL.Nat32,
    'standards' : IDL.Vec(IDL.Text),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'symbol' : AssetSymbol,
  });
  const BankFeatures = IDL.Record({ 'supported_assets' : IDL.Vec(BankAsset) });
  const GetFeaturesResult = IDL.Variant({
    'Ok' : IDL.Record({ 'features' : BankFeatures }),
    'Err' : Error,
  });
  const FetchWalletBalancesInput = IDL.Record({
    'wallet_ids' : IDL.Vec(WalletId),
  });
  const WalletBalance = IDL.Record({
    'decimals' : IDL.Nat32,
    'balance' : IDL.Nat,
    'last_update_timestamp' : TimestampRFC3339,
    'wallet_id' : WalletId,
  });
  const FetchWalletBalancesResult = IDL.Variant({
    'Ok' : IDL.Record({ 'balances' : IDL.Vec(WalletBalance) }),
    'Err' : Error,
  });
  const GetAccountInput = IDL.Record({ 'account_id' : IDL.Opt(AccountId) });
  const GetAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : Error,
  });
  const GetOperationInput = IDL.Record({ 'operation_id' : OperationId });
  const GetOperationResult = IDL.Variant({
    'Ok' : IDL.Record({ 'operation' : Operation }),
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
  const GetWalletInput = IDL.Record({ 'wallet_id' : WalletId });
  const GetWalletResult = IDL.Variant({
    'Ok' : IDL.Record({ 'wallet' : Wallet }),
    'Err' : Error,
  });
  const ListOperationsInput = IDL.Record({
    'status' : IDL.Opt(OperationStatus),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'code' : IDL.Opt(IDL.Text),
    'read' : IDL.Opt(IDL.Bool),
    'from_dt' : IDL.Opt(TimestampRFC3339),
  });
  const ListOperationsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'operations' : IDL.Vec(Operation) }),
    'Err' : Error,
  });
  const ListWalletOperationsInput = IDL.Record({
    'status' : IDL.Opt(OperationStatus),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'code' : IDL.Opt(IDL.Text),
    'read' : IDL.Opt(IDL.Bool),
    'from_dt' : IDL.Opt(TimestampRFC3339),
    'wallet_id' : WalletId,
  });
  const ListWalletOperationsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'operations' : IDL.Vec(Operation) }),
    'Err' : Error,
  });
  const ListWalletTransfersInput = IDL.Record({
    'status' : IDL.Opt(IDL.Text),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'from_dt' : IDL.Opt(TimestampRFC3339),
    'wallet_id' : WalletId,
  });
  const TransferListItem = IDL.Record({
    'to' : IDL.Text,
    'status' : TransferStatus,
    'created_at' : TimestampRFC3339,
    'transfer_id' : TransferId,
    'amount' : IDL.Nat,
  });
  const ListWalletTransfersResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfers' : IDL.Vec(TransferListItem) }),
    'Err' : Error,
  });
  const WalletListItem = IDL.Record({
    'id' : WalletId,
    'decimals' : IDL.Nat32,
    'asset_name' : IDL.Opt(IDL.Text),
    'balance' : IDL.Opt(WalletBalanceInfo),
    'name' : IDL.Opt(IDL.Text),
    'address' : IDL.Text,
    'asset_symbol' : AssetSymbol,
    'nr_owners' : IDL.Nat8,
  });
  const ListWalletResult = IDL.Variant({
    'Ok' : IDL.Record({ 'wallets' : IDL.Vec(WalletListItem) }),
    'Err' : Error,
  });
  const RegisterAccountInput = IDL.Record({
    'identities' : IDL.Vec(IDL.Principal),
  });
  const RegisterAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : Error,
  });
  const TransferInput = IDL.Record({
    'to' : IDL.Text,
    'fee' : IDL.Opt(IDL.Nat),
    'execution_plan' : IDL.Opt(TransferExecutionSchedule),
    'expiration_dt' : IDL.Opt(TimestampRFC3339),
    'metadata' : IDL.Opt(IDL.Vec(TransferMetadata)),
    'from_wallet_id' : WalletId,
    'network' : IDL.Opt(Network),
    'amount' : IDL.Nat,
  });
  const TransferResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfer' : Transfer }),
    'Err' : Error,
  });
  return IDL.Service({
    'bank_settings' : IDL.Func([], [BankSettingsResult], ['query']),
    'confirm_account' : IDL.Func(
        [ConfirmAccountInput],
        [ConfirmAccountResult],
        [],
      ),
    'create_wallet' : IDL.Func([CreateWalletInput], [CreateWalletResult], []),
    'edit_account' : IDL.Func([EditAccountInput], [EditAccountResult], []),
    'edit_operation' : IDL.Func(
        [EditOperationInput],
        [EditOperationResult],
        [],
      ),
    'features' : IDL.Func([], [GetFeaturesResult], ['query']),
    'fetch_wallet_balances' : IDL.Func(
        [FetchWalletBalancesInput],
        [FetchWalletBalancesResult],
        [],
      ),
    'get_account' : IDL.Func([GetAccountInput], [GetAccountResult], ['query']),
    'get_operation' : IDL.Func(
        [GetOperationInput],
        [GetOperationResult],
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
    'get_wallet' : IDL.Func([GetWalletInput], [GetWalletResult], ['query']),
    'list_operations' : IDL.Func(
        [ListOperationsInput],
        [ListOperationsResult],
        ['query'],
      ),
    'list_wallet_operations' : IDL.Func(
        [ListWalletOperationsInput],
        [ListWalletOperationsResult],
        ['query'],
      ),
    'list_wallet_transfers' : IDL.Func(
        [ListWalletTransfersInput],
        [ListWalletTransfersResult],
        ['query'],
      ),
    'list_wallets' : IDL.Func([], [ListWalletResult], ['query']),
    'register_account' : IDL.Func(
        [RegisterAccountInput],
        [RegisterAccountResult],
        [],
      ),
    'transfer' : IDL.Func([TransferInput], [TransferResult], []),
  });
};
export const init = ({ IDL }) => {
  const AccountRole = IDL.Variant({
    'Guest' : IDL.Null,
    'User' : IDL.Null,
    'Admin' : IDL.Null,
  });
  const BankPermission = IDL.Record({
    'access_roles' : IDL.Vec(AccountRole),
    'permission_id' : IDL.Text,
  });
  const ApprovalThresholdPolicy = IDL.Variant({
    'VariableThreshold' : IDL.Nat8,
    'FixedThreshold' : IDL.Nat8,
  });
  const WalletPolicy = IDL.Variant({
    'approval_threshold' : ApprovalThresholdPolicy,
  });
  const BankInit = IDL.Record({
    'permissions' : IDL.Opt(IDL.Vec(BankPermission)),
    'approval_threshold' : IDL.Opt(IDL.Nat8),
    'owners' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'wallet_policies' : IDL.Opt(IDL.Vec(WalletPolicy)),
  });
  return [IDL.Opt(BankInit)];
};
