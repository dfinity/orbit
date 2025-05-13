use super::{scheduler::Scheduler, JobType, ScheduledJob};
use crate::{
    core::ic_cdk::next_time,
    errors::RequestExecuteError,
    models::{Request, RequestStatus},
    repositories::RequestRepository,
    services::RequestService,
};
use async_trait::async_trait;
use futures::future;
use orbit_essentials::cdk::{call, id};
use orbit_essentials::repository::Repository;

#[derive(Debug, Default)]
pub struct Job {
    request_repository: RequestRepository,
    request_service: RequestService,
}

#[async_trait]
impl ScheduledJob for Job {
    const JOB_TYPE: JobType = JobType::ExecuteScheduledRequests;
    async fn run() -> bool {
        Self::default().execute_scheduled_requests().await
    }
}

/// This job is responsible for processing the requests that have been approved and
/// are ready to be executed.
impl Job {
    pub const MAX_BATCH_SIZE: usize = 20;

    /// The maximum number of processing requests must be smaller than 500 (queue capacity between any pair of canisters)
    /// and we also leave some slack.
    pub const MAX_PROCESSING_REQUESTS: usize = 400;

    /// Processes all the requests that have been approved but are not yet executed.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` requests at once.
    ///
    /// At any point in time, at most `MAX_PROCESSING_REQUESTS` requests can be processing at the same time.
    async fn execute_scheduled_requests(&self) -> bool {
        let current_time = next_time();
        let mut requests = self
            .request_repository
            .find_scheduled(None, Some(current_time));

        let num_processing_requests = self.request_repository.get_num_processing();
        let batch_size = std::cmp::min(
            Self::MAX_PROCESSING_REQUESTS.saturating_sub(num_processing_requests),
            Self::MAX_BATCH_SIZE,
        );

        let processing_all_requests = requests.len() <= batch_size;

        // truncate the list to avoid processing too many requests at once
        requests.truncate(batch_size);

        // update the status of the requests to avoid processing them again
        for request in requests.iter_mut() {
            let request_processing_time = next_time();
            request.status = RequestStatus::Processing {
                started_at: request_processing_time,
            };
            request.last_modification_timestamp = request_processing_time;
            self.request_repository
                .insert(request.to_key(), request.to_owned());
        }

        // batch the requests to be executed
        let calls = requests
            .clone()
            .into_iter()
            .map(|request| self.execute_request(request));

        // wait for all the requests to be executed
        let results = future::join_all(calls).await;

        // update the status of the requests
        for result in results.into_iter() {
            match result {
                Ok(()) => (),
                Err((request, e)) => {
                    let request_failed_time = next_time();
                    self.request_service
                        .fail_request(request, e.to_string(), request_failed_time)
                        .await;
                }
            }
        }

        processing_all_requests
    }

    /// Executes a single request.
    ///
    /// This function will handle the request execution for the given operation type.
    /// In production, the actual request execution is wrapped inside an inter-canister call
    /// to catch traps and report them as `RequestExecuteError::InternalError`.
    async fn execute_request(
        &self,
        request: Request,
    ) -> Result<(), (Request, RequestExecuteError)> {
        if cfg!(test) {
            self.request_service
                .try_execute_request(request.id)
                .await
                .map_err(|e| (request, e))
        } else {
            call::<_, (_,)>(id(), "try_execute_request", (request.id,))
                .await
                .map(|res| res.0)
                .map_err(|(_code, msg)| {
                    (request, RequestExecuteError::InternalError { reason: msg })
                })
        }
    }
}

pub fn schedule_request_execution(at_ns: u64) {
    Scheduler::schedule::<Job>(at_ns);
}
