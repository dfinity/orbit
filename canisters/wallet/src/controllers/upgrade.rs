use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::update;
use lazy_static::lazy_static;

use crate::{
    core::{
        middlewares::{authorize, call_context},
        PERMISSION_WRITE_UPGRADE,
    },
    services::UpgradeService,
    transport::{UpgradeInput, UpgradeResponse},
};

#[update(name = "upgrade")]
async fn upgrade(input: UpgradeInput) -> ApiResult<UpgradeResponse> {
    CONTROLLER.upgrade(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: UpgradeController = UpgradeController::new(UpgradeService::default());
}

#[derive(Debug)]
pub struct UpgradeController {
    upgrade_service: UpgradeService,
}

impl UpgradeController {
    fn new(upgrade_service: UpgradeService) -> Self {
        Self { upgrade_service }
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_WRITE_UPGRADE])]
    async fn upgrade(&self, input: UpgradeInput) -> ApiResult<UpgradeResponse> {
        let upgrade = self
            .upgrade_service
            .create_upgrade(input, &call_context())
            .await?;

        Ok(UpgradeResponse {
            upgrade: upgrade.to_dto(),
        })
    }
}
