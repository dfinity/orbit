export const idlFactory = ({ IDL }) => {
  const BankID = IDL.Principal;
  const DefaultBankInit = IDL.Variant({
    'InitSharedBankCanister' : IDL.Null,
    'SpecifiedBankCanister' : BankID,
  });
  const CanisterInit = IDL.Record({ 'default_bank' : DefaultBankInit });
  const UUID = IDL.Text;
  const AccountId = UUID;
  const AssociateIdentityWithAccountInput = IDL.Record({
    'account_id' : AccountId,
  });
  const AccountIdentityID = IDL.Principal;
  const AccountIdentity = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'identity' : AccountIdentityID,
  });
  const AccountBank = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'canister_id' : BankID,
  });
  const Account = IDL.Record({
    'id' : AccountId,
    'unconfirmed_identities' : IDL.Vec(AccountIdentity),
    'name' : IDL.Opt(IDL.Text),
    'main_bank' : IDL.Opt(BankID),
    'banks' : IDL.Vec(AccountBank),
    'identities' : IDL.Vec(AccountIdentity),
  });
  const ApiError = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const AssociateIdentityWithAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : ApiError,
  });
  const RemoveAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : ApiError,
  });
  const GetAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : ApiError,
  });
  const GetMainBankResult = IDL.Variant({
    'Ok' : IDL.Record({ 'bank' : IDL.Opt(AccountBank) }),
    'Err' : ApiError,
  });
  const ListBanksResult = IDL.Variant({
    'Ok' : IDL.Record({ 'banks' : IDL.Vec(AccountBank) }),
    'Err' : ApiError,
  });
  const ManageAccountInput = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'main_bank' : IDL.Opt(BankID),
    'banks' : IDL.Opt(IDL.Vec(AccountBank)),
    'identities' : IDL.Opt(IDL.Vec(AccountIdentity)),
  });
  const ManageAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : ApiError,
  });
  const RegisterAccountBankInput = IDL.Variant({
    'PrivateBank' : IDL.Record({
      'id' : BankID,
      'use_shared_bank' : IDL.Opt(IDL.Record({ 'is_main' : IDL.Bool })),
    }),
    'SharedBank' : IDL.Null,
  });
  const RegisterAccountInput = IDL.Record({
    'bank' : RegisterAccountBankInput,
    'name' : IDL.Opt(IDL.Text),
  });
  const RegisterAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : ApiError,
  });
  return IDL.Service({
    'associate_identity_with_account' : IDL.Func(
        [AssociateIdentityWithAccountInput],
        [AssociateIdentityWithAccountResult],
        [],
      ),
    'delete_account' : IDL.Func([], [RemoveAccountResult], []),
    'get_account' : IDL.Func([], [GetAccountResult], ['query']),
    'get_main_bank' : IDL.Func([], [GetMainBankResult], ['query']),
    'list_banks' : IDL.Func([], [ListBanksResult], ['query']),
    'manage_account' : IDL.Func(
        [ManageAccountInput],
        [ManageAccountResult],
        [],
      ),
    'register_account' : IDL.Func(
        [RegisterAccountInput],
        [RegisterAccountResult],
        [],
      ),
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
