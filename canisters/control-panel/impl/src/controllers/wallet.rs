//! Wallet services.
use crate::core::metrics::COUNTER_DEPLOY_WALLET_TOTAL;
use crate::core::middlewares::{call_context, logger};
use crate::services::{DeployService, DEPLOY_SERVICE, USER_SERVICE};
use crate::{core::CallContext, errors::UserError, services::UserService};
use candid::Principal;
use control_panel_api::{
    DeployWalletResponse, GetMainWalletResponse, ListWalletsResponse, UserWalletDTO,
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use prometheus::labels;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::Arc;

// The following code implementing canister locks is taken from
// https://internetcomputer.org/docs/current/developer-docs/security/rust-canister-development-security-best-practices#recommendation-10

pub struct State {
    pending_requests: BTreeSet<Principal>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State{pending_requests: BTreeSet::new()});
}

pub struct CallerGuard {
    principal: Principal,
}

impl CallerGuard {
    pub fn new(principal: Principal) -> ApiResult<Self> {
        STATE.with(|state| {
            let pending_requests = &mut state.borrow_mut().pending_requests;
            if pending_requests.contains(&principal) {
                return Err(UserError::ConcurrentWalletDeployment)?;
            }
            pending_requests.insert(principal);
            Ok(Self { principal })
        })
    }
}

impl Drop for CallerGuard {
    fn drop(&mut self) {
        STATE.with(|state| {
            state.borrow_mut().pending_requests.remove(&self.principal);
        })
    }
}

// end of canister locking code

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
    let caller = ic_cdk::caller();
    let _ = CallerGuard::new(caller)?;

    let out = CONTROLLER.deploy_wallet().await;

    COUNTER_DEPLOY_WALLET_TOTAL.with(|c| {
        c.borrow()
            .with(&labels! {
                "status" => match &out {
                    Ok(_) => "ok",
                    Err(_) => "fail",
                }
            })
            .inc()
    });

    out
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
    async fn deploy_wallet(&self) -> ApiResult<DeployWalletResponse> {
        let ctx = CallContext::get();
        let deployed_wallet_id = self.deploy_service.deploy_wallet(&ctx).await?;

        Ok(DeployWalletResponse {
            canister_id: deployed_wallet_id,
        })
    }
}
