use super::indexes::{
    request_index::RequestIndexRepository, request_resource_index::RequestResourceIndexRepository,
};
use crate::{
    core::{
        metrics::{metrics_observe_insert_request, metrics_observe_remove_request},
        observer::Observer,
        with_memory_manager, Memory, REQUEST_MEMORY_ID,
    },
    errors::RepositoryError,
    jobs::{jobs_observe_insert_request, jobs_observe_remove_request},
    models::{
        indexes::{
            request_index::RequestIndexFields, request_resource_index::RequestResourceIndexCriteria,
        },
        resource::Resource,
        ListRequestsOperationType, Request, RequestId, RequestKey, RequestStatusCode,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::{
    repository::{IndexRepository, IndexedRepository, Repository, StableDb},
    types::{Timestamp, UUID},
};
use station_api::ListRequestsSortBy;
use std::{cell::RefCell, collections::HashSet, sync::Arc, u64};

thread_local! {
    static DB: RefCell<StableBTreeMap<RequestKey, Request, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
        RefCell::new(
            StableBTreeMap::init(memory_manager.get(REQUEST_MEMORY_ID))
        )
    });
}

lazy_static! {
    pub static ref REQUEST_REPOSITORY: Arc<RequestRepository> =
        Arc::new(RequestRepository::default());
}

/// A repository that enables managing system requests in stable memory.
#[derive(Debug)]
pub struct RequestRepository {
    index: RequestIndexRepository,
    resource_index: RequestResourceIndexRepository,
    change_observer: Observer<(Request, Option<Request>)>,
    remove_observer: Observer<Request>,
}

impl Default for RequestRepository {
    fn default() -> Self {
        let mut change_observer = Observer::default();
        metrics_observe_insert_request(&mut change_observer);
        jobs_observe_insert_request(&mut change_observer);

        let mut remove_observer = Observer::default();
        metrics_observe_remove_request(&mut remove_observer);
        jobs_observe_remove_request(&mut remove_observer);

        Self {
            change_observer,
            remove_observer,
            index: RequestIndexRepository::default(),
            resource_index: Default::default(),
        }
    }
}

impl StableDb<RequestKey, Request, VirtualMemory<Memory>> for RequestRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<RequestKey, Request, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<RequestKey, Request, VirtualMemory<Memory>> for RequestRepository {
    fn remove_entry_indexes(&self, entry: &Request) {
        entry.to_index_for_resource().iter().for_each(|index| {
            self.resource_index.remove(index);
        });

        entry.to_indexes().iter().for_each(|(index_key, _)| {
            self.index.remove(index_key);
        });
    }

    fn add_entry_indexes(&self, entry: &Request) {
        entry.to_index_for_resource().into_iter().for_each(|index| {
            self.resource_index.insert(index);
        });

        entry
            .to_indexes()
            .into_iter()
            .for_each(|(index_key, index_fields)| {
                self.index.insert(index_key, index_fields);
            });
    }
}

impl Repository<RequestKey, Request, VirtualMemory<Memory>> for RequestRepository {
    fn insert(&self, key: RequestKey, value: Request) -> Option<Request> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.save_entry_indexes(&value, prev.as_ref());

            let args = (value, prev);
            self.change_observer.notify(&args);

            args.1
        })
    }

    fn remove(&self, key: &RequestKey) -> Option<Request> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            if let Some(prev) = &prev {
                self.remove_entry_indexes(prev);

                self.remove_observer.notify(prev);
            }

            prev
        })
    }
}

impl RequestRepository {
    /// Find requests that have the provided status and would be expired between the provided timestamps.
    pub fn find_by_status_and_expiration_dt(
        &self,
        status: RequestStatusCode,
        expiration_dt_from: Option<Timestamp>,
        expiration_dt_to: Option<Timestamp>,
    ) -> Vec<Request> {
        self.index
            .find_by_status(status, None)
            .iter()
            .filter_map(|(request_id, fields)| {
                let min = expiration_dt_from.unwrap_or(u64::MIN);
                let max = expiration_dt_to.unwrap_or(u64::MAX);

                if fields.expiration_dt < min || fields.expiration_dt > max {
                    return None;
                }

                self.get(&RequestKey { id: *request_id })
            })
            .collect::<Vec<Request>>()
    }

