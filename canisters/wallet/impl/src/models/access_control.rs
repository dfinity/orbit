use super::specifier::{AddressSpecifier, CommonSpecifier};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;
use std::fmt::{Display, Formatter};

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccessControlPolicy {
    pub id: UUID,
    pub user: UserSpecifier,
    pub resource: ResourceSpecifier,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceSpecifier {
    Transfer(TransferActionSpecifier),
    Upgrade(UpgradeActionSpecifier),
    CanisterSettings(CanisterSettingsActionSpecifier),
    Proposal(ProposalActionSpecifier),
    Common(ResourceType, CommonActionSpecifier),
}

impl ResourceSpecifier {
    pub fn to_key(&self) -> String {
        self.to_string()
    }
}

impl Display for ResourceSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceSpecifier::Common(resource_type, action) => {
                write!(f, "{}_{}", resource_type, action)
            }
            ResourceSpecifier::Transfer(action) => write!(f, "transfer_{}", action),
            ResourceSpecifier::CanisterSettings(action) => {
                write!(f, "canister_settings_{}", action)
            }
            ResourceSpecifier::Proposal(action) => write!(f, "proposal_{}", action),
            ResourceSpecifier::Upgrade(action) => write!(f, "upgrade_{}", action),
        }
    }
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceType {
    Account,
    User,
    UserGroup,
    AddressBook,
    AccessPolicy,
}

pub type AccountSpecifier = CommonSpecifier;
pub type UserSpecifier = CommonSpecifier;

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Account => write!(f, "account"),
            ResourceType::User => write!(f, "user"),
            ResourceType::UserGroup => write!(f, "user_group"),
            ResourceType::AddressBook => write!(f, "address_book"),
            ResourceType::AccessPolicy => write!(f, "access_policy"),
        }
    }
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommonActionSpecifier {
    List,
    Create,
    Read(CommonSpecifier),
    Update(CommonSpecifier),
    Delete(CommonSpecifier),
}

impl Display for CommonActionSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonActionSpecifier::Create => write!(f, "create"),
            CommonActionSpecifier::List => write!(f, "list"),
            CommonActionSpecifier::Read(_) => write!(f, "read"),
            CommonActionSpecifier::Update(_) => write!(f, "update"),
            CommonActionSpecifier::Delete(_) => write!(f, "delete"),
        }
    }
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CanisterSettingsActionSpecifier {
    Read,
    ReadFeatures,
}

impl Display for CanisterSettingsActionSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CanisterSettingsActionSpecifier::Read => write!(f, "read"),
            CanisterSettingsActionSpecifier::ReadFeatures => write!(f, "read_features"),
        }
    }
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UpgradeActionSpecifier {
    Create,
}

impl Display for UpgradeActionSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UpgradeActionSpecifier::Create => write!(f, "create"),
        }
    }
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalActionSpecifier {
    List,
    Read(CommonSpecifier),
}

impl Display for ProposalActionSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalActionSpecifier::List => write!(f, "list"),
            ProposalActionSpecifier::Read(_) => write!(f, "read"),
        }
    }
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransferActionSpecifier {
    Create(AccountSpecifier, AddressSpecifier),
    Read(AccountSpecifier, AddressSpecifier),
    Delete(AccountSpecifier, AddressSpecifier),
}

impl Display for TransferActionSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferActionSpecifier::Create(_, _) => write!(f, "create"),
            TransferActionSpecifier::Read(_, _) => write!(f, "read"),
            TransferActionSpecifier::Delete(_, _) => write!(f, "delete"),
        }
    }
}

