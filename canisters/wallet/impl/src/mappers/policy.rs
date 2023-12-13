use super::HelperMapper;
use crate::models::{
    access_control::{
        AccessControlPolicy, CanisterSettingsActionSpecifier, CommonActionSpecifier,
        ProposalActionSpecifier, ResourceSpecifier, ResourceType, TransferActionSpecifier,
        UpgradeActionSpecifier,
    },
    criteria::{Criteria, Percentage},
    specifier::{AddressSpecifier, CommonSpecifier, UserSpecifier},
};
use uuid::Uuid;
use wallet_api::{
    CriteriaDTO, ResourceSpecifierCommonArgsDTO, TransferSpecifierDTO, UserSpecifierDTO,
};

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

impl From<AccessControlPolicy> for wallet_api::AccessControlPolicyDTO {
    fn from(policy: AccessControlPolicy) -> Self {
        Self {
            id: Uuid::from_bytes(policy.id).hyphenated().to_string(),
            user: policy.user.into(),
            resource: policy.resource.into(),
        }
    }
}

impl From<ResourceSpecifier> for wallet_api::ResourceSpecifierDTO {
    fn from(specifier: ResourceSpecifier) -> Self {
        match specifier {
            ResourceSpecifier::Transfer(transfer_action_specifier) => {
                wallet_api::ResourceSpecifierDTO::Transfer(transfer_action_specifier.into())
            }
            ResourceSpecifier::Proposal(proposal_action_specifier) => {
                wallet_api::ResourceSpecifierDTO::Proposal(proposal_action_specifier.into())
            }
            ResourceSpecifier::CanisterSettings(canister_settings_action_specifier) => {
                wallet_api::ResourceSpecifierDTO::CanisterSettings(
                    canister_settings_action_specifier.into(),
                )
            }
            ResourceSpecifier::Upgrade(upgrade_action_specifier) => {
                wallet_api::ResourceSpecifierDTO::Upgrade(upgrade_action_specifier.into())
            }
            ResourceSpecifier::Common(resource, action) => {
                wallet_api::ResourceSpecifierDTO::Common(ResourceSpecifierCommonArgsDTO {
                    resource_type: resource.into(),
                    action: action.into(),
                })
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

impl From<AddressSpecifier> for wallet_api::AddressSpecifierDTO {
    fn from(specifier: AddressSpecifier) -> Self {
        match specifier {
            AddressSpecifier::Any => wallet_api::AddressSpecifierDTO::Any,
        }
    }
}

impl From<TransferActionSpecifier> for wallet_api::TransferActionSpecifierDTO {
    fn from(specifier: TransferActionSpecifier) -> Self {
        match specifier {
            TransferActionSpecifier::Create(account, address) => {
                wallet_api::TransferActionSpecifierDTO::Create(TransferSpecifierDTO {
                    account: account.into(),
                    address: address.into(),
                })
            }
            TransferActionSpecifier::Read(account, address) => {
                wallet_api::TransferActionSpecifierDTO::Read(TransferSpecifierDTO {
                    account: account.into(),
                    address: address.into(),
                })
            }
            TransferActionSpecifier::Delete(account, address) => {
                wallet_api::TransferActionSpecifierDTO::Delete(TransferSpecifierDTO {
                    account: account.into(),
                    address: address.into(),
                })
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

impl From<CanisterSettingsActionSpecifier> for wallet_api::CanisterSettingsActionSpecifierDTO {
    fn from(specifier: CanisterSettingsActionSpecifier) -> Self {
        match specifier {
            CanisterSettingsActionSpecifier::Read => {
                wallet_api::CanisterSettingsActionSpecifierDTO::Read
            }
            CanisterSettingsActionSpecifier::ReadFeatures => {
                wallet_api::CanisterSettingsActionSpecifierDTO::ReadFeatures
            }
        }
    }
}

impl From<UpgradeActionSpecifier> for wallet_api::UpgradeActionSpecifierDTO {
    fn from(specifier: UpgradeActionSpecifier) -> Self {
        match specifier {
            UpgradeActionSpecifier::Create => wallet_api::UpgradeActionSpecifierDTO::Create,
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
