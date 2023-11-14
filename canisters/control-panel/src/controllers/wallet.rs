//! Wallet services.
use crate::core::middlewares::{call_context, log_call, log_call_result};
use crate::{
    core::CallContext,
    services::UserService,
    transport::{GetMainWalletResponse, ListWalletsResponse, UserWalletDTO},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;

// Canister entrypoints for the controller.
#[query(name = "list_wallets")]
async fn list_wallets() -> ApiResult<ListWalletsResponse> {
    CONTROLLER.list_wallets().await
}

#[query(name = "get_main_wallet")]
async fn get_main_wallet() -> ApiResult<GetMainWalletResponse> {
    CONTROLLER.get_main_wallet().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: WalletController = WalletController::new(UserService::default());
}

#[derive(Debug)]
pub struct WalletController {
    user_service: UserService,
}

impl WalletController {
    fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    /// Returns list of wallets associated with the user.
    #[with_middleware(guard = "log_call", when = "before", context = "call_context")]
    #[with_middleware(guard = "log_call_result", when = "after", context = "call_context")]
    async fn list_wallets(&self) -> ApiResult<ListWalletsResponse> {
        let ctx = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&CallContext::get().caller(), &ctx)?;

        Ok(ListWalletsResponse {
            wallets: user.wallets.into_iter().map(UserWalletDTO::from).collect(),
        })
    }
    /// Returns main wallet associated with the user if any.
    #[with_middleware(guard = "log_call", when = "before", context = "call_context")]
    #[with_middleware(guard = "log_call_result", when = "after", context = "call_context")]
    async fn get_main_wallet(&self) -> ApiResult<GetMainWalletResponse> {
        let ctx = CallContext::get();
        let main_wallet = self.user_service.get_main_wallet(&ctx)?;

        Ok(GetMainWalletResponse {
            wallet: main_wallet.map(UserWalletDTO::from),
        })
    }
}
