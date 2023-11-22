//! This module contains all the jobs that run in the background to perform tasks within the canister.
//!
//! The jobs are registered in the `register_jobs` function and are executed based on the defined timer intervals.
use crate::core::ic_cdk::spawn;
use async_trait::async_trait;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

mod cancel_expired_proposals;
mod execute_created_transfers;
mod execute_scheduled_proposals;
mod schedule_adopted_proposals;

#[async_trait]
pub trait ScheduledJob {
    /// Whether the job can be executed concurrently.
    const ALLOW_CONCURRENT_EXECUTION: bool = true;

    /// The interval in seconds at which the job will be executed.
    const INTERVAL_SECS: u64;

    /// Executes the job.
    async fn run();
}

/// Registers the job to be executed at the defined interval, using the `spawn` function inside a timer.
pub fn register_job<Job: ScheduledJob>() {
    let interval = Duration::from_secs(Job::INTERVAL_SECS);
    let is_running = Arc::new(AtomicBool::new(false));

    ic_cdk_timers::set_timer_interval(interval, move || {
        let is_running = is_running.clone();

        spawn(async move {
            if Job::ALLOW_CONCURRENT_EXECUTION || !is_running.load(Ordering::SeqCst) {
                is_running.store(true, Ordering::SeqCst);
                Job::run().await;
                is_running.store(false, Ordering::SeqCst);
            }
        });
    });
}

/// Register all the jobs that run in the background to perform tasks within the canister.
pub async fn register_jobs() {
    register_job::<schedule_adopted_proposals::Job>();
    register_job::<execute_scheduled_proposals::Job>();
    register_job::<cancel_expired_proposals::Job>();
    register_job::<execute_created_transfers::Job>();
}
