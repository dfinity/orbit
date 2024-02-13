use super::HelperMapper;
use crate::models::{
    access_control::{
        AccessControlPolicy, CanisterSettingsActionSpecifier, ChangeCanisterActionSpecifier,
        CommonActionSpecifier, ProposalActionSpecifier, ResourceSpecifier, ResourceType,
        TransferActionSpecifier,
    },
    criteria::{Criteria, Percentage},
    specifier::{CommonSpecifier, ProposalSpecifier, UserSpecifier},
    ProposalPolicy,
};
use uuid::Uuid;
use wallet_api::{
    AccessPolicyInfoDTO, CriteriaDTO, ProposalPolicyInfoDTO, TransferSpecifierDTO, UserSpecifierDTO,
};

pub type AccessPolicyInfo = AccessPolicyInfoDTO;
pub type ProposalPolicyInfo = ProposalPolicyInfoDTO;

impl From<Criteria> for CriteriaDTO {
    fn from(criteria: Criteria) -> Self {
        match criteria {
            Criteria::AutoAdopted => CriteriaDTO::AutoAdopted,
            Criteria::ApprovalThreshold(specifier, threshold) => {
                CriteriaDTO::ApprovalThreshold(specifier.into(), threshold.0)
            }
            Criteria::MinimumVotes(specifier, votes) => {
                CriteriaDTO::MinimumVotes(specifier.into(), votes)
            }
            Criteria::HasAddressBookMetadata(metadata) => {
                CriteriaDTO::HasAddressBookMetadata(metadata)
            }
            Criteria::Or(criterias) => {
                CriteriaDTO::Or(criterias.into_iter().map(Into::into).collect())
            }
            Criteria::And(criterias) => {
                CriteriaDTO::And(criterias.into_iter().map(Into::into).collect())
            }
            Criteria::Not(criteria) => CriteriaDTO::Not(Box::new(Into::into(*criteria))),
        }
    }
}

impl From<CriteriaDTO> for Criteria {
    fn from(dto: CriteriaDTO) -> Self {
        match dto {
            CriteriaDTO::AutoAdopted => Criteria::AutoAdopted,
            CriteriaDTO::ApprovalThreshold(specifier, threshold) => {
                Criteria::ApprovalThreshold(specifier.into(), Percentage(threshold))
            }
            CriteriaDTO::MinimumVotes(specifier, votes) => {
                Criteria::MinimumVotes(specifier.into(), votes)
            }
            CriteriaDTO::HasAddressBookMetadata(metadata) => {
                Criteria::HasAddressBookMetadata(metadata)
            }
            CriteriaDTO::Or(criterias) => {
                Criteria::Or(criterias.into_iter().map(Into::into).collect())
            }
            CriteriaDTO::And(criterias) => {
                Criteria::And(criterias.into_iter().map(Into::into).collect())
            }
            CriteriaDTO::Not(criteria) => Criteria::Not(Box::new(Into::into(*criteria))),
        }
    }
}

impl From<UserSpecifierDTO> for UserSpecifier {
    fn from(dto: UserSpecifierDTO) -> Self {
        match dto {
            UserSpecifierDTO::Any => UserSpecifier::Any,
            UserSpecifierDTO::Group(ids) => UserSpecifier::Group(
                ids.into_iter()
                    .map(|id| *HelperMapper::to_uuid(id).expect("invalid uuid").as_bytes())
                    .collect(),
            ),
            UserSpecifierDTO::Id(ids) => UserSpecifier::Id(
                ids.into_iter()
                    .map(|id| *HelperMapper::to_uuid(id).expect("invalid uuid").as_bytes())
                    .collect(),
            ),
            UserSpecifierDTO::Owner => UserSpecifier::Owner,
            UserSpecifierDTO::Proposer => UserSpecifier::Proposer,
        }
    }
}

