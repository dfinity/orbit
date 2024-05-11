use super::indexes::{
    request_approver_index::RequestApproverIndexRepository,
    request_creation_time_index::RequestCreationTimeIndexRepository,
    request_expiration_time_index::RequestExpirationTimeIndexRepository,
    request_key_creation_time_index::RequestKeyCreationTimeIndexRepository,
    request_key_expiration_time_index::RequestKeyExpirationTimeIndexRepository,
    request_operation_type_index::RequestOperationTypeIndexRepository,
    request_requester_index::RequestRequesterIndexRepository,
    request_resource_index::RequestResourceIndexRepository,
    request_scheduled_index::RequestScheduledIndexRepository,
    request_sort_index::RequestSortIndexRepository,
    request_status_index::RequestStatusIndexRepository,
    request_status_modification_index::RequestStatusModificationIndexRepository,
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
            request_approver_index::{RequestApproverIndex, RequestApproverIndexCriteria},
            request_creation_time_index::RequestCreationTimeIndexCriteria,
            request_expiration_time_index::RequestExpirationTimeIndexCriteria,
            request_key_creation_time_index::RequestKeyCreationTimeIndexCriteria,
            request_key_expiration_time_index::RequestKeyExpirationTimeIndexCriteria,
            request_operation_type_index::{
                RequestOperationTypeIndex, RequestOperationTypeIndexCriteria,
            },
            request_requester_index::{RequestRequesterIndex, RequestRequesterIndexCriteria},
            request_resource_index::RequestResourceIndexCriteria,
            request_scheduled_index::RequestScheduledIndexCriteria,
            request_sort_index::RequestSortIndexKey,
            request_status_index::{RequestStatusIndex, RequestStatusIndexCriteria},
            request_status_modification_index::RequestStatusModificationIndexCriteria,
        },
        request_operation_filter_type::RequestOperationFilterType,
        resource::Resource,
        Request, RequestId, RequestKey, RequestStatusCode, UserId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::{
    repository::{
        IdentitySelectionFilter, IndexRepository, NotSelectionFilter, OrSelectionFilter,
        RefreshIndexMode, Repository, SelectionFilter, SortDirection, SortingStrategy,
    },
    types::{Timestamp, UUID},
};
use station_api::ListRequestsSortBy;
use std::{cell::RefCell, collections::HashSet, sync::Arc};

thread_local! {
    static DB: RefCell<StableBTreeMap<RequestKey, Request, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
        RefCell::new(
            StableBTreeMap::init(memory_manager.get(REQUEST_MEMORY_ID))
        )
    });

    /// The observer that listens to changes in the request repository.
    static CHANGE_OBSERVER: RefCell<Observer<(Request, Option<Request>)>> = Default::default();

    /// The observer that listens to removals in the request repository.
    static REMOVE_OBSERVER: RefCell<Observer<Request>> = Default::default();
}

lazy_static! {
    pub static ref REQUEST_REPOSITORY: Arc<RequestRepository> =
        Arc::new(RequestRepository::default());
}

/// A repository that enables managing system requests in stable memory.
#[derive(Debug)]
pub struct RequestRepository {
    approver_index: RequestApproverIndexRepository,
    creation_dt_index: RequestCreationTimeIndexRepository,
    expiration_dt_index: RequestExpirationTimeIndexRepository,
    status_index: RequestStatusIndexRepository,
    scheduled_index: RequestScheduledIndexRepository,
    requester_index: RequestRequesterIndexRepository,
    status_modification_index: RequestStatusModificationIndexRepository,
    prefixed_creation_time_index: RequestKeyCreationTimeIndexRepository,
    prefixed_expiration_time_index: RequestKeyExpirationTimeIndexRepository,
    sort_index: RequestSortIndexRepository,
    resource_index: RequestResourceIndexRepository,
    operation_type_index: RequestOperationTypeIndexRepository,
}

impl Default for RequestRepository {
    fn default() -> Self {
        CHANGE_OBSERVER.with(|observer| {
            metrics_observe_insert_request(&mut observer.borrow_mut());
            jobs_observe_insert_request(&mut observer.borrow_mut());
        });

        REMOVE_OBSERVER.with(|observer| {
            metrics_observe_remove_request(&mut observer.borrow_mut());
            jobs_observe_remove_request(&mut observer.borrow_mut());
        });

        Self {
           ..Default::default(),
        }
    }
}

