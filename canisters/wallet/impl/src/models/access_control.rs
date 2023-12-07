use super::specifier::UserSpecifier;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Resource {
    TransferProposal = 0,
    AddAccountProposal = 1,
    EditAccountProposal = 2,
    AddUserProposal = 3,
    EditUserProposal = 4,
    EditUserStatusProposal = 5,
    AddUserGroupProposal = 6,
    EditUserGroupProposal = 7,
    RemoveUserGroupProposal = 8,
    User = 9,
    UserGroup = 10,
    AddressBook = 11,
    Account = 12,
    Proposal = 13,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AccessModifier {
    Default = 0,
    Write = 1,
    All = 2,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccessControlPolicy {
    pub id: UUID,
    pub specifier: UserSpecifier,
    pub access: AccessModifier,
    pub resource: Resource,
}

impl From<Resource> for u8 {
    fn from(resource: Resource) -> Self {
        resource as u8
    }
}

impl TryFrom<u8> for Resource {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Resource::TransferProposal),
            1 => Ok(Resource::AddAccountProposal),
            2 => Ok(Resource::EditAccountProposal),
            3 => Ok(Resource::AddUserProposal),
            4 => Ok(Resource::EditUserProposal),
            5 => Ok(Resource::EditUserStatusProposal),
            6 => Ok(Resource::AddUserGroupProposal),
            7 => Ok(Resource::EditUserGroupProposal),
            8 => Ok(Resource::RemoveUserGroupProposal),
            9 => Ok(Resource::User),
            10 => Ok(Resource::UserGroup),
            11 => Ok(Resource::AddressBook),
            12 => Ok(Resource::Account),
            13 => Ok(Resource::Proposal),
            _ => Err(()),
        }
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::TransferProposal => write!(f, "transfer_proposal"),
            Resource::AddAccountProposal => write!(f, "add_account_proposal"),
            Resource::EditAccountProposal => write!(f, "edit_account_proposal"),
            Resource::AddUserProposal => write!(f, "add_user_proposal"),
            Resource::EditUserProposal => write!(f, "edit_user_proposal"),
            Resource::EditUserStatusProposal => write!(f, "edit_user_status_proposal"),
            Resource::AddUserGroupProposal => write!(f, "add_user_group_proposal"),
            Resource::EditUserGroupProposal => write!(f, "edit_user_group_proposal"),
            Resource::RemoveUserGroupProposal => write!(f, "remove_user_group_proposal"),
            Resource::User => write!(f, "user"),
            Resource::UserGroup => write!(f, "user_group"),
            Resource::AddressBook => write!(f, "address_book"),
            Resource::Account => write!(f, "account"),
            Resource::Proposal => write!(f, "proposal"),
        }
    }
}

impl TryFrom<u8> for AccessModifier {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AccessModifier::Default),
            1 => Ok(AccessModifier::Write),
            2 => Ok(AccessModifier::All),
            _ => Err(()),
        }
    }
}

impl Display for AccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessModifier::Default => write!(f, "default"),
            AccessModifier::Write => write!(f, "write"),
            AccessModifier::All => write!(f, "all"),
        }
    }
}

impl From<AccessModifier> for u8 {
    fn from(access_modifier: AccessModifier) -> Self {
        access_modifier as u8
    }
}