    /// Find requests that have the provided status and has been modified between the provided timestamps.
    pub fn find_by_status(
        &self,
        status: RequestStatusCode,
        from_last_modified_dt: Option<Timestamp>,
        to_last_modified_dt: Option<Timestamp>,
    ) -> Vec<Request> {
        self.index
            .find_by_status(status, None)
            .iter()
            .filter_map(|(request_id, fields)| {
                if let Some(from_dt) = from_last_modified_dt {
                    if fields.last_modified_at < from_dt {
                        return None;
                    }
                }

                if let Some(to_dt) = to_last_modified_dt {
                    if fields.last_modified_at > to_dt {
                        return None;
                    }
                }

                self.get(&RequestKey { id: *request_id })
            })
            .collect::<Vec<Request>>()
    }

    /// Find requests that are scheduled between the provided timestamps.
    pub fn find_scheduled(
        &self,
        from_dt: Option<Timestamp>,
        to_dt: Option<Timestamp>,
    ) -> Vec<Request> {
        self.index
            .find_by_scheduled_at_between(
                from_dt.unwrap_or(u64::MIN),
                to_dt.unwrap_or(u64::MAX),
                None,
            )
            .iter()
            .filter_map(|(request_id, _)| self.get(&RequestKey { id: *request_id }))
            .collect::<Vec<Request>>()
    }

    /// Get the list of Resource for a request id.
    pub fn get_resources(&self, request_id: &RequestId) -> Vec<Resource> {
        self.resource_index
            .find_by_criteria(RequestResourceIndexCriteria {
                request_id: *request_id,
            })
            .into_iter()
            .collect()
    }

    /// Find request ids based on the provided condition.
    ///
    /// The request ids are sorted based on the provided sort strategy.
    pub fn find_ids_where(
        &self,
        condition: RequestWhereClause,
        sort_by: Option<ListRequestsSortBy>,
    ) -> Result<Vec<UUID>, RepositoryError> {
        let mut entries = Vec::<(RequestId, RequestIndexFields)>::new();

        // first find the initial result set that would narrow down the search space
        entries.extend(self.index.find_by_created_at_between(
            condition.created_dt_from.unwrap_or(0),
            condition.created_dt_to.unwrap_or(u64::MAX),
            None,
        ));

        // transform lists to constant lookup time
        let where_approvals: HashSet<_> = condition.approvers.iter().cloned().collect();
        let where_not_approvals: HashSet<_> = condition.not_approvers.iter().cloned().collect();
        let where_requesters: HashSet<_> = condition.requesters.iter().cloned().collect();
        let where_not_requesters: HashSet<_> = condition.not_requesters.iter().cloned().collect();
        let where_status: HashSet<_> = condition.statuses.iter().collect();
        let where_not_ids: HashSet<_> = condition.excluded_ids.iter().collect();

        // filter the result set based on the condition
        entries = entries
            .into_iter()
            .filter(|(id, fields)| {
                if !where_not_ids.is_empty() && where_not_ids.contains(id) {
                    return false;
                }

                if !where_status.is_empty() && !where_status.contains(&fields.status) {
                    return false;
                }

                if fields.expiration_dt < condition.expiration_dt_from.unwrap_or(u64::MIN)
                    || fields.expiration_dt > condition.expiration_dt_to.unwrap_or(u64::MAX)
                {
                    return false;
                }

                if !condition.operation_types.is_empty()
                    && !condition
                        .operation_types
                        .iter()
                        .any(|filter_by_operation| fields.operation_type.eq(filter_by_operation))
                {
                    return false;
                }

                if !where_requesters.is_empty() && !where_requesters.contains(&fields.requested_by)
                {
                    return false;
                }

                if !where_not_requesters.is_empty()
                    && where_not_requesters.contains(&fields.requested_by)
                {
                    return false;
                }

                let mut all_approvals = fields.approved_by.to_owned();
                all_approvals.extend(fields.rejected_by.to_owned());

                if !where_approvals.is_empty()
                    && !all_approvals
                        .iter()
                        .any(|approver| where_approvals.contains(approver))
                {
                    return false;
                }

                if !where_not_approvals.is_empty()
                    && all_approvals
                        .iter()
                        .any(|approver| where_not_approvals.contains(approver))
                {
                    return false;
                }

                true
            })
            .collect::<Vec<(RequestId, RequestIndexFields)>>();

        // Sorts the request IDs based on the provided sort field and direction.
        entries.sort_by(|(a_id, a), (b_id, b)| {
            // Default sort by creation timestamp descending
            let mut ord = a.created_at.cmp(&b.created_at);
            let mut dir = station_api::SortDirection::Desc;

            if let Some(sort_by) = &sort_by {
                match sort_by {
                    ListRequestsSortBy::CreatedAt(direction) => {
                        ord = a.created_at.cmp(&b.created_at);
                        dir = direction.clone();
                    }
                    ListRequestsSortBy::ExpirationDt(direction) => {
                        ord = a.expiration_dt.cmp(&b.expiration_dt);
                        dir = direction.clone();
                    }
                    ListRequestsSortBy::LastModificationDt(direction) => {
                        ord = a.last_modified_at.cmp(&b.last_modified_at);
                        dir = direction.clone();
                    }
                }
            }

            match ord {
                std::cmp::Ordering::Equal => a_id.cmp(b_id),
                _ => match dir {
                    station_api::SortDirection::Asc => ord.reverse(),
                    station_api::SortDirection::Desc => ord,
                },
            }
        });

        // self.sort_ids_with_strategy(&mut ids, &sort_by);
        Ok(entries.iter().map(|(id, _)| *id).collect())
    }

