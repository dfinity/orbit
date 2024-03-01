use super::indexes::{
    proposal_account_index::ProposalAccountIndexRepository,
    proposal_creation_time_index::ProposalCreationTimeIndexRepository,
    proposal_expiration_time_index::ProposalExpirationTimeIndexRepository,
    proposal_proposer_index::ProposalProposerIndexRepository,
    proposal_scheduled_index::ProposalScheduledIndexRepository,
    proposal_status_index::ProposalStatusIndexRepository,
    proposal_voter_index::ProposalVoterIndexRepository,
};
use crate::{
    core::{utils::match_date_range, with_memory_manager, Memory, PROPOSAL_MEMORY_ID},
    errors::{MapperError, RepositoryError},
    mappers::{HelperMapper, ProposalStatusMapper},
    models::{
        indexes::{
            proposal_account_index::{ProposalAccountIndex, ProposalAccountIndexCriteria},
            proposal_creation_time_index::ProposalCreationTimeIndexCriteria,
            proposal_expiration_time_index::{
                ProposalExpirationTimeIndex, ProposalExpirationTimeIndexCriteria,
            },
            proposal_proposer_index::{ProposalProposerIndex, ProposalProposerIndexCriteria},
            proposal_scheduled_index::ProposalScheduledIndexCriteria,
            proposal_status_index::{ProposalStatusIndex, ProposalStatusIndexCriteria},
            proposal_voter_index::{ProposalVoterIndex, ProposalVoterIndexCriteria},
        },
        AccountId, Proposal, ProposalKey, ProposalStatusCode, UserId,
    },
};
use ic_canister_core::{
    repository::{IndexRepository, RefreshIndexMode, Repository},
    types::{Timestamp, UUID},
};
use ic_stable_structures::{memory_manager::VirtualMemory, Cell, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, collections::HashSet, sync::Arc};
use wallet_api::{ListProposalsOperationTypeDTO, ListProposalsSortBy};

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
    account_index: ProposalAccountIndexRepository,
    creation_dt_index: ProposalCreationTimeIndexRepository,
    expiration_dt_index: ProposalExpirationTimeIndexRepository,
    status_index: ProposalStatusIndexRepository,
    scheduled_index: ProposalScheduledIndexRepository,
    proposer_index: ProposalProposerIndexRepository,
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
            self.voter_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_voters()),
                    current: value.to_index_for_voters(),
                });
            self.proposer_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_for_proposer()),
                    current: Some(value.to_index_for_proposer()),
                });
            self.account_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().and_then(|prev| prev.to_index_for_account()),
                    current: value.to_index_for_account(),
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
            self.expiration_dt_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_expiration_dt()),
                    current: Some(value.to_index_by_expiration_dt()),
                });
            self.status_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().clone().map(|prev| prev.to_index_by_status()),
                    current: Some(value.to_index_by_status()),
                });

            prev
        })
    }

    fn remove(&self, key: &ProposalKey) -> Option<Proposal> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);
            self.voter_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_voters()),
                });
            self.proposer_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_for_proposer()),
                });
            self.account_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().and_then(|prev| prev.to_index_for_account()),
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
            self.status_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_status()),
                });

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl ProposalRepository {
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
        status: String,
        from_last_modified_dt: Option<Timestamp>,
        to_last_modified_dt: Option<Timestamp>,
    ) -> Vec<Proposal> {
        todo!()
        // let proposals = self
        //     .status_index
        //     .find_by_criteria(ProposalStatusIndexCriteria {
        //         status: status.to_owned(),
        //         from_dt: from_last_update_dt,
        //         to_dt: to_last_update_dt,
        //     });

        // proposals
        //     .iter()
        //     .filter_map(|id| self.get(&Proposal::key(*id)))
        //     .collect::<Vec<Proposal>>()
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

    pub fn find_where(
        &self,
        _condition: ProposalWhereClause,
        _sort_by: Option<ListProposalsSortBy>,
    ) -> Result<Vec<Proposal>, RepositoryError> {
        let proposals = vec![];

        Ok(proposals)
    }

    pub fn find_ids_where(
        &self,
        condition: ProposalWhereClause,
        _sort_by: Option<ListProposalsSortBy>,
    ) -> Result<Vec<UUID>, RepositoryError> {
        let strategy = self.build_where_filtering_strategy(&condition);
        let proposal_ids = self.find_with_strategy(strategy, &mut None)?;
        // let proposal_ids = self.find_with_strategy(strategy, &condition)?;

        Ok(proposal_ids.into_iter().collect())
    }

    // fn sort_proposals(&self, proposals: &mut [Proposal], sort_by: &Option<ListProposalsSortBy>) {
    //     match sort_by {
    //         Some(wallet_api::ListProposalsSortBy::CreatedAt(direction)) => {
    //             proposals.sort_by(|a, b| match direction {
    //                 wallet_api::SortDirection::Asc => a.created_timestamp.cmp(&b.created_timestamp),
    //                 wallet_api::SortDirection::Desc => {
    //                     b.created_timestamp.cmp(&a.created_timestamp)
    //                 }
    //             });
    //         }
    //         Some(wallet_api::ListProposalsSortBy::ExpirationDt(direction)) => {
    //             proposals.sort_by(|a, b| match direction {
    //                 wallet_api::SortDirection::Asc => a.expiration_dt.cmp(&b.expiration_dt),
    //                 wallet_api::SortDirection::Desc => b.expiration_dt.cmp(&a.expiration_dt),
    //             });
    //         }
    //         Some(wallet_api::ListProposalsSortBy::LastModificationDt(direction)) => {
    //             proposals.sort_by(|a, b| match direction {
    //                 wallet_api::SortDirection::Asc => a
    //                     .last_modification_timestamp
    //                     .cmp(&b.last_modification_timestamp),
    //                 wallet_api::SortDirection::Desc => b
    //                     .last_modification_timestamp
    //                     .cmp(&a.last_modification_timestamp),
    //             });
    //         }
    //         None => {
    //             // Default sort by created timestamp descending
    //             proposals.sort_by(|a, b| b.created_timestamp.cmp(&a.created_timestamp));
    //         }
    //     }
    // }

    fn build_where_filtering_strategy(
        &self,
        condition: &ProposalWhereClause,
    ) -> WhereSelectionStrategy {
        let mut and = Vec::new();

        if condition.created_dt_from.is_some() || condition.created_dt_to.is_some() {
            and.push(WhereSelectionStrategy::CreationDt {
                from: condition.created_dt_from,
                to: condition.created_dt_to,
            });
        }

        if condition.expiration_dt_from.is_some() || condition.expiration_dt_to.is_some() {
            and.push(WhereSelectionStrategy::ExpirationDt {
                from: condition.expiration_dt_from,
                to: condition.expiration_dt_to,
            });
        }

        if !condition.statuses.is_empty() {
            and.push(WhereSelectionStrategy::Or(
                condition
                    .statuses
                    .iter()
                    .map(|status| WhereSelectionStrategy::Status(status.to_owned()))
                    .collect(),
            ));
        }

        if !condition.account_ids().unwrap_or_default().is_empty() {
            and.push(WhereSelectionStrategy::Or(
                condition
                    .account_ids()
                    .unwrap_or_default()
                    .iter()
                    .map(|account_id| WhereSelectionStrategy::Account(*account_id))
                    .collect(),
            ));
        }

        if !condition.voters.is_empty() {
            and.push(WhereSelectionStrategy::Or(
                condition
                    .voters
                    .iter()
                    .map(|voter| WhereSelectionStrategy::Voter(*voter))
                    .collect(),
            ));
        }

        if !condition.proposers.is_empty() {
            and.push(WhereSelectionStrategy::Or(
                condition
                    .proposers
                    .iter()
                    .map(|proposer| WhereSelectionStrategy::Proposer(*proposer))
                    .collect(),
            ));
        }

        WhereSelectionStrategy::And(and)
    }

    fn find_with_strategy(
        &self,
        strategy: WhereSelectionStrategy,
        selected_ids: &mut Option<HashSet<UUID>>,
    ) -> Result<HashSet<UUID>, RepositoryError> {
        let proposal_ids = match strategy {
            WhereSelectionStrategy::Account(account_id) => match selected_ids {
                Some(ids) => ids
                    .iter()
                    .filter(|id| {
                        self.account_index.exists(&ProposalAccountIndex {
                            account_id,
                            proposal_id: **id,
                        })
                    })
                    .cloned()
                    .collect::<HashSet<UUID>>(),
                None => self
                    .account_index
                    .find_by_criteria(ProposalAccountIndexCriteria { account_id }),
            },
            WhereSelectionStrategy::Voter(voter_id) => match selected_ids {
                Some(ids) => ids
                    .iter()
                    .filter(|id| {
                        self.voter_index.exists(&ProposalVoterIndex {
                            voter_id,
                            proposal_id: **id,
                        })
                    })
                    .cloned()
                    .collect::<HashSet<UUID>>(),
                None => self
                    .voter_index
                    .find_by_criteria(ProposalVoterIndexCriteria { voter_id }),
            },
            WhereSelectionStrategy::Proposer(proposer_id) => match selected_ids {
                Some(ids) => ids
                    .iter()
                    .filter(|id| {
                        self.proposer_index.exists(&ProposalProposerIndex {
                            proposer_id,
                            proposal_id: **id,
                        })
                    })
                    .cloned()
                    .collect::<HashSet<UUID>>(),
                None => self
                    .proposer_index
                    .find_by_criteria(ProposalProposerIndexCriteria { proposer_id }),
            },
            WhereSelectionStrategy::Status(status) => match selected_ids {
                Some(ids) => ids
                    .iter()
                    .filter(|id| {
                        self.status_index.exists(&ProposalStatusIndex {
                            status: ProposalStatusMapper::from_status_code_dto(&status).to_string(),
                            proposal_id: **id,
                        })
                    })
                    .cloned()
                    .collect::<HashSet<UUID>>(),
                None => self
                    .status_index
                    .find_by_criteria(ProposalStatusIndexCriteria {
                        status: ProposalStatusMapper::from_status_code_dto(&status).to_string(),
                    }),
            },
            WhereSelectionStrategy::ExpirationDt { from, to } => match selected_ids {
                Some(_ids) => todo!(),
                None => {
                    self.expiration_dt_index
                        .find_by_criteria(ProposalExpirationTimeIndexCriteria {
                            from_dt: from,
                            to_dt: to,
                        })
                }
            },
            WhereSelectionStrategy::CreationDt { from, to } => match selected_ids {
                Some(_ids) => todo!(),
                None => {
                    self.creation_dt_index
                        .find_by_criteria(ProposalCreationTimeIndexCriteria {
                            from_dt: from,
                            to_dt: to,
                        })
                }
            },
            WhereSelectionStrategy::And(strategies) => {
                let mut temp_ids: Option<HashSet<UUID>> = None;
                if strategies.is_empty() && selected_ids.is_none() {
                    return Ok(self.creation_dt_index.find_by_criteria(
                        ProposalCreationTimeIndexCriteria {
                            from_dt: None,
                            to_dt: None,
                        },
                    ));
                }

                for strategy in strategies {
                    // If temp_ids is None, it means this is the first iteration, or no IDs have been found yet.
                    // In such a case, directly use the result of find_with_strategy.
                    // Otherwise, intersect the current temp_ids with the new set of found IDs.
                    let found_ids = self.find_with_strategy(strategy, &mut temp_ids.clone())?;

                    temp_ids = match &temp_ids {
                        Some(current_ids) => {
                            // If temp_ids is already Some, intersect it with found_ids
                            Some(current_ids.intersection(&found_ids).cloned().collect())
                        }
                        None => {
                            // If this is the first strategy or no ids have been found, use found_ids directly
                            Some(found_ids)
                        }
                    };

                    // If after any strategy, no IDs are left, break early as the intersection will remain empty.
                    if temp_ids.as_ref().map_or(false, |ids| ids.is_empty()) {
                        break;
                    }
                }

                temp_ids.unwrap_or_default()
            }
            WhereSelectionStrategy::Or(strategies) => {
                let mut found_ids = HashSet::<UUID>::new();
                if strategies.is_empty() && selected_ids.is_none() {
                    return Ok(self.creation_dt_index.find_by_criteria(
                        ProposalCreationTimeIndexCriteria {
                            from_dt: None,
                            to_dt: None,
                        },
                    ));
                }

                for strategy in strategies {
                    let ids = self.find_with_strategy(strategy, selected_ids)?;
                    found_ids.extend(ids);
                }

                match selected_ids {
                    Some(existing_ids) => {
                        // If selected_ids is Some, intersect found_ids with it
                        found_ids = found_ids.intersection(existing_ids).cloned().collect();
                    }
                    None => {
                        // If selected_ids is None, just use found_ids as is
                    }
                }

                found_ids
            }
        };

        Ok(proposal_ids)
    }
}

