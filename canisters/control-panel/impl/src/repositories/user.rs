use crate::{
    core::{with_memory_manager, Memory, USER_MEMORY_ID},
    mappers::SubscribedUser,
    models::{User, UserKey, UserSubscriptionStatus},
};
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<UserKey, User, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref USER_REPOSITORY: Arc<UserRepository> = Arc::new(UserRepository {});
}

/// A repository that enables managing users in stable memory.
#[derive(Default, Debug)]
pub struct UserRepository {}

impl Repository<UserKey, User> for UserRepository {
    fn list(&self) -> Vec<User> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &UserKey) -> Option<User> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: UserKey, value: User) -> Option<User> {
        DB.with(|m| m.borrow_mut().insert(key, value.clone()))
    }

    fn remove(&self, key: &UserKey) -> Option<User> {
        DB.with(|m| m.borrow_mut().remove(key))
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl UserRepository {
    pub fn get_subscribed_users(&self) -> Vec<SubscribedUser> {
        self.list()
            .into_iter()
            .filter_map(|u| {
                if let UserSubscriptionStatus::Pending(email) = u.subscription_status {
                    Some(SubscribedUser {
                        user_principal: u.id,
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
    use crate::models::UserSubscriptionStatus;
    use candid::Principal;

    #[test]
    fn check_user_insert_and_get() {
        let repository = UserRepository::default();
        let user = User {
            id: Principal::from_slice(&[u8::MAX; 29]),
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
        };

        assert!(repository.get(&UserKey(user.id)).is_none());

        repository.insert(UserKey(user.id), user.clone());
        assert_eq!(repository.get(&UserKey(user.id)), Some(user));
    }

    #[test]
    fn get_subscribed_users() {
        let repository = UserRepository::default();

        let unsubscribed_user = User {
            id: Principal::from_slice(&[0; 29]),
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
        };
        repository.insert(UserKey(unsubscribed_user.id), unsubscribed_user.clone());

        let email = "john@example.com".to_string();
        let subscribed_user = User {
            id: Principal::from_slice(&[1; 29]),
            subscription_status: UserSubscriptionStatus::Pending(email.clone()),
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
        };
        repository.insert(UserKey(subscribed_user.id), subscribed_user.clone());

        let another_email = "martin@example.com".to_string();
        let another_subscribed_user = User {
            id: Principal::from_slice(&[2; 29]),
            subscription_status: UserSubscriptionStatus::Pending(another_email.clone()),
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
        };
        repository.insert(
            UserKey(another_subscribed_user.id),
            another_subscribed_user.clone(),
        );

        let all_users = repository.list();
        assert_eq!(all_users.len(), 3);

        let subscribed_users = repository.get_subscribed_users();
        assert_eq!(subscribed_users.len(), 2);
        let subscribed = SubscribedUser {
            user_principal: subscribed_user.id,
            email,
        };
        assert!(subscribed_users.contains(&subscribed));
        let another_subscribed = SubscribedUser {
            user_principal: another_subscribed_user.id,
            email: another_email,
        };
        assert!(subscribed_users.contains(&another_subscribed));
    }

    #[test]
    fn check_user_removal() {
        let repository = UserRepository::default();
        let user = User {
            id: Principal::from_slice(&[u8::MAX; 29]),
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
        };

        repository.insert(UserKey(user.id), user.clone());
        assert_eq!(repository.get(&UserKey(user.id)), Some(user.clone()));
        repository.remove(&UserKey(user.id));
        assert!(repository.get(&UserKey(user.id)).is_none());
    }
}
