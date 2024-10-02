use crate::{
    core::ic_cdk::api::call::arg_data_raw_size,
    core::ic_cdk::api::{time, trap},
    core::limiter::Limiter,
    core::middlewares::{authorize, call_context, use_canister_call_metric},
    core::CallContext,
    errors::{RequestError, RequestExecuteError},
    mappers::HelperMapper,
    models::rate_limiter::RequestRateLimiterKey,
    models::resource::{RequestResourceAction, Resource},
    services::{RequestService, REQUEST_SERVICE},
};
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::types::UUID;
use orbit_essentials::with_middleware;
use station_api::{
    CreateRequestInput, CreateRequestResponse, GetNextApprovableRequestInput,
    GetNextApprovableRequestResponse, GetRequestInput, GetRequestResponse, ListRequestsInput,
    ListRequestsResponse, RequestAdditionalInfoDTO, RequestCallerPrivilegesDTO,
    SubmitRequestApprovalInput, SubmitRequestApprovalResponse,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

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
    CONTROLLER.create_request(input, arg_data_raw_size()).await
}

#[update(name = "try_execute_request", hidden = true)]
async fn try_execute_request(id: UUID) -> Result<(), RequestExecuteError> {
    CONTROLLER.try_execute_request(id).await
}

const RATE_LIMITER_RESOLUTION: Duration = Duration::from_secs(10);
const RATE_LIMITER_TIME_WINDOW: Duration = Duration::from_secs(300);
const REQUEST_RATE_LIMITER_MAX_COUNT: usize = 2000; // 2000 requests per 5mins
const REQUEST_RATE_LIMITER_MAX_SIZE: usize = 10_000_000; // total request size of 10MB per 5mins

thread_local! {
    static REQUEST_COUNT_RATE_LIMITER: RefCell<HashMap<RequestRateLimiterKey, Limiter>> = RefCell::new(HashMap::new());
    static REQUEST_SIZE_RATE_LIMITER: RefCell<HashMap<RequestRateLimiterKey, Limiter>> = RefCell::new(HashMap::new());
}

fn rate_limit(
    limiters: &mut HashMap<RequestRateLimiterKey, Limiter>,
    max_value: usize,
    key: &RequestRateLimiterKey,
    value: usize,
) -> ApiResult<()> {
    let now = UNIX_EPOCH + Duration::from_nanos(time());
    let limiter = match limiters.get_mut(key) {
        Some(limiter) => limiter,
        None => {
            let limiter = Limiter::new(RATE_LIMITER_RESOLUTION, RATE_LIMITER_TIME_WINDOW);
            limiters.insert(*key, limiter);
            limiters.get_mut(key).unwrap()
        }
    };

    limiter.purge_old(now);
    let count = limiter.get_count();
    if count + value > max_value {
        return Err(RequestError::RateLimited.into());
    }

    limiter.add(now, value);

    Ok(())
}

async fn rate_limit_create_request(ctx: &CallContext, msg_arg_data_size: usize) -> ApiResult<()> {
    let user_id = ctx.user().map(|u| u.id);

    let request_rate_limiter_key = RequestRateLimiterKey { user_id };
    REQUEST_COUNT_RATE_LIMITER.with(|l| {
        rate_limit(
            &mut l.borrow_mut(),
            REQUEST_RATE_LIMITER_MAX_COUNT,
            &request_rate_limiter_key,
            1,
        )
    })?;
    REQUEST_SIZE_RATE_LIMITER.with(|l| {
        rate_limit(
            &mut l.borrow_mut(),
            REQUEST_RATE_LIMITER_MAX_SIZE,
            &request_rate_limiter_key,
            msg_arg_data_size,
        )
    })
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
    #[with_middleware(tail = use_canister_call_metric("create_request", &result))]
    async fn create_request(
        &self,
        input: CreateRequestInput,
        msg_arg_data_size: usize,
    ) -> ApiResult<CreateRequestResponse> {
        let ctx = &call_context();

        // rate-limiting
        rate_limit_create_request(ctx, msg_arg_data_size).await?;

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
    #[with_middleware(tail = use_canister_call_metric("submit_request_approval", &result))]
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

    // No authorization middleware as the caller is checked to be the station canister.
    async fn try_execute_request(&self, id: UUID) -> Result<(), RequestExecuteError> {
        let ctx = call_context();
        if !ctx.caller_is_self() {
            trap("The method `try_execute_request` can only be called by the station canister.");
        }
        self.request_service.try_execute_request(id).await
    }
}
