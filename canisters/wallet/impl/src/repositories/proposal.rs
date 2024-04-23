use super::indexes::{
    operation_type_to_proposal_id_index::OperationTypeToProposalIdIndexRepository,
    proposal_creation_time_index::ProposalCreationTimeIndexRepository,
    proposal_expiration_time_index::ProposalExpirationTimeIndexRepository,
    proposal_key_creation_time_index::ProposalKeyCreationTimeIndexRepository,
    proposal_key_expiration_time_index::ProposalKeyExpirationTimeIndexRepository,
    proposal_proposer_index::ProposalProposerIndexRepository,
    proposal_resource_index::ProposalResourceIndexRepository,
    proposal_scheduled_index::ProposalScheduledIndexRepository,
    proposal_sort_index::ProposalSortIndexRepository,
    proposal_status_index::ProposalStatusIndexRepository,
    proposal_status_modification_index::ProposalStatusModificationIndexRepository,
    proposal_voter_index::ProposalVoterIndexRepository,
};
use crate::{
    core::{metrics::PROPOSAL_METRICS, with_memory_manager, Memory, PROPOSAL_MEMORY_ID},
    errors::RepositoryError,
    models::{
        indexes::{
            operation_type_to_proposal_id_index::{
                OperationTypeToProposalIdIndex, OperationTypeToProposalIdIndexCriteria,
            },
            proposal_creation_time_index::ProposalCreationTimeIndexCriteria,
            proposal_expiration_time_index::ProposalExpirationTimeIndexCriteria,
            proposal_key_creation_time_index::ProposalKeyCreationTimeIndexCriteria,
            proposal_key_expiration_time_index::ProposalKeyExpirationTimeIndexCriteria,
            proposal_proposer_index::{ProposalProposerIndex, ProposalProposerIndexCriteria},
            proposal_resource_index::ProposalResourceIndexCriteria,
            proposal_scheduled_index::ProposalScheduledIndexCriteria,
            proposal_sort_index::ProposalSortIndexKey,
            proposal_status_index::{ProposalStatusIndex, ProposalStatusIndexCriteria},
            proposal_status_modification_index::ProposalStatusModificationIndexCriteria,
            proposal_voter_index::{ProposalVoterIndex, ProposalVoterIndexCriteria},
        },
        proposal_operation_filter_type::ProposalOperationFilterType,
        resource::Resource,
        Proposal, ProposalId, ProposalKey, ProposalStatusCode, UserId,
    },
};
use ic_canister_core::{
    repository::{
        IdentitySelectionFilter, IndexRepository, NotSelectionFilter, OrSelectionFilter,
        RefreshIndexMode, Repository, SelectionFilter, SortDirection, SortingStrategy,
    },
    types::{Timestamp, UUID},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, collections::HashSet, sync::Arc};
use wallet_api::ListProposalsSortBy;

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalKey, Proposal, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref PROPOSAL_REPOSITORY: Arc<ProposalRepository> =
        Arc::new(ProposalRepository::default());
}

/// A repository that enables managing system proposals in stable memory.
#[derive(Default, Debug)]
pub struct ProposalRepository {
    voter_index: ProposalVoterIndexRepository,
    creation_dt_index: ProposalCreationTimeIndexRepository,
    expiration_dt_index: ProposalExpirationTimeIndexRepository,
    status_index: ProposalStatusIndexRepository,
    scheduled_index: ProposalScheduledIndexRepository,
    proposer_index: ProposalProposerIndexRepository,
    status_modification_index: ProposalStatusModificationIndexRepository,
    prefixed_creation_time_index: ProposalKeyCreationTimeIndexRepository,
    prefixed_expiration_time_index: ProposalKeyExpirationTimeIndexRepository,
    sort_index: ProposalSortIndexRepository,
    resource_index: ProposalResourceIndexRepository,
    operation_type_index: OperationTypeToProposalIdIndexRepository,
}

