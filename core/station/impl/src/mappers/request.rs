use crate::{
    core::ic_cdk::api::time,
    models::{
        Request, RequestAdditionalInfo, RequestCallerPrivileges, RequestExecutionPlan,
        RequestOperation, RequestStatus, UserId,
    },
};
use orbit_essentials::{
    types::{Timestamp, UUID},
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use station_api::{RequestDTO, RequestExecutionScheduleDTO};
use uuid::Uuid;

impl Request {
    pub fn new(
        request_id: UUID,
        requester: UserId,
        expiration_dt: Timestamp,
        operation: RequestOperation,
        execution_plan: RequestExecutionPlan,
        title: String,
        summary: Option<String>,
    ) -> Request {
        Request {
            id: request_id,
            title,
            operation,
            summary,
            requested_by: requester,
            status: RequestStatus::Created,
            expiration_dt,
            execution_plan,
            approvals: vec![],
            created_timestamp: time(),
            last_modification_timestamp: time(),
        }
    }

    pub fn to_dto(self) -> RequestDTO {
        RequestDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            requested_by: Uuid::from_bytes(self.requested_by).hyphenated().to_string(),
            status: self.status.into(),
            operation: self.operation.into(),
            title: self.title,
            summary: self.summary,
            expiration_dt: timestamp_to_rfc3339(&self.expiration_dt),
            execution_plan: self.execution_plan.into(),
            created_at: timestamp_to_rfc3339(&self.created_timestamp),
            approvals: self
                .approvals
                .iter()
                .map(|approval| approval.to_owned().into())
                .collect(),
        }
    }
}

impl From<RequestExecutionScheduleDTO> for RequestExecutionPlan {
    fn from(dto: RequestExecutionScheduleDTO) -> Self {
        match dto {
            RequestExecutionScheduleDTO::Immediate => Self::Immediate,
            RequestExecutionScheduleDTO::Scheduled { execution_time } => Self::Scheduled {
                execution_time: rfc3339_to_timestamp(&execution_time),
            },
        }
    }
}

impl From<RequestExecutionPlan> for RequestExecutionScheduleDTO {
    fn from(plan: RequestExecutionPlan) -> Self {
        match plan {
            RequestExecutionPlan::Immediate => Self::Immediate,
            RequestExecutionPlan::Scheduled { execution_time } => Self::Scheduled {
                execution_time: timestamp_to_rfc3339(&execution_time),
            },
        }
    }
}

impl From<RequestCallerPrivileges> for station_api::RequestCallerPrivilegesDTO {
    fn from(privileges: RequestCallerPrivileges) -> Self {
        Self {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_approve: privileges.can_approve,
        }
    }
}

impl From<RequestAdditionalInfo> for station_api::RequestAdditionalInfoDTO {
    fn from(info: RequestAdditionalInfo) -> Self {
        Self {
            id: Uuid::from_bytes(info.id).hyphenated().to_string(),
            requester_name: info.requester_name,
            approvers: info
                .approvers
                .into_iter()
                .map(|approver| approver.into())
                .collect(),
            evaluation_result: info.evaluation_result.map(|result| result.into()),
        }
    }
}
