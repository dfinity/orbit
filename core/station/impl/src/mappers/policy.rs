use super::HelperMapper;
use crate::models::{
    criteria::{Criteria, Percentage},
    resource::{
        AccessPolicyResourceAction, AccountResourceAction, ChangeCanisterResourceAction, Resource,
        ResourceAction, ResourceId, ResourceIds, UserResourceAction,
    },
    specifier::{CommonSpecifier, ProposalSpecifier, ResourceSpecifier, UserSpecifier},
    CriteriaResult, EvaluatedCriteria, EvaluationStatus, ProposalEvaluationResult, ProposalPolicy,
    ProposalPolicyCallerPrivileges,
};
use station_api::{
    ApprovalThresholdDTO, CriteriaDTO, CriteriaResultDTO, EvaluatedCriteriaDTO,
    EvaluationStatusDTO, MinimumVotesDTO, ProposalEvaluationResultDTO, UserSpecifierDTO,
};
use uuid::Uuid;

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

impl From<ProposalEvaluationResult> for ProposalEvaluationResultDTO {
    fn from(value: ProposalEvaluationResult) -> Self {
        ProposalEvaluationResultDTO {
            proposal_id: Uuid::from_bytes(value.proposal_id).hyphenated().to_string(),
            status: value.status.into(),
            policy_results: value.policy_results.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<EvaluationStatus> for EvaluationStatusDTO {
    fn from(value: EvaluationStatus) -> Self {
        match value {
            EvaluationStatus::Adopted => EvaluationStatusDTO::Adopted,
            EvaluationStatus::Rejected => EvaluationStatusDTO::Rejected,
            EvaluationStatus::Pending => EvaluationStatusDTO::Pending,
        }
    }
}

impl From<CriteriaResult> for CriteriaResultDTO {
    fn from(value: CriteriaResult) -> Self {
        CriteriaResultDTO {
            status: value.status.into(),
            evaluated_criteria: value.evaluated_criteria.into(),
        }
    }
}

impl From<EvaluatedCriteria> for EvaluatedCriteriaDTO {
    fn from(value: EvaluatedCriteria) -> Self {
        match value {
            EvaluatedCriteria::AutoAdopted => EvaluatedCriteriaDTO::AutoAdopted,
            EvaluatedCriteria::ApprovalThreshold {
                min_required_votes,
                total_possible_votes,
                votes,
            } => EvaluatedCriteriaDTO::ApprovalThreshold {
                min_required_votes,
                total_possible_votes,
                votes: votes
                    .into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect(),
            },
            EvaluatedCriteria::MinimumVotes {
                min_required_votes,
                total_possible_votes,
                votes,
            } => EvaluatedCriteriaDTO::MinimumVotes {
                min_required_votes,
                total_possible_votes,
                votes: votes
                    .into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect(),
            },
            EvaluatedCriteria::HasAddressBookMetadata { metadata } => {
                EvaluatedCriteriaDTO::HasAddressBookMetadata {
                    metadata: metadata.into(),
                }
            }
            EvaluatedCriteria::HasAddressInAddressBook => {
                EvaluatedCriteriaDTO::HasAddressInAddressBook
            }
            EvaluatedCriteria::Or(criterias) => {
                EvaluatedCriteriaDTO::Or(criterias.into_iter().map(Into::into).collect())
            }
            EvaluatedCriteria::And(criterias) => {
                EvaluatedCriteriaDTO::And(criterias.into_iter().map(Into::into).collect())
            }
            EvaluatedCriteria::Not(criteria) => {
                EvaluatedCriteriaDTO::Not(Box::new(Into::into(*criteria)))
            }
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

impl From<CommonSpecifier> for station_api::CommonSpecifierDTO {
    fn from(specifier: CommonSpecifier) -> Self {
        match specifier {
            CommonSpecifier::Any => station_api::CommonSpecifierDTO::Any,
            CommonSpecifier::Group(ids) => station_api::CommonSpecifierDTO::Group(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
            CommonSpecifier::Id(ids) => station_api::CommonSpecifierDTO::Id(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<station_api::CommonSpecifierDTO> for CommonSpecifier {
    fn from(dto: station_api::CommonSpecifierDTO) -> Self {
        match dto {
            station_api::CommonSpecifierDTO::Any => CommonSpecifier::Any,
            station_api::CommonSpecifierDTO::Group(ids) => CommonSpecifier::Group(
                ids.into_iter()
                    .map(|id| {
                        *HelperMapper::to_uuid(id)
                            .expect("invalid uuid for group")
                            .as_bytes()
                    })
                    .collect(),
            ),
            station_api::CommonSpecifierDTO::Id(ids) => CommonSpecifier::Id(
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

impl From<station_api::ResourceSpecifierDTO> for ResourceSpecifier {
    fn from(dto: station_api::ResourceSpecifierDTO) -> Self {
        match dto {
            station_api::ResourceSpecifierDTO::Any => ResourceSpecifier::Any,
            station_api::ResourceSpecifierDTO::Resource(resource) => {
                ResourceSpecifier::Resource(resource.into())
            }
        }
    }
}

impl From<ResourceSpecifier> for station_api::ResourceSpecifierDTO {
    fn from(specifier: ResourceSpecifier) -> Self {
        match specifier {
            ResourceSpecifier::Any => station_api::ResourceSpecifierDTO::Any,
            ResourceSpecifier::Resource(resource) => {
                station_api::ResourceSpecifierDTO::Resource(resource.into())
            }
        }
    }
}

impl ProposalPolicy {
    pub fn to_dto(self) -> station_api::ProposalPolicyDTO {
        station_api::ProposalPolicyDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            specifier: self.specifier.into(),
            criteria: self.criteria.into(),
        }
    }
}

impl From<ProposalSpecifier> for station_api::ProposalSpecifierDTO {
    fn from(specifier: ProposalSpecifier) -> Self {
        match specifier {
            ProposalSpecifier::AddAccount => station_api::ProposalSpecifierDTO::AddAccount,
            ProposalSpecifier::AddUser => station_api::ProposalSpecifierDTO::AddUser,
            ProposalSpecifier::EditAccount(account) => {
                station_api::ProposalSpecifierDTO::EditAccount(account.into())
            }
            ProposalSpecifier::EditUser(user) => {
                station_api::ProposalSpecifierDTO::EditUser(user.into())
            }
            ProposalSpecifier::AddAddressBookEntry => {
                station_api::ProposalSpecifierDTO::AddAddressBookEntry
            }
            ProposalSpecifier::EditAddressBookEntry(address_book_entry) => {
                station_api::ProposalSpecifierDTO::EditAddressBookEntry(address_book_entry.into())
            }
            ProposalSpecifier::RemoveAddressBookEntry(address_book_entry) => {
                station_api::ProposalSpecifierDTO::RemoveAddressBookEntry(address_book_entry.into())
            }
            ProposalSpecifier::Transfer(account) => {
                station_api::ProposalSpecifierDTO::Transfer(account.into())
            }
            ProposalSpecifier::ChangeCanister => station_api::ProposalSpecifierDTO::ChangeCanister,
            ProposalSpecifier::EditAccessPolicy(policy) => {
                station_api::ProposalSpecifierDTO::EditAccessPolicy(policy.into())
            }
            ProposalSpecifier::AddProposalPolicy => {
                station_api::ProposalSpecifierDTO::AddProposalPolicy
            }
            ProposalSpecifier::EditProposalPolicy(policy) => {
                station_api::ProposalSpecifierDTO::EditProposalPolicy(policy.into())
            }
            ProposalSpecifier::RemoveProposalPolicy(policy) => {
                station_api::ProposalSpecifierDTO::RemoveProposalPolicy(policy.into())
            }
            ProposalSpecifier::AddUserGroup => station_api::ProposalSpecifierDTO::AddUserGroup,
            ProposalSpecifier::EditUserGroup(group) => {
                station_api::ProposalSpecifierDTO::EditUserGroup(group.into())
            }
            ProposalSpecifier::RemoveUserGroup(group) => {
                station_api::ProposalSpecifierDTO::RemoveUserGroup(group.into())
            }
        }
    }
}

impl From<station_api::ProposalSpecifierDTO> for ProposalSpecifier {
    fn from(dto: station_api::ProposalSpecifierDTO) -> Self {
        match dto {
            station_api::ProposalSpecifierDTO::AddAccount => ProposalSpecifier::AddAccount,
            station_api::ProposalSpecifierDTO::AddUser => ProposalSpecifier::AddUser,
            station_api::ProposalSpecifierDTO::EditAccount(account) => {
                ProposalSpecifier::EditAccount(account.into())
            }
            station_api::ProposalSpecifierDTO::EditUser(user) => {
                ProposalSpecifier::EditUser(user.into())
            }
            station_api::ProposalSpecifierDTO::AddAddressBookEntry => {
                ProposalSpecifier::AddAddressBookEntry
            }
            station_api::ProposalSpecifierDTO::EditAddressBookEntry(address_book_entry) => {
                ProposalSpecifier::EditAddressBookEntry(address_book_entry.into())
            }
            station_api::ProposalSpecifierDTO::RemoveAddressBookEntry(address_book_entry) => {
                ProposalSpecifier::RemoveAddressBookEntry(address_book_entry.into())
            }
            station_api::ProposalSpecifierDTO::Transfer(transfer_specifier) => {
                ProposalSpecifier::Transfer(transfer_specifier.into())
            }
            station_api::ProposalSpecifierDTO::ChangeCanister => ProposalSpecifier::ChangeCanister,
            station_api::ProposalSpecifierDTO::EditAccessPolicy(policy) => {
                ProposalSpecifier::EditAccessPolicy(policy.into())
            }
            station_api::ProposalSpecifierDTO::AddProposalPolicy => {
                ProposalSpecifier::AddProposalPolicy
            }
            station_api::ProposalSpecifierDTO::EditProposalPolicy(policy) => {
                ProposalSpecifier::EditProposalPolicy(policy.into())
            }
            station_api::ProposalSpecifierDTO::RemoveProposalPolicy(policy) => {
                ProposalSpecifier::RemoveProposalPolicy(policy.into())
            }
            station_api::ProposalSpecifierDTO::AddUserGroup => ProposalSpecifier::AddUserGroup,
            station_api::ProposalSpecifierDTO::EditUserGroup(group) => {
                ProposalSpecifier::EditUserGroup(group.into())
            }
            station_api::ProposalSpecifierDTO::RemoveUserGroup(group) => {
                ProposalSpecifier::RemoveUserGroup(group.into())
            }
        }
    }
}

impl From<ProposalPolicyCallerPrivileges> for station_api::ProposalPolicyCallerPrivilegesDTO {
    fn from(privileges: ProposalPolicyCallerPrivileges) -> Self {
        station_api::ProposalPolicyCallerPrivilegesDTO {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_delete: privileges.can_delete,
            can_edit: privileges.can_edit,
        }
    }
}

impl ProposalSpecifier {
    pub fn to_resources(&self) -> Vec<Resource> {
        match self {
            ProposalSpecifier::AddAccount => vec![Resource::Account(AccountResourceAction::Create)],
            ProposalSpecifier::AddUser => vec![Resource::User(UserResourceAction::Create)],

            ProposalSpecifier::Transfer(account_specifier) => match account_specifier {
                ResourceIds::Any => vec![Resource::Account(AccountResourceAction::Transfer(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| {
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(*id)))
                    })
                    .collect::<_>(),
            },

            ProposalSpecifier::EditUser(user_spec) => match user_spec {
                ResourceIds::Any => {
                    vec![Resource::User(UserResourceAction::Update(ResourceId::Any))]
                }
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| {
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(*id)))
                    })
                    .collect::<_>(),
            },

            ProposalSpecifier::EditAccount(resource_ids) => match resource_ids {
                ResourceIds::Any => vec![Resource::Account(AccountResourceAction::Update(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::Account(AccountResourceAction::Update(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            ProposalSpecifier::AddAddressBookEntry => {
                vec![Resource::AddressBook(ResourceAction::Create)]
            }

            ProposalSpecifier::EditAddressBookEntry(resource_ids) => match resource_ids {
                ResourceIds::Any => vec![Resource::AddressBook(ResourceAction::Update(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::AddressBook(ResourceAction::Update(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            ProposalSpecifier::RemoveAddressBookEntry(resource_ids) => match resource_ids {
                ResourceIds::Any => vec![Resource::AddressBook(ResourceAction::Delete(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::AddressBook(ResourceAction::Delete(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            ProposalSpecifier::ChangeCanister => vec![Resource::ChangeCanister(
                ChangeCanisterResourceAction::Create,
            )],
            ProposalSpecifier::EditAccessPolicy(resource_specifier) => match resource_specifier {
                ResourceSpecifier::Any => {
                    vec![Resource::AccessPolicy(AccessPolicyResourceAction::Update)]
                }
                ResourceSpecifier::Resource(resource) => vec![resource.clone()],
            },
            ProposalSpecifier::AddProposalPolicy => {
                vec![Resource::ProposalPolicy(ResourceAction::Create)]
            }
            ProposalSpecifier::EditProposalPolicy(resources) => match resources {
                ResourceIds::Any => vec![Resource::ProposalPolicy(ResourceAction::Update(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            ProposalSpecifier::RemoveProposalPolicy(resources) => match resources {
                ResourceIds::Any => vec![Resource::ProposalPolicy(ResourceAction::Delete(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            ProposalSpecifier::AddUserGroup => vec![Resource::UserGroup(ResourceAction::Create)],
            ProposalSpecifier::EditUserGroup(resources) => match resources {
                ResourceIds::Any => {
                    vec![Resource::UserGroup(ResourceAction::Update(ResourceId::Any))]
                }
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::UserGroup(ResourceAction::Update(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            ProposalSpecifier::RemoveUserGroup(resources) => match resources {
                ResourceIds::Any => {
                    vec![Resource::UserGroup(ResourceAction::Delete(ResourceId::Any))]
                }
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
        }
    }
}
