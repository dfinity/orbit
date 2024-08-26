use super::indexes::notification_user_index::NotificationUserIndexRepository;
use crate::{
    core::{utils::SortDirection, with_memory_manager, Memory, NOTIFICATION_MEMORY_ID},
    models::{
        indexes::notification_user_index::NotificationUserIndexCriteria, Notification,
        NotificationKey, NotificationStatus, UserId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::{
    repository::{IndexRepository, IndexedRepository, Repository, StableDb},
    types::Timestamp,
};
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

impl StableDb<NotificationKey, Notification, VirtualMemory<Memory>> for NotificationRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<NotificationKey, Notification, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<NotificationKey, Notification, VirtualMemory<Memory>>
    for NotificationRepository
{
    fn remove_entry_indexes(&self, value: &Notification) {
        self.user_index.remove(&value.to_index_for_target_user());
    }

    fn add_entry_indexes(&self, value: &Notification) {
        self.user_index.insert(value.to_index_for_target_user());
    }

    /// Clears all the indexes.
    fn clear_indexes(&self) {
        self.user_index.clear();
    }
}

impl Repository<NotificationKey, Notification, VirtualMemory<Memory>> for NotificationRepository {
    fn insert(&self, key: NotificationKey, value: Notification) -> Option<Notification> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.save_entry_indexes(&value, prev.as_ref());

            prev
        })
    }

    fn remove(&self, key: &NotificationKey) -> Option<Notification> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            if let Some(prev) = &prev {
                self.remove_entry_indexes(prev);
            }

            prev
        })
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
        let mut notifications: Vec<Notification> = self
            .user_index
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
            .collect();

        match condition.sort_by {
            Some(sort_by) => self.sort_by(&mut notifications, sort_by).to_vec(),
            None => self
                .sort_by(
                    &mut notifications,
                    NotificationSortBy::CreatedDt(SortDirection::Desc),
                )
                .to_vec(),
        }
    }

    pub fn sort_by<'a>(
        &self,
        notifications: &'a mut [Notification],
        sort_by: NotificationSortBy,
    ) -> &'a [Notification] {
        match sort_by {
            NotificationSortBy::CreatedDt(direction) => {
                notifications.sort_by(|a, b| match direction {
                    SortDirection::Asc => a.created_timestamp.cmp(&b.created_timestamp),
                    SortDirection::Desc => b.created_timestamp.cmp(&a.created_timestamp),
                });

                notifications
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotificationSortBy {
    CreatedDt(SortDirection),
}

#[derive(Debug)]
pub struct NotificationFindByUserWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub notification_type: Option<String>,
    pub status: Option<NotificationStatus>,
    pub sort_by: Option<NotificationSortBy>,
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
