use crate::{
    core::middlewares::{authorize, call_context},
    models::resource::{
        ExternalCanisterResourceAction, ReadExternalCanisterResourceTarget, Resource,
    },
    services::{ExternalCanisterService, EXTERNAL_CANISTER_SERVICE},
};
use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterStatusResponse};
use ic_cdk_macros::update;
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use std::sync::Arc;

// Canister entrypoints for the controller.
#[update(name = "canister_status")]
async fn canister_status(input: CanisterIdRecord) -> ApiResult<CanisterStatusResponse> {
    CONTROLLER.canister_status(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: StatusController =
        StatusController::new(Arc::clone(&EXTERNAL_CANISTER_SERVICE));
}

#[derive(Debug, Default)]
pub struct StatusController {
    status_service: Arc<ExternalCanisterService>,
}

impl StatusController {
    fn new(status_service: Arc<ExternalCanisterService>) -> Self {
        Self { status_service }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::Read(ReadExternalCanisterResourceTarget::Canister(input.canister_id)))]))]
    async fn canister_status(&self, input: CanisterIdRecord) -> ApiResult<CanisterStatusResponse> {
        self.status_service.canister_status(input).await
    }
}
