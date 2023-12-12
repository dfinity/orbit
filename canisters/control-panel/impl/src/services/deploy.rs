use super::UserService;
use crate::{core::CallContext, errors::UserError, services::USER_SERVICE};
use candid::Principal;
use control_panel_api::{ManageUserInput, UserWalletDTO};
use ic_canister_core::api::ServiceResult;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref DEPLOY_SERVICE: Arc<DeployService> =
        Arc::new(DeployService::new(Arc::clone(&USER_SERVICE)));
}

#[derive(Default, Debug)]
pub struct DeployService {
    user_service: Arc<UserService>,
}

impl DeployService {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    /// Deploys a wallet canister for the user.
    pub async fn deploy_wallet(&self, ctx: &CallContext) -> ServiceResult<Principal> {
        let user = self.user_service.get_user(&ctx.caller(), ctx)?;
        // TODO: Add logic that deploys a wallet canister from the available wasm module in the canister config.
        let deployed_wallet = Principal::anonymous();

        if !user.wallets.is_empty() {
            return Err(UserError::DeployWalletQuotaExceeded)?;
        }

        self.user_service
            .manage_user(
                ManageUserInput {
                    main_wallet: Some(deployed_wallet),
                    wallets: Some(vec![UserWalletDTO {
                        canister_id: deployed_wallet,
                        name: None,
                    }]),
                },
                ctx,
            )
            .await?;

        Ok(deployed_wallet)
    }
}
