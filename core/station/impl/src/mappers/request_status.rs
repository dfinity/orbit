use crate::models::{RequestStatus, RequestStatusCode};
use orbit_essentials::utils::{rfc3339_to_timestamp, timestamp_to_rfc3339};
use station_api::{RequestStatusCodeDTO, RequestStatusDTO};

impl From<RequestStatus> for RequestStatusDTO {
    fn from(status: RequestStatus) -> Self {
        match status {
            RequestStatus::Created => RequestStatusDTO::Created,
            RequestStatus::Approved => RequestStatusDTO::Approved,
            RequestStatus::Rejected => RequestStatusDTO::Rejected,
            RequestStatus::Completed { completed_at } => RequestStatusDTO::Completed {
                completed_at: timestamp_to_rfc3339(&completed_at),
            },
            RequestStatus::Failed { reason } => RequestStatusDTO::Failed { reason },
            RequestStatus::Processing { started_at } => RequestStatusDTO::Processing {
                started_at: timestamp_to_rfc3339(&started_at),
            },
            RequestStatus::Scheduled { scheduled_at } => RequestStatusDTO::Scheduled {
                scheduled_at: timestamp_to_rfc3339(&scheduled_at),
            },
            RequestStatus::Cancelled { reason } => RequestStatusDTO::Cancelled { reason },
        }
    }
}

impl From<RequestStatus> for RequestStatusCodeDTO {
    fn from(status: RequestStatus) -> Self {
        match status {
            RequestStatus::Created => RequestStatusCodeDTO::Created,
            RequestStatus::Approved => RequestStatusCodeDTO::Approved,
            RequestStatus::Rejected => RequestStatusCodeDTO::Rejected,
            RequestStatus::Completed { .. } => RequestStatusCodeDTO::Completed,
            RequestStatus::Failed { .. } => RequestStatusCodeDTO::Failed,
            RequestStatus::Processing { .. } => RequestStatusCodeDTO::Processing,
            RequestStatus::Scheduled { .. } => RequestStatusCodeDTO::Scheduled,
            RequestStatus::Cancelled { .. } => RequestStatusCodeDTO::Cancelled,
        }
    }
}

impl From<RequestStatusDTO> for RequestStatus {
    fn from(status: RequestStatusDTO) -> Self {
        match status {
            RequestStatusDTO::Created => RequestStatus::Created,
            RequestStatusDTO::Approved => RequestStatus::Approved,
            RequestStatusDTO::Rejected => RequestStatus::Rejected,
            RequestStatusDTO::Completed { completed_at } => RequestStatus::Completed {
                completed_at: rfc3339_to_timestamp(completed_at.as_str()),
            },
            RequestStatusDTO::Failed { reason } => RequestStatus::Failed { reason },
            RequestStatusDTO::Processing { started_at } => RequestStatus::Processing {
                started_at: rfc3339_to_timestamp(started_at.as_str()),
            },
            RequestStatusDTO::Scheduled { scheduled_at } => RequestStatus::Scheduled {
                scheduled_at: rfc3339_to_timestamp(&scheduled_at),
            },
            RequestStatusDTO::Cancelled { reason } => RequestStatus::Cancelled { reason },
        }
    }
}

impl From<RequestStatusCodeDTO> for RequestStatusCode {
    fn from(status: RequestStatusCodeDTO) -> Self {
        match status {
            RequestStatusCodeDTO::Created => RequestStatusCode::Created,
            RequestStatusCodeDTO::Approved => RequestStatusCode::Approved,
            RequestStatusCodeDTO::Rejected => RequestStatusCode::Rejected,
            RequestStatusCodeDTO::Completed => RequestStatusCode::Completed,
            RequestStatusCodeDTO::Failed => RequestStatusCode::Failed,
            RequestStatusCodeDTO::Processing => RequestStatusCode::Processing,
            RequestStatusCodeDTO::Scheduled => RequestStatusCode::Scheduled,
            RequestStatusCodeDTO::Cancelled => RequestStatusCode::Cancelled,
        }
    }
}

#[derive(Debug)]
pub struct RequestStatusMapper;

impl RequestStatusMapper {
    pub fn from_status_code_dto(status: &RequestStatusCodeDTO) -> &str {
        match status {
            RequestStatusCodeDTO::Created => "created",
            RequestStatusCodeDTO::Approved => "approved",
            RequestStatusCodeDTO::Rejected => "rejected",
            RequestStatusCodeDTO::Completed => "completed",
            RequestStatusCodeDTO::Failed => "failed",
            RequestStatusCodeDTO::Processing => "processing",
            RequestStatusCodeDTO::Scheduled => "scheduled",
            RequestStatusCodeDTO::Cancelled => "cancelled",
        }
    }
}