#[derive(Debug, Clone)]
pub struct ProposalWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub expiration_dt_from: Option<Timestamp>,
    pub expiration_dt_to: Option<Timestamp>,
    pub operation_types: Vec<ListProposalsOperationTypeDTO>,
    pub statuses: Vec<ProposalStatusCode>,
    pub voters: Vec<UUID>,
    pub proposers: Vec<UUID>,
}

impl ProposalWhereClause {
    fn account_ids(&self) -> Result<HashSet<UUID>, MapperError> {
        let mut account_ids = HashSet::<UUID>::new();
        for operation_type in &self.operation_types {
            if let ListProposalsOperationTypeDTO::Transfer(Some(account_id)) = operation_type {
                match HelperMapper::to_uuid(account_id.to_owned()) {
                    Ok(account_id) => account_ids.insert(*account_id.as_bytes()),
                    Err(e) => return Err(e),
                };
            }
        }

        Ok(account_ids)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereSelectionStrategy {
    Account(UUID),
    Voter(UUID),
    Proposer(UUID),
    Status(ProposalStatusCode),
    ExpirationDt {
        from: Option<Timestamp>,
        to: Option<Timestamp>,
    },
    CreationDt {
        from: Option<Timestamp>,
        to: Option<Timestamp>,
    },
    And(Vec<WhereSelectionStrategy>),
    Or(Vec<WhereSelectionStrategy>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        proposal_test_utils::{self, mock_proposal},
        Metadata, ProposalOperation, ProposalStatus, TransferOperation, TransferOperationInput,
    };
    use num_bigint::BigUint;
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
            ProposalStatus::Created.to_string(),
        );

        let middle_eleven = repository.find_by_expiration_dt_and_status(
            Some(30),
            Some(40),
            ProposalStatus::Created.to_string(),
        );

        let first_three = repository.find_by_expiration_dt_and_status(
            None,
            Some(2),
            ProposalStatus::Created.to_string(),
        );

        assert_eq!(last_six.len(), 6);
        assert_eq!(middle_eleven.len(), 11);
        assert_eq!(first_three.len(), 3);
    }

