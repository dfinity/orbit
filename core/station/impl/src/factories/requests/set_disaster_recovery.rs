use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{Request, RequestExecutionPlan, RequestOperation, SetDisasterRecoveryOperation},
    services::SystemService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use station_api::{CreateRequestInput, SetDisasterRecoveryOperationInput};

pub struct SetDisasterRecoveryRequestCreate;

#[async_trait]
impl Create<SetDisasterRecoveryOperationInput> for SetDisasterRecoveryRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: SetDisasterRecoveryOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::SetDisasterRecovery(SetDisasterRecoveryOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "SetDisasterRecovery".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

#[allow(dead_code)]
pub struct SetDisasterRecoveryRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o SetDisasterRecoveryOperation,
}

impl<'p, 'o> SetDisasterRecoveryRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o SetDisasterRecoveryOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for SetDisasterRecoveryRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        SystemService::set_disaster_recovery_committee(self.operation.input.committee.clone());

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
