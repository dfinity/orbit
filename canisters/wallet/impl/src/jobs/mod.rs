//! This module contains all the jobs that run in the background to perform tasks within the Wallet canister.
//!
//! The jobs are registered in the `register_jobs` function and are executed based on the defined timer intervals.

mod cancel_expired_proposals;
mod execute_created_transfers;
mod execute_scheduled_proposals;
mod schedule_adopted_proposals;

/// Register all the jobs that run in the background to perform tasks within the Wallet canister.
pub async fn register_jobs() {
    schedule_adopted_proposals::Job::register();
    execute_scheduled_proposals::Job::register();
    cancel_expired_proposals::Job::register();
    execute_created_transfers::Job::register();
}