impl Repository<RequestKey, Request> for RequestRepository {
    fn list(&self) -> Vec<Request> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &RequestKey) -> Option<Request> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: RequestKey, value: Request) -> Option<Request> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.approver_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_approvers()),
                    current: value.to_index_for_approvers(),
                });
            self.operation_type_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_by_operation_types()),
                    current: value.to_index_by_operation_types(),
                });
            self.requester_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_for_requester()),
                    current: Some(value.to_index_for_requester()),
                });
            self.scheduled_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().and_then(|prev| prev.to_index_by_scheduled()),
                    current: value.to_index_by_scheduled(),
                });
            self.creation_dt_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_creation_dt()),
                    current: Some(value.to_index_by_creation_dt()),
                });
            self.prefixed_creation_time_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev
                        .clone()
                        .map(|prev| prev.to_index_by_key_and_creation_dt()),
                    current: Some(value.to_index_by_key_and_creation_dt()),
                });
            self.expiration_dt_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_expiration_dt()),
                    current: Some(value.to_index_by_expiration_dt()),
                });
            self.prefixed_expiration_time_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev
                        .clone()
                        .map(|prev| prev.to_index_by_key_and_expiration_dt()),
                    current: Some(value.to_index_by_key_and_expiration_dt()),
                });
            self.status_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_status()),
                    current: Some(value.to_index_by_status()),
                });
            self.sort_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_for_sorting()),
                    current: Some(value.to_index_for_sorting()),
                });
            self.status_modification_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev
                        .clone()
                        .map(|prev| prev.to_index_by_status_and_modification()),
                    current: Some(value.to_index_by_status_and_modification()),
                });

            self.resource_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map(|prev| prev.to_index_for_resource())
                        .unwrap_or_default(),
                    current: value.to_index_for_resource(),
                });

            let args = (value, prev);
            CHANGE_OBSERVER.with(|observer| observer.borrow().notify(&args));

            args.1
        })
    }

    fn remove(&self, key: &RequestKey) -> Option<Request> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            self.approver_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_approvers()),
                });
            self.operation_type_index.refresh_index_on_modification(
                RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_by_operation_types()),
                },
            );
            self.requester_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_for_requester()),
                });
            self.scheduled_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().and_then(|prev| prev.to_index_by_scheduled()),
                });
            self.creation_dt_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_creation_dt()),
                });
            self.expiration_dt_index.refresh_index_on_modification(
                RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_expiration_dt()),
                },
            );
            self.prefixed_creation_time_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev
                        .clone()
                        .map(|prev| prev.to_index_by_key_and_creation_dt()),
                });
            self.prefixed_expiration_time_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev
                        .clone()
                        .map(|prev| prev.to_index_by_key_and_expiration_dt()),
                });
            self.status_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_status()),
                });
            self.sort_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_for_sorting()),
                });
            self.status_modification_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev
                        .clone()
                        .map(|prev| prev.to_index_by_status_and_modification()),
                });
            self.resource_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map(|prev| prev.to_index_for_resource())
                        .unwrap_or_default(),
                });

            if let Some(prev) = &prev {
                REMOVE_OBSERVER.with(|observer| observer.borrow().notify(prev));
            }

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl RequestRepository {
    pub fn exists(&self, key: &RequestKey) -> bool {
        DB.with(|m| m.borrow().contains_key(key))
    }

    pub fn find_by_expiration_dt_and_status(
        &self,
        expiration_dt_from: Option<Timestamp>,
        expiration_dt_to: Option<Timestamp>,
        status: String,
    ) -> Vec<Request> {
        let requests =
            self.expiration_dt_index
                .find_by_criteria(RequestExpirationTimeIndexCriteria {
                    from_dt: expiration_dt_from,
                    to_dt: expiration_dt_to,
                });

        requests
            .iter()
            .filter_map(|id| match self.get(&Request::key(*id)) {
                Some(request) => {
                    if request
                        .status
                        .to_type()
                        .to_string()
                        .eq_ignore_ascii_case(status.as_str())
                    {
                        Some(request)
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect::<Vec<Request>>()
    }

    pub fn find_by_status(
        &self,
        status: RequestStatusCode,
        from_last_modified_dt: Option<Timestamp>,
        to_last_modified_dt: Option<Timestamp>,
    ) -> Vec<Request> {
        let ids = self.status_modification_index.find_by_criteria(
            RequestStatusModificationIndexCriteria {
                status,
                from_dt: from_last_modified_dt,
                to_dt: to_last_modified_dt,
            },
        );

        ids.iter()
            .filter_map(|id| self.get(&Request::key(*id)))
            .collect::<Vec<Request>>()
    }

    pub fn find_scheduled(
        &self,
        from_dt: Option<Timestamp>,
        to_dt: Option<Timestamp>,
    ) -> Vec<Request> {
        let requests = self
            .scheduled_index
            .find_by_criteria(RequestScheduledIndexCriteria { from_dt, to_dt });

        requests
            .iter()
            .filter_map(|id| self.get(&Request::key(*id)))
            .collect::<Vec<Request>>()
    }

    /// Checks if the request is of the provided status.
    pub fn exists_status(&self, request_id: &RequestId, status: RequestStatusCode) -> bool {
        self.status_index.exists(&RequestStatusIndex {
            request_id: *request_id,
            status,
        })
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

    /// Checks if the user has added their approval decision to the request.
    pub fn exists_approver(&self, request_id: &RequestId, approver_id: &UserId) -> bool {
        self.approver_index.exists(&RequestApproverIndex {
            approver_id: *approver_id,
            request_id: *request_id,
        })
    }

    /// Checks if the user has requested the request.
    pub fn exists_requester(&self, request_id: &RequestId, requester_id: &UserId) -> bool {
        self.requester_index.exists(&RequestRequesterIndex {
            requester_id: *requester_id,
            request_id: *request_id,
        })
    }

    pub fn find_ids_where(
        &self,
        condition: RequestWhereClause,
        sort_by: Option<ListRequestsSortBy>,
    ) -> Result<Vec<UUID>, RepositoryError> {
        let filters = self.build_where_filtering_strategy(condition);
        let request_ids = self.find_with_filters(filters);
        let mut ids = request_ids.into_iter().collect::<Vec<_>>();

        self.sort_ids_with_strategy(&mut ids, &sort_by);

        Ok(ids)
    }

    /// Sorts the request IDs based on the provided sort strategy.
    ///
    /// If no sort strategy is provided, it defaults to sorting by creation timestamp descending.
    fn sort_ids_with_strategy(
        &self,
        request_ids: &mut [UUID],
        sort_by: &Option<ListRequestsSortBy>,
    ) {
        match sort_by {
            Some(station_api::ListRequestsSortBy::CreatedAt(direction)) => {
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Creation,
                    direction: match direction {
                        station_api::SortDirection::Asc => Some(SortDirection::Ascending),
                        station_api::SortDirection::Desc => Some(SortDirection::Descending),
                    },
                };

                sort_strategy.sort(request_ids);
            }
            Some(station_api::ListRequestsSortBy::ExpirationDt(direction)) => {
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Expiration,
                    direction: match direction {
                        station_api::SortDirection::Asc => Some(SortDirection::Ascending),
                        station_api::SortDirection::Desc => Some(SortDirection::Descending),
                    },
                };

                sort_strategy.sort(request_ids);
            }
            Some(station_api::ListRequestsSortBy::LastModificationDt(direction)) => {
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Modification,
                    direction: match direction {
                        station_api::SortDirection::Asc => Some(SortDirection::Ascending),
                        station_api::SortDirection::Desc => Some(SortDirection::Descending),
                    },
                };

                sort_strategy.sort(request_ids);
            }
            None => {
                // Default sort by creation timestamp descending
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Creation,
                    direction: Some(SortDirection::Descending),
                };

                sort_strategy.sort(request_ids);
            }
        }
    }

    fn build_where_filtering_strategy<'a>(
        &'a self,
        condition: RequestWhereClause,
    ) -> Vec<Box<dyn SelectionFilter<'a, IdType = UUID> + 'a>> {
        let mut filters = Vec::new();

        if condition.created_dt_from.is_some() || condition.created_dt_to.is_some() {
            filters.push(Box::new(CreationDtSelectionFilter {
                repository: &self.creation_dt_index,
                prefixed_repository: &self.prefixed_creation_time_index,
                from: condition.created_dt_from,
                to: condition.created_dt_to,
            }) as Box<dyn SelectionFilter<IdType = UUID>>);
        }

        if condition.expiration_dt_from.is_some() || condition.expiration_dt_to.is_some() {
            filters.push(Box::new(ExpirationDtSelectionFilter {
                repository: &self.expiration_dt_index,
                prefixed_repository: &self.prefixed_expiration_time_index,
                from: condition.expiration_dt_from,
                to: condition.expiration_dt_to,
            }) as Box<dyn SelectionFilter<IdType = UUID>>);
        }

        if !condition.statuses.is_empty() {
            let includes_status = Box::new(OrSelectionFilter {
                filters: condition
                    .statuses
                    .iter()
                    .map(|status| {
                        Box::new(StatusSelectionFilter {
                            repository: &self.status_index,
                            status: status.to_owned(),
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(includes_status);
        }

        if !condition.operation_types.is_empty() {
            let includes_operation_type = Box::new(OrSelectionFilter {
                filters: condition
                    .operation_types
                    .iter()
                    .map(|operation_type| {
                        Box::new(OperationTypeSelectionFilter {
                            repository: &self.operation_type_index,
                            operation_type: operation_type.to_owned(),
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(includes_operation_type);
        }

        if !condition.approvers.is_empty() {
            let includes_approver = Box::new(OrSelectionFilter {
                filters: condition
                    .approvers
                    .iter()
                    .map(|approver_id| {
                        Box::new(ApproverSelectionFilter {
                            repository: &self.approver_index,
                            approver_id: *approver_id,
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(includes_approver);
        }

        if !condition.requesters.is_empty() {
            let includes_requester = Box::new(OrSelectionFilter {
                filters: condition
                    .requesters
                    .iter()
                    .map(|requester_id| {
                        Box::new(RequesterSelectionFilter {
                            repository: &self.requester_index,
                            requester_id: *requester_id,
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(includes_requester);
        }

        if filters.is_empty() {
            // If no filters are provided, return all
            filters.push(Box::new(CreationDtSelectionFilter {
                repository: &self.creation_dt_index,
                prefixed_repository: &self.prefixed_creation_time_index,
                from: None,
                to: None,
            }) as Box<dyn SelectionFilter<IdType = UUID>>);
        }

        // NotSelectionFilter doesn't select anything, only filters
        if !condition.excluded_ids.is_empty() {
            let excludes_ids = Box::new(NotSelectionFilter {
                input: Box::new(IdentitySelectionFilter {
                    ids: condition.excluded_ids.iter().cloned().collect(),
                }),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(excludes_ids);
        }

        if !condition.not_approvers.is_empty() {
            let excludes_approver = Box::new(NotSelectionFilter {
                input: Box::new(OrSelectionFilter {
                    filters: condition
                        .not_approvers
                        .iter()
                        .map(|approver_id| {
                            Box::new(ApproverSelectionFilter {
                                repository: &self.approver_index,
                                approver_id: *approver_id,
                            })
                                as Box<dyn SelectionFilter<IdType = UUID>>
                        })
                        .collect(),
                }),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(excludes_approver);
        }

        if !condition.not_requesters.is_empty() {
            let excludes_requester = Box::new(NotSelectionFilter {
                input: Box::new(OrSelectionFilter {
                    filters: condition
                        .not_requesters
                        .iter()
                        .map(|requester_id| {
                            Box::new(RequesterSelectionFilter {
                                repository: &self.requester_index,
                                requester_id: *requester_id,
                            })
                                as Box<dyn SelectionFilter<IdType = UUID>>
                        })
                        .collect(),
                }),
            }) as Box<dyn SelectionFilter<IdType = UUID>>;

            filters.push(excludes_requester);
        }

        filters
    }
}

#[derive(Debug, Clone)]
pub struct RequestWhereClause {
    pub created_dt_from: Option<Timestamp>,
    pub created_dt_to: Option<Timestamp>,
    pub expiration_dt_from: Option<Timestamp>,
    pub expiration_dt_to: Option<Timestamp>,
    pub operation_types: Vec<RequestOperationFilterType>,
    pub statuses: Vec<RequestStatusCode>,
    pub approvers: Vec<UUID>,
    pub not_approvers: Vec<UUID>,
    pub requesters: Vec<UUID>,
    pub not_requesters: Vec<UUID>,
    pub excluded_ids: Vec<UUID>,
}

#[derive(Debug, Clone)]
pub(crate) struct CreationDtSelectionFilter<'a> {
    repository: &'a RequestCreationTimeIndexRepository,
    prefixed_repository: &'a RequestKeyCreationTimeIndexRepository,
    from: Option<Timestamp>,
    to: Option<Timestamp>,
}

impl<'a> SelectionFilter<'a> for CreationDtSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.prefixed_repository
            .exists_by_criteria(RequestKeyCreationTimeIndexCriteria {
                request_id: *id,
                from_dt: self.from,
                to_dt: self.to,
            })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(RequestCreationTimeIndexCriteria {
                from_dt: self.from,
                to_dt: self.to,
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExpirationDtSelectionFilter<'a> {
    repository: &'a RequestExpirationTimeIndexRepository,
    prefixed_repository: &'a RequestKeyExpirationTimeIndexRepository,
    from: Option<Timestamp>,
    to: Option<Timestamp>,
}

impl<'a> SelectionFilter<'a> for ExpirationDtSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.prefixed_repository
            .exists_by_criteria(RequestKeyExpirationTimeIndexCriteria {
                request_id: *id,
                from_dt: self.from,
                to_dt: self.to,
            })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(RequestExpirationTimeIndexCriteria {
                from_dt: self.from,
                to_dt: self.to,
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OperationTypeSelectionFilter<'a> {
    repository: &'a RequestOperationTypeIndexRepository,
    operation_type: RequestOperationFilterType,
}

impl<'a> SelectionFilter<'a> for OperationTypeSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RequestOperationTypeIndex {
            operation_type: self.operation_type.clone(),
            request_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(RequestOperationTypeIndexCriteria {
                operation_type: self.operation_type.clone(),
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ApproverSelectionFilter<'a> {
    repository: &'a RequestApproverIndexRepository,
    approver_id: UUID,
}

impl<'a> SelectionFilter<'a> for ApproverSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RequestApproverIndex {
            approver_id: self.approver_id,
            request_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(RequestApproverIndexCriteria {
                approver_id: self.approver_id,
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RequesterSelectionFilter<'a> {
    repository: &'a RequestRequesterIndexRepository,
    requester_id: UUID,
}

impl<'a> SelectionFilter<'a> for RequesterSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RequestRequesterIndex {
            requester_id: self.requester_id,
            request_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(RequestRequesterIndexCriteria {
                requester_id: self.requester_id,
            })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StatusSelectionFilter<'a> {
    repository: &'a RequestStatusIndexRepository,
    status: RequestStatusCode,
}

impl<'a> SelectionFilter<'a> for StatusSelectionFilter<'a> {
    type IdType = UUID;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RequestStatusIndex {
            status: self.status.to_owned(),
            request_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository
            .find_by_criteria(RequestStatusIndexCriteria {
                status: self.status.to_owned(),
            })
    }
}

#[derive(Debug, Clone)]
enum TimestampType {
    Creation,
    Expiration,
    Modification,
}

#[derive(Debug, Clone)]
struct TimestampSortingStrategy<'a> {
    index: &'a RequestSortIndexRepository,
    timestamp_type: TimestampType,
    direction: Option<SortDirection>,
}

impl<'a> SortingStrategy<'a> for TimestampSortingStrategy<'a> {
    type IdType = UUID;

    fn sort(&self, ids: &mut [Self::IdType]) {
        let direction = self.direction.unwrap_or(SortDirection::Ascending);
        let mut id_with_timestamps: Vec<(Timestamp, Self::IdType)> = ids
            .iter()
            .map(|id| {
                let key = RequestSortIndexKey { request_id: *id };
                let timestamp = self
                    .index
                    .get(&key)
                    .map(|index| match self.timestamp_type {
                        TimestampType::Creation => index.creation_timestamp,
                        TimestampType::Expiration => index.expiration_timestamp,
                        TimestampType::Modification => index.modification_timestamp,
                    })
                    .unwrap_or_default();
                (timestamp, *id)
            })
            .collect();

        id_with_timestamps.sort_by(|a, b| {
            {
                let ord = a.0.cmp(&b.0); // Compare timestamps
                match direction {
                    SortDirection::Ascending => ord,
                    SortDirection::Descending => ord.reverse(),
                }
            }
            .then_with(|| a.1.cmp(&b.1)) // Compare request IDs if timestamps are equal
        });

        let sorted_ids: Vec<UUID> = id_with_timestamps.into_iter().map(|(_, id)| id).collect();
        ids.copy_from_slice(&sorted_ids);
    }
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

        let last_six = repository.find_by_expiration_dt_and_status(
            Some(45),
            None,
            RequestStatusCode::Created.to_string(),
        );

        let middle_eleven = repository.find_by_expiration_dt_and_status(
            Some(30),
            Some(40),
            RequestStatusCode::Created.to_string(),
        );

        let first_three = repository.find_by_expiration_dt_and_status(
            None,
            Some(2),
            RequestStatusCode::Created.to_string(),
        );

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

        let requests = repository.find_by_expiration_dt_and_status(
            Some(20),
            None,
            request.status.to_type().to_string(),
        );

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
            operation_types: vec![RequestOperationFilterType::AddUserGroup],
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
