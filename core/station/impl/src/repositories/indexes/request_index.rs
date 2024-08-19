use crate::{
    core::{
        utils::{MAX_UUID, MIN_UUID},
        with_memory_manager, Memory, REQUEST_INDEX_MEMORY_ID,
    },
    models::{
        indexes::request_index::{RequestIndexFields, RequestIndexKey, RequestIndexKeyKind},
        RequestId, RequestStatusCode,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::{Repository, StableDb};
use std::{cell::RefCell, collections::HashMap};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestIndexKey, RequestIndexFields, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that holds indexed fields of the requests and the lookup keys to find them.
#[derive(Default, Debug)]
pub struct RequestIndexRepository {}

impl StableDb<RequestIndexKey, RequestIndexFields, VirtualMemory<Memory>>
    for RequestIndexRepository
{
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(
            &mut StableBTreeMap<RequestIndexKey, RequestIndexFields, VirtualMemory<Memory>>,
        ) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl Repository<RequestIndexKey, RequestIndexFields, VirtualMemory<Memory>>
    for RequestIndexRepository
{
}

impl RequestIndexRepository {
    /// Returns all the requests that are created between the given timestamps.
    pub fn find_by_created_at_between(
        &self,
        start: u64,
        end: u64,
        take_limit: Option<usize>,
    ) -> HashMap<RequestId, RequestIndexFields> {
        self.find_by_criteria(
            RequestIndexKeyKind::CreatedAt(start),
            RequestIndexKeyKind::CreatedAt(end),
            take_limit,
        )
    }

    /// Returns all the requests that are scheduled between the given timestamps.
    pub fn find_by_scheduled_at_between(
        &self,
        start: u64,
        end: u64,
        take_limit: Option<usize>,
    ) -> HashMap<RequestId, RequestIndexFields> {
        self.find_by_criteria(
            RequestIndexKeyKind::ScheduledAt(start),
            RequestIndexKeyKind::ScheduledAt(end),
            take_limit,
        )
    }

    /// Returns all the requests that are in the given status.
    pub fn find_by_status(
        &self,
        status: RequestStatusCode,
        take_limit: Option<usize>,
    ) -> HashMap<RequestId, RequestIndexFields> {
        self.find_by_criteria(
            RequestIndexKeyKind::Status(status.clone()),
            RequestIndexKeyKind::Status(status),
            take_limit,
        )
    }

    /// Returns all the entries that are between the given keys.
    fn find_by_criteria(
        &self,
        start_key: RequestIndexKeyKind,
        end_key: RequestIndexKeyKind,
        take_limit: Option<usize>,
    ) -> HashMap<RequestId, RequestIndexFields> {
        DB.with(|m| {
            m.borrow()
                .range(
                    RequestIndexKey {
                        kind: start_key,
                        request_id: MIN_UUID,
                    }..=RequestIndexKey {
                        kind: end_key,
                        request_id: MAX_UUID,
                    },
                )
                .take(take_limit.unwrap_or(usize::MAX))
                .map(
                    |(
                        RequestIndexKey {
                            kind: _,
                            request_id,
                        },
                        value,
                    )| (request_id, value.clone()),
                )
                .collect()
        })
    }
}
