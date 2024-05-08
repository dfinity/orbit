use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use ic_cdk_timers::TimerId;
use orbit_essentials::cdk::{api::time, spawn};

use crate::models::RequestId;

use super::{
    cancel_expired_requests, execute_created_transfers, execute_scheduled_requests, ScheduledJob,
};

thread_local! {
    static CANCEL_EXPIRED_REQUESTS_TIMERS: RefCell<EntityTimerMap<RequestId>> = RefCell::new(EntityTimerMap::default());
    static EXECUTE_SCHEDULED_REQUESTS_TIMERS: RefCell<HashSet<AbsoluteJobTimeNs>> = RefCell::new(HashSet::new());
    static EXECUTE_CREATED_TRANSFERS_TIMERS: RefCell<HashSet<AbsoluteJobTimeNs>> = RefCell::new(HashSet::new());

    static CANCEL_EXPIRED_REQUESTS_RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static EXECUTE_SCHEDULED_REQUESTS_RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static EXECUTE_CREATED_TRANSFERS_RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

pub type AbsoluteJobTimeNs = u64;
pub type RelativeJobTimeNs = u64;

struct ScheduledEntities<T: Eq + PartialEq + Hash> {
    timer_id: TimerId,
    entity_ids: HashSet<T>,
}

#[derive(Default)]
struct EntityTimerMap<T: Eq + PartialEq + Hash> {
    time_to_schedules: HashMap<AbsoluteJobTimeNs, ScheduledEntities<T>>,
    entity_id_to_time: HashMap<T, AbsoluteJobTimeNs>,
}

impl<T: Eq + PartialEq + Hash> EntityTimerMap<T> {
    fn insert(&mut self, coarse_time: AbsoluteJobTimeNs, timer_id: TimerId, entity_id: T) {
        self.time_to_schedules
            .entry(coarse_time)
            .or_insert_with(|| ScheduledEntities {
                entity_ids: HashSet::new(),
                timer_id,
            })
            .entity_ids
            .insert(entity_id);
    }

    fn get(&self, time: AbsoluteJobTimeNs) -> Option<&TimerId> {
        self.time_to_schedules
            .get(&time)
            .map(|scheduled_entities| &scheduled_entities.timer_id)
    }

    fn remove_by_time(&mut self, time: AbsoluteJobTimeNs) {
        if let Some(scheduled_entities) = self.time_to_schedules.remove(&time) {
            for entity_id in scheduled_entities.entity_ids {
                self.entity_id_to_time.remove(&entity_id);
            }
        }
    }

    fn remove_by_entity(&mut self, entity_id: &T) {
        if let Some(time) = self.entity_id_to_time.remove(entity_id) {
            if let Some(scheduled_entities) = self.time_to_schedules.get_mut(&time) {
                scheduled_entities.entity_ids.remove(entity_id);

                if scheduled_entities.entity_ids.is_empty() {
                    self.time_to_schedules.remove(&time);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Scheduler;

pub enum JobType {
    CancelExpiredRequests { request_id: RequestId },
    ExecuteScheduledRequests,
    ExecuteCreatedTransfers,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    fn coarse_time_to_timeout(time: AbsoluteJobTimeNs, tolerance: Duration) -> AbsoluteJobTimeNs {
        time - time % tolerance.as_nanos() as AbsoluteJobTimeNs
    }

    fn absolute_time_to_relative(abs_time: AbsoluteJobTimeNs) -> RelativeJobTimeNs {
        let now = time();

        if abs_time < now {
            return 0;
        }

        abs_time - now
    }

    pub fn cancel_job(&mut self, job_type: JobType) {
        match job_type {
            JobType::CancelExpiredRequests { request_id } => {
                CANCEL_EXPIRED_REQUESTS_TIMERS
                    .with(|timers| timers.borrow_mut().remove_by_entity(&request_id));
            }
            JobType::ExecuteScheduledRequests | JobType::ExecuteCreatedTransfers => {
                // can't be cancelled
            }
        }
    }

    pub fn schedule_now(&mut self, job_type: JobType) {
        self.schedule_at(job_type, time());
    }

    pub fn schedule_at(&mut self, job_type: JobType, abs_time_ns: AbsoluteJobTimeNs) {
        match job_type {
            JobType::CancelExpiredRequests { request_id } => {
                // coarse time

                let coarse_time = Self::coarse_time_to_timeout(
                    abs_time_ns,
                    cancel_expired_requests::Job::TIME_TOLERANCE,
                );

                // check if the timer is already scheduled

                if CANCEL_EXPIRED_REQUESTS_TIMERS
                    .with(|timers| timers.borrow().get(coarse_time).is_some())
                {
                    return;
                }

                // schedule the timer

                let relative_job_time = Self::absolute_time_to_relative(coarse_time);
                let timer_id =
                    ic_cdk_timers::set_timer(Duration::from_nanos(relative_job_time), move || {
                        spawn(async move {
                            cancel_expired_requests::Job::run().await;

                            CANCEL_EXPIRED_REQUESTS_TIMERS
                                .with(|timers| timers.borrow_mut().remove_by_time(coarse_time))
                        });
                    });

                CANCEL_EXPIRED_REQUESTS_TIMERS.with(|timers| {
                    timers
                        .borrow_mut()
                        .insert(coarse_time, timer_id, request_id)
                });
            }
            JobType::ExecuteScheduledRequests => {
                let coarse_time = Self::coarse_time_to_timeout(
                    abs_time_ns,
                    execute_scheduled_requests::Job::TIME_TOLERANCE,
                );

                if EXECUTE_SCHEDULED_REQUESTS_TIMERS
                    .with(|timers| timers.borrow().get(&coarse_time).is_some())
                {
                    return;
                }

                let relative_job_time = Self::absolute_time_to_relative(coarse_time);
                ic_cdk_timers::set_timer(Duration::from_nanos(relative_job_time), || {
                    spawn(async move {
                        execute_scheduled_requests::Job::run().await;
                    });
                });

                EXECUTE_SCHEDULED_REQUESTS_TIMERS
                    .with(|timers| timers.borrow_mut().insert(coarse_time));
            }
            JobType::ExecuteCreatedTransfers => {
                let coarse_time = Self::coarse_time_to_timeout(
                    abs_time_ns,
                    execute_created_transfers::Job::TIME_TOLERANCE,
                );

                if EXECUTE_CREATED_TRANSFERS_TIMERS
                    .with(|timers| timers.borrow().get(&coarse_time).is_some())
                {
                    return;
                }

                let relative_job_time = Self::absolute_time_to_relative(coarse_time);
                ic_cdk_timers::set_timer(Duration::from_nanos(relative_job_time), || {
                    spawn(async move {
                        execute_created_transfers::Job::run().await;
                    });
                });

                EXECUTE_CREATED_TRANSFERS_TIMERS
                    .with(|timers| timers.borrow_mut().insert(coarse_time));
            }
        }
    }

    // async fn run_job<Job: ScheduledJob>(&mut self, job: Job) {}
}
