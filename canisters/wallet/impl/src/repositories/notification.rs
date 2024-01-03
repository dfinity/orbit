use super::indexes::notification_user_index::NotificationUserIndexRepository;
use crate::{
    core::{with_memory_manager, Memory, NOTIFICATION_MEMORY_ID},
    models::{
        indexes::notification_user_index::NotificationUserIndexCriteria, Notification,
        NotificationKey, NotificationStatus, UserId,
    },
};
use ic_canister_core::{
    repository::{IndexRepository, Repository},
    types::Timestamp,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<NotificationKey, Notification, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(NOTIFICATION_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref NOTIFICATION_REPOSITORY: Arc<NotificationRepository> =
        Arc::new(NotificationRepository::default());
}

/// A repository that enables managing notifications in stable memory.
#[derive(Default, Debug)]
pub struct NotificationRepository {
    user_index: NotificationUserIndexRepository,
}

impl Repository<NotificationKey, Notification> for NotificationRepository {
    fn list(&self) -> Vec<Notification> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &NotificationKey) -> Option<Notification> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: NotificationKey, value: Notification) -> Option<Notification> {
        DB.with(|m| match m.borrow_mut().insert(key, value.clone()) {
            Some(prev) => {
                let prev_user_index = prev.to_index_for_target_user();
                if prev_user_index != value.to_index_for_target_user() {
                    self.user_index.remove(&prev_user_index);
                    self.user_index.insert(value.to_index_for_target_user());
                }

                Some(prev)
            }
            None => {
                self.user_index.insert(value.to_index_for_target_user());

                None
            }
        })
    }

    fn remove(&self, key: &NotificationKey) -> Option<Notification> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(prev) => {
                self.user_index.remove(&prev.to_index_for_target_user());

                Some(prev)
            }
            None => None,
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl NotificationRepository {
    pub fn find_by_user_id(&self, user_id: UserId) -> Vec<Notification> {
        self.user_index
            .find_by_criteria(NotificationUserIndexCriteria {
                user_id: user_id.to_owned(),
                from_dt: None,
                to_dt: None,
            })
            .iter()
            .filter_map(|id| self.get(&Notification::key(*id)))
            .collect()
    }

    pub fn find_by_user_where(
        &self,
        user_id: UserId,
        condition: NotificationFindByUserWhereClause,
    ) -> Vec<Notification> {
        self.user_index
            .find_by_criteria(NotificationUserIndexCriteria {
                user_id: user_id.to_owned(),
                from_dt: condition.created_dt_from,
                to_dt: condition.created_dt_to,
            })
            .iter()
            .filter_map(|id| match self.get(&Notification::key(*id)) {
                Some(notification) => {
                    let mut match_type = true;
                    let mut match_status = true;

                    if let Some(notification_type) = &condition.notification_type {
                        match_type =
                            notification.notification_type.to_string() == *notification_type;
                    }

                    if let Some(status) = &condition.status {
                        match_status = notification.status == *status;
                    }

                    match match_type && match_status {
                        true => Some(notification),
                        false => None,
                    }
                }
                None => None,
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct NotificationFindByUserWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub notification_type: Option<String>,
    pub status: Option<NotificationStatus>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::notification_test_utils::mock_notification;
    use uuid::Uuid;

    #[test]
    fn perform_crud() {
        let repository = NotificationRepository::default();
        let notification = mock_notification();

        assert!(repository.get(&notification.to_key()).is_none());

        repository.insert(notification.to_key(), notification.clone());

        assert!(repository.get(&notification.to_key()).is_some());
        assert!(repository.remove(&notification.to_key()).is_some());
        assert!(repository.get(&notification.to_key()).is_none());
    }

    #[test]
    fn find_by_user_id() {
        let repository = NotificationRepository::default();
        let mut notification = mock_notification();
        let user_id = Uuid::new_v4();
        notification.target_user_id = *user_id.as_bytes();

        repository.insert(notification.to_key(), notification.clone());

        assert_eq!(
            repository.find_by_user_id(*user_id.as_bytes()),
            vec![notification]
        );
    }
}