    #[test]
    fn no_of_future_expiration_dt() {
        let repository = ProposalRepository::default();
        let mut proposal = proposal_test_utils::mock_proposal();
        proposal.expiration_dt = 10;

        repository.insert(proposal.to_key(), proposal.clone());

        let proposals = repository.find_by_expiration_dt_and_status(
            Some(20),
            None,
            proposal.status.to_string(),
        );

        assert!(proposals.is_empty());
    }

    // #[test]
    // fn pick_optmized_lookup_strategy() {
    //     let mut condition = ProposalWhereClause {
    //         created_dt_from: None,
    //         created_dt_to: None,
    //         expiration_dt_from: Some(10),
    //         expiration_dt_to: None,
    //         operation_types: vec![],
    //         statuses: vec![],
    //         voters: vec![],
    //         proposers: vec![],
    //     };

    //     assert_eq!(
    //         WhereSelectionStrategy::ExpirationDt,
    //         PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
    //     );

    //     condition.created_dt_from = Some(10);

    //     assert_eq!(
    //         WhereSelectionStrategy::CreationDt,
    //         PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
    //     );

    //     condition.statuses = vec![ProposalStatusCode::Created];

    //     assert_eq!(
    //         WhereSelectionStrategy::Status,
    //         PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
    //     );

