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
            proposal_account_index::ProposalAccountIndexCriteria,
            proposal_creation_time_index::ProposalCreationTimeIndexCriteria,
            proposal_expiration_time_index::ProposalExpirationTimeIndexCriteria,
            proposal_proposer_index::ProposalProposerIndexCriteria,
            proposal_scheduled_index::ProposalScheduledIndexCriteria,
            proposal_status_index::ProposalStatusIndexCriteria,
            proposal_voter_index::ProposalVoterIndexCriteria,
        },
        AccountId, Proposal, ProposalKey, ProposalStatusCode, UserId,
    },
};
use ic_canister_core::{
    repository::{IndexRepository, RefreshIndexMode, Repository},
    types::{Timestamp, UUID},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
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
        from_last_update_dt: Option<Timestamp>,
        to_last_update_dt: Option<Timestamp>,
    ) -> Vec<Proposal> {
        let proposals = self
            .status_index
            .find_by_criteria(ProposalStatusIndexCriteria {
                status: status.to_owned(),
                from_dt: from_last_update_dt,
                to_dt: to_last_update_dt,
            });

        proposals
            .iter()
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

    pub fn find_by_account(
        &self,
        account_id: AccountId,
        created_from_dt: Option<Timestamp>,
        created_to_dt: Option<Timestamp>,
    ) -> Vec<Proposal> {
        let filtered_by_accounts =
            self.account_index
                .find_by_criteria(ProposalAccountIndexCriteria {
                    account_id: account_id.to_owned(),
                    from_dt: created_from_dt.to_owned(),
                    to_dt: created_to_dt.to_owned(),
                });

        filtered_by_accounts
            .iter()
            .filter_map(|id| self.get(&Proposal::key(*id)))
            .collect()
    }

    pub fn find_by_account_where(
        &self,
        account_id: AccountId,
        condition: ProposalWhereClause,
    ) -> Vec<Proposal> {
        self.account_index
            .find_by_criteria(ProposalAccountIndexCriteria {
                account_id: account_id.to_owned(),
                from_dt: condition.created_dt_from.to_owned(),
                to_dt: condition.created_dt_to.to_owned(),
            })
            .iter()
            .filter_map(|id| self.check_condition(*id, &condition))
            .collect()
    }

    pub fn find_by_voter_where(
        &self,
        user_id: UserId,
        condition: ProposalWhereClause,
    ) -> Vec<Proposal> {
        self.voter_index
            .find_by_criteria(ProposalVoterIndexCriteria {
                user_id: user_id.to_owned(),
                from_dt: condition.created_dt_from,
                to_dt: condition.created_dt_to,
            })
            .iter()
            .filter_map(|id| self.check_condition(*id, &condition))
            .collect()
    }

    pub fn find_where(
        &self,
        condition: ProposalWhereClause,
        sort_by: Option<ListProposalsSortBy>,
    ) -> Result<Vec<Proposal>, RepositoryError> {
        let strategy = self.pick_most_selective_where_filter(&condition);
        let proposal_ids = self.find_with_strategy(strategy, &condition)?;
        let mut proposals = proposal_ids
            .iter()
            .filter_map(|id| self.check_condition(*id, &condition))
            .collect::<Vec<Proposal>>();

        self.sort_proposals(&mut proposals, &sort_by);

        Ok(proposals)
    }

    fn sort_proposals(&self, proposals: &mut [Proposal], sort_by: &Option<ListProposalsSortBy>) {
        match sort_by {
            Some(wallet_api::ListProposalsSortBy::CreatedAt(direction)) => {
                proposals.sort_by(|a, b| match direction {
                    wallet_api::SortDirection::Asc => a.created_timestamp.cmp(&b.created_timestamp),
                    wallet_api::SortDirection::Desc => {
                        b.created_timestamp.cmp(&a.created_timestamp)
                    }
                });
            }
            Some(wallet_api::ListProposalsSortBy::ExpirationDt(direction)) => {
                proposals.sort_by(|a, b| match direction {
                    wallet_api::SortDirection::Asc => a.expiration_dt.cmp(&b.expiration_dt),
                    wallet_api::SortDirection::Desc => b.expiration_dt.cmp(&a.expiration_dt),
                });
            }
            Some(wallet_api::ListProposalsSortBy::LastModificationDt(direction)) => {
                proposals.sort_by(|a, b| match direction {
                    wallet_api::SortDirection::Asc => a
                        .last_modification_timestamp
                        .cmp(&b.last_modification_timestamp),
                    wallet_api::SortDirection::Desc => b
                        .last_modification_timestamp
                        .cmp(&a.last_modification_timestamp),
                });
            }
            None => {
                // Default sort by created timestamp descending
                proposals.sort_by(|a, b| b.created_timestamp.cmp(&a.created_timestamp));
            }
        }
    }

    fn pick_most_selective_where_filter(
        &self,
        condition: &ProposalWhereClause,
    ) -> WhereSelectionStrategy {
        let mut strategy = WhereSelectionStrategy::CreationDt;

        if condition.expiration_dt_from.is_some() || condition.expiration_dt_to.is_some() {
            strategy = WhereSelectionStrategy::ExpirationDt;
        }

        if condition.created_dt_from.is_some() || condition.created_dt_to.is_some() {
            strategy = WhereSelectionStrategy::CreationDt;
        }

        if !condition.statuses.is_empty() {
            strategy = WhereSelectionStrategy::Status;
        }

        if !condition.account_ids().unwrap_or_default().is_empty() {
            strategy = WhereSelectionStrategy::Account;
        }

        if !condition.voters.is_empty() {
            strategy = WhereSelectionStrategy::Voter;
        }

        if !condition.proposers.is_empty() {
            strategy = WhereSelectionStrategy::Proposer;
        }

        strategy
    }

    fn find_with_strategy(
        &self,
        strategy: WhereSelectionStrategy,
        condition: &ProposalWhereClause,
    ) -> Result<HashSet<UUID>, RepositoryError> {
        let ids = match strategy {
            WhereSelectionStrategy::Account => {
                let mut proposal_ids = HashSet::<UUID>::new();
                let account_ids = condition.account_ids().map_err(|e| {
                    RepositoryError::CriteriaValidationError {
                        reason: e.to_string(),
                    }
                })?;

                for account_id in account_ids {
                    proposal_ids.extend(self.account_index.find_by_criteria(
                        ProposalAccountIndexCriteria {
                            account_id,
                            from_dt: condition.created_dt_from,
                            to_dt: condition.created_dt_to,
                        },
                    ));
                }

                proposal_ids
            }
            WhereSelectionStrategy::Voter => {
                let mut proposal_ids = HashSet::<UUID>::new();
                let user_ids: HashSet<_> = condition.voters.iter().collect();

                for user_id in user_ids {
                    proposal_ids.extend(self.voter_index.find_by_criteria(
                        ProposalVoterIndexCriteria {
                            user_id: *user_id,
                            from_dt: condition.created_dt_from,
                            to_dt: condition.created_dt_to,
                        },
                    ));
                }

                proposal_ids
            }
            WhereSelectionStrategy::Proposer => {
                let mut proposal_ids = HashSet::<UUID>::new();
                let user_ids: HashSet<_> = condition.proposers.iter().collect();

                for user_id in user_ids {
                    proposal_ids.extend(self.proposer_index.find_by_criteria(
                        ProposalProposerIndexCriteria {
                            user_id: *user_id,
                            from_dt: condition.created_dt_from,
                            to_dt: condition.created_dt_to,
                        },
                    ));
                }

                proposal_ids
            }
            WhereSelectionStrategy::Status => {
                let mut proposal_ids = HashSet::<UUID>::new();
                let statuses: HashSet<_> = condition
                    .statuses
                    .iter()
                    .map(ProposalStatusMapper::from_status_code_dto)
                    .collect();

                for status in statuses {
                    proposal_ids.extend(self.status_index.find_by_criteria(
                        ProposalStatusIndexCriteria {
                            status: status.to_string(),
                            from_dt: condition.created_dt_from,
                            to_dt: condition.created_dt_to,
                        },
                    ));
                }

                proposal_ids
            }
            WhereSelectionStrategy::ExpirationDt => {
                self.expiration_dt_index
                    .find_by_criteria(ProposalExpirationTimeIndexCriteria {
                        from_dt: condition.expiration_dt_from,
                        to_dt: condition.expiration_dt_to,
                    })
            }
            WhereSelectionStrategy::CreationDt => {
                self.creation_dt_index
                    .find_by_criteria(ProposalCreationTimeIndexCriteria {
                        from_dt: condition.created_dt_from,
                        to_dt: condition.created_dt_to,
                    })
            }
        };

        Ok(ids)
    }

    fn check_condition(
        &self,
        proposal_id: UUID,
        condition: &ProposalWhereClause,
    ) -> Option<Proposal> {
        match self.get(&Proposal::key(proposal_id)) {
            Some(proposal) => {
                let mut match_operation_types = true;
                let mut match_statuses = true;
                let mut match_voters = true;
                let mut match_proposers = true;
                let match_creation_dt_range = match_date_range(
                    &proposal.created_timestamp,
                    &condition.created_dt_from,
                    &condition.created_dt_to,
                );
                let match_expiration_dt_range = match_date_range(
                    &proposal.expiration_dt,
                    &condition.expiration_dt_from,
                    &condition.expiration_dt_to,
                );

                if !condition.operation_types.is_empty() {
                    match_operation_types = condition
                        .operation_types
                        .iter()
                        .any(|operation_type| proposal.operation.is_of_type(operation_type));
                }

                if !condition.statuses.is_empty() {
                    match_statuses = condition
                        .statuses
                        .iter()
                        .any(|s| ProposalStatusCode::from(proposal.status.clone()) == *s);
                }

                if !condition.voters.is_empty() {
                    match_voters = condition
                        .voters
                        .iter()
                        .any(|v| proposal.voters().contains(v));
                }

                if !condition.proposers.is_empty() {
                    match_proposers = condition.proposers.contains(&proposal.proposed_by);
                }

                match match_expiration_dt_range
                    && match_creation_dt_range
                    && match_operation_types
                    && match_statuses
                    && match_proposers
                    && match_voters
                {
                    true => Some(proposal),
                    false => None,
                }
            }
            None => None,
        }
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
    Account,
    Voter,
    Proposer,
    Status,
    ExpirationDt,
    CreationDt,
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
    fn find_by_account() {
        let repository = ProposalRepository::default();
        let mut proposal = mock_proposal();
        let user_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        proposal.proposed_by = *user_id.as_bytes();
        proposal.operation = ProposalOperation::Transfer(TransferOperation {
            transfer_id: None,
            input: TransferOperationInput {
                amount: candid::Nat(BigUint::from(100u32)),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
                from_account_id: *account_id.as_bytes(),
                description: None,
            },
        });

        repository.insert(proposal.to_key(), proposal.clone());

        assert_eq!(
            repository.find_by_account(*account_id.as_bytes(), None, None),
            vec![proposal]
        );
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

    #[test]
    fn pick_optmized_lookup_strategy() {
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

        assert_eq!(
            WhereSelectionStrategy::ExpirationDt,
            PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
        );

        condition.created_dt_from = Some(10);

        assert_eq!(
            WhereSelectionStrategy::CreationDt,
            PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
        );

        condition.statuses = vec![ProposalStatusCode::Created];

        assert_eq!(
            WhereSelectionStrategy::Status,
            PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
        );

        condition.operation_types = vec![ListProposalsOperationTypeDTO::Transfer(Some(
            Uuid::new_v4().to_string(),
        ))];

        assert_eq!(
            WhereSelectionStrategy::Account,
            PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
        );

        condition.voters = vec![[0; 16]];

        assert_eq!(
            WhereSelectionStrategy::Voter,
            PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
        );

        condition.proposers = vec![[0; 16]];

        assert_eq!(
            WhereSelectionStrategy::Proposer,
            PROPOSAL_REPOSITORY.pick_most_selective_where_filter(&condition)
        );
    }

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
}

#[cfg(feature = "canbench-rs")]
mod benchs {
    use super::*;
    use crate::models::proposal_test_utils::mock_proposal;
    use canbench_rs::{bench, BenchResult};
    use uuid::Uuid;

    #[bench]
    fn batch_insert_100_proposals() {
        add_proposals_to_repository(100);
    }

    #[bench(raw)]
    fn list_all_proposals() -> BenchResult {
        add_proposals_to_repository(1_000);

        canbench_rs::bench_fn(|| {
            let _ = PROPOSAL_REPOSITORY.list();
        })
    }

    #[bench(raw)]
    fn filter_all_proposals_by_default_filters() -> BenchResult {
        add_proposals_to_repository(1_000);

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

    fn add_proposals_to_repository(count: usize) {
        for _ in 0..count {
            let mut proposal = mock_proposal();
            proposal.id = *Uuid::new_v4().as_bytes();

            PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());
        }
    }
}