    #[cfg(test)]
    pub fn with_empty_observers() -> Self {
        Self {
            change_observer: Observer::default(),
            remove_observer: Observer::default(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub expiration_dt_from: Option<Timestamp>,
    pub expiration_dt_to: Option<Timestamp>,
    pub operation_types: Vec<ListRequestsOperationType>,
    pub statuses: Vec<RequestStatusCode>,
    pub approvers: Vec<UUID>,
    pub not_approvers: Vec<UUID>,
    pub requesters: Vec<UUID>,
    pub not_requesters: Vec<UUID>,
    pub excluded_ids: Vec<UUID>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        indexes::request_resource_index::RequestResourceIndex,
        request_test_utils::{self, mock_request},
        resource::{AccountResourceAction, ResourceId},
        AddUserGroupOperation, AddUserGroupOperationInput, EditUserGroupOperation,
        EditUserGroupOperationInput, RequestOperation, RequestStatus, TransferOperation,
        TransferOperationInput,
    };
    use uuid::Uuid;

    #[test]
    fn perform_crud() {
        let repository = RequestRepository::default();
        let request = mock_request();

        assert!(repository.get(&request.to_key()).is_none());

        repository.insert(request.to_key(), request.clone());

        assert!(repository.get(&request.to_key()).is_some());
        assert!(repository.remove(&request.to_key()).is_some());
        assert!(repository.get(&request.to_key()).is_none());
    }

    #[test]
    fn find_by_expiration_dt_and_status() {
        let repository = RequestRepository::default();
        for i in 0..=50 {
            let mut request = request_test_utils::mock_request();
            request.id = *Uuid::new_v4().as_bytes();
            request.expiration_dt = i;
            request.status = RequestStatus::Created;
            repository.insert(request.to_key(), request.clone());
        }

        let last_six =
            repository.find_by_status_and_expiration_dt(RequestStatusCode::Created, Some(45), None);

        let middle_eleven = repository.find_by_status_and_expiration_dt(
            RequestStatusCode::Created,
            Some(30),
            Some(40),
        );

        let first_three =
            repository.find_by_status_and_expiration_dt(RequestStatusCode::Created, None, Some(2));

        assert_eq!(last_six.len(), 6);
        assert_eq!(middle_eleven.len(), 11);
        assert_eq!(first_three.len(), 3);
    }

    #[test]
    fn no_future_expiration_dt() {
        let repository = RequestRepository::default();
        let mut request = request_test_utils::mock_request();
        request.expiration_dt = 10;

        repository.insert(request.to_key(), request.clone());

        let requests =
            repository.find_by_status_and_expiration_dt(request.status.into(), Some(20), None);

        assert!(requests.is_empty());
    }

    #[test]
    fn find_with_expiration_dt() {
        let mut request = request_test_utils::mock_request();
        request.id = *Uuid::new_v4().as_bytes();
        request.created_timestamp = 5;
        request.expiration_dt = 10;
        request.status = RequestStatus::Created;
        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        let mut request_not_match = request_test_utils::mock_request();
        request_not_match.id = *Uuid::new_v4().as_bytes();
        request_not_match.created_timestamp = 5;
        request_not_match.expiration_dt = 9;
        request_not_match.status = RequestStatus::Created;
        REQUEST_REPOSITORY.insert(request_not_match.to_key(), request_not_match.clone());

        let mut condition = RequestWhereClause {
            created_dt_from: None,
            created_dt_to: None,
            expiration_dt_from: Some(10),
            expiration_dt_to: None,
            operation_types: vec![],
            statuses: vec![],
            approvers: vec![],
            not_approvers: vec![],
            requesters: vec![],
            not_requesters: vec![],
            excluded_ids: vec![],
        };

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(requests.len(), 1);

        let found_request = REQUEST_REPOSITORY
            .get(&RequestKey { id: requests[0] })
            .unwrap();

        assert_eq!(found_request, request);

        condition.expiration_dt_from = Some(11);

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert!(requests.is_empty());
    }

    #[test]
    fn find_with_creation_dt() {
        let mut request = request_test_utils::mock_request();
        request.id = *Uuid::new_v4().as_bytes();
        request.created_timestamp = 10;
        request.status = RequestStatus::Created;
        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        let mut request_not_match = request_test_utils::mock_request();
        request_not_match.id = *Uuid::new_v4().as_bytes();
        request_not_match.created_timestamp = 12;
        request_not_match.status = RequestStatus::Created;
        REQUEST_REPOSITORY.insert(request_not_match.to_key(), request_not_match.clone());

        let mut condition = RequestWhereClause {
            created_dt_from: Some(9),
            created_dt_to: Some(11),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: vec![],
            statuses: vec![],
            approvers: vec![],
            not_approvers: vec![],
            requesters: vec![],
            not_requesters: vec![],
            excluded_ids: vec![],
        };

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(requests.len(), 1);

        let found_request = REQUEST_REPOSITORY
            .get(&RequestKey { id: requests[0] })
            .unwrap();

        assert_eq!(found_request, request);

        condition.created_dt_from = Some(8);
        condition.created_dt_to = Some(9);

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert!(requests.is_empty());
    }

    #[test]
    fn find_with_default_filters() {
        for i in 0..100 {
            let mut request = mock_request();
            request.id = *Uuid::new_v4().as_bytes();
            request.created_timestamp = i;
            request.expiration_dt = i + 100;
            request.status = match i % 2 {
                0 => RequestStatus::Created,
                1 => RequestStatus::Approved,
                _ => RequestStatus::Rejected,
            };

            REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());
        }

        let condition = RequestWhereClause {
            created_dt_from: Some(50),
            created_dt_to: Some(100),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: Vec::new(),
            requesters: Vec::new(),
            approvers: Vec::new(),
            not_approvers: vec![],
            statuses: vec![RequestStatusCode::Created],
            not_requesters: vec![],
            excluded_ids: vec![],
        };

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(requests.len(), 25);

        let condition = RequestWhereClause {
            created_dt_from: Some(0),
            created_dt_to: Some(100),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: Vec::new(),
            requesters: Vec::new(),
            approvers: Vec::new(),
            not_approvers: vec![],
            statuses: vec![RequestStatusCode::Approved],
            not_requesters: vec![],
            excluded_ids: vec![],
        };

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(requests.len(), 50);

        let condition = RequestWhereClause {
            created_dt_from: Some(0),
            created_dt_to: Some(100),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: Vec::new(),
            requesters: Vec::new(),
            approvers: Vec::new(),
            not_approvers: vec![],
            statuses: vec![RequestStatusCode::Approved, RequestStatusCode::Created],
            not_requesters: vec![],
            excluded_ids: vec![],
        };

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(requests.len(), 100);

        let condition = RequestWhereClause {
            created_dt_from: Some(0),
            created_dt_to: Some(100),
            expiration_dt_from: Some(110),
            expiration_dt_to: Some(120),
            operation_types: Vec::new(),
            requesters: Vec::new(),
            approvers: Vec::new(),
            not_approvers: vec![],
            statuses: vec![RequestStatusCode::Approved],
            not_requesters: vec![],
            excluded_ids: vec![],
        };

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(requests.len(), 5);
    }

    #[test]
    fn find_with_empty_where_clause_should_return_all() {
        request_repository_test_utils::add_requests_to_repository(100);

        let condition = RequestWhereClause {
            created_dt_from: None,
            created_dt_to: None,
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: vec![],
            statuses: vec![],
            approvers: vec![],
            not_approvers: vec![],
            requesters: vec![],
            not_requesters: vec![],
            excluded_ids: vec![],
        };

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(requests.len(), 100);
    }

    #[test]
    fn update_resource_index_on_request_creation() {
        let request = mock_request();

        let RequestOperation::Transfer(TransferOperation {
            input: TransferOperationInput {
                from_account_id, ..
            },
            ..
        }) = request.operation
        else {
            panic!("Expected transfer operation");
        };

        assert!(matches!(request.operation, RequestOperation::Transfer(_)));

        REQUEST_REPOSITORY.insert(RequestKey { id: request.id }, request.clone());

        assert!(REQUEST_REPOSITORY
            .resource_index
            .exists(&RequestResourceIndex {
                request_id: request.id,
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
            }));

        assert!(REQUEST_REPOSITORY
            .resource_index
            .exists(&RequestResourceIndex {
                request_id: request.id,
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    from_account_id,
                ))),
            }));
    }

    #[test]
    fn find_only_specified_types() {
        let mut add_group_request = mock_request();
        add_group_request.last_modification_timestamp = 1;
        add_group_request.status = RequestStatus::Approved;
        add_group_request.operation = RequestOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "foo".to_string(),
            },
        });
        REQUEST_REPOSITORY.insert(add_group_request.to_key(), add_group_request.clone());

        let mut edit_group_request = mock_request();
        edit_group_request.last_modification_timestamp = 2;
        add_group_request.status = RequestStatus::Approved;
        edit_group_request.operation = RequestOperation::EditUserGroup(EditUserGroupOperation {
            input: EditUserGroupOperationInput {
                user_group_id: *Uuid::new_v4().as_bytes(),
                name: "bar".to_string(),
            },
        });
        REQUEST_REPOSITORY.insert(edit_group_request.to_key(), edit_group_request.clone());

        let condition = RequestWhereClause {
            created_dt_from: Some(0),
            created_dt_to: Some(100),
            expiration_dt_from: None,
            expiration_dt_to: None,
            operation_types: vec![ListRequestsOperationType::AddUserGroup],
            requesters: Vec::new(),
            approvers: Vec::new(),
            not_approvers: vec![],
            statuses: vec![RequestStatusCode::Approved],
            not_requesters: vec![],
            excluded_ids: vec![],
        };

        let requests = REQUEST_REPOSITORY
            .find_ids_where(condition.clone(), None)
            .unwrap();

        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0], add_group_request.id);
    }
}

