use std::time::Duration;

use super::{to_coarse_time, JobStateDatabase, ScheduledJob, TimerResourceGuard};
use crate::core::ic_cdk::{api::time, spawn};

#[cfg(all(not(test), not(feature = "canbench")))]
use ic_cdk_timers::{clear_timer, set_timer};
#[cfg(any(test, feature = "canbench"))]
use mock_timers::{clear_timer, set_timer};

pub struct Scheduler;

impl Scheduler {
    pub fn cancel_scheduled_timer<Job: ScheduledJob>(at_ns: u64) {
        let coarse_time = to_coarse_time(at_ns, Job::JOB_TOLERANCE_NS);
        if let Some(timer_id) = JobStateDatabase::remove_scheduled_task(Job::JOB_TYPE, coarse_time)
        {
            clear_timer(timer_id)
        }
    }

    pub fn schedule<Job: ScheduledJob>(at_ns: u64) {
        let coarse_time_ns = to_coarse_time(at_ns, Job::JOB_TOLERANCE_NS);

        if let Some(timer_id) =
            JobStateDatabase::check_existing_timer(Job::JOB_TYPE, coarse_time_ns)
        {
            // timer is already scheduled, just update the database
            JobStateDatabase::add_scheduled_task(Job::JOB_TYPE, coarse_time_ns, timer_id);
        } else {
            // schedule the timer
            let timer_id = set_timer(
                Duration::from_nanos(coarse_time_ns.saturating_sub(time())),
                move || {
                    spawn(async move {
                        // check if the job is already running
                        if !JobStateDatabase::is_running(Job::JOB_TYPE) {
                            // this guard will remove the scheduled task from the database at the end of the scope, even if the job panics
                            let _guard = TimerResourceGuard::new(Job::JOB_TYPE, coarse_time_ns);

                            JobStateDatabase::set_running(Job::JOB_TYPE, true);

                            let job_complete = Job::run().await;

                            if !job_complete {
                                Self::schedule::<Job>(time())
                            }

                            // at this point, or if the job panics, _guard will be dropped and the scheduled task will be cleaned up from the database
                        } else {
                            // if the job is already running, reschedule this timer just in case
                            Self::schedule::<Job>(time().saturating_add(Job::JOB_TOLERANCE_NS))
                        };
                    });
                },
            );

            // add the scheduled task to the database
            JobStateDatabase::add_scheduled_task(Job::JOB_TYPE, coarse_time_ns, timer_id);
        }
    }
}

#[cfg(any(test, feature = "canbench"))]
mod mock_timers {
    use ic_cdk_timers::TimerId;
    use std::time::Duration;

    pub fn set_timer(_duration: Duration, _callback: impl FnOnce() + Send + 'static) -> TimerId {
        Default::default()
    }

    pub fn clear_timer(_timer_id: TimerId) {}
}
