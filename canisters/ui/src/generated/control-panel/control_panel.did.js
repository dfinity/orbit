export const idlFactory = ({ IDL }) => {
  const WalletID = IDL.Principal;
  const DefaultWalletInit = IDL.Variant({
    'SpecifiedWalletCanister' : WalletID,
    'InitSharedWalletCanister' : IDL.Null,
  });
  const CanisterInit = IDL.Record({ 'default_wallet' : DefaultWalletInit });
  const UUID = IDL.Text;
  const UserId = UUID;
  const AssociateIdentityWithUserInput = IDL.Record({ 'user_id' : UserId });
  const UserIdentityID = IDL.Principal;
  const UserIdentity = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'identity' : UserIdentityID,
  });
  const UserWallet = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'canister_id' : WalletID,
  });
  const User = IDL.Record({
    'id' : UserId,
    'unconfirmed_identities' : IDL.Vec(UserIdentity),
    'name' : IDL.Opt(IDL.Text),
    'wallets' : IDL.Vec(UserWallet),
    'identities' : IDL.Vec(UserIdentity),
    'main_wallet' : IDL.Opt(WalletID),
  });
  const ApiError = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const AssociateIdentityWithUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const RemoveUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
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
  const ListWalletsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'wallets' : IDL.Vec(UserWallet) }),
    'Err' : ApiError,
  });
  const ManageUserInput = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'wallets' : IDL.Opt(IDL.Vec(UserWallet)),
    'identities' : IDL.Opt(IDL.Vec(UserIdentity)),
    'main_wallet' : IDL.Opt(WalletID),
  });
  const ManageUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const RegisterUserWalletInput = IDL.Variant({
    'SharedWallet' : IDL.Null,
    'PrivateWallet' : IDL.Record({
      'id' : WalletID,
      'use_shared_wallet' : IDL.Opt(IDL.Record({ 'is_main' : IDL.Bool })),
    }),
  });
  const RegisterUserInput = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'wallet' : RegisterUserWalletInput,
  });
  const RegisterUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  return IDL.Service({
    'associate_identity_with_user' : IDL.Func(
        [AssociateIdentityWithUserInput],
        [AssociateIdentityWithUserResult],
        [],
      ),
    'delete_user' : IDL.Func([], [RemoveUserResult], []),
    'get_main_wallet' : IDL.Func([], [GetMainWalletResult], ['query']),
    'get_user' : IDL.Func([], [GetUserResult], ['query']),
    'list_wallets' : IDL.Func([], [ListWalletsResult], ['query']),
    'manage_user' : IDL.Func([ManageUserInput], [ManageUserResult], []),
    'register_user' : IDL.Func([RegisterUserInput], [RegisterUserResult], []),
  });
};
export const init = ({ IDL }) => {
  const WalletID = IDL.Principal;
  const DefaultWalletInit = IDL.Variant({
    'SpecifiedWalletCanister' : WalletID,
    'InitSharedWalletCanister' : IDL.Null,
  });
  const CanisterInit = IDL.Record({ 'default_wallet' : DefaultWalletInit });
  return [IDL.Opt(CanisterInit)];
};
