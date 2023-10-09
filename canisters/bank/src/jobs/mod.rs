mod process_transfers;
pub use process_transfers::ProcessTransfersJob;

/// Register all the jobs to run in the background based on the defined intervals set for the timers.
pub async fn register_jobs() {
    ProcessTransfersJob::register();
}
