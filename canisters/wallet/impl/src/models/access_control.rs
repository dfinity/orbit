use super::specifier::CommonSpecifier;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::storable;
use std::fmt::{Display, Formatter};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccessControlPolicy {
    pub id: UUID,
    pub user: UserSpecifier,
    pub resource: ResourceSpecifier,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccessPolicyCallerPrivileges {
    pub id: UUID,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceSpecifier {
    Transfer(TransferActionSpecifier),
    ChangeCanister(ChangeCanisterActionSpecifier),
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
            ResourceSpecifier::ChangeCanister(action) => write!(f, "change_canister_{}", action),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceType {
    Account,
    User,
    UserGroup,
    AddressBook,
    AccessPolicy,
    ProposalPolicy,
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
            ResourceType::ProposalPolicy => write!(f, "proposal_policy"),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CanisterSettingsActionSpecifier {
    Read,
    ReadConfig,
}

impl Display for CanisterSettingsActionSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CanisterSettingsActionSpecifier::Read => write!(f, "read"),
            CanisterSettingsActionSpecifier::ReadConfig => write!(f, "read_config"),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChangeCanisterActionSpecifier {
    Create,
}

impl Display for ChangeCanisterActionSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeCanisterActionSpecifier::Create => write!(f, "create"),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransferActionSpecifier {
    Create(AccountSpecifier),
    Read(AccountSpecifier),
    Delete(AccountSpecifier),
}

impl Display for TransferActionSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferActionSpecifier::Create(_) => write!(f, "create"),
            TransferActionSpecifier::Read(_) => write!(f, "read"),
            TransferActionSpecifier::Delete(_) => write!(f, "delete"),
        }
    }
}

pub type UserActionSpecifier = CommonActionSpecifier;
pub type AccountActionSpecifier = CommonActionSpecifier;
pub type UserGroupActionSpecifier = CommonActionSpecifier;
pub type AddressBookActionSpecifier = CommonActionSpecifier;
pub type AccessPolicyActionSpecifier = CommonActionSpecifier;
pub type ProposalPolicyActionSpecifier = CommonActionSpecifier;

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
            ResourceSpecifier::Transfer(TransferActionSpecifier::Create(CommonSpecifier::Any,))
                .to_key(),
            "transfer_create"
        );
        assert_eq!(
            ResourceSpecifier::Transfer(TransferActionSpecifier::Read(CommonSpecifier::Any,))
                .to_key(),
            "transfer_read"
        );
        assert_eq!(
            ResourceSpecifier::Transfer(TransferActionSpecifier::Delete(CommonSpecifier::Any,))
                .to_key(),
            "transfer_delete"
        );
        assert_eq!(
            ResourceSpecifier::CanisterSettings(CanisterSettingsActionSpecifier::Read).to_key(),
            "canister_settings_read"
        );
        assert_eq!(
            ResourceSpecifier::CanisterSettings(CanisterSettingsActionSpecifier::ReadConfig)
                .to_key(),
            "canister_settings_read_config"
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
            ResourceSpecifier::ChangeCanister(ChangeCanisterActionSpecifier::Create).to_key(),
            "change_canister_create"
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
        assert_eq!(
            ResourceSpecifier::Common(ResourceType::ProposalPolicy, UserGroupActionSpecifier::List)
                .to_key(),
            "proposal_policy_list"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                UserGroupActionSpecifier::Read(CommonSpecifier::Any)
            )
            .to_key(),
            "proposal_policy_read"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                UserGroupActionSpecifier::Update(CommonSpecifier::Any)
            )
            .to_key(),
            "proposal_policy_update"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                UserGroupActionSpecifier::Delete(CommonSpecifier::Any)
            )
            .to_key(),
            "proposal_policy_delete"
        );
        assert_eq!(
            ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                UserGroupActionSpecifier::Create
            )
            .to_key(),
            "proposal_policy_create"
        );
    }
}

#[cfg(any(test, feature = "canbench"))]
pub mod access_control_test_utils {
    use super::*;
    use uuid::Uuid;

    pub fn mock_access_policy() -> AccessControlPolicy {
        AccessControlPolicy {
            id: *Uuid::new_v4().as_bytes(),
            user: UserSpecifier::Any,
            resource: ResourceSpecifier::Common(
                ResourceType::Account,
                AccountActionSpecifier::Create,
            ),
        }
    }
}