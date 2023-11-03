export const idlFactory = ({ IDL }) => {
  const BankID = IDL.Principal;
  const DefaultBankInit = IDL.Variant({
    'InitSharedBankCanister' : IDL.Null,
    'SpecifiedBankCanister' : BankID,
  });
  const CanisterInit = IDL.Record({ 'default_bank' : DefaultBankInit });
  const UUID = IDL.Text;
  const UserId = UUID;
  const AssociateIdentityWithUserInput = IDL.Record({ 'user_id' : UserId });
  const UserIdentityID = IDL.Principal;
  const UserIdentity = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'identity' : UserIdentityID,
  });
  const UserBank = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'canister_id' : BankID,
  });
  const User = IDL.Record({
    'id' : UserId,
    'unconfirmed_identities' : IDL.Vec(UserIdentity),
    'name' : IDL.Opt(IDL.Text),
    'main_bank' : IDL.Opt(BankID),
    'banks' : IDL.Vec(UserBank),
    'identities' : IDL.Vec(UserIdentity),
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
  const GetMainBankResult = IDL.Variant({
    'Ok' : IDL.Record({ 'bank' : IDL.Opt(UserBank) }),
    'Err' : ApiError,
  });
  const GetUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const ListBanksResult = IDL.Variant({
    'Ok' : IDL.Record({ 'banks' : IDL.Vec(UserBank) }),
    'Err' : ApiError,
  });
  const ManageUserInput = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'main_bank' : IDL.Opt(BankID),
    'banks' : IDL.Opt(IDL.Vec(UserBank)),
    'identities' : IDL.Opt(IDL.Vec(UserIdentity)),
  });
  const ManageUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const RegisterUserBankInput = IDL.Variant({
    'PrivateBank' : IDL.Record({
      'id' : BankID,
      'use_shared_bank' : IDL.Opt(IDL.Record({ 'is_main' : IDL.Bool })),
    }),
    'SharedBank' : IDL.Null,
  });
  const RegisterUserInput = IDL.Record({
    'bank' : RegisterUserBankInput,
    'name' : IDL.Opt(IDL.Text),
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
    'get_main_bank' : IDL.Func([], [GetMainBankResult], ['query']),
    'get_user' : IDL.Func([], [GetUserResult], ['query']),
    'list_banks' : IDL.Func([], [ListBanksResult], ['query']),
    'manage_user' : IDL.Func([ManageUserInput], [ManageUserResult], []),
    'register_user' : IDL.Func([RegisterUserInput], [RegisterUserResult], []),
  });
};
export const init = ({ IDL }) => {
  const BankID = IDL.Principal;
  const DefaultBankInit = IDL.Variant({
    'InitSharedBankCanister' : IDL.Null,
    'SpecifiedBankCanister' : BankID,
  });
  const CanisterInit = IDL.Record({ 'default_bank' : DefaultBankInit });
  return [IDL.Opt(CanisterInit)];
};
