use super::indexes::{
    proposal_account_index::ProposalAccountIndexRepository,
    proposal_expiration_time_index::ProposalExpirationTimeIndexRepository,
    proposal_scheduled_index::ProposalScheduledIndexRepository,
    proposal_status_index::ProposalStatusIndexRepository,
    proposal_user_index::ProposalUserIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, PROPOSAL_MEMORY_ID},
    models::{
        indexes::{
            proposal_account_index::ProposalAccountIndexCriteria,
            proposal_expiration_time_index::ProposalExpirationTimeIndexCriteria,
            proposal_scheduled_index::ProposalScheduledIndexCriteria,
            proposal_status_index::ProposalStatusIndexCriteria,
            proposal_user_index::ProposalUserIndexCriteria,
        },
        AccountId, Proposal, ProposalKey, ProposalOperationType, ProposalStatusCode, UserId,
    },
};
use ic_canister_core::{
    repository::{IndexRepository, RefreshIndexMode, Repository},
    types::Timestamp,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::cell::RefCell;

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalKey, Proposal, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref PROPOSAL_REPOSITORY: ProposalRepository = ProposalRepository::default();
}

/// A repository that enables managing system proposals in stable memory.
#[derive(Default, Debug)]
pub struct ProposalRepository {
    user_index: ProposalUserIndexRepository,
    account_index: ProposalAccountIndexRepository,
    expiration_dt_index: ProposalExpirationTimeIndexRepository,
    status_index: ProposalStatusIndexRepository,
    scheduled_index: ProposalScheduledIndexRepository,
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
            self.user_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_users()),
                    current: value.to_index_for_users(),
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
            self.user_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_users()),
                });
            self.account_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().and_then(|prev| prev.to_index_for_account()),
                });
            self.scheduled_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().and_then(|prev| prev.to_index_by_scheduled()),
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
}

impl ProposalRepository {
    pub fn find_by_user_id(&self, user_id: UserId) -> Vec<Proposal> {
        self.user_index
            .find_by_criteria(ProposalUserIndexCriteria {
                user_id: user_id.to_owned(),
                from_dt: None,
                to_dt: None,
            })
            .iter()
            .filter_map(|id| self.get(&Proposal::key(*id)))
            .collect()
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
        self.find_by_account(
            account_id,
            condition.created_dt_from,
            condition.created_dt_to,
        )
        .iter()
        .filter(|proposal| {
            let mut match_operation_type = true;
            let mut match_status = true;

            if let Some(operation_type) = &condition.operation_type {
                match_operation_type =
                    ProposalOperationType::from(proposal.operation.clone()) == *operation_type;
            }

            if let Some(status) = &condition.status {
                match_status = status
                    .iter()
                    .any(|s| ProposalStatusCode::from(proposal.status.clone()) == *s);
            }

            match_operation_type && match_status
        })
        .map(|o| o.to_owned())
        .collect::<Vec<_>>()
    }

    pub fn find_by_user_where(
        &self,
        user_id: UserId,
        condition: ProposalFindByUserWhereClause,
    ) -> Vec<Proposal> {
        self.user_index
            .find_by_criteria(ProposalUserIndexCriteria {
                user_id: user_id.to_owned(),
                from_dt: condition.created_dt_from,
                to_dt: condition.created_dt_to,
            })
            .iter()
            .filter_map(|id| match self.get(&Proposal::key(*id)) {
                Some(proposal) => {
                    let mut match_operation_type = true;
                    let mut match_status = true;

                    if let Some(operation_type) = &condition.operation_type {
                        match_operation_type =
                            ProposalOperationType::from(proposal.operation.clone())
                                == *operation_type;
                    }

                    if let Some(status) = &condition.status {
                        match_status = status
                            .iter()
                            .any(|s| ProposalStatusCode::from(proposal.status.clone()) == *s);
                    }

                    match match_operation_type && match_status {
                        true => Some(proposal),
                        false => None,
                    }
                }
                None => None,
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct ProposalWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub operation_type: Option<ProposalOperationType>,
    pub status: Option<Vec<ProposalStatusCode>>,
}

#[derive(Debug)]
pub struct ProposalFindByUserWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub operation_type: Option<ProposalOperationType>,
    pub status: Option<Vec<ProposalStatusCode>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        proposal_test_utils::{self, mock_proposal},
        ProposalOperation, ProposalStatus, ProposalVote, ProposalVoteStatus, TransferOperation,
        TransferOperationInput,
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
    fn find_by_originator_user_id() {
        let repository = ProposalRepository::default();
        let mut proposal = mock_proposal();
        let user_id = Uuid::new_v4();
        proposal.proposed_by = *user_id.as_bytes();

        repository.insert(proposal.to_key(), proposal.clone());

        assert_eq!(
            repository.find_by_user_id(*user_id.as_bytes()),
            vec![proposal]
        );
    }

    #[test]
    fn find_by_decision_user_id() {
        let repository = ProposalRepository::default();
        let mut proposal = mock_proposal();
        let user_id = Uuid::new_v4();
        proposal.votes = vec![ProposalVote {
            user_id: *user_id.as_bytes(),
            decided_dt: 0,
            last_modification_timestamp: 0,
            status: ProposalVoteStatus::Accepted,
            status_reason: None,
        }];

        repository.insert(proposal.to_key(), proposal.clone());

        assert_eq!(
            repository.find_by_user_id(*user_id.as_bytes()),
            vec![proposal]
        );
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
                metadata: vec![],
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
                from_account_id: *account_id.as_bytes(),
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
}