impl Repository<ProposalKey, Proposal> for ProposalRepository {
    fn list(&self) -> Vec<Proposal> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &ProposalKey) -> Option<Proposal> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: ProposalKey, value: Proposal) -> Option<Proposal> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            // Update metrics when a proposal is upserted.
            PROPOSAL_METRICS
                .iter()
                .for_each(|metric| metric.sum(&value, prev.as_ref()));

            self.voter_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_voters()),
                    current: value.to_index_for_voters(),
                });
            self.operation_type_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_by_operation_types()),
                    current: value.to_index_by_operation_types(),
                });
            self.proposer_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_for_proposer()),
                    current: Some(value.to_index_for_proposer()),
                });
            self.scheduled_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().and_then(|prev| prev.to_index_by_scheduled()),
                    current: value.to_index_by_scheduled(),
                });
            self.creation_dt_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_creation_dt()),
                    current: Some(value.to_index_by_creation_dt()),
                });
            self.prefixed_creation_time_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev
                        .clone()
                        .map(|prev| prev.to_index_by_key_and_creation_dt()),
                    current: Some(value.to_index_by_key_and_creation_dt()),
                });
            self.expiration_dt_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_expiration_dt()),
                    current: Some(value.to_index_by_expiration_dt()),
                });
            self.prefixed_expiration_time_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev
                        .clone()
                        .map(|prev| prev.to_index_by_key_and_expiration_dt()),
                    current: Some(value.to_index_by_key_and_expiration_dt()),
                });
            self.status_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_status()),
                    current: Some(value.to_index_by_status()),
                });
            self.sort_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_for_sorting()),
                    current: Some(value.to_index_for_sorting()),
                });
            self.status_modification_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev
                        .clone()
                        .map(|prev| prev.to_index_by_status_and_modification()),
                    current: Some(value.to_index_by_status_and_modification()),
                });

            self.resource_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map(|prev| prev.to_index_for_resource())
                        .unwrap_or_default(),
                    current: value.to_index_for_resource(),
                });

            prev
        })
    }

    fn remove(&self, key: &ProposalKey) -> Option<Proposal> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            // Update metrics when a proposal is removed.
            if let Some(prev) = &prev {
                PROPOSAL_METRICS.iter().for_each(|metric| metric.sub(prev));
            }

            self.voter_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_voters()),
                });
            self.operation_type_index.refresh_index_on_modification(
                RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_by_operation_types()),
                },
            );
            self.proposer_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_for_proposer()),
                });
            self.scheduled_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().and_then(|prev| prev.to_index_by_scheduled()),
                });
            self.creation_dt_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_creation_dt()),
                });
            self.expiration_dt_index.refresh_index_on_modification(
                RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_expiration_dt()),
                },
            );
            self.prefixed_creation_time_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev
                        .clone()
                        .map(|prev| prev.to_index_by_key_and_creation_dt()),
                });
            self.prefixed_expiration_time_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev
                        .clone()
                        .map(|prev| prev.to_index_by_key_and_expiration_dt()),
                });
            self.status_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_status()),
                });
            self.sort_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_for_sorting()),
                });
            self.status_modification_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev
                        .clone()
                        .map(|prev| prev.to_index_by_status_and_modification()),
                });
            self.resource_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map(|prev| prev.to_index_for_resource())
                        .unwrap_or_default(),
                });

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl ProposalRepository {
    pub fn exists(&self, key: &ProposalKey) -> bool {
        DB.with(|m| m.borrow().contains_key(key))
    }

    pub fn find_by_expiration_dt_and_status(
        &self,
        expiration_dt_from: Option<Timestamp>,
        expiration_dt_to: Option<Timestamp>,
        status: String,
    ) -> Vec<Proposal> {
        let proposals =
            self.expiration_dt_index
                .find_by_criteria(ProposalExpirationTimeIndexCriteria {
                    from_dt: expiration_dt_from,
                    to_dt: expiration_dt_to,
                });

        proposals
            .iter()
            .filter_map(|id| match self.get(&Proposal::key(*id)) {
                Some(proposal) => {
                    if proposal
                        .status
                        .to_type()
                        .to_string()
                        .eq_ignore_ascii_case(status.as_str())
                    {
                        Some(proposal)
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect::<Vec<Proposal>>()
    }

    pub fn find_by_status(
        &self,
        status: ProposalStatusCode,
        from_last_modified_dt: Option<Timestamp>,
        to_last_modified_dt: Option<Timestamp>,
    ) -> Vec<Proposal> {
        let ids = self.status_modification_index.find_by_criteria(
            ProposalStatusModificationIndexCriteria {
                status,
                from_dt: from_last_modified_dt,
                to_dt: to_last_modified_dt,
            },
        );

        ids.iter()
            .filter_map(|id| self.get(&Proposal::key(*id)))
            .collect::<Vec<Proposal>>()
    }

    pub fn find_scheduled(
        &self,
        from_dt: Option<Timestamp>,
        to_dt: Option<Timestamp>,
    ) -> Vec<Proposal> {
        let proposals = self
            .scheduled_index
            .find_by_criteria(ProposalScheduledIndexCriteria { from_dt, to_dt });

        proposals
            .iter()
            .filter_map(|id| self.get(&Proposal::key(*id)))
            .collect::<Vec<Proposal>>()
    }

    /// Checks if the proposal is of the provided status.
    pub fn exists_status(&self, proposal_id: &ProposalId, status: ProposalStatusCode) -> bool {
        self.status_index.exists(&ProposalStatusIndex {
            proposal_id: *proposal_id,
            status,
        })
    }

    /// Get the list of Resource for a proposal id.
    pub fn get_resources(&self, proposal_id: &ProposalId) -> Vec<Resource> {
        self.resource_index
            .find_by_criteria(ProposalResourceIndexCriteria {
                proposal_id: *proposal_id,
            })
            .into_iter()
            .collect()
    }

    /// Checks if the user has voted on the proposal.
    pub fn exists_voter(&self, proposal_id: &ProposalId, voter_id: &UserId) -> bool {
        self.voter_index.exists(&ProposalVoterIndex {
            voter_id: *voter_id,
            proposal_id: *proposal_id,
        })
    }

    /// Checks if the user has proposed the proposal.
    pub fn exists_proposer(&self, proposal_id: &ProposalId, proposer_id: &UserId) -> bool {
        self.proposer_index.exists(&ProposalProposerIndex {
            proposer_id: *proposer_id,
            proposal_id: *proposal_id,
        })
    }

    pub fn find_ids_where(
        &self,
        condition: ProposalWhereClause,
        sort_by: Option<ListProposalsSortBy>,
    ) -> Result<Vec<UUID>, RepositoryError> {
        let filters = self.build_where_filtering_strategy(condition);
        let proposal_ids = self.find_with_filters(filters);
        let mut ids = proposal_ids.into_iter().collect::<Vec<_>>();

        self.sort_ids_with_strategy(&mut ids, &sort_by);

        Ok(ids)
    }

    /// Sorts the proposal IDs based on the provided sort strategy.
    ///
    /// If no sort strategy is provided, it defaults to sorting by creation timestamp descending.
    fn sort_ids_with_strategy(
        &self,
        proposal_ids: &mut [UUID],
        sort_by: &Option<ListProposalsSortBy>,
    ) {
        match sort_by {
            Some(wallet_api::ListProposalsSortBy::CreatedAt(direction)) => {
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Creation,
                    direction: match direction {
                        wallet_api::SortDirection::Asc => Some(SortDirection::Ascending),
                        wallet_api::SortDirection::Desc => Some(SortDirection::Descending),
                    },
                };

                sort_strategy.sort(proposal_ids);
            }
            Some(wallet_api::ListProposalsSortBy::ExpirationDt(direction)) => {
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Expiration,
                    direction: match direction {
                        wallet_api::SortDirection::Asc => Some(SortDirection::Ascending),
                        wallet_api::SortDirection::Desc => Some(SortDirection::Descending),
                    },
                };

                sort_strategy.sort(proposal_ids);
            }
            Some(wallet_api::ListProposalsSortBy::LastModificationDt(direction)) => {
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Modification,
                    direction: match direction {
                        wallet_api::SortDirection::Asc => Some(SortDirection::Ascending),
                        wallet_api::SortDirection::Desc => Some(SortDirection::Descending),
                    },
                };

                sort_strategy.sort(proposal_ids);
            }
            None => {
                // Default sort by creation timestamp descending
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Creation,
                    direction: Some(SortDirection::Descending),
                };

                sort_strategy.sort(proposal_ids);
            }
        }
    }

    fn build_where_filtering_strategy<'a>(
        &'a self,
        condition: ProposalWhereClause,
    ) -> Vec<Box<dyn SelectionFilter<'a, IdType = UUID> + 'a>> {
        let mut filters = Vec::new();

        if condition.created_dt_from.is_some() || condition.created_dt_to.is_some() {
            filters.push(Box::new(CreationDtSelectionFilter {
                repository: &self.creation_dt_index,
                prefixed_repository: &self.prefixed_creation_time_index,
                from: condition.created_dt_from,
                to: condition.created_dt_to,
            }) as Box<dyn SelectionFilter<IdType = UUID>>);
        }

        if condition.expiration_dt_from.is_some() || condition.expiration_dt_to.is_some() {
            filters.push(Box::new(ExpirationDtSelectionFilter {
                repository: &self.expiration_dt_index,
                prefixed_repository: &self.prefixed_expiration_time_index,
                from: condition.expiration_dt_from,
                to: condition.expiration_dt_to,
            }) as Box<dyn SelectionFilter<IdType = UUID>>);
        }

        if !condition.statuses.is_empty() {
            let includes_status = Box::new(OrSelectionFilter {
                filters: condition
                    .statuses
                    .iter()
                    .map(|status| {
                        Box::new(StatusSelectionFilter {
                            repository: &self.status_index,
                            status: status.to_owned(),
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(includes_status);
        }

        if !condition.operation_types.is_empty() {
            let includes_operation_type = Box::new(OrSelectionFilter {
                filters: condition
                    .operation_types
                    .iter()
                    .map(|operation_type| {
                        Box::new(OperationTypeSelectionFilter {
                            repository: &self.operation_type_index,
                            operation_type: operation_type.to_owned(),
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(includes_operation_type);
        }

        if !condition.voters.is_empty() {
            let includes_voter = Box::new(OrSelectionFilter {
                filters: condition
                    .voters
                    .iter()
                    .map(|voter_id| {
                        Box::new(VoterSelectionFilter {
                            repository: &self.voter_index,
                            voter_id: *voter_id,
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(includes_voter);
        }

        if !condition.proposers.is_empty() {
            let includes_proposer = Box::new(OrSelectionFilter {
                filters: condition
                    .proposers
                    .iter()
                    .map(|proposer_id| {
                        Box::new(ProposerSelectionFilter {
                            repository: &self.proposer_index,
                            proposer_id: *proposer_id,
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(includes_proposer);
        }

        if filters.is_empty() {
            // If no filters are provided, return all
            filters.push(Box::new(CreationDtSelectionFilter {
                repository: &self.creation_dt_index,
                prefixed_repository: &self.prefixed_creation_time_index,
                from: None,
                to: None,
            }) as Box<dyn SelectionFilter<IdType = UUID>>);
        }

        // NotSelectionFilter doesn't select anything, only filters
        if !condition.excluded_ids.is_empty() {
            let excludes_ids = Box::new(NotSelectionFilter {
                input: Box::new(IdentitySelectionFilter {
                    ids: condition.excluded_ids.iter().cloned().collect(),
                }),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(excludes_ids);
        }

        if !condition.not_voters.is_empty() {
            let excludes_voter = Box::new(NotSelectionFilter {
                input: Box::new(OrSelectionFilter {
                    filters: condition
                        .not_voters
                        .iter()
                        .map(|voter_id| {
                            Box::new(VoterSelectionFilter {
                                repository: &self.voter_index,
                                voter_id: *voter_id,
                            })
                                as Box<dyn SelectionFilter<IdType = UUID>>
                        })
                        .collect(),
                }),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(excludes_voter);
        }

        if !condition.not_proposers.is_empty() {
            let excludes_proposer = Box::new(NotSelectionFilter {
                input: Box::new(OrSelectionFilter {
                    filters: condition
                        .not_proposers
                        .iter()
                        .map(|proposer_id| {
                            Box::new(ProposerSelectionFilter {
                                repository: &self.proposer_index,
                                proposer_id: *proposer_id,
                            })
                                as Box<dyn SelectionFilter<IdType = UUID>>
                        })
                        .collect(),
                }),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(excludes_proposer);
        }

        filters
    }
}

#[derive(Debug, Clone)]
pub struct ProposalWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub expiration_dt_from: Option<Timestamp>,
    pub expiration_dt_to: Option<Timestamp>,
    pub operation_types: Vec<ProposalOperationFilterType>,
    pub statuses: Vec<ProposalStatusCode>,
    pub voters: Vec<UUID>,
    pub not_voters: Vec<UUID>,
    pub proposers: Vec<UUID>,
    pub not_proposers: Vec<UUID>,
    pub excluded_ids: Vec<UUID>,
}

#[derive(Debug, Clone)]
pub(crate) struct CreationDtSelectionFilter<'a> {
    repository: &'a ProposalCreationTimeIndexRepository,
    prefixed_repository: &'a ProposalKeyCreationTimeIndexRepository,
    from: Option<Timestamp>,
    to: Option<Timestamp>,
}

impl<'a> SelectionFilter<'a> for CreationDtSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.prefixed_repository
            .exists_by_criteria(ProposalKeyCreationTimeIndexCriteria {
                proposal_id: *id,
                from_dt: self.from,
                to_dt: self.to,
            })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(ProposalCreationTimeIndexCriteria {
                from_dt: self.from,
                to_dt: self.to,
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExpirationDtSelectionFilter<'a> {
    repository: &'a ProposalExpirationTimeIndexRepository,
    prefixed_repository: &'a ProposalKeyExpirationTimeIndexRepository,
    from: Option<Timestamp>,
    to: Option<Timestamp>,
}

impl<'a> SelectionFilter<'a> for ExpirationDtSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.prefixed_repository
            .exists_by_criteria(ProposalKeyExpirationTimeIndexCriteria {
                proposal_id: *id,
                from_dt: self.from,
                to_dt: self.to,
            })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(ProposalExpirationTimeIndexCriteria {
                from_dt: self.from,
                to_dt: self.to,
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OperationTypeSelectionFilter<'a> {
    repository: &'a OperationTypeToProposalIdIndexRepository,
    operation_type: ProposalOperationFilterType,
}

impl<'a> SelectionFilter<'a> for OperationTypeSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&OperationTypeToProposalIdIndex {
            operation_type: self.operation_type.clone(),
            proposal_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(OperationTypeToProposalIdIndexCriteria {
                operation_type: self.operation_type.clone(),
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VoterSelectionFilter<'a> {
    repository: &'a ProposalVoterIndexRepository,
    voter_id: UUID,
}

impl<'a> SelectionFilter<'a> for VoterSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&ProposalVoterIndex {
            voter_id: self.voter_id,
            proposal_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(ProposalVoterIndexCriteria {
                voter_id: self.voter_id,
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ProposerSelectionFilter<'a> {
    repository: &'a ProposalProposerIndexRepository,
    proposer_id: UUID,
}

impl<'a> SelectionFilter<'a> for ProposerSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&ProposalProposerIndex {
            proposer_id: self.proposer_id,
            proposal_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(ProposalProposerIndexCriteria {
                proposer_id: self.proposer_id,
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StatusSelectionFilter<'a> {
    repository: &'a ProposalStatusIndexRepository,
    status: ProposalStatusCode,
}

impl<'a> SelectionFilter<'a> for StatusSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&ProposalStatusIndex {
            status: self.status.to_owned(),
            proposal_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(ProposalStatusIndexCriteria {
                status: self.status.to_owned(),
            })
    }
}

#[derive(Debug, Clone)]
enum TimestampType {
    Creation,
    Expiration,
    Modification,
}

#[derive(Debug, Clone)]
struct TimestampSortingStrategy<'a> {
    index: &'a ProposalSortIndexRepository,
    timestamp_type: TimestampType,
    direction: Option<SortDirection>,
}

impl<'a> SortingStrategy<'a> for TimestampSortingStrategy<'a> {
    type IdType = UUID;

    fn sort(&self, ids: &mut [Self::IdType]) {
        let direction = self.direction.unwrap_or(SortDirection::Ascending);
        let mut id_with_timestamps: Vec<(Timestamp, Self::IdType)> = ids
            .iter()
            .map(|id| {
                let key = ProposalSortIndexKey { proposal_id: *id };
                let timestamp = self
                    .index
                    .get(&key)
                    .map(|index| match self.timestamp_type {
                        TimestampType::Creation => index.creation_timestamp,
                        TimestampType::Expiration => index.expiration_timestamp,
                        TimestampType::Modification => index.modification_timestamp,
                    })
                    .unwrap_or_default();
                (timestamp, *id)
            })
            .collect();

        id_with_timestamps.sort_by(|a, b| {
            {
                let ord = a.0.cmp(&b.0); // Compare timestamps
                match direction {
                    SortDirection::Ascending => ord,
                    SortDirection::Descending => ord.reverse(),
                }
            }
            .then_with(|| a.1.cmp(&b.1)) // Compare proposal IDs if timestamps are equal
        });

        let sorted_ids: Vec<UUID> = id_with_timestamps.into_iter().map(|(_, id)| id).collect();
        ids.copy_from_slice(&sorted_ids);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        indexes::proposal_resource_index::ProposalResourceIndex,
        proposal_test_utils::{self, mock_proposal},
        resource::{AccountResourceAction, ResourceId},
        AddUserGroupOperation, AddUserGroupOperationInput, EditUserGroupOperation,
        EditUserGroupOperationInput, ProposalOperation, ProposalStatus, TransferOperation,
        TransferOperationInput,
    };
    use uuid::Uuid;

    #[test]
    fn perform_crud() {
        let repository = ProposalRepository::default();
        let proposal = mock_proposal();

        assert!(repository.get(&proposal.to_key()).is_none());

        repository.insert(proposal.to_key(), proposal.clone());

        assert!(repository.get(&proposal.to_key()).is_some());
        assert!(repository.remove(&proposal.to_key()).is_some());
        assert!(repository.get(&proposal.to_key()).is_none());
    }

    #[test]
    fn find_by_expiration_dt_and_status() {
        let repository = ProposalRepository::default();
        for i in 0..=50 {
            let mut proposal = proposal_test_utils::mock_proposal();
            proposal.id = *Uuid::new_v4().as_bytes();
            proposal.expiration_dt = i;
            proposal.status = ProposalStatus::Created;
            repository.insert(proposal.to_key(), proposal.clone());
        }

        let last_six = repository.find_by_expiration_dt_and_status(
            Some(45),
            None,
            ProposalStatusCode::Created.to_string(),
        );

        let middle_eleven = repository.find_by_expiration_dt_and_status(
            Some(30),
            Some(40),
            ProposalStatusCode::Created.to_string(),
        );

        let first_three = repository.find_by_expiration_dt_and_status(
            None,
            Some(2),
            ProposalStatusCode::Created.to_string(),
        );

        assert_eq!(last_six.len(), 6);
        assert_eq!(middle_eleven.len(), 11);
        assert_eq!(first_three.len(), 3);
    }

    #[test]
    fn no_future_expiration_dt() {
        let repository = ProposalRepository::default();
        let mut proposal = proposal_test_utils::mock_proposal();
        proposal.expiration_dt = 10;

        repository.insert(proposal.to_key(), proposal.clone());

        let proposals = repository.find_by_expiration_dt_and_status(
            Some(20),
            None,
            proposal.status.to_type().to_string(),
        );

        assert!(proposals.is_empty());
    }

    #[test]
    fn find_with_expiration_dt() {
        let mut proposal = proposal_test_utils::mock_proposal();
        proposal.id = *Uuid::new_v4().as_bytes();
        proposal.created_timestamp = 5;
        proposal.expiration_dt = 10;
        proposal.status = ProposalStatus::Created;
        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        let mut proposal_not_match = proposal_test_utils::mock_proposal();
        proposal_not_match.id = *Uuid::new_v4().as_bytes();
        proposal_not_match.created_timestamp = 5;
        proposal_not_match.expiration_dt = 9;
        proposal_not_match.status = ProposalStatus::Created;
        PROPOSAL_REPOSITORY.insert(proposal_not_match.to_key(), proposal_not_match.clone());

        let mut condition = ProposalWhereClause {
            created_dt_from: None,
            created_dt_to: None,
            expiration_dt_from: Some(10),
            expiration_dt_to: None,
            operation_types: vec![],
            statuses: vec![],
            voters: vec![],
            not_voters: vec![],
            proposers: vec![],
            not_proposers: vec![],
            excluded_ids: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 1);

        let found_proposal = PROPOSAL_REPOSITORY
            .get(&ProposalKey { id: proposals[0] })
            .unwrap();

        assert_eq!(found_proposal, proposal);

        condition.expiration_dt_from = Some(11);

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert!(proposals.is_empty());
    }

    #[test]
    fn find_with_creation_dt() {
        let mut proposal = proposal_test_utils::mock_proposal();
        proposal.id = *Uuid::new_v4().as_bytes();
        proposal.created_timestamp = 10;
        proposal.status = ProposalStatus::Created;
        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        let mut proposal_not_match = proposal_test_utils::mock_proposal();
        proposal_not_match.id = *Uuid::new_v4().as_bytes();
        proposal_not_match.created_timestamp = 12;
        proposal_not_match.status = ProposalStatus::Created;
        PROPOSAL_REPOSITORY.insert(proposal_not_match.to_key(), proposal_not_match.clone());

        let mut condition = ProposalWhereClause {
            created_dt_from: Some(9),
            created_dt_to: Some(11),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: vec![],
            statuses: vec![],
            voters: vec![],
            not_voters: vec![],
            proposers: vec![],
            not_proposers: vec![],
            excluded_ids: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 1);

        let found_proposal = PROPOSAL_REPOSITORY
            .get(&ProposalKey { id: proposals[0] })
            .unwrap();

        assert_eq!(found_proposal, proposal);

        condition.created_dt_from = Some(8);
        condition.created_dt_to = Some(9);

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert!(proposals.is_empty());
    }

    #[test]
    fn find_with_default_filters() {
        for i in 0..100 {
            let mut proposal = mock_proposal();
            proposal.id = *Uuid::new_v4().as_bytes();
            proposal.created_timestamp = i;
            proposal.expiration_dt = i + 100;
            proposal.status = match i % 2 {
                0 => ProposalStatus::Created,
                1 => ProposalStatus::Adopted,
                _ => ProposalStatus::Rejected,
            };

            PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());
        }

        let condition = ProposalWhereClause {
            created_dt_from: Some(50),
            created_dt_to: Some(100),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: Vec::new(),
            proposers: Vec::new(),
            voters: Vec::new(),
            not_voters: vec![],
            statuses: vec![ProposalStatusCode::Created],
            not_proposers: vec![],
            excluded_ids: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 25);

        let condition = ProposalWhereClause {
            created_dt_from: Some(0),
            created_dt_to: Some(100),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: Vec::new(),
            proposers: Vec::new(),
            voters: Vec::new(),
            not_voters: vec![],
            statuses: vec![ProposalStatusCode::Adopted],
            not_proposers: vec![],
            excluded_ids: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 50);

        let condition = ProposalWhereClause {
            created_dt_from: Some(0),
            created_dt_to: Some(100),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: Vec::new(),
            proposers: Vec::new(),
            voters: Vec::new(),
            not_voters: vec![],
            statuses: vec![ProposalStatusCode::Adopted, ProposalStatusCode::Created],
            not_proposers: vec![],
            excluded_ids: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 100);

        let condition = ProposalWhereClause {
            created_dt_from: Some(0),
            created_dt_to: Some(100),
            expiration_dt_from: Some(110),
            expiration_dt_to: Some(120),
            operation_types: Vec::new(),
            proposers: Vec::new(),
            voters: Vec::new(),
            not_voters: vec![],
            statuses: vec![ProposalStatusCode::Adopted],
            not_proposers: vec![],
            excluded_ids: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 5);
    }

    #[test]
    fn find_with_empty_where_clause_should_return_all() {
        proposal_repository_test_utils::add_proposals_to_repository(100);

        let condition = ProposalWhereClause {
            created_dt_from: None,
            created_dt_to: None,
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: vec![],
            statuses: vec![],
            voters: vec![],
            not_voters: vec![],
            proposers: vec![],
            not_proposers: vec![],
            excluded_ids: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 100);
    }

    #[test]
    fn update_resource_index_on_proposal_creation() {
        let proposal = mock_proposal();

        let ProposalOperation::Transfer(TransferOperation {
            input: TransferOperationInput {
                from_account_id, ..
            },
            ..
        }) = proposal.operation
        else {
            panic!("Expected transfer operation");
        };

        assert!(matches!(proposal.operation, ProposalOperation::Transfer(_)));

        PROPOSAL_REPOSITORY.insert(ProposalKey { id: proposal.id }, proposal.clone());

        assert!(PROPOSAL_REPOSITORY
            .resource_index
            .exists(&ProposalResourceIndex {
                proposal_id: proposal.id,
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
            }));

        assert!(PROPOSAL_REPOSITORY
            .resource_index
            .exists(&ProposalResourceIndex {
                proposal_id: proposal.id,
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    from_account_id,
                ))),
            }));
    }

    #[test]
    fn find_only_specified_types() {
        let mut add_group_proposal = mock_proposal();
        add_group_proposal.last_modification_timestamp = 1;
        add_group_proposal.status = ProposalStatus::Adopted;
        add_group_proposal.operation = ProposalOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "foo".to_string(),
            },
        });
        PROPOSAL_REPOSITORY.insert(add_group_proposal.to_key(), add_group_proposal.clone());

        let mut edit_group_proposal = mock_proposal();
        edit_group_proposal.last_modification_timestamp = 2;
        add_group_proposal.status = ProposalStatus::Adopted;
        edit_group_proposal.operation = ProposalOperation::EditUserGroup(EditUserGroupOperation {
            input: EditUserGroupOperationInput {
                user_group_id: *Uuid::new_v4().as_bytes(),
                name: "bar".to_string(),
            },
        });
        PROPOSAL_REPOSITORY.insert(edit_group_proposal.to_key(), edit_group_proposal.clone());

        let condition = ProposalWhereClause {
            created_dt_from: Some(0),
            created_dt_to: Some(100),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: vec![ProposalOperationFilterType::AddUserGroup],
            proposers: Vec::new(),
            voters: Vec::new(),
            not_voters: vec![],
            statuses: vec![ProposalStatusCode::Adopted],
            not_proposers: vec![],
            excluded_ids: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0], add_group_proposal.id);
    }
}

#[cfg(feature = "canbench")]
mod benchs {
    use super::*;
    use crate::models::{proposal_test_utils::mock_proposal, ProposalStatus};
    use canbench_rs::{bench, BenchResult};
    use uuid::Uuid;

    #[bench(raw)]
    fn repository_batch_insert_100_proposals() -> BenchResult {
        canbench_rs::bench_fn(|| {
            proposal_repository_test_utils::add_proposals_to_repository(100);
        })
    }

    #[bench(raw)]
    fn repository_list_all_proposals() -> BenchResult {
        proposal_repository_test_utils::add_proposals_to_repository(1_000);

        canbench_rs::bench_fn(|| {
            let _ = PROPOSAL_REPOSITORY.list();
        })
    }

    #[bench(raw)]
    fn repository_filter_all_proposal_ids_by_default_filters() -> BenchResult {
        for i in 0..2_500 {
            let mut proposal = mock_proposal();
            proposal.id = *Uuid::new_v4().as_bytes();
            proposal.created_timestamp = i;
            proposal.status = match i % 2 {
                0 => ProposalStatus::Created,
                1 => ProposalStatus::Adopted,
                _ => ProposalStatus::Rejected,
            };

            PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());
        }

        canbench_rs::bench_fn(|| {
            let _ = PROPOSAL_REPOSITORY.find_ids_where(
                ProposalWhereClause {
                    created_dt_from: Some(500),
                    created_dt_to: Some(1500),
                    expiration_dt_from: None,
                    expiration_dt_to: None,
                    operation_types: Vec::new(),
                    proposers: Vec::new(),
                    voters: Vec::new(),
                    not_voters: vec![],
                    statuses: vec![ProposalStatusCode::Created],
                    excluded_ids: vec![],
                    not_proposers: vec![],
                },
                None,
            );
        })
    }
}

#[cfg(any(test, feature = "canbench"))]
mod proposal_repository_test_utils {
    use super::*;
    use crate::models::proposal_test_utils::mock_proposal;
    use uuid::Uuid;

    pub fn add_proposals_to_repository(count: usize) {
        for _ in 0..count {
            let mut proposal = mock_proposal();
            proposal.id = *Uuid::new_v4().as_bytes();

            PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());
        }
    }
}