#[cfg(feature = "canbench")]
mod benchs {
    use super::*;
    use crate::models::{request_test_utils::mock_request, RequestStatus};
    use canbench_rs::{bench, BenchResult};
    use uuid::Uuid;

    #[bench(raw)]
    fn repository_batch_insert_100_requests() -> BenchResult {
        canbench_rs::bench_fn(|| {
            request_repository_test_utils::add_requests_to_repository(100);
        })
    }

    #[bench(raw)]
    fn repository_list_all_requests() -> BenchResult {
        request_repository_test_utils::add_requests_to_repository(1_000);

        canbench_rs::bench_fn(|| {
            let _ = REQUEST_REPOSITORY.list();
        })
    }

    #[bench(raw)]
    fn repository_filter_all_request_ids_by_default_filters() -> BenchResult {
        for i in 0..2_500 {
            let mut request = mock_request();
            request.id = *Uuid::new_v4().as_bytes();
            request.created_timestamp = i;
            request.status = match i % 2 {
                0 => RequestStatus::Created,
                1 => RequestStatus::Approved,
                _ => RequestStatus::Rejected,
            };

            REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());
        }

        canbench_rs::bench_fn(|| {
            let _ = REQUEST_REPOSITORY.find_ids_where(
                RequestWhereClause {
                    created_dt_from: Some(500),
                    created_dt_to: Some(1500),
                    expiration_dt_from: None,
                    expiration_dt_to: None,
                    operation_types: Vec::new(),
                    requesters: Vec::new(),
                    approvers: Vec::new(),
                    not_approvers: vec![],
                    statuses: vec![RequestStatusCode::Created],
                    excluded_ids: vec![],
                    not_requesters: vec![],
                },
                None,
            );
        })
    }
}

#[cfg(any(test, feature = "canbench"))]
mod request_repository_test_utils {
    use super::*;
    use crate::models::request_test_utils::mock_request;
    use uuid::Uuid;

    pub fn add_requests_to_repository(count: usize) {
        for _ in 0..count {
            let mut request = mock_request();
            request.id = *Uuid::new_v4().as_bytes();

            REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());
        }
    }
}