    //     condition.operation_types = vec![ListProposalsOperationTypeDTO::Transfer(Some(
    //         Uuid::new_v4().to_string(),
    //     ))];

    //     assert_eq!(
    //         WhereSelectionStrategy::Account,
    //         PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
    //     );

    //     condition.voters = vec![[0; 16]];

    //     assert_eq!(
    //         WhereSelectionStrategy::Voter,
    //         PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
    //     );

    //     condition.proposers = vec![[0; 16]];

    //     assert_eq!(
    //         WhereSelectionStrategy::Proposer,
    //         PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
    //     );
    // }

    #[test]
    fn find_where_with_expiration_dt() {
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
            proposers: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0], proposal);

        condition.expiration_dt_from = Some(11);

        let proposals = PROPOSAL_REPOSITORY
            .find_where(condition.clone(), None)
            .unwrap();

        assert!(proposals.is_empty());
    }

    #[test]
    fn find_where_with_creation_dt() {
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
            proposers: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0], proposal);

        condition.created_dt_from = Some(8);
        condition.created_dt_to = Some(9);

        let proposals = PROPOSAL_REPOSITORY
            .find_where(condition.clone(), None)
            .unwrap();

        assert!(proposals.is_empty());
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
            proposers: vec![],
        };

        let proposals = PROPOSAL_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(proposals.len(), 100);
    }
}

