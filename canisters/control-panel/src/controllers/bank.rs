//! Bank services.
use crate::core::middlewares::{call_context, log_call, log_call_result};
use crate::{
    core::CallContext,
    services::UserService,
    transport::{GetMainBankResponse, ListBanksResponse, UserBankDTO},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;

// Canister entrypoints for the controller.
#[query(name = "list_banks")]
async fn list_banks() -> ApiResult<ListBanksResponse> {
    CONTROLLER.list_banks().await
}

#[query(name = "get_main_bank")]
async fn get_main_bank() -> ApiResult<GetMainBankResponse> {
    CONTROLLER.get_main_bank().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: BankController = BankController::new(UserService::new());
}

#[derive(Debug)]
pub struct BankController {
    user_service: UserService,
}

impl BankController {
    fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    /// Returns list of banks associated with the user.
    #[with_middleware(guard = "log_call", when = "before", context = "call_context")]
    #[with_middleware(guard = "log_call_result", when = "after", context = "call_context")]
    async fn list_banks(&self) -> ApiResult<ListBanksResponse> {
        let ctx = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&CallContext::get().caller(), &ctx)?;

        Ok(ListBanksResponse {
            banks: user.banks.into_iter().map(UserBankDTO::from).collect(),
        })
    }
    /// Returns main bank associated with the user if any.
    #[with_middleware(guard = "log_call", when = "before", context = "call_context")]
    #[with_middleware(guard = "log_call_result", when = "after", context = "call_context")]
    async fn get_main_bank(&self) -> ApiResult<GetMainBankResponse> {
        let ctx = CallContext::get();
        let main_bank = self.user_service.get_main_bank(&ctx)?;

        Ok(GetMainBankResponse {
            bank: main_bank.map(UserBankDTO::from),
        })
    }
}
