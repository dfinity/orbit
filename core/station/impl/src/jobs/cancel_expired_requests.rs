use crate::{core::ic_cdk::api::time, jobs::JobType};
use crate::{
    core::ic_cdk::next_time,
    models::{RequestStatus, RequestStatusCode},
    repositories::RequestRepository,
};
use async_trait::async_trait;
use orbit_essentials::repository::Repository;

use super::{scheduler::Scheduler, ScheduledJob};

#[derive(Debug, Default)]
pub struct Job {
    request_repository: RequestRepository,
}

#[async_trait]
impl ScheduledJob for Job {
    const JOB_TYPE: JobType = JobType::CancelExpiredRequests;
    async fn run() -> bool {
        Self::default().cancel_requests().await
    }
}

/// This job is responsible for canceling the requests that have expired while not approved/rejected.
impl Job {
    /// Cancel the requests that have expired while still pending.
    async fn cancel_requests(&self) -> bool {
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

        true
    }
}

pub fn schedule_expiration(at_ns: u64) {
    Scheduler::schedule::<Job>(at_ns.saturating_sub(time()));
}

pub fn cancel_scheduled_expiration(at_ns: u64) {
    Scheduler::cancel_scheduled_timer::<Job>(at_ns);
}
