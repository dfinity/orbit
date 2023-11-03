use super::indexes::{
    operation_transfer_index::OperationTransferIndexRepository,
    operation_user_index::OperationUserIndexRepository,
    operation_account_index::OperationAccountIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, OPERATION_MEMORY_ID},
    models::{
        indexes::{
            operation_transfer_index::OperationTransferIndexCriteria,
            operation_user_index::OperationUserIndexCriteria,
            operation_account_index::OperationAccountIndexCriteria,
        },
        Operation, OperationCode, OperationKey, OperationStatus, TransferId, UserId, AccountId,
    },
};
use ic_canister_core::{
    repository::{IndexRepository, Repository},
    types::Timestamp,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<OperationKey, Operation, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(OPERATION_MEMORY_ID))
    )
  })
}

/// A repository that enables managing system operations in stable memory.
#[derive(Default, Debug)]
pub struct OperationRepository {
    user_index: OperationUserIndexRepository,
    account_index: OperationAccountIndexRepository,
    transfer_index: OperationTransferIndexRepository,
}

impl Repository<OperationKey, Operation> for OperationRepository {
    fn get(&self, key: &OperationKey) -> Option<Operation> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: OperationKey, value: Operation) -> Option<Operation> {
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

    fn remove(&self, key: &OperationKey) -> Option<Operation> {
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

impl OperationRepository {
    pub fn find_by_transfer_id(&self, transfer_id: TransferId) -> Vec<Operation> {
        self.transfer_index
            .find_by_criteria(OperationTransferIndexCriteria {
                transfer_id: transfer_id.to_owned(),
                from_dt: None,
                to_dt: None,
            })
            .iter()
            .filter_map(|id| self.get(&Operation::key(*id)))
            .collect()
    }

    pub fn find_by_user_id(&self, user_id: UserId) -> Vec<Operation> {
        self.user_index
            .find_by_criteria(OperationUserIndexCriteria {
                user_id: user_id.to_owned(),
                from_dt: None,
                to_dt: None,
            })
            .iter()
            .filter_map(|id| self.get(&Operation::key(*id)))
            .collect()
    }

    pub fn find_by_account_and_user_id(
        &self,
        account_id: AccountId,
        user_id: UserId,
        created_from_dt: Option<Timestamp>,
        created_to_dt: Option<Timestamp>,
    ) -> Vec<Operation> {
        let filtered_by_accounts =
            self.account_index
                .find_by_criteria(OperationAccountIndexCriteria {
                    account_id: account_id.to_owned(),
                    from_dt: created_from_dt.to_owned(),
                    to_dt: created_to_dt.to_owned(),
                });
        let filtered_by_users = self
            .user_index
            .find_by_criteria(OperationUserIndexCriteria {
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
            .filter_map(|id| self.get(&Operation::key(*id)))
            .collect()
    }

    pub fn find_by_account_where(
        &self,
        key: (UserId, AccountId),
        condition: OperationWhereClause,
    ) -> Vec<Operation> {
        let (user_id, account_id) = key;
        let operations = self.find_by_account_and_user_id(
            account_id,
            user_id,
            condition.created_dt_from,
            condition.created_dt_to,
        );

        operations
            .iter()
            .filter(|operation| {
                let mut match_code = true;
                let mut match_status = true;

                if let Some(code) = condition.code.clone() {
                    match_code = operation.code == code;
                }

                if let Some(status) = condition.status.clone() {
                    match_status = operation.status == status;
                }

                match_code && match_status
            })
            .map(|o| o.to_owned())
            .collect::<Vec<_>>()
    }

    pub fn find_by_user_where(
        &self,
        user_id: UserId,
        condition: OperationFindByUserWhereClause,
    ) -> Vec<Operation> {
        self.user_index
            .find_by_criteria(OperationUserIndexCriteria {
                user_id: user_id.to_owned(),
                from_dt: condition.created_dt_from,
                to_dt: condition.created_dt_to,
            })
            .iter()
            .filter_map(|id| match self.get(&Operation::key(*id)) {
                Some(operation) => {
                    let mut match_code = true;
                    let mut match_read = true;
                    let mut match_status = true;

                    if let Some(code) = condition.code.clone() {
                        match_code = operation.code == code;
                    }

                    if let Some(read) = condition.read {
                        match_read = operation.decisions.iter().any(|operation| {
                            operation.user_id == user_id && operation.read == read
                        });
                    }

                    if let Some(status) = condition.status.clone() {
                        match_status = operation.status == status;
                    }

                    match match_code && match_read && match_status {
                        true => Some(operation),
                        false => None,
                    }
                }
                None => None,
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct OperationWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub code: Option<OperationCode>,
    pub status: Option<OperationStatus>,
}

#[derive(Debug)]
pub struct OperationFindByUserWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub code: Option<OperationCode>,
    pub status: Option<OperationStatus>,
    pub read: Option<bool>,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::models::{
        operation_test_utils, OperationDecision, OPERATION_METADATA_KEY_TRANSFER_ID,
        OPERATION_METADATA_KEY_ACCOUNT_ID,
    };

    #[test]
    fn perform_crud() {
        let repository = OperationRepository::default();
        let operation = operation_test_utils::mock_operation();

        assert!(repository.get(&operation.to_key()).is_none());

        repository.insert(operation.to_key(), operation.clone());

        assert!(repository.get(&operation.to_key()).is_some());
        assert!(repository.remove(&operation.to_key()).is_some());
        assert!(repository.get(&operation.to_key()).is_none());
    }

    #[test]
    fn find_by_transfer_id() {
        let repository = OperationRepository::default();
        let mut operation = operation_test_utils::mock_operation();
        let transfer_id = Uuid::new_v4();
        operation.metadata = vec![(
            OPERATION_METADATA_KEY_TRANSFER_ID.to_string(),
            transfer_id.to_string(),
        )];

        repository.insert(operation.to_key(), operation.clone());

        assert_eq!(
            repository.find_by_transfer_id(*transfer_id.as_bytes()),
            vec![operation]
        );
    }

    #[test]
    fn find_by_originator_user_id() {
        let repository = OperationRepository::default();
        let mut operation = operation_test_utils::mock_operation();
        let user_id = Uuid::new_v4();
        operation.proposed_by = Some(*user_id.as_bytes());

        repository.insert(operation.to_key(), operation.clone());

        assert_eq!(
            repository.find_by_user_id(*user_id.as_bytes()),
            vec![operation]
        );
    }

    #[test]
    fn find_by_decision_user_id() {
        let repository = OperationRepository::default();
        let mut operation = operation_test_utils::mock_operation();
        let user_id = Uuid::new_v4();
        operation.decisions = vec![OperationDecision {
            user_id: *user_id.as_bytes(),
            read: false,
            decided_dt: None,
            last_modification_timestamp: 0,
            status: OperationStatus::Pending,
            status_reason: None,
        }];

        repository.insert(operation.to_key(), operation.clone());

        assert_eq!(
            repository.find_by_user_id(*user_id.as_bytes()),
            vec![operation]
        );
    }

    #[test]
    fn find_by_account_and_user() {
        let repository = OperationRepository::default();
        let mut operation = operation_test_utils::mock_operation();
        let user_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        operation.proposed_by = Some(*user_id.as_bytes());
        operation.metadata = vec![(
            OPERATION_METADATA_KEY_ACCOUNT_ID.to_string(),
            account_id.to_string(),
        )];

        repository.insert(operation.to_key(), operation.clone());

        assert_eq!(
            repository.find_by_account_and_user_id(
                *account_id.as_bytes(),
                *user_id.as_bytes(),
                None,
                None
            ),
            vec![operation]
        );
    }
}
