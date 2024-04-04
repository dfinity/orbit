export const idlFactory = ({ IDL }) => {
  const CanisterUpgrade = IDL.Record({
    'upgrader_wasm_module' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'wallet_wasm_module' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const CanisterInit = IDL.Record({
    'upgrader_wasm_module' : IDL.Vec(IDL.Nat8),
    'wallet_wasm_module' : IDL.Vec(IDL.Nat8),
  });
  const CanisterInstall = IDL.Variant({
    'Upgrade' : CanisterUpgrade,
    'Init' : CanisterInit,
  });
  const WalletID = IDL.Principal;
  const UserWallet = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'canister_id' : WalletID,
  });
  const User = IDL.Record({
    'id' : IDL.Principal,
    'wallets' : IDL.Vec(UserWallet),
    'main_wallet' : IDL.Opt(WalletID),
  });
  const ApiError = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const RemoveUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const DeployWalletResult = IDL.Variant({
    'Ok' : IDL.Record({ 'canister_id' : WalletID }),
    'Err' : ApiError,
  });
  const GetMainWalletResult = IDL.Variant({
    'Ok' : IDL.Record({ 'wallet' : IDL.Opt(UserWallet) }),
    'Err' : ApiError,
  });
  const GetUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'status_code' : IDL.Nat16,
  });
  const ListWalletsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'wallets' : IDL.Vec(UserWallet) }),
    'Err' : ApiError,
  });
  const ManageUserInput = IDL.Record({
    'wallets' : IDL.Opt(IDL.Vec(UserWallet)),
    'main_wallet' : IDL.Opt(WalletID),
  });
  const ManageUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const RegisterUserInput = IDL.Record({
    'wallet_id' : IDL.Opt(IDL.Principal),
  });
  const RegisterUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const SubscribeToWaitingListResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : ApiError,
  });
  const UserSubscriptionStatus = IDL.Variant({
    'Unsubscribed' : IDL.Null,
    'Approved' : IDL.Null,
    'Denylisted' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const UpdateWaitingListInput = IDL.Record({
    'users' : IDL.Vec(IDL.Principal),
    'new_status' : UserSubscriptionStatus,
  });
  const UpdateWaitingListResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : ApiError,
  });
  return IDL.Service({
    'delete_user' : IDL.Func([], [RemoveUserResult], []),
    'deploy_wallet' : IDL.Func([], [DeployWalletResult], []),
    'get_main_wallet' : IDL.Func([], [GetMainWalletResult], ['query']),
    'get_user' : IDL.Func([], [GetUserResult], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'list_wallets' : IDL.Func([], [ListWalletsResult], ['query']),
    'manage_user' : IDL.Func([ManageUserInput], [ManageUserResult], []),
    'register_user' : IDL.Func([RegisterUserInput], [RegisterUserResult], []),
    'subscribe_to_waiting_list' : IDL.Func(
        [IDL.Text],
        [SubscribeToWaitingListResult],
        [],
      ),
    'update_waiting_list' : IDL.Func(
        [UpdateWaitingListInput],
        [UpdateWaitingListResult],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const CanisterUpgrade = IDL.Record({
    'upgrader_wasm_module' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'wallet_wasm_module' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const CanisterInit = IDL.Record({
    'upgrader_wasm_module' : IDL.Vec(IDL.Nat8),
    'wallet_wasm_module' : IDL.Vec(IDL.Nat8),
  });
  const CanisterInstall = IDL.Variant({
    'Upgrade' : CanisterUpgrade,
    'Init' : CanisterInit,
  });
  return [IDL.Opt(CanisterInstall)];
};
