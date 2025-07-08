use crate::{
    core::ic_cdk::next_time,
    models::{
        Request, RequestAdditionalInfo, RequestCallerPrivileges, RequestExecutionPlan,
        RequestOperation, RequestStatus, UserId,
    },
};
use orbit_essentials::{
    types::{Timestamp, UUID},
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use station_api::{
    CallExternalCanisterOperationDTO, RequestDTO, RequestExecutionScheduleDTO, RequestOperationDTO,
};
use uuid::Uuid;

impl Request {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        request_id: UUID,
        requester: UserId,
        expiration_dt: Timestamp,
        operation: RequestOperation,
        execution_plan: RequestExecutionPlan,
        title: String,
        summary: Option<String>,
        deduplication_key: Option<String>,
    ) -> Request {
        let now = next_time();

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
            created_timestamp: now,
            last_modification_timestamp: now,
            deduplication_key,
        }
    }

    fn inner_to_dto(self, with_full_info: bool) -> RequestDTO {
        RequestDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            requested_by: Uuid::from_bytes(self.requested_by).hyphenated().to_string(),
            status: self.status.into(),
            operation: if with_full_info {
                match self.operation {
                    RequestOperation::CallExternalCanister(operation) => {
                        let arg = operation.input.arg.clone();
                        let mut operation_dto: CallExternalCanisterOperationDTO = operation.into();

                        operation_dto.arg = arg;

                        RequestOperationDTO::CallExternalCanister(Box::new(operation_dto))
                    }
                    _ => self.operation.into(),
                }
            } else {
                self.operation.into()
            },
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
            deduplication_key: self.deduplication_key,
        }
    }

    pub fn to_dto_with_full_info(self) -> RequestDTO {
        self.inner_to_dto(true)
    }

    pub fn to_dto(self) -> RequestDTO {
        self.inner_to_dto(false)
    }

    pub fn from_request_creation_input(
        request_id: UUID,
        requested_by_user: UserId,
        request_config: station_api::CreateRequestInput,
        request_operation: RequestOperation,
        request_default_title: String,
    ) -> Request {
        let mut expiration_dt = request_config
            .expiration_dt
            .map(|dt| rfc3339_to_timestamp(&dt))
            .unwrap_or(Request::default_expiration_dt_ns());

        let execution_plan = request_config
            .execution_plan
            .map(Into::into)
            .unwrap_or(RequestExecutionPlan::Immediate);

        if let RequestExecutionPlan::Scheduled { execution_time } = execution_plan {
            // Ensure that if the execution time is set, the expiration time is not later than the execution time.
            expiration_dt = expiration_dt.min(execution_time);
        }

        Request::new(
            request_id,
            requested_by_user,
            expiration_dt,
            request_operation,
            execution_plan,
            request_config.title.unwrap_or(request_default_title),
            request_config.summary,
            request_config.deduplication_key,
        )
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
