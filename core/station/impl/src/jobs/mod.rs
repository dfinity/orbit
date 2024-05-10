//! This module contains all the jobs that run in the background to perform tasks within the canister.
//!
//! The jobs are registered in the `register_jobs` function and are executed based on the defined timer intervals.
use crate::{
    core::observer::Observer,
    models::{Request, RequestExecutionPlan, RequestStatus, Transfer, TransferStatus},
    repositories::REQUEST_REPOSITORY,
};
use async_trait::async_trait;
use orbit_essentials::cdk::next_time;
use orbit_essentials::repository::Repository;

mod cancel_expired_requests;
mod execute_created_transfers;
mod execute_scheduled_requests;
mod scheduler;

#[async_trait]
pub trait ScheduledJob: Send + Sync {
    /// Executes the job. Returns `true`` if the job was completed or `false` there is more work to be done.
    async fn run() -> bool;
}

pub fn jobs_observe_insert_request(observer: &mut Observer<(Request, Option<Request>)>) {
    observer.add_listener(Box::new(|(request, prev)| match &request.status {
        RequestStatus::Created => {
            if prev
                .as_ref()
                .map(|p| p.status != RequestStatus::Created)
                .unwrap_or(true)
            {
                // todo: add logging
                return;
            }

            cancel_expired_requests::schedule_expiration(&request.id, request.expiration_dt);
        }
        RequestStatus::Approved => {
            if prev
                .as_ref()
                .map(|p| p.status != RequestStatus::Created)
                .unwrap_or(true)
            {
                // todo: add logging
                return;
            }

            cancel_expired_requests::cancel_scheduled_expiration(&request.id);

            let request_processing_time = next_time();
            let scheduled_at = match &request.execution_plan {
                RequestExecutionPlan::Immediate => request_processing_time,
                RequestExecutionPlan::Scheduled { execution_time } => *execution_time,
            };

            let mut request = request.clone();

            request.status = RequestStatus::Scheduled { scheduled_at };
            request.last_modification_timestamp = request_processing_time;

            REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());

            execute_scheduled_requests::schedule_request_execution(scheduled_at);
        }
        RequestStatus::Rejected | RequestStatus::Cancelled { .. } => {
            if prev
                .as_ref()
                .map(|p| p.status != RequestStatus::Created)
                .unwrap_or(true)
            {
                // todo: add logging
                return;
            }
            cancel_expired_requests::cancel_scheduled_expiration(&request.id);
        }
        RequestStatus::Scheduled { .. } => {
            // do nothing, these will exectuted by the timers already set when the request was approved
        }
        RequestStatus::Processing { .. }
        | RequestStatus::Completed { .. }
        | RequestStatus::Failed { .. } => {
            // do nothing
        }
    }));
}

pub fn jobs_observe_remove_request(observer: &mut Observer<Request>) {
    observer.add_listener(Box::new(|prev| {
        if let Request {
            id,
            status: RequestStatus::Created,
            ..
        } = prev
        {
            cancel_expired_requests::cancel_scheduled_expiration(id);
        }
    }));
}

pub fn jobs_observe_insert_transfer(observer: &mut Observer<(Transfer, Option<Transfer>)>) {
    observer.add_listener(Box::new(|(transfer, prev)| {
        if let (
            Transfer {
                status: TransferStatus::Created,
                ..
            },
            None,
        ) = (transfer, prev)
        {
            execute_created_transfers::schedule_process_transfers(next_time());
        }
    }));
}
