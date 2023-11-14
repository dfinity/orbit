use ic_canister_core::api::ApiResult;
use ic_cdk_macros::update;

use crate::{
    core::{CallContext, WithCallContext, PERMISSION_WRITE_UPGRADE},
    services::UpgradeService,
    transport::{UpgradeInput, UpgradeResponse},
};

#[update(name = "upgrade")]
async fn upgrade(input: UpgradeInput) -> ApiResult<UpgradeResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_UPGRADE);

    let upgrade = UpgradeService::with_call_context(CallContext::get())
        .create_upgrade(input)
        .await?;

    Ok(UpgradeResponse {
        upgrade: upgrade.to_dto(),
    })
}
