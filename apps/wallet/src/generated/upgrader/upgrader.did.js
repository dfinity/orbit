export const idlFactory = ({ IDL }) => {
  const InitArg = IDL.Record({ 'target_canister' : IDL.Principal });
  const Metadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const Account = IDL.Record({
    'id' : IDL.Text,
    'decimals' : IDL.Nat32,
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'standard' : IDL.Text,
    'symbol' : IDL.Text,
  });
  const GetDisasterRecoveryAccountsResponse = IDL.Record({
    'accounts' : IDL.Vec(Account),
  });
  const Error = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const GetDisasterRecoveryAccountsResult = IDL.Variant({
    'Ok' : GetDisasterRecoveryAccountsResponse,
    'Err' : Error,
  });
  const Asset = IDL.Record({
    'id' : IDL.Text,
    'decimals' : IDL.Nat32,
    'standards' : IDL.Vec(IDL.Text),
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'symbol' : IDL.Text,
  });
  const MultiAssetAccount = IDL.Record({
    'id' : IDL.Text,
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'assets' : IDL.Vec(IDL.Text),
    'seed' : IDL.Vec(IDL.Nat8),
  });
  const GetDisasterRecoveryAccountsAndAssetsResponse = IDL.Record({
    'assets' : IDL.Vec(Asset),
    'accounts' : IDL.Vec(MultiAssetAccount),
  });
  const GetDisasterRecoveryAccountsAndAssetsResult = IDL.Variant({
    'Ok' : GetDisasterRecoveryAccountsAndAssetsResponse,
    'Err' : Error,
  });
  const AdminUser = IDL.Record({
    'id' : IDL.Text,
    'name' : IDL.Text,
    'identities' : IDL.Vec(IDL.Principal),
  });
  const DisasterRecoveryCommittee = IDL.Record({
    'users' : IDL.Vec(AdminUser),
    'quorum' : IDL.Nat16,
  });
  const GetDisasterRecoveryCommitteeResponse = IDL.Record({
    'committee' : IDL.Opt(DisasterRecoveryCommittee),
  });
  const GetDisasterRecoveryCommitteeResult = IDL.Variant({
    'Ok' : GetDisasterRecoveryCommitteeResponse,
    'Err' : Error,
  });
  const InstallMode = IDL.Variant({
    'Upgrade' : IDL.Null,
    'Install' : IDL.Null,
    'Reinstall' : IDL.Null,
  });
  const StationRecoveryRequestInstallCodeOperation = IDL.Record({
    'arg' : IDL.Vec(IDL.Nat8),
    'wasm_sha256' : IDL.Vec(IDL.Nat8),
    'install_mode' : InstallMode,
  });
  const StationRecoveryRequestOperation = IDL.Variant({
    'InstallCode' : StationRecoveryRequestInstallCodeOperation,
  });
  const StationRecoveryRequest = IDL.Record({
    'user_id' : IDL.Text,
    'operation' : StationRecoveryRequestOperation,
    'submitted_at' : IDL.Text,
  });
  const RecoveryStatus = IDL.Variant({
    'Idle' : IDL.Null,
    'InProgress' : IDL.Record({ 'since' : IDL.Text }),
  });
  const RecoveryFailure = IDL.Record({ 'reason' : IDL.Text });
  const RecoveryResult = IDL.Variant({
    'Success' : IDL.Null,
    'Failure' : RecoveryFailure,
  });
  const GetDisasterRecoveryStateResponse = IDL.Record({
    'recovery_requests' : IDL.Vec(StationRecoveryRequest),
    'assets' : IDL.Vec(Asset),
    'recovery_status' : RecoveryStatus,
    'committee' : IDL.Opt(DisasterRecoveryCommittee),
    'multi_asset_accounts' : IDL.Vec(MultiAssetAccount),
    'accounts' : IDL.Vec(Account),
    'last_recovery_result' : IDL.Opt(RecoveryResult),
  });
  const GetDisasterRecoveryStateResult = IDL.Variant({
    'Ok' : GetDisasterRecoveryStateResponse,
    'Err' : Error,
  });
  const GetLogsInputPagination = IDL.Record({
    'offset' : IDL.Opt(IDL.Nat64),
    'limit' : IDL.Opt(IDL.Nat64),
  });
  const GetLogsInput = IDL.Record({
    'pagination' : IDL.Opt(GetLogsInputPagination),
  });
  const LogEntry = IDL.Record({
    'time' : IDL.Text,
    'entry_type' : IDL.Text,
    'message' : IDL.Text,
    'data_json' : IDL.Text,
  });
  const GetLogsResponse = IDL.Record({
    'total' : IDL.Nat64,
    'logs' : IDL.Vec(LogEntry),
    'next_offset' : IDL.Opt(IDL.Nat64),
  });
  const GetLogsResult = IDL.Variant({ 'Ok' : GetLogsResponse, 'Err' : Error });
  const IsCommitteeMemberResponse = IDL.Record({
    'is_committee_member' : IDL.Bool,
  });
  const IsCommitteeMemberResult = IDL.Variant({
    'Ok' : IsCommitteeMemberResponse,
    'Err' : Error,
  });
  const WasmModuleExtraChunks = IDL.Record({
    'wasm_module_hash' : IDL.Vec(IDL.Nat8),
    'store_canister' : IDL.Principal,
    'extra_chunks_key' : IDL.Text,
  });
  const RequestDisasterRecoveryInstallCodeInput = IDL.Record({
    'arg' : IDL.Vec(IDL.Nat8),
    'module_extra_chunks' : IDL.Opt(WasmModuleExtraChunks),
    'install_mode' : InstallMode,
    'module' : IDL.Vec(IDL.Nat8),
  });
  const RequestDisasterRecoveryInput = IDL.Variant({
    'InstallCode' : RequestDisasterRecoveryInstallCodeInput,
  });
  const RequestDisasterRecoveryResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : Error,
  });
  const SetDisasterRecoveryAccountsInput = IDL.Record({
    'accounts' : IDL.Vec(Account),
  });
  const SetDisasterRecoveryResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : Error,
  });
  const SetDisasterRecoveryAccountsAndAssetsInput = IDL.Record({
    'assets' : IDL.Vec(Asset),
    'accounts' : IDL.Vec(MultiAssetAccount),
  });
  const SetDisasterRecoveryCommitteeInput = IDL.Record({
    'committee' : DisasterRecoveryCommittee,
  });
  const UpgradeParams = IDL.Record({
    'arg' : IDL.Vec(IDL.Nat8),
    'module_extra_chunks' : IDL.Opt(WasmModuleExtraChunks),
    'module' : IDL.Vec(IDL.Nat8),
  });
  const TriggerUpgradeError = IDL.Variant({
    'NotController' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'UnexpectedError' : IDL.Text,
  });
  const TriggerUpgradeResponse = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : TriggerUpgradeError,
  });
  return IDL.Service({
    'get_disaster_recovery_accounts' : IDL.Func(
        [],
        [GetDisasterRecoveryAccountsResult],
        ['query'],
      ),
    'get_disaster_recovery_accounts_and_assets' : IDL.Func(
        [],
        [GetDisasterRecoveryAccountsAndAssetsResult],
        ['query'],
      ),
    'get_disaster_recovery_committee' : IDL.Func(
        [],
        [GetDisasterRecoveryCommitteeResult],
        ['query'],
      ),
    'get_disaster_recovery_state' : IDL.Func(
        [],
        [GetDisasterRecoveryStateResult],
        ['query'],
      ),
    'get_logs' : IDL.Func([GetLogsInput], [GetLogsResult], ['query']),
    'is_committee_member' : IDL.Func([], [IsCommitteeMemberResult], ['query']),
    'request_disaster_recovery' : IDL.Func(
        [RequestDisasterRecoveryInput],
        [RequestDisasterRecoveryResult],
        [],
      ),
    'set_disaster_recovery_accounts' : IDL.Func(
        [SetDisasterRecoveryAccountsInput],
        [SetDisasterRecoveryResult],
        [],
      ),
    'set_disaster_recovery_accounts_and_assets' : IDL.Func(
        [SetDisasterRecoveryAccountsAndAssetsInput],
        [SetDisasterRecoveryResult],
        [],
      ),
    'set_disaster_recovery_committee' : IDL.Func(
        [SetDisasterRecoveryCommitteeInput],
        [SetDisasterRecoveryResult],
        [],
      ),
    'trigger_upgrade' : IDL.Func([UpgradeParams], [TriggerUpgradeResponse], []),
  });
};
export const init = ({ IDL }) => {
  const InitArg = IDL.Record({ 'target_canister' : IDL.Principal });
  return [InitArg];
};
