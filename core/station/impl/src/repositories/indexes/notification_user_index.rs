use crate::{
    core::{with_memory_manager, Memory, NOTIFICATION_USER_INDEX_MEMORY_ID},
    models::{
        indexes::notification_user_index::{NotificationUserIndex, NotificationUserIndexCriteria},
        NotificationId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<NotificationUserIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(NOTIFICATION_USER_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding notifications based on the user in stable memory.
#[derive(Default, Debug)]
pub struct NotificationUserIndexRepository {}

impl IndexRepository<NotificationUserIndex, NotificationId> for NotificationUserIndexRepository {
    type FindByCriteria = NotificationUserIndexCriteria;

    fn exists(&self, key: &NotificationUserIndex) -> bool {
        DB.with(|m| m.borrow().get(key).is_some())
    }

    fn insert(&self, key: NotificationUserIndex) {
        DB.with(|m| m.borrow_mut().insert(key, ()));
    }

    fn remove(&self, key: &NotificationUserIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(key).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<NotificationId> {
        DB.with(|db| {
            let start_key = NotificationUserIndex {
                user_id: criteria.user_id.to_owned(),
                created_at: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
                notification_id: [u8::MIN; 16],
            };
            let end_key = NotificationUserIndex {
                user_id: criteria.user_id.to_owned(),
                created_at: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
                notification_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.notification_id)
                .collect::<HashSet<NotificationId>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = NotificationUserIndexRepository::default();
        let index = NotificationUserIndex {
            notification_id: [0; 16],
            created_at: 10,
            user_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = NotificationUserIndexRepository::default();
        let index = NotificationUserIndex {
            notification_id: [0; 16],
            created_at: 10,
            user_id: [1; 16],
        };

        repository.insert(index.clone());

        let criteria = NotificationUserIndexCriteria {
            user_id: [1; 16],
            from_dt: None,
            to_dt: None,
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.notification_id));
    }
}
