use crate::{
    core::{with_memory_manager, Memory, TRANSFER_EXPIRATION_TIME_INDEX_MEMORY_ID},
    models::{
        indexes::transfer_expiration_time_index::{
            TransferExpirationTimeIndex, TransferExpirationTimeIndexCriteria,
        },
        TransferId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<TransferExpirationTimeIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_EXPIRATION_TIME_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct TransferExpirationTimeIndexRepository {}

impl IndexRepository<TransferExpirationTimeIndex, TransferId>
    for TransferExpirationTimeIndexRepository
{
    type FindByCriteria = TransferExpirationTimeIndexCriteria;

    fn exists(&self, index: &TransferExpirationTimeIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: TransferExpirationTimeIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &TransferExpirationTimeIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<TransferId> {
        DB.with(|db| {
            let start_key = TransferExpirationTimeIndex {
                expiration_dt: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
                transfer_id: [std::u8::MIN; 16],
            };
            let end_key = TransferExpirationTimeIndex {
                expiration_dt: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
                transfer_id: [std::u8::MAX; 16],
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
        let repository = TransferExpirationTimeIndexRepository::default();
        let index = TransferExpirationTimeIndex {
            expiration_dt: 10,
            transfer_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = TransferExpirationTimeIndexRepository::default();
        let index = TransferExpirationTimeIndex {
            expiration_dt: 10,
            transfer_id: [1; 16],
        };

        repository.insert(index.clone());
        repository.insert(TransferExpirationTimeIndex {
            expiration_dt: 11,
            transfer_id: [2; 16],
        });

        let criteria = TransferExpirationTimeIndexCriteria {
            from_dt: None,
            to_dt: Some(10),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.transfer_id));
    }
}