pub type UserActionSpecifier = CommonActionSpecifier;
pub type AccountActionSpecifier = CommonActionSpecifier;
pub type UserGroupActionSpecifier = CommonActionSpecifier;
pub type AddressBookActionSpecifier = CommonActionSpecifier;
pub type AccessPolicyActionSpecifier = CommonActionSpecifier;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_key_representation() {
        assert_eq!(
            ResourceSpecifier::Common(ResourceType::User, UserActionSpecifier::Create).to_key(),
            "user_create"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::User,
                UserActionSpecifier::Read(CommonSpecifier::Any)
            )
            .to_key(),
            "user_read"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::User,
                UserActionSpecifier::Update(CommonSpecifier::Any)
            )
            .to_key(),
            "user_update"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::User,
                UserActionSpecifier::Delete(CommonSpecifier::Any)
            )
            .to_key(),
            "user_delete"
        );
        assert_eq!(
            ResourceSpecifier::Common(ResourceType::User, UserActionSpecifier::List).to_key(),
            "user_list"
        );
        assert_eq!(
            ResourceSpecifier::Common(ResourceType::UserGroup, UserGroupActionSpecifier::Create)
                .to_key(),
            "user_group_create"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::UserGroup,
                UserGroupActionSpecifier::Read(CommonSpecifier::Any)
            )
            .to_key(),
            "user_group_read"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::UserGroup,
                UserGroupActionSpecifier::Update(CommonSpecifier::Any)
            )
            .to_key(),
            "user_group_update"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::UserGroup,
                UserGroupActionSpecifier::Delete(CommonSpecifier::Any)
            )
            .to_key(),
            "user_group_delete"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Create
            )
            .to_key(),
            "address_book_create"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Read(CommonSpecifier::Any)
            )
            .to_key(),
            "address_book_read"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Update(CommonSpecifier::Any)
            )
            .to_key(),
            "address_book_update"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Delete(CommonSpecifier::Any)
            )
            .to_key(),
            "address_book_delete"
        );
        assert_eq!(
            ResourceSpecifier::Common(ResourceType::Account, AccountActionSpecifier::Create)
                .to_key(),
            "account_create"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::Account,
                AccountActionSpecifier::Read(CommonSpecifier::Any)
            )
            .to_key(),
            "account_read"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::Account,
                AccountActionSpecifier::Update(CommonSpecifier::Any)
            )
            .to_key(),
            "account_update"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::Account,
                AccountActionSpecifier::Delete(CommonSpecifier::Any)
            )
            .to_key(),
            "account_delete"
        );
        assert_eq!(
            ResourceSpecifier::Common(ResourceType::Account, AccountActionSpecifier::List).to_key(),
            "account_list"
        );
        assert_eq!(
            ResourceSpecifier::Transfer(TransferActionSpecifier::Create(
                CommonSpecifier::Any,
                AddressSpecifier::Any
            ))
            .to_key(),
            "transfer_create"
        );
        assert_eq!(
            ResourceSpecifier::Transfer(TransferActionSpecifier::Read(
                CommonSpecifier::Any,
                AddressSpecifier::Any
            ))
            .to_key(),
            "transfer_read"
        );
        assert_eq!(
            ResourceSpecifier::Transfer(TransferActionSpecifier::Delete(
                CommonSpecifier::Any,
                AddressSpecifier::Any
            ))
            .to_key(),
            "transfer_delete"
        );
        assert_eq!(
            ResourceSpecifier::CanisterSettings(CanisterSettingsActionSpecifier::Read).to_key(),
            "canister_settings_read"
        );
        assert_eq!(
            ResourceSpecifier::CanisterSettings(CanisterSettingsActionSpecifier::ReadFeatures)
                .to_key(),
            "canister_settings_read_features"
        );
        assert_eq!(
            ResourceSpecifier::Proposal(ProposalActionSpecifier::List).to_key(),
            "proposal_list"
        );
        assert_eq!(
            ResourceSpecifier::Proposal(ProposalActionSpecifier::Read(CommonSpecifier::Any))
                .to_key(),
            "proposal_read"
        );
        assert_eq!(
            ResourceSpecifier::Upgrade(UpgradeActionSpecifier::Create).to_key(),
            "upgrade_create"
        );
        assert_eq!(
            ResourceSpecifier::Common(ResourceType::AccessPolicy, UserGroupActionSpecifier::List)
                .to_key(),
            "access_policy_list"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::AccessPolicy,
                UserGroupActionSpecifier::Read(CommonSpecifier::Any)
            )
            .to_key(),
            "access_policy_read"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::AccessPolicy,
                UserGroupActionSpecifier::Update(CommonSpecifier::Any)
            )
            .to_key(),
            "access_policy_update"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::AccessPolicy,
                UserGroupActionSpecifier::Delete(CommonSpecifier::Any)
            )
            .to_key(),
            "access_policy_delete"
        );
        assert_eq!(
            ResourceSpecifier::Common(ResourceType::AccessPolicy, UserGroupActionSpecifier::Create)
                .to_key(),
            "access_policy_create"
        );
    }
}

#[cfg(test)]
pub mod access_control_test_utils {
    use super::*;

    pub fn mock_access_policy() -> AccessControlPolicy {
        AccessControlPolicy {
            id: [0; 16],
            user: UserSpecifier::Any,
            resource: ResourceSpecifier::Common(
                ResourceType::Account,
                AccountActionSpecifier::Create,
            ),
        }
    }
}
