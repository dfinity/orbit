//! This module contains all the jobs that run in the background to perform tasks within the Bank canister.
//!
//! The jobs are registered in the `register_jobs` function and are executed based on the defined timer intervals.

mod cancel_expired_transfers;
mod process_transfers;

pub async fn register_jobs() {
    process_transfers::Job::register();
    cancel_expired_transfers::Job::register();
}
