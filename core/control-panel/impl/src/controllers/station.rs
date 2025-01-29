//! Station services.
use crate::core::middlewares::use_canister_call_metric;
use crate::errors::UserError;
use crate::mappers::user_station::UpdateUserStationInputInto;
use crate::services::{
    DeployService, UserStationService, DEPLOY_SERVICE, USER_SERVICE, USER_STATION_SERVICE,
};
use crate::{core::CallContext, services::UserService};
use candid::Principal;
use control_panel_api::{
    CanDeployStationResponse, DeployStationInput, DeployStationResponse, ListUserStationsInput,
    ListUserStationsResponse, ManageUserStationsInput, UserStationDTO,
};
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::utils::{CallerGuard, State};
use orbit_essentials::with_middleware;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

thread_local! {
    static STATE: Rc<RefCell<State<Principal>>> = Rc::new(RefCell::new(State::default()));
}

// Canister entrypoints for the controller.
#[query(name = "list_user_stations")]
async fn list_user_stations(input: ListUserStationsInput) -> ApiResult<ListUserStationsResponse> {
    CONTROLLER.list_user_stations(input).await
}

#[update(name = "manage_user_stations")]
async fn manage_user_stations(input: ManageUserStationsInput) -> ApiResult<()> {
    CONTROLLER.manage_user_stations(input).await
}

#[update(name = "deploy_station")]
async fn deploy_station(input: DeployStationInput) -> ApiResult<DeployStationResponse> {
    CONTROLLER.deploy_station(input).await
}

#[query(name = "can_deploy_station")]
async fn can_deploy_station() -> ApiResult<CanDeployStationResponse> {
    CONTROLLER.can_deploy_station().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: StationController = StationController::new(
        Arc::clone(&USER_SERVICE),
        Arc::clone(&USER_STATION_SERVICE),
        Arc::clone(&DEPLOY_SERVICE)
    );
}

#[derive(Debug)]
pub struct StationController {
    user_service: Arc<UserService>,
    user_station_service: Arc<UserStationService>,
    deploy_service: Arc<DeployService>,
}

impl StationController {
    fn new(
        user_service: Arc<UserService>,
        user_station_service: Arc<UserStationService>,
        deploy_service: Arc<DeployService>,
    ) -> Self {
        Self {
            user_service,
            user_station_service,
            deploy_service,
        }
    }

    /// Returns list of stations associated with the user.
    async fn list_user_stations(
        &self,
        input: ListUserStationsInput,
    ) -> ApiResult<ListUserStationsResponse> {
        let ctx = CallContext::get();
        let user: crate::models::User = ctx.user()?;
        let stations = self.user_station_service.list_stations(
            &user.id,
            &input.filter_by_labels.unwrap_or_default(),
            &ctx,
        )?;

        Ok(ListUserStationsResponse {
            stations: stations.into_iter().map(UserStationDTO::from).collect(),
        })
    }

    /// Manages the stations associated with the user.
    #[with_middleware(tail = use_canister_call_metric("manage_user_stations", &result))]
    async fn manage_user_stations(&self, input: ManageUserStationsInput) -> ApiResult<()> {
        let ctx = CallContext::get();
        let user = ctx.user()?;

        match input {
            ManageUserStationsInput::Add(stations) => {
                self.user_station_service.add_stations(
                    &user.id,
                    stations.into_iter().map(Into::into).collect(),
                    &ctx,
                )?;
            }
            ManageUserStationsInput::Remove(canister_ids) => {
                self.user_station_service
                    .remove_stations(&user.id, canister_ids, &ctx)?;
            }
            ManageUserStationsInput::Update(stations) => {
                self.user_station_service.update_stations(
                    &user.id,
                    stations
                        .into_iter()
                        .map(|e| e.into_user_station())
                        .collect::<Vec<_>>(),
                    &ctx,
                )?;
            }
        }

        Ok(())
    }

    /// Deploys a new station for the user and returns its id.
    #[with_middleware(tail = use_canister_call_metric("deploy_station", &result))]
    async fn deploy_station(&self, input: DeployStationInput) -> ApiResult<DeployStationResponse> {
        let ctx = CallContext::get();
        let _lock = STATE
            .with(|state| CallerGuard::new(state.clone(), ctx.caller(), None))
            .ok_or(UserError::ConcurrentStationDeployment)?;

        let deployed_station_id = self.deploy_service.deploy_station(input, &ctx).await?;

        Ok(DeployStationResponse {
            canister_id: deployed_station_id,
        })
    }

    /// Checks if the user can deploy a new station.
    async fn can_deploy_station(&self) -> ApiResult<CanDeployStationResponse> {
        let ctx = CallContext::get();
        self.user_service
            .can_deploy_station(&ctx)
            .map(|can_deploy_station| can_deploy_station.into())
    }
}
