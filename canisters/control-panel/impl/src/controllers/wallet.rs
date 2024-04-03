//! Wallet services.
use crate::core::middlewares::{call_context, logger, use_status_metric};
use crate::errors::UserError;
use crate::services::{DeployService, DEPLOY_SERVICE, USER_SERVICE};
use crate::{core::CallContext, services::UserService};
use candid::Principal;
use control_panel_api::{
    CanDeployWalletResponse, DeployWalletResponse, GetMainWalletResponse, ListWalletsResponse,
    UserWalletDTO,
};
use ic_canister_core::api::ApiResult;
use ic_canister_core::utils::{CallerGuard, State};
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

thread_local! {
    static STATE: Rc<RefCell<State<Principal>>> = Rc::new(RefCell::new(State::default()));
}

// Canister entrypoints for the controller.
#[query(name = "list_wallets")]
async fn list_wallets() -> ApiResult<ListWalletsResponse> {
    CONTROLLER.list_wallets().await
}

#[query(name = "get_main_wallet")]
async fn get_main_wallet() -> ApiResult<GetMainWalletResponse> {
    CONTROLLER.get_main_wallet().await
}

#[update(name = "deploy_wallet")]
async fn deploy_wallet() -> ApiResult<DeployWalletResponse> {
    CONTROLLER.deploy_wallet().await
}

#[query(name = "can_deploy_wallet")]
async fn can_deploy_wallet() -> ApiResult<CanDeployWalletResponse> {
    CONTROLLER.can_deploy_wallet().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: WalletController =
        WalletController::new(Arc::clone(&USER_SERVICE), Arc::clone(&DEPLOY_SERVICE));
}

#[derive(Debug)]
pub struct WalletController {
    user_service: Arc<UserService>,
    deploy_service: Arc<DeployService>,
}

impl WalletController {
    fn new(user_service: Arc<UserService>, deploy_service: Arc<DeployService>) -> Self {
        Self {
            user_service,
            deploy_service,
        }
    }

    /// Returns list of wallets associated with the user.
    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    async fn list_wallets(&self) -> ApiResult<ListWalletsResponse> {
        let ctx = CallContext::get();
        let user = self
            .user_service
            .get_user(&CallContext::get().caller(), &ctx)?;

        Ok(ListWalletsResponse {
            wallets: user.wallets.into_iter().map(UserWalletDTO::from).collect(),
        })
    }
    /// Returns main wallet associated with the user if any.
    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    async fn get_main_wallet(&self) -> ApiResult<GetMainWalletResponse> {
        let ctx = CallContext::get();
        let main_wallet = self.user_service.get_main_wallet(&ctx)?;

        Ok(GetMainWalletResponse {
            wallet: main_wallet.map(UserWalletDTO::from),
        })
    }

    /// Deploys a new wallet for the user and returns its id.
    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    #[with_middleware(tail = use_status_metric("deploy_wallet", &result))]
    async fn deploy_wallet(&self) -> ApiResult<DeployWalletResponse> {
        let ctx = CallContext::get();
        let _lock = STATE
            .with(|state| CallerGuard::new(state.clone(), ctx.caller()))
            .ok_or(UserError::ConcurrentWalletDeployment)?;

        let deployed_wallet_id = self.deploy_service.deploy_wallet(&ctx).await?;

        Ok(DeployWalletResponse {
            canister_id: deployed_wallet_id,
        })
    }

    /// Checks if the user can deploy a new wallet.
    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    #[with_middleware(tail = use_status_metric("can_deploy_wallet", &result))]
    async fn can_deploy_wallet(&self) -> ApiResult<CanDeployWalletResponse> {
        let ctx = CallContext::get();
        self.user_service.can_deploy_wallet(&ctx).await
    }
}
