use crate::{
    core::{metrics::USER_METRICS, with_memory_manager, Memory, USER_MEMORY_ID},
    mappers::SubscribedUser,
    models::{
        indexes::{
            user_identity_index::UserIdentityIndexCriteria,
            user_status_index::{UserIndexSubscriptionStatus, UserStatusIndexCriteria},
        },
        User, UserKey, UserSubscriptionStatus,
    },
    repositories::indexes::{
        user_identity_index::UserIdentityIndexRepository,
        user_status_index::UserStatusIndexRepository,
    },
};
use candid::Principal;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::StableDb;
use orbit_essentials::repository::{IndexRepository, IndexedRepository, Repository};
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<UserKey, User, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref USER_REPOSITORY: Arc<UserRepository> = Arc::new(UserRepository::default());
}

/// A repository that enables managing users in stable memory.
#[derive(Default, Debug)]
pub struct UserRepository {
    identity_index: UserIdentityIndexRepository,
    status_index: UserStatusIndexRepository,
}

impl StableDb<UserKey, User, VirtualMemory<Memory>> for UserRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<UserKey, User, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<UserKey, User, VirtualMemory<Memory>> for UserRepository {
    fn remove_entry_indexes(&self, entry: &User) {
        self.identity_index.remove(&entry.to_index_for_identity());
        self.status_index.remove(&entry.to_index_for_status());
    }

    fn add_entry_indexes(&self, entry: &User) {
        self.identity_index.insert(entry.to_index_for_identity());
        self.status_index.insert(entry.to_index_for_status());
    }

    fn clear_indexes(&self) {
        self.identity_index.clear();
        self.status_index.clear();
    }
}

impl Repository<UserKey, User, VirtualMemory<Memory>> for UserRepository {
    fn insert(&self, key: UserKey, value: User) -> Option<User> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            // Update metrics when a user is upserted.
            USER_METRICS.with(|metrics| {
                metrics
                    .iter()
                    .for_each(|metric| metric.borrow_mut().sum(&value, prev.as_ref()))
            });

            self.save_entry_indexes(&value, prev.as_ref());

            prev
        })
    }

    fn remove(&self, key: &UserKey) -> Option<User> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            // Update metrics when a user is removed.
            if let Some(prev) = &prev {
                USER_METRICS.with(|metrics| {
                    metrics
                        .iter()
                        .for_each(|metric| metric.borrow_mut().sub(prev))
                });

                self.remove_entry_indexes(prev);
            }

            prev
        })
    }
}

impl UserRepository {
    /// Returns the user associated with the given identity if it exists.
    pub fn find_by_identity(&self, identity: &Principal) -> Option<User> {
        self.identity_index
            .find_by_criteria(UserIdentityIndexCriteria {
                identity_id: identity.to_owned(),
            })
            .iter()
            .find_map(|id| self.get(&UserKey(*id)))
    }

    pub fn get_subscribed_users(&self) -> Vec<SubscribedUser> {
        self.status_index
            .find_by_criteria(UserStatusIndexCriteria {
                status: UserIndexSubscriptionStatus::Pending,
            })
            .iter()
            .filter_map(|id| {
                let user = self.get(&UserKey(*id)).unwrap();
                if let UserSubscriptionStatus::Pending(email) = user.subscription_status {
                    Some(SubscribedUser {
                        user_principal: user.identity,
                        email,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{user_model_utils::mock_user, UserSubscriptionStatus};
    use candid::Principal;

    #[test]
    fn check_user_insert_and_get() {
        let repository = UserRepository::default();
        let user = mock_user();

        assert!(repository.get(&UserKey(user.id)).is_none());

        repository.insert(UserKey(user.id), user.clone());
        assert_eq!(repository.get(&UserKey(user.id)), Some(user));
    }

    #[test]
    fn get_subscribed_users() {
        let repository = UserRepository::default();

        let mut unsubscribed_user = mock_user();
        unsubscribed_user.subscription_status = UserSubscriptionStatus::Unsubscribed;
        repository.insert(UserKey(unsubscribed_user.id), unsubscribed_user.clone());

        let email = "john@example.com".to_string();
        let mut subscribed_user = mock_user();
        subscribed_user.subscription_status = UserSubscriptionStatus::Pending(email.clone());
        repository.insert(UserKey(subscribed_user.id), subscribed_user.clone());

        let another_email = "martin@example.com".to_string();
        let mut another_subscribed_user = mock_user();
        another_subscribed_user.subscription_status =
            UserSubscriptionStatus::Pending(another_email.clone());
        repository.insert(
            UserKey(another_subscribed_user.id),
            another_subscribed_user.clone(),
        );

        let all_users = repository.list();
        assert_eq!(all_users.len(), 3);

        let subscribed_users = repository.get_subscribed_users();
        assert_eq!(subscribed_users.len(), 2);
        let subscribed = SubscribedUser {
            user_principal: subscribed_user.identity,
            email,
        };
        assert!(subscribed_users.contains(&subscribed));
        let another_subscribed = SubscribedUser {
            user_principal: another_subscribed_user.identity,
            email: another_email,
        };
        assert!(subscribed_users.contains(&another_subscribed));
    }

    #[test]
    fn check_user_removal() {
        let repository = UserRepository::default();
        let user = mock_user();

        repository.insert(UserKey(user.id), user.clone());
        assert_eq!(repository.get(&UserKey(user.id)), Some(user.clone()));
        repository.remove(&UserKey(user.id));
        assert!(repository.get(&UserKey(user.id)).is_none());
    }

    #[test]
    fn check_user_find_by_identity() {
        let user = mock_user();

        USER_REPOSITORY.insert(user.to_key(), user.clone());
        assert_eq!(
            USER_REPOSITORY.find_by_identity(&user.identity),
            Some(user.clone())
        );
    }

    #[test]
    fn check_user_find_by_identity_not_found() {
        let user = mock_user();

        USER_REPOSITORY.insert(user.to_key(), user.clone());
        assert_eq!(
            USER_REPOSITORY.find_by_identity(&Principal::from_slice(&[0; 29])),
            None
        );
    }

    #[test]
    fn check_find_by_identity_gets_correct_user_from_many() {
        for _ in 0..10 {
            let mock_user = mock_user();
            USER_REPOSITORY.insert(mock_user.to_key(), mock_user);
        }

        let user = mock_user();
        USER_REPOSITORY.insert(user.to_key(), user.clone());

        for _ in 0..10 {
            let mock_user = mock_user();
            USER_REPOSITORY.insert(mock_user.to_key(), mock_user);
        }

        assert_eq!(USER_REPOSITORY.len(), 21);
        assert_eq!(
            USER_REPOSITORY.find_by_identity(&user.identity),
            Some(user.clone())
        );
    }
}
