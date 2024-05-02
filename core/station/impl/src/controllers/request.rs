use crate::{
    core::middlewares::{authorize, call_context, use_status_metric},
    mappers::HelperMapper,
    models::resource::{RequestResourceAction, Resource},
    services::{RequestService, REQUEST_SERVICE},
};
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{
    CreateRequestInput, CreateRequestResponse, GetNextApprovableRequestInput,
    GetNextApprovableRequestResponse, GetRequestInput, GetRequestResponse, ListRequestsInput,
    ListRequestsResponse, RequestAdditionalInfoDTO, RequestCallerPrivilegesDTO,
    SubmitRequestApprovalInput, SubmitRequestApprovalResponse,
};
use std::sync::Arc;

// Canister entrypoints for the controller.
#[query(name = "list_requests")]
async fn list_requests(input: ListRequestsInput) -> ApiResult<ListRequestsResponse> {
    CONTROLLER.list_requests(input).await
}

#[query(name = "get_request")]
async fn get_request(input: GetRequestInput) -> ApiResult<GetRequestResponse> {
    CONTROLLER.get_request(input).await
}

#[query(name = "get_next_approvable_request")]
async fn get_next_approvable_request(
    input: GetNextApprovableRequestInput,
) -> ApiResult<GetNextApprovableRequestResponse> {
    CONTROLLER.get_next_approvable_request(input).await
}

#[update(name = "submit_request_approval")]
async fn submit_request_approval(
    input: SubmitRequestApprovalInput,
) -> ApiResult<SubmitRequestApprovalResponse> {
    CONTROLLER.submit_request_approval(input).await
}

#[update(name = "create_request")]
async fn create_request(input: CreateRequestInput) -> ApiResult<CreateRequestResponse> {
    CONTROLLER.create_request(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: RequestController = RequestController::new(Arc::clone(&REQUEST_SERVICE));
}

#[derive(Debug)]
pub struct RequestController {
    request_service: Arc<RequestService>,
}

impl RequestController {
    fn new(request_service: Arc<RequestService>) -> Self {
        Self { request_service }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    #[with_middleware(tail = use_status_metric("call_create_request", &result))]
    async fn create_request(&self, input: CreateRequestInput) -> ApiResult<CreateRequestResponse> {
        let ctx = &call_context();
        let request = self.request_service.create_request(input, ctx).await?;
        let privileges = self
            .request_service
            .get_caller_privileges_for_request(&request.id, ctx)
            .await?;
        let additional_info = self
            .request_service
            .get_request_additional_info(&request, true)?;

        Ok(CreateRequestResponse {
            request: request.to_dto(),
            privileges: privileges.into(),
            additional_info: additional_info.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    async fn get_request(&self, input: GetRequestInput) -> ApiResult<GetRequestResponse> {
        let ctx = &call_context();
        let request = self
            .request_service
            .get_request(HelperMapper::to_uuid(input.request_id)?.as_bytes())?;
        let privileges = self
            .request_service
            .get_caller_privileges_for_request(&request.id, ctx)
            .await?;
        let additional_info = self
            .request_service
            .get_request_additional_info(&request, true)?;

        Ok(GetRequestResponse {
            request: request.to_dto(),
            privileges: privileges.into(),
            additional_info: additional_info.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Request(RequestResourceAction::List)]))]
    async fn list_requests(&self, input: ListRequestsInput) -> ApiResult<ListRequestsResponse> {
        let ctx = call_context();
        let with_evaluation_results = input.with_evaluation_results;
        let result = self.request_service.list_requests(input, &ctx).await?;

        let mut privileges = Vec::new();
        let mut additionals = Vec::new();

        for request in &result.items {
            let privilege = self
                .request_service
                .get_caller_privileges_for_request(&request.id, &ctx)
                .await?;

            let additional_info = self
                .request_service
                .get_request_additional_info(request, with_evaluation_results)?;

            privileges.push(RequestCallerPrivilegesDTO::from(privilege));
            additionals.push(RequestAdditionalInfoDTO::from(additional_info));
        }

        Ok(ListRequestsResponse {
            requests: result.items.into_iter().map(|p| p.to_dto()).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
            additional_info: additionals,
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Request(RequestResourceAction::List)]))]
    async fn get_next_approvable_request(
        &self,
        input: GetNextApprovableRequestInput,
    ) -> ApiResult<GetNextApprovableRequestResponse> {
        let ctx = call_context();
        let result = self
            .request_service
            .get_next_approvable_request(input, Some(&ctx))
            .await?;

        if let Some(request) = result {
            let privileges = self
                .request_service
                .get_caller_privileges_for_request(&request.id, &ctx)
                .await?;

            let additional_info = self
                .request_service
                .get_request_additional_info(&request, true)?;

            Ok(Some(GetRequestResponse {
                request: request.to_dto(),
                privileges: privileges.into(),
                additional_info: additional_info.into(),
            }))
        } else {
            Ok(None)
        }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    #[with_middleware(tail = use_status_metric("call_submit_request_approval", &result))]
    async fn submit_request_approval(
        &self,
        input: SubmitRequestApprovalInput,
    ) -> ApiResult<SubmitRequestApprovalResponse> {
        let ctx = &call_context();
        let request = self
            .request_service
            .submit_request_approval(input, ctx)
            .await?;
        let privileges = self
            .request_service
            .get_caller_privileges_for_request(&request.id, ctx)
            .await?;
        let additional_info = self
            .request_service
            .get_request_additional_info(&request, true)?;

        Ok(SubmitRequestApprovalResponse {
            request: request.to_dto(),
            privileges: privileges.into(),
            additional_info: additional_info.into(),
        })
    }
}