impl From<UserSpecifier> for UserSpecifierDTO {
    fn from(specifier: UserSpecifier) -> Self {
        match specifier {
            UserSpecifier::Any => UserSpecifierDTO::Any,
            UserSpecifier::Group(ids) => UserSpecifierDTO::Group(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
            UserSpecifier::Id(ids) => UserSpecifierDTO::Id(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
            UserSpecifier::Owner => UserSpecifierDTO::Owner,
            UserSpecifier::Proposer => UserSpecifierDTO::Proposer,
        }
    }
}

impl AccessControlPolicy {
    pub fn to_dto(self, info: AccessPolicyInfo) -> wallet_api::AccessPolicyDTO {
        wallet_api::AccessPolicyDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            user: self.user.into(),
            resource: self.resource.into(),
            info,
        }
    }
}

impl From<ResourceSpecifier> for wallet_api::ResourceSpecifierDTO {
    fn from(specifier: ResourceSpecifier) -> Self {
        match specifier {
            ResourceSpecifier::Transfer(action) => {
                wallet_api::ResourceSpecifierDTO::Transfer(action.into())
            }
            ResourceSpecifier::Proposal(action) => {
                wallet_api::ResourceSpecifierDTO::Proposal(action.into())
            }
            ResourceSpecifier::CanisterSettings(action) => {
                wallet_api::ResourceSpecifierDTO::CanisterSettings(action.into())
            }
            ResourceSpecifier::ChangeCanister(action) => {
                wallet_api::ResourceSpecifierDTO::ChangeCanister(action.into())
            }
            ResourceSpecifier::Common(resource, action) => match resource {
                ResourceType::Account => wallet_api::ResourceSpecifierDTO::Account(action.into()),
                ResourceType::User => wallet_api::ResourceSpecifierDTO::User(action.into()),
                ResourceType::UserGroup => {
                    wallet_api::ResourceSpecifierDTO::UserGroup(action.into())
                }
                ResourceType::AddressBook => {
                    wallet_api::ResourceSpecifierDTO::AddressBook(action.into())
                }
                ResourceType::AccessPolicy => {
                    wallet_api::ResourceSpecifierDTO::AccessPolicy(action.into())
                }
                ResourceType::ProposalPolicy => {
                    wallet_api::ResourceSpecifierDTO::ProposalPolicy(action.into())
                }
            },
        }
    }
}

impl From<wallet_api::ResourceSpecifierDTO> for ResourceSpecifier {
    fn from(dto: wallet_api::ResourceSpecifierDTO) -> Self {
        match dto {
            wallet_api::ResourceSpecifierDTO::Transfer(action) => {
                ResourceSpecifier::Transfer(action.into())
            }
            wallet_api::ResourceSpecifierDTO::Proposal(action) => {
                ResourceSpecifier::Proposal(action.into())
            }
            wallet_api::ResourceSpecifierDTO::CanisterSettings(action) => {
                ResourceSpecifier::CanisterSettings(action.into())
            }
            wallet_api::ResourceSpecifierDTO::ChangeCanister(action) => {
                ResourceSpecifier::ChangeCanister(action.into())
            }
            wallet_api::ResourceSpecifierDTO::Account(action) => {
                ResourceSpecifier::Common(ResourceType::Account, action.into())
            }
            wallet_api::ResourceSpecifierDTO::User(action) => {
                ResourceSpecifier::Common(ResourceType::User, action.into())
            }
            wallet_api::ResourceSpecifierDTO::UserGroup(action) => {
                ResourceSpecifier::Common(ResourceType::UserGroup, action.into())
            }
            wallet_api::ResourceSpecifierDTO::AddressBook(action) => {
                ResourceSpecifier::Common(ResourceType::AddressBook, action.into())
            }
            wallet_api::ResourceSpecifierDTO::AccessPolicy(action) => {
                ResourceSpecifier::Common(ResourceType::AccessPolicy, action.into())
            }
            wallet_api::ResourceSpecifierDTO::ProposalPolicy(action) => {
                ResourceSpecifier::Common(ResourceType::ProposalPolicy, action.into())
            }
        }
    }
}

impl From<ResourceType> for wallet_api::ResourceTypeDTO {
    fn from(resource_type: ResourceType) -> Self {
        match resource_type {
            ResourceType::Account => wallet_api::ResourceTypeDTO::Account,
            ResourceType::User => wallet_api::ResourceTypeDTO::User,
            ResourceType::UserGroup => wallet_api::ResourceTypeDTO::UserGroup,
            ResourceType::AddressBook => wallet_api::ResourceTypeDTO::AddressBook,
            ResourceType::AccessPolicy => wallet_api::ResourceTypeDTO::AccessPolicy,
            ResourceType::ProposalPolicy => wallet_api::ResourceTypeDTO::ProposalPolicy,
        }
    }
}

impl From<wallet_api::ResourceTypeDTO> for ResourceType {
    fn from(dto: wallet_api::ResourceTypeDTO) -> Self {
        match dto {
            wallet_api::ResourceTypeDTO::Account => ResourceType::Account,
            wallet_api::ResourceTypeDTO::User => ResourceType::User,
            wallet_api::ResourceTypeDTO::UserGroup => ResourceType::UserGroup,
            wallet_api::ResourceTypeDTO::AddressBook => ResourceType::AddressBook,
            wallet_api::ResourceTypeDTO::AccessPolicy => ResourceType::AccessPolicy,
            wallet_api::ResourceTypeDTO::ProposalPolicy => ResourceType::ProposalPolicy,
        }
    }
}

impl From<CommonSpecifier> for wallet_api::CommonSpecifierDTO {
    fn from(specifier: CommonSpecifier) -> Self {
        match specifier {
            CommonSpecifier::Any => wallet_api::CommonSpecifierDTO::Any,
            CommonSpecifier::Group(ids) => wallet_api::CommonSpecifierDTO::Group(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
            CommonSpecifier::Id(ids) => wallet_api::CommonSpecifierDTO::Id(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<wallet_api::CommonSpecifierDTO> for CommonSpecifier {
    fn from(dto: wallet_api::CommonSpecifierDTO) -> Self {
        match dto {
            wallet_api::CommonSpecifierDTO::Any => CommonSpecifier::Any,
            wallet_api::CommonSpecifierDTO::Group(ids) => CommonSpecifier::Group(
                ids.into_iter()
                    .map(|id| {
                        *HelperMapper::to_uuid(id)
                            .expect("invalid uuid for group")
                            .as_bytes()
                    })
                    .collect(),
            ),
            wallet_api::CommonSpecifierDTO::Id(ids) => CommonSpecifier::Id(
                ids.into_iter()
                    .map(|id| {
                        *HelperMapper::to_uuid(id)
                            .expect("invalid uuid for id")
                            .as_bytes()
                    })
                    .collect(),
            ),
        }
    }
}

impl From<TransferActionSpecifier> for wallet_api::TransferActionSpecifierDTO {
    fn from(specifier: TransferActionSpecifier) -> Self {
        match specifier {
            TransferActionSpecifier::Create(account) => {
                wallet_api::TransferActionSpecifierDTO::Create(TransferSpecifierDTO {
                    account: account.into(),
                })
            }
            TransferActionSpecifier::Read(account) => {
                wallet_api::TransferActionSpecifierDTO::Read(TransferSpecifierDTO {
                    account: account.into(),
                })
            }
            TransferActionSpecifier::Delete(account) => {
                wallet_api::TransferActionSpecifierDTO::Delete(TransferSpecifierDTO {
                    account: account.into(),
                })
            }
        }
    }
}

impl From<wallet_api::TransferActionSpecifierDTO> for TransferActionSpecifier {
    fn from(dto: wallet_api::TransferActionSpecifierDTO) -> Self {
        match dto {
            wallet_api::TransferActionSpecifierDTO::Create(transfer_specifier) => {
                TransferActionSpecifier::Create(transfer_specifier.account.into())
            }
            wallet_api::TransferActionSpecifierDTO::Read(transfer_specifier) => {
                TransferActionSpecifier::Read(transfer_specifier.account.into())
            }
            wallet_api::TransferActionSpecifierDTO::Delete(transfer_specifier) => {
                TransferActionSpecifier::Delete(transfer_specifier.account.into())
            }
        }
    }
}

impl From<ProposalActionSpecifier> for wallet_api::ProposalActionSpecifierDTO {
    fn from(specifier: ProposalActionSpecifier) -> Self {
        match specifier {
            ProposalActionSpecifier::List => wallet_api::ProposalActionSpecifierDTO::List,
            ProposalActionSpecifier::Read(common_specifier) => {
                wallet_api::ProposalActionSpecifierDTO::Read(common_specifier.into())
            }
        }
    }
}

impl From<wallet_api::ProposalActionSpecifierDTO> for ProposalActionSpecifier {
    fn from(dto: wallet_api::ProposalActionSpecifierDTO) -> Self {
        match dto {
            wallet_api::ProposalActionSpecifierDTO::List => ProposalActionSpecifier::List,
            wallet_api::ProposalActionSpecifierDTO::Read(common_specifier) => {
                ProposalActionSpecifier::Read(common_specifier.into())
            }
        }
    }
}

impl From<CanisterSettingsActionSpecifier> for wallet_api::CanisterSettingsActionSpecifierDTO {
    fn from(specifier: CanisterSettingsActionSpecifier) -> Self {
        match specifier {
            CanisterSettingsActionSpecifier::Read => {
                wallet_api::CanisterSettingsActionSpecifierDTO::Read
            }
            CanisterSettingsActionSpecifier::ReadConfig => {
                wallet_api::CanisterSettingsActionSpecifierDTO::ReadConfig
            }
        }
    }
}

impl From<wallet_api::CanisterSettingsActionSpecifierDTO> for CanisterSettingsActionSpecifier {
    fn from(dto: wallet_api::CanisterSettingsActionSpecifierDTO) -> Self {
        match dto {
            wallet_api::CanisterSettingsActionSpecifierDTO::Read => {
                CanisterSettingsActionSpecifier::Read
            }
            wallet_api::CanisterSettingsActionSpecifierDTO::ReadConfig => {
                CanisterSettingsActionSpecifier::ReadConfig
            }
        }
    }
}

impl From<ChangeCanisterActionSpecifier> for wallet_api::ChangeCanisterActionSpecifierDTO {
    fn from(specifier: ChangeCanisterActionSpecifier) -> Self {
        match specifier {
            ChangeCanisterActionSpecifier::Create => {
                wallet_api::ChangeCanisterActionSpecifierDTO::Create
            }
        }
    }
}

impl From<wallet_api::ChangeCanisterActionSpecifierDTO> for ChangeCanisterActionSpecifier {
    fn from(dto: wallet_api::ChangeCanisterActionSpecifierDTO) -> Self {
        match dto {
            wallet_api::ChangeCanisterActionSpecifierDTO::Create => {
                ChangeCanisterActionSpecifier::Create
            }
        }
    }
}

impl From<CommonActionSpecifier> for wallet_api::CommonActionSpecifierDTO {
    fn from(specifier: CommonActionSpecifier) -> Self {
        match specifier {
            CommonActionSpecifier::List => wallet_api::CommonActionSpecifierDTO::List,
            CommonActionSpecifier::Create => wallet_api::CommonActionSpecifierDTO::Create,
            CommonActionSpecifier::Read(common_specifier) => {
                wallet_api::CommonActionSpecifierDTO::Read(common_specifier.into())
            }
            CommonActionSpecifier::Update(common_specifier) => {
                wallet_api::CommonActionSpecifierDTO::Update(common_specifier.into())
            }
            CommonActionSpecifier::Delete(common_specifier) => {
                wallet_api::CommonActionSpecifierDTO::Delete(common_specifier.into())
            }
        }
    }
}

impl From<wallet_api::CommonActionSpecifierDTO> for CommonActionSpecifier {
    fn from(dto: wallet_api::CommonActionSpecifierDTO) -> Self {
        match dto {
            wallet_api::CommonActionSpecifierDTO::List => CommonActionSpecifier::List,
            wallet_api::CommonActionSpecifierDTO::Create => CommonActionSpecifier::Create,
            wallet_api::CommonActionSpecifierDTO::Read(common_specifier) => {
                CommonActionSpecifier::Read(common_specifier.into())
            }
            wallet_api::CommonActionSpecifierDTO::Update(common_specifier) => {
                CommonActionSpecifier::Update(common_specifier.into())
            }
            wallet_api::CommonActionSpecifierDTO::Delete(common_specifier) => {
                CommonActionSpecifier::Delete(common_specifier.into())
            }
        }
    }
}

impl ProposalPolicy {
    pub fn to_dto(self, info: ProposalPolicyInfo) -> wallet_api::ProposalPolicyDTO {
        wallet_api::ProposalPolicyDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            specifier: self.specifier.into(),
            criteria: self.criteria.into(),
            info,
        }
    }
}

impl From<ProposalSpecifier> for wallet_api::ProposalSpecifierDTO {
    fn from(specifier: ProposalSpecifier) -> Self {
        match specifier {
            ProposalSpecifier::AddAccount => wallet_api::ProposalSpecifierDTO::AddAccount,
            ProposalSpecifier::AddUser => wallet_api::ProposalSpecifierDTO::AddUser,
            ProposalSpecifier::EditAccount(account) => {
                wallet_api::ProposalSpecifierDTO::EditAccount(account.into())
            }
            ProposalSpecifier::EditUser(user) => {
                wallet_api::ProposalSpecifierDTO::EditUser(user.into())
            }
            ProposalSpecifier::AddAddressBookEntry => {
                wallet_api::ProposalSpecifierDTO::AddAddressBookEntry
            }
            ProposalSpecifier::EditAddressBookEntry(address_book_entry) => {
                wallet_api::ProposalSpecifierDTO::EditAddressBookEntry(address_book_entry.into())
            }
            ProposalSpecifier::RemoveAddressBookEntry(address_book_entry) => {
                wallet_api::ProposalSpecifierDTO::RemoveAddressBookEntry(address_book_entry.into())
            }
            ProposalSpecifier::Transfer(account) => {
                wallet_api::ProposalSpecifierDTO::Transfer(TransferSpecifierDTO {
                    account: account.into(),
                })
            }
            ProposalSpecifier::ChangeCanister => wallet_api::ProposalSpecifierDTO::ChangeCanister,
            ProposalSpecifier::AddAccessPolicy => wallet_api::ProposalSpecifierDTO::AddAccessPolicy,
            ProposalSpecifier::EditAccessPolicy(policy) => {
                wallet_api::ProposalSpecifierDTO::EditAccessPolicy(policy.into())
            }
            ProposalSpecifier::RemoveAccessPolicy(policy) => {
                wallet_api::ProposalSpecifierDTO::RemoveAccessPolicy(policy.into())
            }
            ProposalSpecifier::AddProposalPolicy => {
                wallet_api::ProposalSpecifierDTO::AddProposalPolicy
            }
            ProposalSpecifier::EditProposalPolicy(policy) => {
                wallet_api::ProposalSpecifierDTO::EditProposalPolicy(policy.into())
            }
            ProposalSpecifier::RemoveProposalPolicy(policy) => {
                wallet_api::ProposalSpecifierDTO::RemoveProposalPolicy(policy.into())
            }
            ProposalSpecifier::AddUserGroup => wallet_api::ProposalSpecifierDTO::AddUserGroup,
            ProposalSpecifier::EditUserGroup(group) => {
                wallet_api::ProposalSpecifierDTO::EditUserGroup(group.into())
            }
            ProposalSpecifier::RemoveUserGroup(group) => {
                wallet_api::ProposalSpecifierDTO::RemoveUserGroup(group.into())
            }
        }
    }
}

impl From<wallet_api::ProposalSpecifierDTO> for ProposalSpecifier {
    fn from(dto: wallet_api::ProposalSpecifierDTO) -> Self {
        match dto {
            wallet_api::ProposalSpecifierDTO::AddAccount => ProposalSpecifier::AddAccount,
            wallet_api::ProposalSpecifierDTO::AddUser => ProposalSpecifier::AddUser,
            wallet_api::ProposalSpecifierDTO::EditAccount(account) => {
                ProposalSpecifier::EditAccount(account.into())
            }
            wallet_api::ProposalSpecifierDTO::EditUser(user) => {
                ProposalSpecifier::EditUser(user.into())
            }
            wallet_api::ProposalSpecifierDTO::AddAddressBookEntry => {
                ProposalSpecifier::AddAddressBookEntry
            }
            wallet_api::ProposalSpecifierDTO::EditAddressBookEntry(address_book_entry) => {
                ProposalSpecifier::EditAddressBookEntry(address_book_entry.into())
            }
            wallet_api::ProposalSpecifierDTO::RemoveAddressBookEntry(address_book_entry) => {
                ProposalSpecifier::RemoveAddressBookEntry(address_book_entry.into())
            }
            wallet_api::ProposalSpecifierDTO::Transfer(transfer_specifier) => {
                ProposalSpecifier::Transfer(transfer_specifier.account.into())
            }
            wallet_api::ProposalSpecifierDTO::ChangeCanister => ProposalSpecifier::ChangeCanister,
            wallet_api::ProposalSpecifierDTO::AddAccessPolicy => ProposalSpecifier::AddAccessPolicy,
            wallet_api::ProposalSpecifierDTO::EditAccessPolicy(policy) => {
                ProposalSpecifier::EditAccessPolicy(policy.into())
            }
            wallet_api::ProposalSpecifierDTO::RemoveAccessPolicy(policy) => {
                ProposalSpecifier::RemoveAccessPolicy(policy.into())
            }
            wallet_api::ProposalSpecifierDTO::AddProposalPolicy => {
                ProposalSpecifier::AddProposalPolicy
            }
            wallet_api::ProposalSpecifierDTO::EditProposalPolicy(policy) => {
                ProposalSpecifier::EditProposalPolicy(policy.into())
            }
            wallet_api::ProposalSpecifierDTO::RemoveProposalPolicy(policy) => {
                ProposalSpecifier::RemoveProposalPolicy(policy.into())
            }
            wallet_api::ProposalSpecifierDTO::AddUserGroup => ProposalSpecifier::AddUserGroup,
            wallet_api::ProposalSpecifierDTO::EditUserGroup(group) => {
                ProposalSpecifier::EditUserGroup(group.into())
            }
            wallet_api::ProposalSpecifierDTO::RemoveUserGroup(group) => {
                ProposalSpecifier::RemoveUserGroup(group.into())
            }
        }
    }
}
