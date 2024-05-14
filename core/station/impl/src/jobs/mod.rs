//! This module contains all the jobs that run in the background to perform tasks within the canister.
//!
//! The jobs are registered in the `register_jobs` function and are executed based on the defined timer intervals.
use std::cell::RefCell;
use std::collections::HashMap;

use crate::core::ic_cdk::next_time;
use crate::core::ic_timers::TimerId;
use crate::models::{RequestExecutionPlan, RequestStatusCode};
use crate::{
    core::observer::Observer,
    models::{Request, RequestStatus, Transfer, TransferStatus},
    repositories::REQUEST_REPOSITORY,
};
use async_trait::async_trait;
use orbit_essentials::repository::Repository;

mod cancel_expired_requests;
mod execute_created_transfers;
mod execute_scheduled_requests;
mod scheduler;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum JobType {
    CancelExpiredRequests,
    ExecuteScheduledRequests,
    ExecuteCreatedTransfers,
}

#[async_trait]
pub trait ScheduledJob: Send + Sync {
    const JOB_TYPE: JobType;
    const JOB_TOLERANCE_NS: u64 = 1_000_000_000;

    /// Executes the job. Returns `true` if the job was completed or `false` there is more work to be done.
    async fn run() -> bool;
}

type TimeJobMap = HashMap<u64, (TimerId, usize)>;

thread_local! {
    static TIME_JOB_MAPS: RefCell<HashMap<JobType,TimeJobMap>> = Default::default();
    static IS_RUNNINGS : RefCell<HashMap<JobType, bool>> = Default::default();
}

struct JobStateDatabase;

impl JobStateDatabase {
    #[cfg(test)]
    fn get_time_job_maps() -> HashMap<JobType, TimeJobMap> {
        TIME_JOB_MAPS.with(|time_job_maps| time_job_maps.borrow().clone())
    }

    fn is_running(job_type: JobType) -> bool {
        IS_RUNNINGS.with(|is_runnings| {
            let is_runnings = is_runnings.borrow();
            *is_runnings.get(&job_type).unwrap_or(&false)
        })
    }

    fn set_running(job_type: JobType, running: bool) {
        IS_RUNNINGS.with(|is_runnings| {
            is_runnings.borrow_mut().insert(job_type, running);
        });
    }

    fn check_existing_timer(job_type: JobType, at_ns: u64) -> Option<TimerId> {
        TIME_JOB_MAPS.with(|time_job_maps| {
            let time_job_maps = time_job_maps.borrow();

            if let Some(job_map) = time_job_maps.get(&job_type) {
                job_map.get(&at_ns).map(|(timer_id, _)| *timer_id)
            } else {
                None
            }
        })
    }

    fn add_scheduled_task(job_type: JobType, at_ns: u64, timer_id: TimerId) {
        TIME_JOB_MAPS.with(|time_job_maps| {
            let mut time_job_maps = time_job_maps.borrow_mut();

            let job_map = time_job_maps.entry(job_type).or_default();
            job_map
                .entry(at_ns)
                .and_modify(|(_, count)| {
                    *count += 1;
                })
                .or_insert((timer_id, 1));
        });
    }

    fn remove_scheduled_task(job_type: JobType, at_ns: u64) -> Option<TimerId> {
        TIME_JOB_MAPS.with(|time_job_maps| {
            let mut time_job_maps = time_job_maps.borrow_mut();

            if let Some(job_map) = time_job_maps.get_mut(&job_type) {
                if let Some((timer_id, count)) = job_map.get_mut(&at_ns) {
                    *count -= 1;
                    if *count == 0 {
                        let timer_id = *timer_id;
                        job_map.remove(&at_ns);
                        return Some(timer_id);
                    } else {
                        return None;
                    }
                }
            }

            None
        })
    }
}

struct TimerResourceGuard {
    job_type: JobType,
    at_ns: u64,
}

impl TimerResourceGuard {
    fn new(job_type: JobType, at_ns: u64) -> Self {
        Self { job_type, at_ns }
    }
}

impl Drop for TimerResourceGuard {
    fn drop(&mut self) {
        JobStateDatabase::remove_scheduled_task(self.job_type, self.at_ns);
        JobStateDatabase::set_running(self.job_type, false);
    }
}

fn to_coarse_time(at_ns: u64, step_ns: u64) -> u64 {
    let remainder = at_ns % step_ns;

    if remainder == 0 {
        at_ns
    } else {
        at_ns - remainder + step_ns
    }
}

pub fn jobs_observe_insert_request(observer: &mut Observer<(Request, Option<Request>)>) {
    observer.add_listener(Box::new(|(request, prev)| match &request.status {
        RequestStatus::Created => {
            if prev.is_some() {
                return;
            }

            cancel_expired_requests::schedule_expiration(request.expiration_dt);
        }
        RequestStatus::Approved => {
            if let Some(Request {
                status: RequestStatus::Created,
                ..
            }) = prev
            {
                cancel_expired_requests::cancel_scheduled_expiration(request.expiration_dt);

                let request_processing_time = next_time();
                let scheduled_at = match &request.execution_plan {
                    RequestExecutionPlan::Immediate => request_processing_time,
                    RequestExecutionPlan::Scheduled { execution_time } => *execution_time,
                };

                let mut request = request.clone();

                request.status = RequestStatus::Scheduled { scheduled_at };
                request.last_modification_timestamp = request_processing_time;

                REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());

                execute_scheduled_requests::schedule_request_execution(scheduled_at);
            }
        }
        RequestStatus::Rejected | RequestStatus::Cancelled { .. } => {
            if let Some(Request {
                status: RequestStatus::Created,
                ..
            }) = prev
            {
                cancel_expired_requests::cancel_scheduled_expiration(request.expiration_dt);
            }
        }
        RequestStatus::Scheduled { .. } => {
            // do nothing, these will exectuted by the timers already set when the request was approved
        }
        RequestStatus::Processing { .. }
        | RequestStatus::Completed { .. }
        | RequestStatus::Failed { .. } => {
            // do nothing
        }
    }));
}

pub fn jobs_observe_remove_request(observer: &mut Observer<Request>) {
    observer.add_listener(Box::new(|prev| {
        if let Request {
            status: RequestStatus::Created,
            expiration_dt,
            ..
        } = prev
        {
            cancel_expired_requests::cancel_scheduled_expiration(*expiration_dt);
        }
    }));
}

pub fn jobs_observe_insert_transfer(observer: &mut Observer<(Transfer, Option<Transfer>)>) {
    observer.add_listener(Box::new(|(transfer, prev)| {
        if let (
            Transfer {
                status: TransferStatus::Created,
                ..
            },
            None,
        ) = (transfer, prev)
        {
            execute_created_transfers::schedule_process_transfers(next_time());
        }
    }));
}

pub fn initialize_job_timers() {
    // start the expiration timer for each request that is in Created state
    for request in REQUEST_REPOSITORY.find_by_status(RequestStatusCode::Created, None, None) {
        cancel_expired_requests::schedule_expiration(request.expiration_dt);
    }
    // start the execution timer for each request that is in Scheduled state
    for request in REQUEST_REPOSITORY.find_by_status(RequestStatusCode::Scheduled, None, None) {
        if let RequestStatus::Scheduled { scheduled_at } = request.status {
            execute_scheduled_requests::schedule_request_execution(scheduled_at);
        }
    }

    // start the execution timer for Transfers
    execute_created_transfers::schedule_process_transfers(next_time());
}
