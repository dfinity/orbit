use super::{
    specifier::{AccountSpecifier, AddressSpecifier, UserSpecifier},
    AccountId, UserId,
};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

/// Resource identifiers are used to uniquely identify a resource in the system (e.g. a user, an account, etc.).
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceIdentifier {
    Transfer(Option<AccountId>, Option<String>),
    Account(Option<AccountId>),
    UserGroup,
    User(Option<UserId>),
    UserStatus(Option<UserId>),
    AddressBook,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceSpecifier {
    Transfer(AccountSpecifier, AddressSpecifier),
    Account(AccountSpecifier),
    UserGroup,
    User(UserSpecifier),
    UserStatus(UserSpecifier),
    AddressBook,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AccessModifier {
    Default = 0,
    Create = 1,
    Read = 2,
    Update = 3,
    Delete = 4,
    All = 5,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccessControlPolicy {
    pub id: UUID,
    pub user: UserSpecifier,
    pub access: AccessModifier,
    pub resource: ResourceSpecifier,
}

impl Display for ResourceSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceSpecifier::User(_) => write!(f, "user"),
            ResourceSpecifier::UserGroup => write!(f, "user_group"),
            ResourceSpecifier::AddressBook => write!(f, "address_book"),
            ResourceSpecifier::Account(_) => write!(f, "account"),
            ResourceSpecifier::UserStatus(_) => write!(f, "user_status"),
            ResourceSpecifier::Transfer(_, _) => write!(f, "transfer"),
        }
    }
}

impl Display for ResourceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceIdentifier::User(_) => write!(f, "user"),
            ResourceIdentifier::UserGroup => write!(f, "user_group"),
            ResourceIdentifier::AddressBook => write!(f, "address_book"),
            ResourceIdentifier::Account(_) => write!(f, "account"),
            ResourceIdentifier::UserStatus(_) => write!(f, "user_status"),
            ResourceIdentifier::Transfer(_, _) => write!(f, "transfer"),
        }
    }
}

impl TryFrom<u8> for AccessModifier {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AccessModifier::Default),
            1 => Ok(AccessModifier::Create),
            2 => Ok(AccessModifier::Read),
            3 => Ok(AccessModifier::Update),
            4 => Ok(AccessModifier::Delete),
            5 => Ok(AccessModifier::All),
            _ => Err(()),
        }
    }
}

impl Display for AccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessModifier::Default => write!(f, "default"),
            AccessModifier::Read => write!(f, "read"),
            AccessModifier::Create => write!(f, "create"),
            AccessModifier::Update => write!(f, "update"),
            AccessModifier::Delete => write!(f, "delete"),
            AccessModifier::All => write!(f, "all"),
        }
    }
}

impl From<AccessModifier> for u8 {
    fn from(access_modifier: AccessModifier) -> Self {
        access_modifier as u8
    }
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
    fn resource_string_representation() {
        assert_eq!(
            ResourceSpecifier::User(UserSpecifier::Any).to_string(),
            "user"
        );
        assert_eq!(ResourceSpecifier::UserGroup.to_string(), "user_group");
        assert_eq!(ResourceSpecifier::AddressBook.to_string(), "address_book");
        assert_eq!(
            ResourceSpecifier::Account(AccountSpecifier::Any).to_string(),
            "account"
        );
        assert_eq!(
            ResourceSpecifier::UserStatus(UserSpecifier::Any).to_string(),
            "user_status"
        );
        assert_eq!(
            ResourceSpecifier::Transfer(AccountSpecifier::Any, AddressSpecifier::Any).to_string(),
            "transfer"
        );
    }

    #[test]
    fn access_modifier_match_number_representation() {
        assert_eq!(AccessModifier::Default as u8, 0);
        assert_eq!(
            AccessModifier::try_from(0).unwrap(),
            AccessModifier::Default
        );
        assert_eq!(AccessModifier::Create as u8, 1);
        assert_eq!(AccessModifier::try_from(1).unwrap(), AccessModifier::Create);
        assert_eq!(AccessModifier::Read as u8, 2);
        assert_eq!(AccessModifier::try_from(2).unwrap(), AccessModifier::Read);
        assert_eq!(AccessModifier::Update as u8, 3);
        assert_eq!(AccessModifier::try_from(3).unwrap(), AccessModifier::Update);
        assert_eq!(AccessModifier::Delete as u8, 4);
        assert_eq!(AccessModifier::try_from(4).unwrap(), AccessModifier::Delete);
        assert_eq!(AccessModifier::All as u8, 5);
        assert_eq!(AccessModifier::try_from(5).unwrap(), AccessModifier::All);
    }

    #[test]
    fn access_modifier_string_representation() {
        assert_eq!(AccessModifier::Default.to_string(), "default");
        assert_eq!(AccessModifier::Create.to_string(), "create");
        assert_eq!(AccessModifier::Read.to_string(), "read");
        assert_eq!(AccessModifier::Update.to_string(), "update");
        assert_eq!(AccessModifier::Delete.to_string(), "delete");
        assert_eq!(AccessModifier::All.to_string(), "all");
    }
}

#[cfg(test)]
pub mod access_control_test_utils {
    use super::*;

    pub fn mock_access_policy() -> AccessControlPolicy {
        AccessControlPolicy {
            id: [0; 16],
            user: UserSpecifier::Any,
            resource: ResourceSpecifier::Account(AccountSpecifier::Any),
            access: AccessModifier::All,
        }
    }
}
