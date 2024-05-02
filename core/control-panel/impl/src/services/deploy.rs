use super::UserService;
use crate::{
    core::{canister_config, CallContext, INITIAL_STATION_CYCLES},
    errors::{DeployError, UserError},
    models::CanDeployStation,
    services::USER_SERVICE,
};
use candid::{Encode, Principal};
use control_panel_api::DeployStationInput;
use ic_cdk::api::id as self_canister_id;
use ic_cdk::api::management_canister::main::{self as mgmt};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
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

    /// Deploys a station canister for the user.
    pub async fn deploy_station(
        &self,
        input: DeployStationInput,
        ctx: &CallContext,
    ) -> ServiceResult<Principal> {
        let user = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;

        let config = canister_config();
        let station_wasm_module = config.station_wasm_module;
        let upgrader_wasm_module = config.upgrader_wasm_module;

        let can_deploy_station_response = user.can_deploy_station();
        match can_deploy_station_response {
            CanDeployStation::Allowed(_) => {}
            CanDeployStation::QuotaExceeded => {
                return Err(UserError::DeployStationQuotaExceeded)?;
            }
            CanDeployStation::NotAllowed(subscription_status) => {
                return Err(UserError::BadUserSubscriptionStatus {
                    subscription_status: subscription_status.into(),
                })?;
            }
        }

        // Creates the station canister with some initial cycles
        let (station_canister,) = mgmt::create_canister(
            mgmt::CreateCanisterArgument { settings: None },
            INITIAL_STATION_CYCLES,
        )
        .await
        .map_err(|(_, err)| DeployError::Failed {
            reason: err.to_string(),
        })?;

        // Adds the station canister as a controller of itself so that it can change its own settings
        mgmt::update_settings(mgmt::UpdateSettingsArgument {
            canister_id: station_canister.canister_id,
            settings: mgmt::CanisterSettings {
                controllers: Some(vec![self_canister_id(), station_canister.canister_id]),
                ..Default::default()
            },
        })
        .await
        .map_err(|(_, err)| DeployError::Failed {
            reason: err.to_string(),
        })?;

        // installs the station canister with the associated upgrader wasm module
        mgmt::install_code(mgmt::InstallCodeArgument {
            mode: mgmt::CanisterInstallMode::Install,
            canister_id: station_canister.canister_id,
            wasm_module: station_wasm_module,
            arg: Encode!(&station_api::SystemInstall::Init(station_api::SystemInit {
                name: input.station_name.clone(),
                admins: vec![station_api::AdminInitInput {
                    identity: user.identity,
                    name: input.admin_name.clone(),
                }],
                upgrader_wasm_module,
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
            .add_deployed_station(
                &user.id,
                station_canister.canister_id,
                input.station_name,
                ctx,
            )
            .await?;

        if user.main_station.is_none() {
            self.user_service
                .set_main_station(&user.id, station_canister.canister_id, ctx)
                .await?;
        }

        Ok(station_canister.canister_id)
    }
}
