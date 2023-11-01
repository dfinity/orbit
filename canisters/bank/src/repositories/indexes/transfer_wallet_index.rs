use crate::{
    core::{
        ic_cdk::api::{time, trap},
        with_memory_manager, Memory, TRANSFER_WALLET_INDEX_MEMORY_ID,
    },
    errors::RepositoryError,
    models::{
        indexes::transfer_wallet_index::{TransferWalletIndex, TransferWalletIndexCriteria},
        TransferId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  /// The memory reference to the Transfer repository.
  static DB: RefCell<StableBTreeMap<TransferWalletIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_WALLET_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables managing transfer in stable memory.
#[derive(Default, Debug)]
pub struct TransferWalletIndexRepository {}

impl IndexRepository<TransferWalletIndex, TransferId> for TransferWalletIndexRepository {
    type FindByCriteria = TransferWalletIndexCriteria;

    fn exists(&self, index: &TransferWalletIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: TransferWalletIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &TransferWalletIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<TransferId> {
        DB.with(|db| {
            let (from_dt, to_dt) = match (criteria.from_dt, criteria.to_dt) {
                (Some(start), Some(end)) => (start, end),
                (Some(start), None) => (start, time()),
                (None, Some(end)) => (
                    end.saturating_sub(TransferWalletIndex::DEFAULT_CRITERIA_INTERVAL_NS),
                    end,
                ),
                _ => (
                    time().saturating_sub(TransferWalletIndex::DEFAULT_CRITERIA_INTERVAL_NS),
                    time(),
                ),
            };
            if from_dt > to_dt {
                trap(RepositoryError::CriteriaOutOfRange.to_string().as_str());
            }

            let start_key = TransferWalletIndex {
                wallet_id: criteria.wallet_id,
                created_timestamp: from_dt,
                transfer_id: [u8::MIN; 16],
            };
            let end_key = TransferWalletIndex {
                wallet_id: criteria.wallet_id,
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
        let repository = TransferWalletIndexRepository::default();
        let index = TransferWalletIndex {
            transfer_id: [1; 16],
            created_timestamp: time(),
            wallet_id: [2; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = TransferWalletIndexRepository::default();
        let now = time();
        let index = TransferWalletIndex {
            transfer_id: [1; 16],
            created_timestamp: now,
            wallet_id: [2; 16],
        };

        repository.insert(index.clone());
        repository.insert(TransferWalletIndex {
            transfer_id: [2; 16],
            created_timestamp: now + 1,
            wallet_id: [2; 16],
        });

        let criteria = TransferWalletIndexCriteria {
            wallet_id: [2; 16],
            from_dt: None,
            to_dt: Some(now),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.transfer_id));
    }
}
