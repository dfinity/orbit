use super::AccountId;
use ic_canister_macros::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalOperationFilterType {
    Transfer(Option<AccountId>),
    AddAccount,
    EditAccount,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    ChangeCanister,
    EditAccessPolicy,
    AddProposalPolicy,
    EditProposalPolicy,
    RemoveProposalPolicy,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
}
