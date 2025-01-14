use std::time::Duration;

use crate::core::ic_timers::{clear_timer, set_timer};

use super::{to_coarse_time, JobStateDatabase, ScheduledJob, TimerResourceGuard};
use crate::core::ic_cdk::{api::time, spawn};

pub struct Scheduler;

impl Scheduler {
    pub fn cancel_scheduled_timer<Job: ScheduledJob>(at_ns: u64) {
        let coarse_time = to_coarse_time(at_ns, Job::JOB_TOLERANCE_NS);
        if let Some(timer_id) = JobStateDatabase::remove_scheduled_task(Job::JOB_TYPE, coarse_time)
        {
            clear_timer(timer_id)
        }
    }

    pub async fn run_scheduled<Job: ScheduledJob>(scheduled_at_ns: u64) {
        // check if the job is already running
        if !JobStateDatabase::is_running(Job::JOB_TYPE) {
            // this guard will remove the scheduled task from the database at the end of the scope, even if the job panics
            let _guard = TimerResourceGuard::new(Job::JOB_TYPE, scheduled_at_ns);

            JobStateDatabase::set_running(Job::JOB_TYPE, true);

            let job_complete = Job::run().await;

            if !job_complete {
                Self::schedule::<Job>(time().saturating_add(Job::JOB_TOLERANCE_NS))
            }

            // at this point, or if the job panics, _guard will be dropped and the scheduled task will be cleaned up from the database
        } else {
            // if the job is already running, reschedule this timer just in case
            Self::schedule::<Job>(time().saturating_add(Job::JOB_TOLERANCE_NS));
        };
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
                        Self::run_scheduled::<Job>(coarse_time_ns).await;
                    });
                },
            );

            // add the scheduled task to the database
            JobStateDatabase::add_scheduled_task(Job::JOB_TYPE, coarse_time_ns, timer_id);
        }
    }
}

#[cfg(test)]
mod test {

    use core::panic;
    use std::panic::{set_hook, take_hook};

    use crate::jobs::{to_coarse_time, JobStateDatabase, JobType, ScheduledJob};
    use async_trait::async_trait;
    use futures::future::pending;

    struct OneShotJob;

    #[async_trait]
    impl ScheduledJob for OneShotJob {
        const JOB_TYPE: JobType = JobType::CancelExpiredRequests;
        const JOB_TOLERANCE_NS: u64 = 1_000_000_000;
        async fn run() -> bool {
            // job is complete
            true
        }
    }

    struct LongJob;

    #[async_trait]
    impl ScheduledJob for LongJob {
        const JOB_TYPE: JobType = JobType::CancelExpiredRequests;
        const JOB_TOLERANCE_NS: u64 = 1_000_000_000;

        async fn run() -> bool {
            pending().await
        }
    }

    struct JobWithMoreWorkToDo;
    #[async_trait]
    impl ScheduledJob for JobWithMoreWorkToDo {
        const JOB_TYPE: JobType = JobType::CancelExpiredRequests;
        const JOB_TOLERANCE_NS: u64 = 1_000_000_000;

        async fn run() -> bool {
            // job is not complete
            false
        }
    }

    struct JobThatPanics;
    #[async_trait]
    impl ScheduledJob for JobThatPanics {
        const JOB_TYPE: JobType = JobType::CancelExpiredRequests;
        const JOB_TOLERANCE_NS: u64 = 1_000_000_000;

        async fn run() -> bool {
            panic!("Job panicking")
        }
    }

    #[test]
    fn coarse_time_tests() {
        assert_eq!(to_coarse_time(0, 1_000), 0);
        assert_eq!(to_coarse_time(1, 1_000), 1_000);
        assert_eq!(to_coarse_time(1_000, 1_000), 1_000);
        assert_eq!(to_coarse_time(1_001, 1_000), 2_000);
    }

    #[tokio::test]
    async fn jobs_can_be_scheduled() {
        super::Scheduler::schedule::<OneShotJob>(0);

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&OneShotJob::JOB_TYPE)
                .expect("Job not scheduled")
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn jobs_are_removed_after_finishing() {
        super::Scheduler::schedule::<OneShotJob>(0);

        super::Scheduler::run_scheduled::<OneShotJob>(0).await;

        assert!(!JobStateDatabase::get_time_job_maps().contains_key(&OneShotJob::JOB_TYPE));
    }

    #[tokio::test]
    async fn jobs_that_panic_still_get_removed() {
        super::Scheduler::schedule::<JobThatPanics>(0);

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&JobThatPanics::JOB_TYPE)
                .expect("Job not scheduled")
                .len(),
            1
        );

        tokio::spawn(async {
            // suppress stack trace
            set_hook(Box::new(|_| {}));

            // call the job that will panic
            super::Scheduler::run_scheduled::<JobThatPanics>(0).await;

            // reset the hook
            let _ = take_hook();
        })
        .await
        .expect_err("Job should panic");

        assert!(!JobStateDatabase::get_time_job_maps().contains_key(&JobThatPanics::JOB_TYPE));
    }

    #[tokio::test]
    async fn jobs_that_dont_complete_reschedule_themselves() {
        super::Scheduler::schedule::<JobWithMoreWorkToDo>(0);

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&JobWithMoreWorkToDo::JOB_TYPE)
                .expect("Job not scheduled at all")
                .get(&0)
                .expect("Job not scheduled at this time")
                .1,
            1
        );

        super::Scheduler::run_scheduled::<JobWithMoreWorkToDo>(0).await;

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&JobWithMoreWorkToDo::JOB_TYPE)
                .expect("Job not scheduled at all")
                .get(&JobWithMoreWorkToDo::JOB_TOLERANCE_NS)
                .expect("Job not scheduled at this time")
                .1,
            1
        );
    }

    #[tokio::test]
    async fn multiple_jobs_with_the_same_coarse_time() {
        super::Scheduler::schedule::<OneShotJob>(4_000_000_001);
        super::Scheduler::schedule::<OneShotJob>(4_999_999_999);
        super::Scheduler::schedule::<OneShotJob>(5_000_000_000);

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&JobType::CancelExpiredRequests)
                .expect("Job not scheduled")
                .len(),
            1
        );

        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&JobType::CancelExpiredRequests)
                .expect("Job not scheduled")
                .get(&5_000_000_000u64)
                .expect("Job not scheduled")
                .1,
            3
        );

        super::Scheduler::run_scheduled::<OneShotJob>(5_000_000_000u64).await;

        assert!(
            !JobStateDatabase::get_time_job_maps().contains_key(&JobType::CancelExpiredRequests)
        );
    }

    #[tokio::test]
    async fn jobs_rescheduled_when_one_is_already_running() {
        super::Scheduler::schedule::<LongJob>(0);

        // trigger running first job
        let _run_1 = tokio::spawn(async {
            super::Scheduler::run_scheduled::<LongJob>(0).await;
        });

        // trigger running first job again
        // this time we can await it because it'll not be executed but rescheduled
        let _run_2 = tokio::spawn(async {
            super::Scheduler::run_scheduled::<LongJob>(0).await;
        })
        .await;

        // job is already running, this run should be rescheduled
        assert_eq!(
            JobStateDatabase::get_time_job_maps()
                .get(&JobType::CancelExpiredRequests)
                .expect("Job not scheduled")
                .len(),
            2
        );
    }
}
