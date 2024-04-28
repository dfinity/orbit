use super::ScheduledJob;
use crate::{
    core::ic_cdk::api::time,
    models::{RequestExecutionPlan, RequestStatus, RequestStatusCode},
    repositories::RequestRepository,
};
use async_trait::async_trait;
use orbit_essentials::repository::Repository;

#[derive(Debug, Default)]
pub struct Job {
    request_repository: RequestRepository,
}

#[async_trait]
impl ScheduledJob for Job {
    const INTERVAL_SECS: u64 = 5;
    const ALLOW_CONCURRENT_EXECUTION: bool = false;

    async fn run() {
        Self::default().process_approved_requests().await;
    }
}

/// This job is responsible for processing the requests that have been adopted and
/// are ready to be scheduled.
impl Job {
    pub const MAX_BATCH_SIZE: usize = 100;

    /// Processes all the requests that have been adopted but are not yet scheduled to be executed.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` requests at once.
    async fn process_approved_requests(&self) {
        let current_time = time();
        let mut requests = self.request_repository.find_by_status(
            RequestStatusCode::Approved,
            None,
            Some(current_time),
        );

        // truncate the list to avoid processing too many requests at once.
        requests.truncate(Self::MAX_BATCH_SIZE);

        // schedule the requests to be executed.
        for request in requests.iter_mut() {
            let scheduled_at = match &request.execution_plan {
                RequestExecutionPlan::Immediate => time(),
                RequestExecutionPlan::Scheduled { execution_time } => *execution_time,
            };

            request.status = RequestStatus::Scheduled { scheduled_at };
            request.last_modification_timestamp = time();
            self.request_repository
                .insert(request.to_key(), request.to_owned());
        }
    }
}
