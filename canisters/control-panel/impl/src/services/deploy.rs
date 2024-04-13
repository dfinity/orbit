use super::UserService;
use crate::{
    core::{canister_config, CallContext, INITIAL_WALLET_CYCLES},
    errors::{DeployError, UserError},
    models::CanDeployWallet,
    services::USER_SERVICE,
};
use candid::{Encode, Principal};
use ic_canister_core::api::ServiceResult;
use ic_cdk::api::id as self_canister_id;
use ic_cdk::api::management_canister::main::{self as mgmt};
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

        let can_deploy_wallet_response = user.can_deploy_wallet();
        match can_deploy_wallet_response {
            CanDeployWallet::Allowed(_) => {}
            CanDeployWallet::QuotaExceeded => {
                return Err(UserError::DeployWalletQuotaExceeded)?;
            }
            CanDeployWallet::NotAllowed(subscription_status) => {
                return Err(UserError::BadUserSubscriptionStatus {
                    subscription_status: subscription_status.into(),
                })?;
            }
        }

        // Creates the wallet canister with some initial cycles
        let (wallet_canister,) = mgmt::create_canister(
            mgmt::CreateCanisterArgument { settings: None },
            INITIAL_WALLET_CYCLES,
        )
        .await
        .map_err(|(_, err)| DeployError::Failed {
            reason: err.to_string(),
        })?;

        // Adds the wallet canister as a controller of itself so that it can change its own settings
        mgmt::update_settings(mgmt::UpdateSettingsArgument {
            canister_id: wallet_canister.canister_id,
            settings: mgmt::CanisterSettings {
                controllers: Some(vec![self_canister_id(), wallet_canister.canister_id]),
                ..Default::default()
            },
        })
        .await
        .map_err(|(_, err)| DeployError::Failed {
            reason: err.to_string(),
        })?;

        // installs the wallet canister with the associated upgrader wasm module
        let config = canister_config();
        mgmt::install_code(mgmt::InstallCodeArgument {
            mode: mgmt::CanisterInstallMode::Install,
            canister_id: wallet_canister.canister_id,
            wasm_module: config.wallet_wasm_module,
            arg: Encode!(&wallet_api::SystemInstall::Init(wallet_api::SystemInit {
                admins: Some(vec![user.identity]),
                upgrader_wasm_module: config.upgrader_wasm_module,
            }))
            .map_err(|err| DeployError::Failed {
                reason: err.to_string(),
            })?,
        })
        .await
        .map_err(|(_, err)| DeployError::Failed {
            reason: err.to_string(),
        })?;

        self.user_service
            .add_deployed_wallet(wallet_canister.canister_id, ctx)
            .await?;

        if user.main_wallet.is_none() {
            self.user_service
                .set_main_wallet(wallet_canister.canister_id, ctx)
                .await?;
        }

        Ok(wallet_canister.canister_id)
    }
}