impl Storable for Resource {
    fn to_bytes(&self) -> Cow<[u8]> {
        let resource: u8 = self.to_owned().into();
        Cow::Owned(resource.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let resource = u8::from_bytes(bytes);
        Resource::try_from(resource).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for AccessModifier {
    fn to_bytes(&self) -> Cow<[u8]> {
        let access_modifier: u8 = self.to_owned().into();
        Cow::Owned(access_modifier.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let access_modifier = u8::from_bytes(bytes);
        AccessModifier::try_from(access_modifier).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_match_number_representation() {
        assert_eq!(Resource::TransferProposal as u8, 0);
        assert_eq!(Resource::try_from(0).unwrap(), Resource::TransferProposal);
        assert_eq!(Resource::AddAccountProposal as u8, 1);
        assert_eq!(Resource::try_from(1).unwrap(), Resource::AddAccountProposal);
        assert_eq!(Resource::EditAccountProposal as u8, 2);
        assert_eq!(
            Resource::try_from(2).unwrap(),
            Resource::EditAccountProposal
        );
        assert_eq!(Resource::AddUserProposal as u8, 3);
        assert_eq!(Resource::try_from(3).unwrap(), Resource::AddUserProposal);
        assert_eq!(Resource::EditUserProposal as u8, 4);
        assert_eq!(Resource::try_from(4).unwrap(), Resource::EditUserProposal);
        assert_eq!(Resource::EditUserStatusProposal as u8, 5);
        assert_eq!(
            Resource::try_from(5).unwrap(),
            Resource::EditUserStatusProposal
        );
        assert_eq!(Resource::AddUserGroupProposal as u8, 6);
        assert_eq!(
            Resource::try_from(6).unwrap(),
            Resource::AddUserGroupProposal
        );
        assert_eq!(Resource::EditUserGroupProposal as u8, 7);
        assert_eq!(
            Resource::try_from(7).unwrap(),
            Resource::EditUserGroupProposal
        );
        assert_eq!(Resource::RemoveUserGroupProposal as u8, 8);
        assert_eq!(
            Resource::try_from(8).unwrap(),
            Resource::RemoveUserGroupProposal
        );
        assert_eq!(Resource::User as u8, 9);
        assert_eq!(Resource::try_from(9).unwrap(), Resource::User);
        assert_eq!(Resource::UserGroup as u8, 10);
        assert_eq!(Resource::try_from(10).unwrap(), Resource::UserGroup);
        assert_eq!(Resource::AddressBook as u8, 11);
        assert_eq!(Resource::try_from(11).unwrap(), Resource::AddressBook);
        assert_eq!(Resource::Account as u8, 12);
        assert_eq!(Resource::try_from(12).unwrap(), Resource::Account);
        assert_eq!(Resource::Proposal as u8, 13);
        assert_eq!(Resource::try_from(13).unwrap(), Resource::Proposal);
    }

    #[test]
    fn resource_string_representation() {
        assert_eq!(Resource::TransferProposal.to_string(), "transfer_proposal");
        assert_eq!(
            Resource::AddAccountProposal.to_string(),
            "add_account_proposal"
        );
        assert_eq!(
            Resource::EditAccountProposal.to_string(),
            "edit_account_proposal"
        );
        assert_eq!(Resource::AddUserProposal.to_string(), "add_user_proposal");
        assert_eq!(Resource::EditUserProposal.to_string(), "edit_user_proposal");
        assert_eq!(
            Resource::EditUserStatusProposal.to_string(),
            "edit_user_status_proposal"
        );
        assert_eq!(
            Resource::AddUserGroupProposal.to_string(),
            "add_user_group_proposal"
        );
        assert_eq!(
            Resource::EditUserGroupProposal.to_string(),
            "edit_user_group_proposal"
        );
        assert_eq!(
            Resource::RemoveUserGroupProposal.to_string(),
            "remove_user_group_proposal"
        );
        assert_eq!(Resource::User.to_string(), "user");
        assert_eq!(Resource::UserGroup.to_string(), "user_group");
        assert_eq!(Resource::AddressBook.to_string(), "address_book");
        assert_eq!(Resource::Account.to_string(), "account");
        assert_eq!(Resource::Proposal.to_string(), "proposal");
    }

    #[test]
    fn access_modifier_match_number_representation() {
        assert_eq!(AccessModifier::Default as u8, 0);
        assert_eq!(
            AccessModifier::try_from(0).unwrap(),
            AccessModifier::Default
        );
        assert_eq!(AccessModifier::Write as u8, 1);
        assert_eq!(AccessModifier::try_from(1).unwrap(), AccessModifier::Write);
        assert_eq!(AccessModifier::All as u8, 2);
        assert_eq!(AccessModifier::try_from(2).unwrap(), AccessModifier::All);
    }

    #[test]
    fn access_modifier_string_representation() {
        assert_eq!(AccessModifier::Default.to_string(), "default");
        assert_eq!(AccessModifier::Write.to_string(), "write");
        assert_eq!(AccessModifier::All.to_string(), "all");
    }
}

#[cfg(test)]
pub mod access_control_test_utils {
    use super::*;

    pub fn mock_access_policy() -> AccessControlPolicy {
        AccessControlPolicy {
            id: [0; 16],
            specifier: UserSpecifier::Any,
            resource: Resource::Account,
            access: AccessModifier::Default,
        }
    }
}
