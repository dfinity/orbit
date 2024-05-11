use super::{scheduler::Scheduler, JobType, ScheduledJob};
use crate::{
    core::ic_cdk::next_time,
    errors::RequestExecuteError,
    factories::requests::{RequestExecuteStage, RequestFactory},
    models::{Request, RequestStatus},
    repositories::RequestRepository,
    services::RequestService,
};
use async_trait::async_trait;
use futures::future;
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

    /// Processes all the requests that have been approved but are not yet executed.
    ///
    /// This function will process a maximum of `MAX_BATCH_SIZE` requests at once.
    async fn execute_scheduled_requests(&self) -> bool {
        let current_time = next_time();
        let mut requests = self
            .request_repository
            .find_scheduled(None, Some(current_time));

        let processing_all_requests = requests.len() <= Self::MAX_BATCH_SIZE;

        // truncate the list to avoid processing too many requests at once
        requests.truncate(Self::MAX_BATCH_SIZE);

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
        let requests = requests.clone();

        // update the status of the requests
        for (pos, result) in results.iter().enumerate() {
            match result {
                Ok(request) => {
                    self.request_repository
                        .insert(request.to_key(), request.to_owned());
                }
                Err(e) => {
                    let request_failed_time = next_time();
                    let mut request = requests[pos].clone();
                    request.status = RequestStatus::Failed {
                        reason: Some(e.to_string()),
                    };
                    request.last_modification_timestamp = request_failed_time;
                    self.request_repository
                        .insert(request.to_key(), request.to_owned());

                    self.request_service.failed_request_hook(&request).await;
                }
            }
        }

        processing_all_requests
    }

    /// Executes a single request.
    ///
    /// This function will handle the request execution for the given operation type.
    async fn execute_request(&self, mut request: Request) -> Result<Request, RequestExecuteError> {
        let executor = RequestFactory::executor(&request);

        let execute_state = executor.execute().await?;

        drop(executor);

        let request_execution_time = next_time();

        request.status = match execute_state {
            RequestExecuteStage::Completed(_) => RequestStatus::Completed {
                completed_at: request_execution_time,
            },
            RequestExecuteStage::Processing(_) => RequestStatus::Processing {
                started_at: request_execution_time,
            },
        };

        request.operation = match execute_state {
            RequestExecuteStage::Completed(operation) => operation,
            RequestExecuteStage::Processing(operation) => operation,
        };

        request.last_modification_timestamp = request_execution_time;

        Ok(request)
    }
}

pub fn schedule_request_execution(at_ns: u64) {
    Scheduler::schedule::<Job>(at_ns);
}
