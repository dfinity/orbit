use orbit_essentials::storable;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalOperationType {
    Transfer = 0,
    AddAccount = 1,
    EditAccount = 2,
    AddUser = 3,
    EditUser = 4,
    AddUserGroup = 6,
    EditUserGroup = 7,
    RemoveUserGroup = 8,
    ChangeCanister = 9,
    EditAccessPolicy = 11,
    AddProposalPolicy = 13,
    EditProposalPolicy = 14,
    RemoveProposalPolicy = 15,
    AddAddressBookEntry = 16,
    EditAddressBookEntry = 17,
    RemoveAddressBookEntry = 18,
}

impl FromStr for ProposalOperationType {
    type Err = ();

    fn from_str(variant: &str) -> Result<ProposalOperationType, Self::Err> {
        match variant {
            "transfer" => Ok(ProposalOperationType::Transfer),
            "add_account" => Ok(ProposalOperationType::AddAccount),
            "edit_account" => Ok(ProposalOperationType::EditAccount),
            "add_address_book_entry" => Ok(ProposalOperationType::AddAddressBookEntry),
            "edit_address_book_entry" => Ok(ProposalOperationType::EditAddressBookEntry),
            "remove_address_book_entry" => Ok(ProposalOperationType::RemoveAddressBookEntry),
            "add_user" => Ok(ProposalOperationType::AddUser),
            "edit_user" => Ok(ProposalOperationType::EditUser),
            "add_user_group" => Ok(ProposalOperationType::AddUserGroup),
            "edit_user_group" => Ok(ProposalOperationType::EditUserGroup),
            "remove_user_group" => Ok(ProposalOperationType::RemoveUserGroup),
            "change_canister" => Ok(ProposalOperationType::ChangeCanister),
            "edit_access_policy" => Ok(ProposalOperationType::EditAccessPolicy),
            "add_proposal_policy" => Ok(ProposalOperationType::AddProposalPolicy),
            "edit_proposal_policy" => Ok(ProposalOperationType::EditProposalPolicy),
            "remove_proposal_policy" => Ok(ProposalOperationType::RemoveProposalPolicy),
            _ => Err(()),
        }
    }
}

impl Display for ProposalOperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalOperationType::Transfer => write!(f, "transfer"),
            ProposalOperationType::AddAccount => write!(f, "add_account"),
            ProposalOperationType::EditAccount => write!(f, "edit_account"),
            ProposalOperationType::AddAddressBookEntry => write!(f, "add_address_book_entry"),
            ProposalOperationType::EditAddressBookEntry => write!(f, "edit_address_book_entry"),
            ProposalOperationType::RemoveAddressBookEntry => write!(f, "remove_address_book_entry"),
            ProposalOperationType::AddUser => write!(f, "add_user"),
            ProposalOperationType::EditUser => write!(f, "edit_user"),
            ProposalOperationType::AddUserGroup => write!(f, "add_user_group"),
            ProposalOperationType::EditUserGroup => write!(f, "edit_user_group"),
            ProposalOperationType::RemoveUserGroup => write!(f, "remove_user_group"),
            ProposalOperationType::ChangeCanister => write!(f, "change_canister"),
            ProposalOperationType::EditAccessPolicy => write!(f, "edit_access_policy"),
            ProposalOperationType::AddProposalPolicy => write!(f, "add_proposal_policy"),
            ProposalOperationType::EditProposalPolicy => write!(f, "edit_proposal_policy"),
            ProposalOperationType::RemoveProposalPolicy => write!(f, "remove_proposal_policy"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_code_match_string_representation() {
        assert_eq!(ProposalOperationType::Transfer.to_string(), "transfer");
        assert_eq!(
            ProposalOperationType::from_str("transfer").unwrap(),
            ProposalOperationType::Transfer
        );
        assert_eq!(
            ProposalOperationType::EditAccount.to_string(),
            "edit_account"
        );
        assert_eq!(
            ProposalOperationType::from_str("edit_account").unwrap(),
            ProposalOperationType::EditAccount
        );
        assert_eq!(ProposalOperationType::AddAccount.to_string(), "add_account");
        assert_eq!(
            ProposalOperationType::from_str("add_account").unwrap(),
            ProposalOperationType::AddAccount
        );
        assert_eq!(
            ProposalOperationType::AddAddressBookEntry.to_string(),
            "add_address_book_entry"
        );
        assert_eq!(
            ProposalOperationType::from_str("add_address_book_entry").unwrap(),
            ProposalOperationType::AddAddressBookEntry
        );
        assert_eq!(
            ProposalOperationType::EditAddressBookEntry.to_string(),
            "edit_address_book_entry"
        );
        assert_eq!(
            ProposalOperationType::from_str("edit_address_book_entry").unwrap(),
            ProposalOperationType::EditAddressBookEntry
        );
        assert_eq!(
            ProposalOperationType::RemoveAddressBookEntry.to_string(),
            "remove_address_book_entry"
        );
        assert_eq!(
            ProposalOperationType::from_str("remove_address_book_entry").unwrap(),
            ProposalOperationType::RemoveAddressBookEntry
        );
        assert_eq!(ProposalOperationType::AddUser.to_string(), "add_user");
        assert_eq!(
            ProposalOperationType::from_str("add_user").unwrap(),
            ProposalOperationType::AddUser
        );
        assert_eq!(ProposalOperationType::EditUser.to_string(), "edit_user");
        assert_eq!(
            ProposalOperationType::from_str("edit_user").unwrap(),
            ProposalOperationType::EditUser
        );
        assert_eq!(
            ProposalOperationType::AddUserGroup.to_string(),
            "add_user_group"
        );
        assert_eq!(
            ProposalOperationType::from_str("add_user_group").unwrap(),
            ProposalOperationType::AddUserGroup
        );
        assert_eq!(
            ProposalOperationType::EditUserGroup.to_string(),
            "edit_user_group"
        );
        assert_eq!(
            ProposalOperationType::from_str("edit_user_group").unwrap(),
            ProposalOperationType::EditUserGroup
        );
        assert_eq!(
            ProposalOperationType::RemoveUserGroup.to_string(),
            "remove_user_group"
        );
        assert_eq!(
            ProposalOperationType::from_str("remove_user_group").unwrap(),
            ProposalOperationType::RemoveUserGroup
        );
        assert_eq!(
            ProposalOperationType::from_str("change_canister").unwrap(),
            ProposalOperationType::ChangeCanister
        );
        assert_eq!(
            ProposalOperationType::from_str("edit_access_policy").unwrap(),
            ProposalOperationType::EditAccessPolicy
        );
        assert_eq!(
            ProposalOperationType::from_str("add_proposal_policy").unwrap(),
            ProposalOperationType::AddProposalPolicy
        );
        assert_eq!(
            ProposalOperationType::from_str("edit_proposal_policy").unwrap(),
            ProposalOperationType::EditProposalPolicy
        );
        assert_eq!(
            ProposalOperationType::from_str("remove_proposal_policy").unwrap(),
            ProposalOperationType::RemoveProposalPolicy
        );
    }
}
