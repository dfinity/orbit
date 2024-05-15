//! This module contains all the jobs that run in the background to perform tasks within the canister.
//!
//! The jobs are registered in the `register_jobs` function and are executed based on the defined timer intervals.
use std::cell::RefCell;
use std::collections::HashMap;

use crate::core::ic_cdk::next_time;
use crate::core::ic_timers::TimerId;
use crate::models::{RequestExecutionPlan, RequestStatusCode};
use crate::repositories::TRANSFER_REPOSITORY;
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
    /// Checks if the job is currently running.
    fn is_running(job_type: JobType) -> bool {
        IS_RUNNINGS.with(|is_runnings| {
            let is_runnings = is_runnings.borrow();
            *is_runnings.get(&job_type).unwrap_or(&false)
        })
    }

    /// Sets the running state of the job.
    fn set_running(job_type: JobType, running: bool) {
        IS_RUNNINGS.with(|is_runnings| {
            is_runnings.borrow_mut().insert(job_type, running);
        });
    }

    /// Checks if a timer already exists for the given job type and time.
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

    /// Adds a new scheduled task to the database, or increments the reference count if the task already exists.
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

    /// After a scheduled task ends (either successfully or not), this will clean up the database.
    fn finalize_scheduled_task(job_type: JobType, at_ns: u64) {
        TIME_JOB_MAPS.with(|time_job_maps| {
            let mut time_job_maps = time_job_maps.borrow_mut();

            if let Some(job_map) = time_job_maps.get_mut(&job_type) {
                job_map.remove(&at_ns);

                if job_map.is_empty() {
                    time_job_maps.remove(&job_type);
                }
            }
        })
    }

    /// Decrements the reference count of the scheduled task. If the reference count reaches 0, the task is cleaned up.
    fn remove_scheduled_task(job_type: JobType, at_ns: u64) -> Option<TimerId> {
        TIME_JOB_MAPS.with(|time_job_maps| {
            let mut time_job_maps = time_job_maps.borrow_mut();

            if let Some(job_map) = time_job_maps.get_mut(&job_type) {
                if let Some((timer_id, count)) = job_map.get_mut(&at_ns) {
                    *count -= 1;
                    if *count == 0 {
                        let timer_id = *timer_id;
                        job_map.remove(&at_ns);

                        if job_map.is_empty() {
                            time_job_maps.remove(&job_type);
                        }

                        return Some(timer_id);
                    } else {
                        return None;
                    }
                }
            }

            None
        })
    }

    /// Returns a copy of the current state of the time job maps for testing purposes.
    #[cfg(test)]
    fn get_time_job_maps() -> HashMap<JobType, TimeJobMap> {
        TIME_JOB_MAPS.with(|time_job_maps| time_job_maps.borrow().clone())
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
        JobStateDatabase::finalize_scheduled_task(self.job_type, self.at_ns);
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

fn schedule_request_for_execution(request: &Request) -> u64 {
    let request_processing_time = next_time();
    let scheduled_at = match &request.execution_plan {
        RequestExecutionPlan::Immediate => request_processing_time,
        RequestExecutionPlan::Scheduled { execution_time } => *execution_time,
    };

    let mut request = request.clone();

    request.status = RequestStatus::Scheduled { scheduled_at };
    request.last_modification_timestamp = request_processing_time;

    REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());

    scheduled_at
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

                let scheduled_at = schedule_request_for_execution(request);

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

    // schedule requests that are already approved, but not yet scheduled
    for request in REQUEST_REPOSITORY.find_by_status(RequestStatusCode::Approved, None, None) {
        schedule_request_for_execution(&request);
    }

    // start the execution timer for each request that is in Scheduled state
    for request in REQUEST_REPOSITORY.find_by_status(RequestStatusCode::Scheduled, None, None) {
        if let RequestStatus::Scheduled { scheduled_at } = request.status {
            execute_scheduled_requests::schedule_request_execution(scheduled_at);
        }
    }

    if !TRANSFER_REPOSITORY
        .find_by_status(TransferStatus::Created.to_string(), None, None)
        .is_empty()
    {
        // kick off execution timer for Transfers, once is enough
        execute_created_transfers::schedule_process_transfers(next_time());
    }
}

#[cfg(test)]
mod test {
    use std::time::{Duration, SystemTime};

    use crate::core::ic_cdk::api::{set_mock_ic_time, time};
    use crate::jobs::scheduler::Scheduler;
    use crate::jobs::{execute_created_transfers, execute_scheduled_requests};
    use crate::models::account_test_utils::mock_account;
    use crate::models::transfer_test_utils::mock_transfer;
    use crate::models::{Account, RequestStatus};
    use crate::repositories::{
        RequestRepository, TransferRepository, ACCOUNT_REPOSITORY, TRANSFER_REPOSITORY,
    };
    use crate::{
        jobs::{cancel_expired_requests, to_coarse_time, JobStateDatabase, ScheduledJob},
        models::{request_test_utils::mock_request, Request},
        repositories::REQUEST_REPOSITORY,
    };
    use orbit_essentials::repository::Repository;

