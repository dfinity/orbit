use crate::jobs::JobType;
use crate::{core::ic_cdk::next_time, models::RequestStatusCode, repositories::RequestRepository};
use async_trait::async_trait;

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
        let requests = self.request_repository.find_by_status_and_expiration_dt(
            RequestStatusCode::Created,
            None,
            Some(current_time),
        );

        for request in requests.into_iter() {
            self.request_repository
                .cancel_request(request, "The request has expired".to_string(), next_time())
                .await;
        }

        true
    }
}

pub fn schedule_expiration(at_ns: u64) {
    Scheduler::schedule::<Job>(at_ns);
}

pub fn cancel_scheduled_expiration(at_ns: u64) {
    Scheduler::cancel_scheduled_timer::<Job>(at_ns);
}
