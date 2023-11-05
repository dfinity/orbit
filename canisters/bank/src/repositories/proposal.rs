use super::indexes::{
    proposal_account_index::ProposalAccountIndexRepository,
    proposal_transfer_index::ProposalTransferIndexRepository,
    proposal_user_index::ProposalUserIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, PROPOSAL_MEMORY_ID},
    models::{
        indexes::{
            proposal_account_index::ProposalAccountIndexCriteria,
            proposal_transfer_index::ProposalTransferIndexCriteria,
            proposal_user_index::ProposalUserIndexCriteria,
        },
        AccountId, Proposal, ProposalKey, ProposalOperationType, ProposalStatus, TransferId,
        UserId,
    },
};
use ic_canister_core::{
    repository::{IndexRepository, Repository},
    types::Timestamp,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalKey, Proposal, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_MEMORY_ID))
    )
  })
}

/// A repository that enables managing system proposals in stable memory.
#[derive(Default, Debug)]
pub struct ProposalRepository {
    user_index: ProposalUserIndexRepository,
    account_index: ProposalAccountIndexRepository,
    transfer_index: ProposalTransferIndexRepository,
}

impl Repository<ProposalKey, Proposal> for ProposalRepository {
    fn get(&self, key: &ProposalKey) -> Option<Proposal> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: ProposalKey, value: Proposal) -> Option<Proposal> {
        DB.with(|m| match m.borrow_mut().insert(key, value.clone()) {
            Some(prev) => {
                let prev_users_index = prev.to_index_for_users();
                if prev_users_index != value.to_index_for_users() {
                    prev_users_index.iter().for_each(|index| {
                        self.user_index.remove(index);
                    });
                    value.to_index_for_users().iter().for_each(|index| {
                        self.user_index.insert(index.to_owned());
                    });
                }

                match (prev.to_index_for_account(), value.to_index_for_account()) {
                    (Some(prev), Some(current)) => {
                        if prev != current {
                            self.account_index.remove(&prev);
                            self.account_index.insert(current);
                        }
                    }
                    (Some(prev), None) => {
                        self.account_index.remove(&prev);
                    }
                    (None, Some(current)) => {
                        self.account_index.insert(current);
                    }
                    _ => {}
                }

                match (prev.to_index_for_transfer(), value.to_index_for_transfer()) {
                    (Some(prev), Some(current)) => {
                        if prev != current {
                            self.transfer_index.remove(&prev);
                            self.transfer_index.insert(current);
                        }
                    }
                    (Some(prev), None) => {
                        self.transfer_index.remove(&prev);
                    }
                    (None, Some(current)) => {
                        self.transfer_index.insert(current);
                    }
                    _ => {}
                }

                Some(prev)
            }
            None => {
                value.to_index_for_users().iter().for_each(|index| {
                    self.user_index.insert(index.to_owned());
                });
                if let Some(account_index) = value.to_index_for_account() {
                    self.account_index.insert(account_index);
                }
                if let Some(transfer_index) = value.to_index_for_transfer() {
                    self.transfer_index.insert(transfer_index);
                }

                None
            }
        })
    }

    fn remove(&self, key: &ProposalKey) -> Option<Proposal> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(prev) => {
                prev.to_index_for_users().iter().for_each(|index| {
                    self.user_index.remove(index);
                });
                if let Some(account_index) = prev.to_index_for_account() {
                    self.account_index.remove(&account_index);
                }
                if let Some(transfer_index) = prev.to_index_for_transfer() {
                    self.transfer_index.remove(&transfer_index);
                }

                Some(prev)
            }
            None => None,
        })
    }
}

impl ProposalRepository {
    pub fn find_by_transfer_id(&self, transfer_id: TransferId) -> Vec<Proposal> {
        self.transfer_index
            .find_by_criteria(ProposalTransferIndexCriteria {
                transfer_id: transfer_id.to_owned(),
                from_dt: None,
                to_dt: None,
            })
            .iter()
            .filter_map(|id| self.get(&Proposal::key(*id)))
            .collect()
    }

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

    pub fn find_by_account_and_user_id(
        &self,
        account_id: AccountId,
        user_id: UserId,
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
        let filtered_by_users = self.user_index.find_by_criteria(ProposalUserIndexCriteria {
            user_id: user_id.to_owned(),
            from_dt: created_from_dt,
            to_dt: created_to_dt,
        });

        let results = filtered_by_accounts
            .intersection(&filtered_by_users)
            .copied()
            .collect::<HashSet<_>>();

        results
            .iter()
            .filter_map(|id| self.get(&Proposal::key(*id)))
            .collect()
    }

    pub fn find_by_account_where(
        &self,
        (user_id, account_id): (UserId, AccountId),
        condition: ProposalWhereClause,
    ) -> Vec<Proposal> {
        self.find_by_account_and_user_id(
            account_id,
            user_id,
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

            if let Some(status) = condition.status.clone() {
                match_status = proposal.status == status;
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
                    let mut match_read = true;
                    let mut match_status = true;

                    if let Some(operation_type) = &condition.operation_type {
                        match_operation_type =
                            ProposalOperationType::from(proposal.operation.clone())
                                == *operation_type;
                    }

                    if let Some(read) = condition.read {
                        match_read = proposal
                            .votes
                            .iter()
                            .any(|proposal| proposal.user_id == user_id && proposal.read == read);
                    }

                    if let Some(status) = &condition.status {
                        match_status = proposal.status == *status;
                    }

                    match match_operation_type && match_read && match_status {
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
    pub status: Option<ProposalStatus>,
}

#[derive(Debug)]
pub struct ProposalFindByUserWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub operation_type: Option<ProposalOperationType>,
    pub status: Option<ProposalStatus>,
    pub read: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        proposal_test_utils::mock_proposal, ProposalOperation, ProposalVote, ProposalVoteStatus,
        TransferOperationContext,
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
    fn find_by_transfer_id() {
        let repository = ProposalRepository::default();
        let mut proposal = mock_proposal();
        let transfer_id = Uuid::new_v4();
        proposal.operation = ProposalOperation::Transfer(TransferOperationContext {
            transfer_id: *transfer_id.as_bytes(),
            account_id: [0; 16],
        });

        repository.insert(proposal.to_key(), proposal.clone());

        assert_eq!(
            repository.find_by_transfer_id(*transfer_id.as_bytes()),
            vec![proposal]
        );
    }

    #[test]
    fn find_by_originator_user_id() {
        let repository = ProposalRepository::default();
        let mut proposal = mock_proposal();
        let user_id = Uuid::new_v4();
        proposal.proposed_by = Some(*user_id.as_bytes());

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
            read: false,
            decided_dt: None,
            last_modification_timestamp: 0,
            status: ProposalVoteStatus::Pending,
            status_reason: None,
        }];

        repository.insert(proposal.to_key(), proposal.clone());

        assert_eq!(
            repository.find_by_user_id(*user_id.as_bytes()),
            vec![proposal]
        );
    }

    #[test]
    fn find_by_account_and_user() {
        let repository = ProposalRepository::default();
        let mut proposal = mock_proposal();
        let user_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        proposal.proposed_by = Some(*user_id.as_bytes());
        proposal.operation = ProposalOperation::Transfer(TransferOperationContext {
            transfer_id: [0; 16],
            account_id: *account_id.as_bytes(),
        });

        repository.insert(proposal.to_key(), proposal.clone());

        assert_eq!(
            repository.find_by_account_and_user_id(
                *account_id.as_bytes(),
                *user_id.as_bytes(),
                None,
                None
            ),
            vec![proposal]
        );
    }
}