    #[tokio::test]
    async fn test_request_insertion() {
        set_mock_ic_time(SystemTime::UNIX_EPOCH);

        assert!(JobStateDatabase::get_time_job_maps()
            .get(&cancel_expired_requests::Job::JOB_TYPE)
            .is_none());

        let expiration = time() + Duration::from_secs(30 * 24 * 60 * 60).as_nanos() as u64;
        let expiration_coarse =
            to_coarse_time(expiration, cancel_expired_requests::Job::JOB_TOLERANCE_NS);

        // insert a new request, expiration timer should be set
        let request_1 = Request {
            status: crate::models::RequestStatus::Created,
            expiration_dt: expiration,
            ..mock_request()
        };
        REQUEST_REPOSITORY.insert(request_1.to_key(), request_1);

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&cancel_expired_requests::Job::JOB_TYPE)
                .expect("Job not scheduled at all")
                .get(&expiration_coarse)
                .expect("Job not scheduled at this time")
                .1,
            1
        );

        // insert another request with same expiration, expiration timer should be set
        let mut request_2 = Request {
            status: crate::models::RequestStatus::Created,
            expiration_dt: expiration,
            ..mock_request()
        };
        REQUEST_REPOSITORY.insert(request_2.to_key(), request_2.clone());

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&cancel_expired_requests::Job::JOB_TYPE)
                .expect("Job not scheduled at all")
                .get(&expiration_coarse)
                .expect("Job not scheduled at this time")
                .1,
            2
        );

        // 2nd request is approved, the timer should be removed
        request_2.status = crate::models::RequestStatus::Approved;
        REQUEST_REPOSITORY.insert(request_2.to_key(), request_2.clone());

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&cancel_expired_requests::Job::JOB_TYPE)
                .expect("Job not scheduled at all")
                .get(&expiration_coarse)
                .expect("Job not scheduled at this time")
                .1,
            1
        );

        let request_2 = REQUEST_REPOSITORY
            .get(&request_2.to_key())
            .expect("Request not found");

        // request 2 should be scheduled now
        let RequestStatus::Scheduled { scheduled_at } = request_2.status else {
            panic!("Request not scheduled");
        };

        // scheduled request should have a timer set
        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&execute_scheduled_requests::Job::JOB_TYPE)
                .expect("Job not scheduled at all")
                .get(&to_coarse_time(
                    scheduled_at,
                    execute_scheduled_requests::Job::JOB_TOLERANCE_NS
                ))
                .expect("Job not scheduled at this time")
                .1,
            1
        );

        // scheduled request is executed, timer should be removed
        Scheduler::run_scheduled::<execute_scheduled_requests::Job>(scheduled_at).await;

        assert!(JobStateDatabase::get_time_job_maps()
            .get(&execute_scheduled_requests::Job::JOB_TYPE)
            .expect("Job not scheduled at all")
            .get(&scheduled_at)
            .is_none(),);

        // first job expires, cleaning up the timer
        Scheduler::run_scheduled::<cancel_expired_requests::Job>(expiration_coarse).await;

        // all timers should be removed
        assert!(JobStateDatabase::get_time_job_maps()
            .get(&cancel_expired_requests::Job::JOB_TYPE)
            .is_none());
    }

    #[tokio::test]
    async fn test_request_removal() {
        set_mock_ic_time(SystemTime::UNIX_EPOCH);

        assert!(JobStateDatabase::get_time_job_maps()
            .get(&cancel_expired_requests::Job::JOB_TYPE)
            .is_none());

        let expiration = time() + Duration::from_secs(30 * 24 * 60 * 60).as_nanos() as u64;
        let expiration_coarse =
            to_coarse_time(expiration, cancel_expired_requests::Job::JOB_TOLERANCE_NS);

        // insert a new request, expiration timer should be set
        let request_1 = Request {
            status: crate::models::RequestStatus::Created,
            expiration_dt: expiration,
            ..mock_request()
        };
        REQUEST_REPOSITORY.insert(request_1.to_key(), request_1.clone());

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&cancel_expired_requests::Job::JOB_TYPE)
                .expect("Job not scheduled at all")
                .get(&expiration_coarse)
                .expect("Job not scheduled at this time")
                .1,
            1
        );

        // remove the request, timer should be removed
        REQUEST_REPOSITORY.remove(&request_1.to_key());

        // all timers should be removed
        assert!(JobStateDatabase::get_time_job_maps()
            .get(&cancel_expired_requests::Job::JOB_TYPE)
            .is_none());
    }

    #[tokio::test]
    async fn test_transfer_schedule_execution() {
        set_mock_ic_time(SystemTime::UNIX_EPOCH);

        let transfer = mock_transfer();
        TRANSFER_REPOSITORY.insert(transfer.to_key(), transfer.clone());

        let coarse_time = to_coarse_time(
            transfer.created_timestamp,
            execute_created_transfers::Job::JOB_TOLERANCE_NS,
        );

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&execute_created_transfers::Job::JOB_TYPE)
                .expect("Job not scheduled at all")
                .get(&coarse_time)
                .expect("Job not scheduled at this time")
                .1,
            1
        );

        Scheduler::run_scheduled::<execute_created_transfers::Job>(coarse_time).await;

        assert!(JobStateDatabase::get_time_job_maps()
            .get(&execute_created_transfers::Job::JOB_TYPE)
            .is_none());
    }

    #[tokio::test]
    async fn test_initialize_job_timers() {
        set_mock_ic_time(SystemTime::UNIX_EPOCH);

        // create a repository with empty observers
        let request_repository = RequestRepository::with_empty_observers();
        let transfer_repository = TransferRepository::with_empty_observers();

        let expiration = time() + Duration::from_secs(30 * 24 * 60 * 60).as_nanos() as u64;
        let expiration_coarse =
            to_coarse_time(expiration, cancel_expired_requests::Job::JOB_TOLERANCE_NS);

        // create one account so transfer requests dont fail
        let account = Account {
            id: [1; 16],
            ..mock_account()
        };
        ACCOUNT_REPOSITORY.insert(account.to_key(), account);

        // create 2 requests that must be scheduled for expiration
        for _ in 0..2 {
            let request = Request {
                status: RequestStatus::Created,
                expiration_dt: expiration,
                ..mock_request()
            };
            request_repository.insert(request.to_key(), request);
        }

        // create 3 requests that must be set to scheduled
        for _ in 0..3 {
            let request = mock_request();
            request_repository.insert(request.to_key(), request);
        }

        // create 4 requests that must be scheduled for execution
        for _ in 0..4 {
            let request = Request {
                status: RequestStatus::Scheduled {
                    scheduled_at: time(),
                },
                ..mock_request()
            };
            request_repository.insert(request.to_key(), request);
        }

        // create 5 transfers that must be scheduled for execution
        for _ in 0..5 {
            let transfer = mock_transfer();
            transfer_repository.insert(transfer.to_key(), transfer);
        }

        // at this time no jobs are scheduled because of the empty observers
        // this emulates the moment after upgrade where the stable structure has data
        assert_eq!(JobStateDatabase::get_time_job_maps().len(), 0);

        // initialize the job timers
        crate::jobs::initialize_job_timers();

        // all 3 job types should have timers set
        assert_eq!(JobStateDatabase::get_time_job_maps().len(), 3);

        // 2 requests are scheduled for expiration
        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&cancel_expired_requests::Job::JOB_TYPE)
                .expect("Job not scheduled at all")
                .get(&expiration_coarse)
                .expect("Job not scheduled at this time")
                .1,
            2
        );

        // 3+4=7 requests are scheduled for execution
        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&execute_scheduled_requests::Job::JOB_TYPE)
                .expect("Job not scheduled at all")
                .iter()
                .map(|(_, (_, count))| count)
                .sum::<usize>(),
            7
        );

        // 5 transfers are scheduled for execution
        assert!(JobStateDatabase::get_time_job_maps()
            .get(&execute_created_transfers::Job::JOB_TYPE)
            .is_some());

        let transfer_job_time = *JobStateDatabase::get_time_job_maps()
            .get(&execute_created_transfers::Job::JOB_TYPE)
            .expect("Job not scheduled at all")
            .keys()
            .next()
            .expect("Job not scheduled at this time");

        // run the transfer job
        Scheduler::run_scheduled::<execute_created_transfers::Job>(transfer_job_time).await;

        // transfer job should be removed
        assert!(JobStateDatabase::get_time_job_maps()
            .get(&execute_created_transfers::Job::JOB_TYPE)
            .is_none());

        // time for expiration
        set_mock_ic_time(SystemTime::UNIX_EPOCH + Duration::from_nanos(expiration_coarse));

        // run the expiration job
        Scheduler::run_scheduled::<cancel_expired_requests::Job>(expiration_coarse).await;

        // expiration jobs should be removed
        assert!(JobStateDatabase::get_time_job_maps()
            .get(&cancel_expired_requests::Job::JOB_TYPE)
            .is_none());

        // run the scheduled requests job
        for at_ns in JobStateDatabase::get_time_job_maps()
            .get(&execute_scheduled_requests::Job::JOB_TYPE)
            .expect("Job not scheduled at all")
            .keys()
        {
            Scheduler::run_scheduled::<execute_scheduled_requests::Job>(*at_ns).await;
        }

        // all scheduled requests should be executed
        assert!(JobStateDatabase::get_time_job_maps()
            .get(&execute_scheduled_requests::Job::JOB_TYPE)
            .is_none());

        // there should be 7 new transfer jobs scheduled
        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&execute_created_transfers::Job::JOB_TYPE)
                .expect("Job not scheduled at all")
                .iter()
                .map(|(_, (_, count))| count)
                .sum::<usize>(),
            7
        );
    }
}
