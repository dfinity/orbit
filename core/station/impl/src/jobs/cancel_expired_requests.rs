use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
};

use crate::{
    core::ic_cdk::next_time,
    models::{RequestId, RequestStatus, RequestStatusCode},
    repositories::RequestRepository,
};
use async_trait::async_trait;
use ic_cdk_timers::TimerId;
use orbit_essentials::{cdk::api::time, repository::Repository};

use super::{
    scheduler::{ScheduleStrategy, Scheduler, TimerAction},
    ScheduledJob,
};

thread_local! {
    static SCHEDULE_REQUESTS_MAP: RefCell<BTreeMap<u64, HashSet<RequestId>>> = Default::default();
    static REQUEST_SCHEDULE_MAP: RefCell<HashMap<RequestId, u64>> = Default::default();
    static SCHEDULE_TIMERID_MAP: RefCell<HashMap<u64, TimerId>> = Default::default();
    static IS_RUNNING : RefCell<bool> = const { RefCell::new(false) };
}

#[derive(Debug, Default)]
pub struct Job {
    request_repository: RequestRepository,
}

#[async_trait]
impl ScheduledJob for Job {
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

#[derive(Clone)]
pub struct ExpiredRequestScheduleStrategy {
    request_id: RequestId,
}

impl ScheduleStrategy for ExpiredRequestScheduleStrategy {
    const TOLERANCE_SEC: u64 = 5;

    fn add_schedule(&self, at_ns: u64) -> TimerAction {
        // remove the request's expiration timer if it existed
        self.remove_schedule(at_ns);

        let with_tolerance = at_ns + Self::TOLERANCE_SEC * 1_000_000_000;
        let timer_action: TimerAction = SCHEDULE_REQUESTS_MAP.with(|timers| {
            let mut timers = timers.borrow_mut();

            if let Some(existing_timer) = timers
                .range(at_ns..with_tolerance)
                .next()
                .map(|(timer_at, _)| *timer_at)
            {
                timers
                    .entry(existing_timer)
                    .or_default()
                    .insert(self.request_id);

                TimerAction::UsedExisting(existing_timer)
            } else {
                timers.entry(at_ns).or_default().insert(self.request_id);

                TimerAction::AddedNew(at_ns)
            }
        });

        match timer_action {
            TimerAction::AddedNew(at) | TimerAction::UsedExisting(at) => {
                REQUEST_SCHEDULE_MAP.with(|request_map| {
                    request_map.borrow_mut().insert(self.request_id, at);
                });
            }
        }

        timer_action
    }

    fn remove_schedule(&self, _at_ns: u64) -> Option<TimerId> {
        REQUEST_SCHEDULE_MAP.with(|request_map| {
            let mut request_map = request_map.borrow_mut();

            if let Some(at_ns) = request_map.remove(&self.request_id) {
                SCHEDULE_REQUESTS_MAP.with(|schedules| {
                    if let Some(request_ids) = schedules.borrow_mut().get_mut(&at_ns) {
                        request_ids.remove(&self.request_id);
                        if request_ids.is_empty() {
                            schedules.borrow_mut().remove(&at_ns);
                        }
                    }
                });

                SCHEDULE_TIMERID_MAP.with(|timer_map| {
                    if let Some(timer_id) = timer_map.borrow_mut().remove(&at_ns) {
                        return Some(timer_id);
                    }
                    None
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

    fn save_timer_id(&self, timer_id: TimerId, at_ns: u64) {
        SCHEDULE_TIMERID_MAP.with(|timer_map| {
            timer_map.borrow_mut().insert(at_ns, timer_id);
        });
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
        REQUEST_SCHEDULE_MAP.with(|request_map| request_map.borrow().get(request_id).copied())
    {
        let strategy = ExpiredRequestScheduleStrategy {
            request_id: request_id.to_owned(),
        };

        Scheduler::cancel_scheduled_timer::<ExpiredRequestScheduleStrategy>(strategy, at_ns);
    }
}
