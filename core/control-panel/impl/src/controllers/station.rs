//! Station services.
use crate::core::middlewares::{call_context, logger, use_canister_call_metric};
use crate::errors::UserError;
use crate::services::{DeployService, DEPLOY_SERVICE, USER_SERVICE};
use crate::{core::CallContext, services::UserService};
use candid::Principal;
use control_panel_api::{
    CanDeployStationResponse, DeployStationInput, DeployStationResponse, GetMainStationResponse,
    ListStationsResponse, UserStationDTO,
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
#[query(name = "list_stations")]
async fn list_stations() -> ApiResult<ListStationsResponse> {
    CONTROLLER.list_stations().await
}

#[query(name = "get_main_station")]
async fn get_main_station() -> ApiResult<GetMainStationResponse> {
    CONTROLLER.get_main_station().await
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
    static ref CONTROLLER: StationController =
        StationController::new(Arc::clone(&USER_SERVICE), Arc::clone(&DEPLOY_SERVICE));
}

#[derive(Debug)]
pub struct StationController {
    user_service: Arc<UserService>,
    deploy_service: Arc<DeployService>,
}

impl StationController {
    fn new(user_service: Arc<UserService>, deploy_service: Arc<DeployService>) -> Self {
        Self {
            user_service,
            deploy_service,
        }
    }

    /// Returns list of stations associated with the user.
    async fn list_stations(&self) -> ApiResult<ListStationsResponse> {
        let ctx = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&CallContext::get().caller(), &ctx)?;

        Ok(ListStationsResponse {
            stations: user
                .stations
                .into_iter()
                .map(UserStationDTO::from)
                .collect(),
        })
    }
    /// Returns main station associated with the user if any.
    async fn get_main_station(&self) -> ApiResult<GetMainStationResponse> {
        let ctx = CallContext::get();
        let main_station = self.user_service.get_main_station(&ctx)?;

        Ok(GetMainStationResponse {
            station: main_station.map(UserStationDTO::from),
        })
    }

    /// Deploys a new station for the user and returns its id.
    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    #[with_middleware(tail = use_canister_call_metric("deploy_station", &result))]
    async fn deploy_station(&self, input: DeployStationInput) -> ApiResult<DeployStationResponse> {
        let ctx = CallContext::get();
        let _lock = STATE
            .with(|state| CallerGuard::new(state.clone(), ctx.caller()))
            .ok_or(UserError::ConcurrentStationDeployment)?;

        let deployed_station_id = self.deploy_service.deploy_station(input, &ctx).await?;

        Ok(DeployStationResponse {
            canister_id: deployed_station_id,
        })
    }

    /// Checks if the user can deploy a new station.
    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    async fn can_deploy_station(&self) -> ApiResult<CanDeployStationResponse> {
        let ctx = CallContext::get();
        self.user_service
            .can_deploy_station(&ctx)
            .await
            .map(|can_deploy_station| can_deploy_station.into())
    }
}
