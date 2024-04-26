use super::AccountId;
use orbit_essentials::storable;

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
    EditPermission,
    AddProposalPolicy,
    EditProposalPolicy,
    RemoveProposalPolicy,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
}
