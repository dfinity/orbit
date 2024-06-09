use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{ManageSystemInfoOperation, Request, RequestExecutionPlan, RequestOperation},
    services::SYSTEM_SERVICE,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct ManageSystemInfoRequestCreate {}

#[async_trait]
impl Create<station_api::ManageSystemInfoOperationInput> for ManageSystemInfoRequestCreate {
    async fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::ManageSystemInfoOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::ManageSystemInfo(ManageSystemInfoOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Manage System Info".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct ManageSystemInfoRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o ManageSystemInfoOperation,
}

impl<'p, 'o> ManageSystemInfoRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o ManageSystemInfoOperation) -> Self {
        Self { request, operation }
    }
}

#[async_trait]
impl Execute for ManageSystemInfoRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        SYSTEM_SERVICE.update_system_info(self.operation.input.clone());

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{read_system_info, test_utils},
        models::ManageSystemInfoOperationInput,
    };
    use tests::mnanage_system_info_test_utils::{
        mock_manage_system_info_api_input, mock_request_api_operation,
    };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_request() {
        let request_id = *Uuid::new_v4().as_bytes();
        let requested_by_user = *Uuid::new_v4().as_bytes();
        let mut create_request = mock_request_api_operation();
        let mut input = mock_manage_system_info_api_input();
        input.name = Some("name".to_string());
        create_request.operation =
            station_api::RequestOperationInput::ManageSystemInfo(input.clone());

        let request = ManageSystemInfoRequestCreate::create(
            request_id,
            requested_by_user,
            create_request,
            input,
        )
        .await
        .unwrap();

        assert_eq!(request.requested_by, requested_by_user);
        assert_eq!(
            request.operation,
            RequestOperation::ManageSystemInfo(ManageSystemInfoOperation {
                input: ManageSystemInfoOperationInput {
                    name: Some("name".to_string()),
                },
            })
        );
        assert_eq!(request.execution_plan, RequestExecutionPlan::Immediate);
        assert_eq!(request.title, "title");
        assert_eq!(request.summary, Some("summary".to_string()));
    }

    #[tokio::test]
    async fn test_execution_completed() {
        test_utils::init_canister_system();

        let request_id = *Uuid::new_v4().as_bytes();
        let requested_by_user = *Uuid::new_v4().as_bytes();
        let mut create_request = mock_request_api_operation();
        let mut input = mock_manage_system_info_api_input();
        input.name = Some("my-updated-name".to_string());
        create_request.operation =
            station_api::RequestOperationInput::ManageSystemInfo(input.clone());

        let request = ManageSystemInfoRequestCreate::create(
            request_id,
            requested_by_user,
            create_request,
            input,
        )
        .await
        .unwrap();

        let operation = match &request.operation {
            RequestOperation::ManageSystemInfo(operation) => operation,
            _ => panic!("Invalid operation"),
        };

        let execute = ManageSystemInfoRequestExecute::new(&request, operation);
        let result = execute.execute().await.unwrap();

        assert_eq!(
            result,
            RequestExecuteStage::Completed(request.operation.clone())
        );

        let info = read_system_info();

        assert_eq!(info.get_name(), "my-updated-name");
    }
}

#[cfg(test)]
mod mnanage_system_info_test_utils {
    pub fn mock_manage_system_info_api_input() -> station_api::ManageSystemInfoOperationInput {
        station_api::ManageSystemInfoOperationInput {
            name: Some("name".to_string()),
        }
    }

    pub fn mock_request_api_operation() -> station_api::CreateRequestInput {
        station_api::CreateRequestInput {
            title: Some("title".to_string()),
            summary: Some("summary".to_string()),
            execution_plan: Some(station_api::RequestExecutionScheduleDTO::Immediate),
            operation: station_api::RequestOperationInput::ManageSystemInfo(
                mock_manage_system_info_api_input(),
            ),
        }
    }
}
