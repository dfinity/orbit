use super::{UserService, UserStationService};
use crate::{
    core::{
        canister_config, CallContext, CMC_CANISTER_ID, INITIAL_STATION_CYCLES, NNS_ROOT_CANISTER_ID,
    },
    errors::{DeployError, UserError},
    models::{CanDeployStation, UserStation},
    services::{USER_SERVICE, USER_STATION_SERVICE},
};
use candid::{Encode, Principal};
use control_panel_api::{DeployStationInput, SubnetSelection};
use ic_cdk::api::id as self_canister_id;
use ic_cdk::api::management_canister::main::{self as mgmt};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::cdk::api::call::call_with_payment128;
use orbit_essentials::cdk::api::management_canister::main::CanisterSettings;
use orbit_essentials::install_chunked_code::install_chunked_code;
use std::sync::Arc;

/// Argument taken by `create_canister` endpoint of the CMC.
#[derive(candid::CandidType, serde::Serialize)]
pub struct CreateCanister {
    pub subnet_selection: Option<SubnetSelection>,
    pub settings: Option<CanisterSettings>,
}

/// Error for create_canister endpoint
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
pub enum CreateCanisterError {
    Refunded {
        refund_amount: u128,
        create_error: String,
    },
}

lazy_static! {
    pub static ref DEPLOY_SERVICE: Arc<DeployService> = Arc::new(DeployService::new(
        Arc::clone(&USER_SERVICE),
        Arc::clone(&USER_STATION_SERVICE)
    ));
}

#[derive(Default, Debug)]
pub struct DeployService {
    user_service: Arc<UserService>,
    user_station_service: Arc<UserStationService>,
}

impl DeployService {
    pub fn new(
        user_service: Arc<UserService>,
        user_station_service: Arc<UserStationService>,
    ) -> Self {
        Self {
            user_service,
            user_station_service,
        }
    }

    /// Deploys a station canister for the user.
    pub async fn deploy_station(
        &self,
        input: DeployStationInput,
        ctx: &CallContext,
    ) -> ServiceResult<Principal> {
        let user = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;
        let config = canister_config().ok_or(DeployError::Failed {
            reason: "Canister config not initialized.".to_string(),
        })?;
        let upgrader_wasm_module = config.upgrader_wasm_module;
        let station_wasm_module = config.station_wasm_module;
        let station_wasm_module_extra_chunks = config.station_wasm_module_extra_chunks;

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
        let station_canister = if let Some(subnet_selection) = input.subnet_selection {
            let create_canister = CreateCanister {
                subnet_selection: Some(subnet_selection),
                settings: None,
            };
            call_with_payment128::<_, (Result<Principal, CreateCanisterError>,)>(
                CMC_CANISTER_ID,
                "create_canister",
                (create_canister,),
                INITIAL_STATION_CYCLES,
            )
            .await
            .map(|res| res.0)
            .map_err(|(_, err)| DeployError::Failed {
                reason: err.to_string(),
            })?
            .map_err(
                |CreateCanisterError::Refunded { create_error, .. }| DeployError::Failed {
                    reason: create_error,
                },
            )?
        } else {
            mgmt::create_canister(
                mgmt::CreateCanisterArgument { settings: None },
                INITIAL_STATION_CYCLES,
            )
            .await
            .map(|res| res.0.canister_id)
            .map_err(|(_, err)| DeployError::Failed {
                reason: err.to_string(),
            })?
        };

        // Adds the station canister as a controller of itself so that it can change its own settings
        mgmt::update_settings(mgmt::UpdateSettingsArgument {
            canister_id: station_canister,
            settings: mgmt::CanisterSettings {
                controllers: Some(vec![self_canister_id(), station_canister]),
                ..Default::default()
            },
        })
        .await
        .map_err(|(_, err)| DeployError::Failed {
            reason: err.to_string(),
        })?;

        // The initial admins added to the station.
        let admins = input
            .admins
            .iter()
            .map(|admin| station_api::AdminInitInput {
                identity: admin.identity,
                name: admin.username.clone(),
            })
            .collect::<Vec<_>>();

        // installs the station canister with the associated upgrader wasm module
        let station_install_arg =
            Encode!(&station_api::SystemInstall::Init(station_api::SystemInit {
                name: input.name.clone(),
                admins,
                upgrader: station_api::SystemUpgraderInput::WasmModule(upgrader_wasm_module),
                quorum: Some(1),
                fallback_controller: Some(NNS_ROOT_CANISTER_ID),
                accounts: None,
            }))
            .map_err(|err| DeployError::Failed {
                reason: err.to_string(),
            })?;
        install_chunked_code(
            station_canister,
            mgmt::CanisterInstallMode::Install,
            station_wasm_module,
            station_wasm_module_extra_chunks,
            station_install_arg,
        )
        .await
        .map_err(|err| DeployError::Failed { reason: err })?;

        self.user_service
            .add_deployed_station(&user.id, station_canister, ctx)
            .await?;

        // Adds the deployed station to the user
        if let Some(info) = input.associate_with_caller {
            self.user_station_service.add_stations(
                &user.id,
                vec![UserStation {
                    canister_id: station_canister,
                    name: input.name,
                    labels: info.labels,
                }],
                ctx,
            )?;
        }

        Ok(station_canister)
    }
}
