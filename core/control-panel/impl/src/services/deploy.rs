use super::{UserService, UserStationService};
use crate::{
    core::{
        canister_config, CallContext, CANISTER_CREATION_CYCLES, INITIAL_STATION_CYCLES,
        INITIAL_UPGRADER_CYCLES, NNS_ROOT_CANISTER_ID,
    },
    errors::{DeployError, UserError},
    models::{CanDeployStation, UserStation},
    services::{USER_SERVICE, USER_STATION_SERVICE},
};
use candid::{Encode, Principal};
use control_panel_api::DeployStationInput;
use ic_cdk::api::id as self_canister_id;
use ic_cdk::api::management_canister::main::{self as mgmt, CanisterIdRecord};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::cmc::create_canister;
use orbit_essentials::install_chunked_code::install_chunked_code;
use orbit_essentials::utils::check_balance_before_transfer;
use std::sync::Arc;

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

        // Creates the station canister
        let station_canister = create_canister(input.subnet_selection, CANISTER_CREATION_CYCLES)
            .await
            .map_err(|err| DeployError::Failed { reason: err })?;

        // Determine the actual number of cycles for canister creation on the subnet
        // to which the station canister and its upgrader canister are deployed.
        let station = CanisterIdRecord {
            canister_id: station_canister,
        };
        let station_cycles: u128 = mgmt::canister_status(station)
            .await
            .map_err(|(_, err)| DeployError::Failed { reason: err })?
            .0
            .cycles
            .0
            .try_into()
            .unwrap();
        let actual_canister_creation_cycles =
            CANISTER_CREATION_CYCLES.saturating_sub(station_cycles);

        // Top up the station so that it has the target cycles balance after deploying the upgrader canister.
        let upgrader_initial_cycles = actual_canister_creation_cycles + INITIAL_UPGRADER_CYCLES;
        let station_initial_cycles = upgrader_initial_cycles + INITIAL_STATION_CYCLES;
        let extra_cycles = station_initial_cycles.saturating_sub(station_cycles);
        if extra_cycles > 0 {
            check_balance_before_transfer(extra_cycles)
                .await
                .map_err(|err| DeployError::Failed { reason: err })?;
            mgmt::deposit_cycles(station, extra_cycles)
                .await
                .map_err(|(_, err)| DeployError::Failed { reason: err })?;
        }

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
        let intial_users = input
            .admins
            .iter()
            .map(|user| station_api::InitUserInput {
                id: None,
                identities: vec![station_api::UserIdentityInput {
                    identity: user.identity,
                }],
                groups: None,
                status: station_api::UserStatusDTO::Active,
                name: user.username.clone(),
            })
            .collect::<Vec<_>>();

        // installs the station canister with the associated upgrader wasm module
        let station_install_arg =
            Encode!(&station_api::SystemInstall::Init(station_api::SystemInit {
                name: input.name.clone(),
                upgrader: station_api::SystemUpgraderInput::Deploy(
                    station_api::DeploySystemUpgraderInput {
                        wasm_module: upgrader_wasm_module,
                        initial_cycles: Some(upgrader_initial_cycles),
                    }
                ),
                fallback_controller: Some(NNS_ROOT_CANISTER_ID),
                initial_config: station_api::InitialConfig::WithAllDefaults {
                    users: intial_users,
                    admin_quorum: 1,
                    operator_quorum: 1,
                },
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
            .add_deployed_station(&user.id, station_canister, ctx)?;

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
