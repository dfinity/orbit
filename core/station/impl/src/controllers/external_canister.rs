use crate::{
    core::middlewares::{authorize, call_context},
    models::resource::{
        ExternalCanisterResourceAction, ReadExternalCanisterResourceTarget, Resource,
    },
    services::{ExternalCanisterService, EXTERNAL_CANISTER_SERVICE},
};
use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterStatusResponse};
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{
    GetExternalCanisterFiltersInput, GetExternalCanisterFiltersResponse, GetExternalCanisterInput,
    GetExternalCanisterResponse, ListExternalCanistersInput, ListExternalCanistersResponse,
};
use std::sync::Arc;

// Canister entrypoints for the controller.
#[update(name = "canister_status")]
async fn canister_status(input: CanisterIdRecord) -> ApiResult<CanisterStatusResponse> {
    CONTROLLER.canister_status(input).await
}

#[query(name = "get_external_canister")]
async fn get_external_canister(
    input: GetExternalCanisterInput,
) -> ApiResult<GetExternalCanisterResponse> {
    CONTROLLER.get_external_canister(input).await
}

#[query(name = "list_external_canisters")]
async fn list_external_canisters(
    input: ListExternalCanistersInput,
) -> ApiResult<ListExternalCanistersResponse> {
    CONTROLLER.list_external_canisters(input).await
}

#[query(name = "get_external_canister_filters")]
async fn get_external_canister_filters(
    input: GetExternalCanisterFiltersInput,
) -> ApiResult<GetExternalCanisterFiltersResponse> {
    CONTROLLER.get_external_canister_filters(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: ExternalCanisterController =
        ExternalCanisterController::new(Arc::clone(&EXTERNAL_CANISTER_SERVICE));
}

#[derive(Debug, Default)]
pub struct ExternalCanisterController {
    canister_service: Arc<ExternalCanisterService>,
}

impl ExternalCanisterController {
    fn new(canister_service: Arc<ExternalCanisterService>) -> Self {
        Self { canister_service }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::Read(ReadExternalCanisterResourceTarget::Canister(input.canister_id)))]))]
    async fn canister_status(&self, input: CanisterIdRecord) -> ApiResult<CanisterStatusResponse> {
        self.canister_service.canister_status(input).await
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::Read(ReadExternalCanisterResourceTarget::Canister(input.canister_id)))]))]
    async fn get_external_canister(
        &self,
        input: GetExternalCanisterInput,
    ) -> ApiResult<GetExternalCanisterResponse> {
        unimplemented!("get_external_canister")
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::List)]))]
    async fn list_external_canisters(
        &self,
        _input: ListExternalCanistersInput,
    ) -> ApiResult<ListExternalCanistersResponse> {
        unimplemented!("list_external_canisters")
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::List)]))]
    async fn get_external_canister_filters(
        &self,
        _input: GetExternalCanisterFiltersInput,
    ) -> ApiResult<GetExternalCanisterFiltersResponse> {
        unimplemented!("get_external_canister_filters")
    }
}
