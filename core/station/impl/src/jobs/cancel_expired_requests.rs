use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
};

use crate::{
    core::ic_cdk::next_time,
    models::{RequestId, RequestStatus, RequestStatusCode},
    repositories::RequestRepository,
};
use async_trait::async_trait;
use orbit_essentials::{cdk::api::time, repository::Repository};

use super::{
    scheduler::{ScheduleStrategy, Scheduler},
    ScheduledJob,
};

thread_local! {
    static TIMERS: RefCell<BTreeMap<u64, HashSet<RequestId>>> = const { RefCell::new(BTreeMap::new()) };
    static REQUEST_MAP: RefCell<BTreeMap<RequestId, u64>> = const { RefCell::new(BTreeMap::new()) };
    static IS_RUNNING : RefCell<bool> = const { RefCell::new(false) };
}

#[derive(Debug, Default)]
pub struct Job {
    request_repository: RequestRepository,
}

#[async_trait]
impl ScheduledJob for Job {
    const INTERVAL_SECS: u64 = 60;
    const ALLOW_CONCURRENT_EXECUTION: bool = false;

    async fn run() -> bool {
        Self::default().cancel_requests().await
    }
}

/// This job is responsible for canceling the requests that have expired while not approved/rejected.
impl Job {
    /// Cancel the requests that have expired while still pending.
    async fn cancel_requests(&self) -> bool {
        let current_time = next_time();
        let mut requests = self.request_repository.find_by_expiration_dt_and_status(
            None,
            Some(current_time),
            RequestStatusCode::Created.to_string(),
        );

        for request in requests.iter_mut() {
            request.status = RequestStatus::Cancelled {
                reason: Some("The request has expired".to_string()),
            };
            request.last_modification_timestamp = next_time();
            self.request_repository
                .insert(request.to_key(), request.to_owned());
        }

        true
    }
}

pub struct ExpiredRequestScheduleStrategy {
    request_id: RequestId,
}

impl ScheduleStrategy for ExpiredRequestScheduleStrategy {
    const TOLERANCE_SEC: u64 = 5;

    fn add_timer(&self, at_ns: u64) -> Option<u64> {
        // remove the request's expiration timer if it existed
        self.remove_timer(at_ns);

        // add the timer
        let with_tolerance = at_ns + Self::TOLERANCE_SEC * 1_000_000_000;
        let timer_at = TIMERS.with(|timers| {
            let mut timers = timers.borrow_mut();

            // see if there is a timer around the same time
            let existing_timer = timers
                .range(at_ns..with_tolerance)
                .next()
                .map(|(timer_at, _)| *timer_at)
                .unwrap_or(at_ns);

            // if there is a timer around the same time, add the request to the existing timer
            timers
                .entry(existing_timer)
                .or_default()
                .insert(self.request_id);

            existing_timer
        });

        REQUEST_MAP.with(|request_map| {
            request_map.borrow_mut().insert(self.request_id, timer_at);
        });

        None
    }

    fn remove_timer(&self, _at_ns: u64) -> Option<u64> {
        REQUEST_MAP.with(|request_map| {
            let mut request_map = request_map.borrow_mut();
            if let Some(at_ns) = request_map.remove(&self.request_id) {
                TIMERS.with(|timers| {
                    if let Some(request_ids) = timers.borrow_mut().get_mut(&at_ns) {
                        request_ids.remove(&self.request_id);
                        if request_ids.is_empty() {
                            timers.borrow_mut().remove(&at_ns);

                            Some(at_ns)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
    }

    fn is_running(&self) -> bool {
        IS_RUNNING.with(|is_running| *is_running.borrow())
    }

    fn set_running(&self, running: bool) {
        IS_RUNNING.with(|is_running| *is_running.borrow_mut() = running);
    }
}

pub fn schedule_expiration(request_id: &RequestId, at_ns: u64) {
    let strategy = ExpiredRequestScheduleStrategy {
        request_id: request_id.to_owned(),
    };

    Scheduler::schedule::<ExpiredRequestScheduleStrategy, Job>(
        strategy,
        at_ns.saturating_sub(time()),
    );
}

pub fn cancel_scheduled_expiration(request_id: &RequestId) {
    if let Some(at_ns) =
        REQUEST_MAP.with(|request_map| request_map.borrow().get(request_id).copied())
    {
        let strategy = ExpiredRequestScheduleStrategy {
            request_id: request_id.to_owned(),
        };

        Scheduler::cancel_scheduled_timer::<ExpiredRequestScheduleStrategy>(strategy, at_ns);
    }
}
