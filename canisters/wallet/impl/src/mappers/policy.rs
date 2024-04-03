use super::HelperMapper;
use crate::models::{
    criteria::{Criteria, Percentage},
    specifier::{CommonSpecifier, ProposalSpecifier, ResourceSpecifier, UserSpecifier},
    ProposalPolicy, ProposalPolicyCallerPrivileges,
};
use uuid::Uuid;
use wallet_api::{
    ApprovalThresholdDTO, CriteriaDTO, MinimumVotesDTO, TransferSpecifierDTO, UserSpecifierDTO,
};

impl From<Criteria> for CriteriaDTO {
    fn from(criteria: Criteria) -> Self {
        match criteria {
            Criteria::AutoAdopted => CriteriaDTO::AutoAdopted,
            Criteria::ApprovalThreshold(specifier, threshold) => {
                CriteriaDTO::ApprovalThreshold(ApprovalThresholdDTO {
                    voters: specifier.into(),
                    threshold: threshold.0,
                })
            }
            Criteria::MinimumVotes(specifier, votes) => {
                CriteriaDTO::MinimumVotes(MinimumVotesDTO {
                    voters: specifier.into(),
                    minimum: votes,
                })
            }
            Criteria::HasAddressBookMetadata(metadata) => {
                CriteriaDTO::HasAddressBookMetadata(metadata.into())
            }
            Criteria::HasAddressInAddressBook => CriteriaDTO::HasAddressInAddressBook,
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
            CriteriaDTO::ApprovalThreshold(config) => {
                Criteria::ApprovalThreshold(config.voters.into(), Percentage(config.threshold))
            }
            CriteriaDTO::MinimumVotes(config) => {
                Criteria::MinimumVotes(config.voters.into(), config.minimum)
            }
            CriteriaDTO::HasAddressBookMetadata(metadata) => {
                Criteria::HasAddressBookMetadata(metadata.into())
            }
            CriteriaDTO::HasAddressInAddressBook => Criteria::HasAddressInAddressBook,
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

impl From<wallet_api::ResourceSpecifierDTO> for ResourceSpecifier {
    fn from(dto: wallet_api::ResourceSpecifierDTO) -> Self {
        match dto {
            wallet_api::ResourceSpecifierDTO::Any => ResourceSpecifier::Any,
            wallet_api::ResourceSpecifierDTO::Resource(resource) => {
                ResourceSpecifier::Resource(resource.into())
            }
        }
    }
}

impl From<ResourceSpecifier> for wallet_api::ResourceSpecifierDTO {
    fn from(specifier: ResourceSpecifier) -> Self {
        match specifier {
            ResourceSpecifier::Any => wallet_api::ResourceSpecifierDTO::Any,
            ResourceSpecifier::Resource(resource) => {
                wallet_api::ResourceSpecifierDTO::Resource(resource.into())
            }
        }
    }
}

impl ProposalPolicy {
    pub fn to_dto(self) -> wallet_api::ProposalPolicyDTO {
        wallet_api::ProposalPolicyDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            specifier: self.specifier.into(),
            criteria: self.criteria.into(),
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
            ProposalSpecifier::EditAccessPolicy(policy) => {
                wallet_api::ProposalSpecifierDTO::EditAccessPolicy(policy.into())
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
            wallet_api::ProposalSpecifierDTO::EditAccessPolicy(policy) => {
                ProposalSpecifier::EditAccessPolicy(policy.into())
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

impl From<ProposalPolicyCallerPrivileges> for wallet_api::ProposalPolicyCallerPrivilegesDTO {
    fn from(privileges: ProposalPolicyCallerPrivileges) -> Self {
        wallet_api::ProposalPolicyCallerPrivilegesDTO {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_delete: privileges.can_delete,
            can_edit: privileges.can_edit,
        }
    }
}
