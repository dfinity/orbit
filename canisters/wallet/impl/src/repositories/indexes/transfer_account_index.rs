use crate::{
    core::{ic_cdk::api::time, with_memory_manager, Memory, TRANSFER_ACCOUNT_INDEX_MEMORY_ID},
    models::{
        indexes::transfer_account_index::{TransferAccountIndex, TransferAccountIndexCriteria},
        TransferId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_cdk::print;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<TransferAccountIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_ACCOUNT_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct TransferAccountIndexRepository {}

impl IndexRepository<TransferAccountIndex, TransferId> for TransferAccountIndexRepository {
    type FindByCriteria = TransferAccountIndexCriteria;

    fn exists(&self, index: &TransferAccountIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: TransferAccountIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &TransferAccountIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<TransferId> {
        DB.with(|db| {
            let (from_dt, to_dt) = match (criteria.from_dt, criteria.to_dt) {
                (Some(start), Some(end)) => (start, end),
                (Some(start), None) => (start, time()),
                (None, Some(end)) => (
                    end.saturating_sub(TransferAccountIndex::DEFAULT_CRITERIA_INTERVAL_NS),
                    end,
                ),
                _ => (
                    time().saturating_sub(TransferAccountIndex::DEFAULT_CRITERIA_INTERVAL_NS),
                    time(),
                ),
            };
            if from_dt > to_dt {
                print(format!("Invalid TransferAccountIndexRepository::FindByCriteria: from_dt {} is greater than to_dt {}", from_dt, to_dt));
                return HashSet::new();
            }

            let start_key = TransferAccountIndex {
                account_id: criteria.account_id,
                created_timestamp: from_dt,
                transfer_id: [u8::MIN; 16],
            };
            let end_key = TransferAccountIndex {
                account_id: criteria.account_id,
                created_timestamp: to_dt,
                transfer_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.transfer_id)
                .collect::<HashSet<TransferId>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = TransferAccountIndexRepository::default();
        let index = TransferAccountIndex {
            transfer_id: [1; 16],
            created_timestamp: time(),
            account_id: [2; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = TransferAccountIndexRepository::default();
        let now = time();
        let index = TransferAccountIndex {
            transfer_id: [1; 16],
            created_timestamp: now,
            account_id: [2; 16],
        };

        repository.insert(index.clone());
        repository.insert(TransferAccountIndex {
            transfer_id: [2; 16],
            created_timestamp: now + 1,
            account_id: [2; 16],
        });

        let criteria = TransferAccountIndexCriteria {
            account_id: [2; 16],
            from_dt: None,
            to_dt: Some(now),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.transfer_id));
    }
}