#[cfg(feature = "canbench-rs")]
mod benchs {
    use crate::models::{proposal_test_utils::mock_proposal, ProposalStatus};

    use super::*;
    use canbench_rs::{bench, BenchResult};
    use uuid::Uuid;

    #[bench]
    fn batch_insert_100_proposals() {
        proposal_repository_test_utils::add_proposals_to_repository(100);
    }

    #[bench(raw)]
    fn list_all_proposals() -> BenchResult {
        proposal_repository_test_utils::add_proposals_to_repository(1_000);

        canbench_rs::bench_fn(|| {
            let _ = PROPOSAL_REPOSITORY.list();
        })
    }

    #[bench(raw)]
    fn filter_all_proposals_by_default_filters() -> BenchResult {
        proposal_repository_test_utils::add_proposals_to_repository(1_000);

        canbench_rs::bench_fn(|| {
            let _ = PROPOSAL_REPOSITORY.find_where(
                ProposalWhereClause {
                    created_dt_from: None,
                    created_dt_to: None,
                    expiration_dt_from: None,
                    expiration_dt_to: None,
                    operation_types: Vec::new(),
                    proposers: Vec::new(),
                    voters: Vec::new(),
                    statuses: Vec::new(),
                },
                None,
            );
        })
    }

    #[bench(raw)]
    fn filter_all_proposal_ids_by_default_filters() -> BenchResult {
        for i in 0..100_000 {
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
                    created_dt_from: Some(25000),
                    created_dt_to: Some(30000),
                    expiration_dt_from: None,
                    expiration_dt_to: None,
                    operation_types: Vec::new(),
                    proposers: Vec::new(),
                    voters: Vec::new(),
                    statuses: vec![ProposalStatusCode::Created],
                },
                None,
            );
        })
    }
}

#[cfg(any(test, feature = "canbench-rs"))]
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
