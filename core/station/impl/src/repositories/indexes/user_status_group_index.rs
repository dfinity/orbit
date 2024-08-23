use crate::{
    core::{with_memory_manager, Memory, USER_STATUS_GROUP_INDEX_MEMORY_ID},
    models::{
        indexes::user_status_group_index::{UserStatusGroupIndex, UserStatusGroupIndexCriteria},
        UserId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<UserStatusGroupIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_STATUS_GROUP_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables fetching efficiently users by their group and user status.
#[derive(Default, Debug)]
pub struct UserStatusGroupIndexRepository {}

impl UserStatusGroupIndexRepository {
    /// Clears the repository by removing all the entries.
    pub fn clear(&self) {
        DB.with(|m| m.borrow_mut().clear_new());
    }
}

impl IndexRepository<UserStatusGroupIndex, UserId> for UserStatusGroupIndexRepository {
    type FindByCriteria = UserStatusGroupIndexCriteria;

    fn exists(&self, index: &UserStatusGroupIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: UserStatusGroupIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &UserStatusGroupIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UserId> {
        DB.with(|db| {
            let start_key = UserStatusGroupIndex {
                group_id: criteria.group_id.to_owned(),
                user_status: criteria.user_status.to_owned(),
                user_id: [u8::MIN; 16],
            };
            let end_key = UserStatusGroupIndex {
                group_id: criteria.group_id,
                user_status: criteria.user_status,
                user_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.user_id)
                .collect::<HashSet<UserId>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UserStatus;

    #[test]
    fn test_index_repository() {
        let repository = UserStatusGroupIndexRepository::default();
        let index = UserStatusGroupIndex {
            group_id: [0; 16],
            user_status: UserStatus::Active,
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
        let repository = UserStatusGroupIndexRepository::default();
        let generate_items_nr = 10;
        for i in 0..generate_items_nr {
            let index = UserStatusGroupIndex {
                group_id: [i; 16],
                user_status: match i % 2 {
                    0 => UserStatus::Active,
                    _ => UserStatus::Inactive,
                },
                user_id: [i + generate_items_nr; 16],
            };
            repository.insert(index.clone());
        }

        let result = repository.find_by_criteria(UserStatusGroupIndexCriteria {
            group_id: [0; 16],
            user_status: UserStatus::Active,
        });

        assert_eq!(result.len(), 1);
        assert!(result.contains(&[generate_items_nr; 16]));
    }
}
