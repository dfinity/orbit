use crate::{
    core::ic_cdk::next_time,
    models::{RequestStatus, RequestStatusCode},
    repositories::RequestRepository,
};
use async_trait::async_trait;
use orbit_essentials::repository::Repository;

use super::ScheduledJob;

#[derive(Debug, Default)]
pub struct Job {
    request_repository: RequestRepository,
}

#[async_trait]
impl ScheduledJob for Job {
    const INTERVAL_SECS: u64 = 60;
    const ALLOW_CONCURRENT_EXECUTION: bool = false;

    async fn run() {
        Self::default().cancel_requests().await;
    }
}

/// This job is responsible for canceling the requests that have expired while not approved/rejected.
impl Job {
    /// Cancel the requests that have expired while still pending.
    async fn cancel_requests(&self) {
        let current_time = next_time();
        let mut requests = self.request_repository.find_by_expiration_dt_and_status(
            None,
            Some(current_time),
            RequestStatusCode::Created.to_string(),
        );

        for request in requests.iter_mut() {
            request.status = RequestStatus::Cancelled {
                reason: Some("The request has expired".to_string()),
            };
            request.last_modification_timestamp = next_time();
            self.request_repository
                .insert(request.to_key(), request.to_owned());
        }
    }
}
