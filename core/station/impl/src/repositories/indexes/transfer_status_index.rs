use crate::{
    core::{with_memory_manager, Memory, TRANSFER_STATUS_INDEX_MEMORY_ID},
    models::indexes::transfer_status_index::{TransferStatusIndex, TransferStatusIndexCriteria},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<TransferStatusIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(TRANSFER_STATUS_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct TransferStatusIndexRepository {}

impl TransferStatusIndexRepository {
    /// Clears the repository by removing all the entries.
    pub fn clear(&self) {
        DB.with(|m| m.borrow_mut().clear_new());
    }
}

impl IndexRepository<TransferStatusIndex, UUID> for TransferStatusIndexRepository {
    type FindByCriteria = TransferStatusIndexCriteria;

    fn exists(&self, index: &TransferStatusIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: TransferStatusIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &TransferStatusIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = TransferStatusIndex {
                status: criteria.status.to_owned(),
                last_modification_timestamp: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
                transfer_id: [u8::MIN; 16],
            };
            let end_key = TransferStatusIndex {
                status: criteria.status.to_owned(),
                last_modification_timestamp: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
                transfer_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.transfer_id)
                .collect::<HashSet<UUID>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TransferStatus;

    #[test]
    fn test_repository_crud() {
        let repository = TransferStatusIndexRepository::default();
        let index = TransferStatusIndex {
            status: TransferStatus::Created.to_string(),
            last_modification_timestamp: 10,
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
        let repository = TransferStatusIndexRepository::default();
        let index = TransferStatusIndex {
            status: TransferStatus::Created.to_string(),
            last_modification_timestamp: 10,
            transfer_id: [1; 16],
        };

        repository.insert(index.clone());
        repository.insert(TransferStatusIndex {
            status: TransferStatus::Created.to_string(),
            last_modification_timestamp: 11,
            transfer_id: [2; 16],
        });

        let criteria = TransferStatusIndexCriteria {
            status: TransferStatus::Created.to_string(),
            from_dt: None,
            to_dt: Some(10),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.transfer_id));
    }
}
