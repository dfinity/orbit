//! Bank services.
use crate::{
    core::ic_cdk::api::print,
    core::CallContext,
    services::UserService,
    transport::{GetMainBankResponse, ListBanksResponse, UserBankDTO},
};
use async_trait::async_trait;
use ic_canister_core::api::{ApiResult, WithLogs};
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
    static ref CONTROLLER: Box<dyn BankController> = {
        let u = BankControllerImpl::new(UserService::new());
        let u = WithLogs(u);
        Box::new(u)
    };
}

#[async_trait]
pub trait BankController: Sync + Send {
    /// Returns list of banks associated with the user.
    async fn list_banks(&self) -> ApiResult<ListBanksResponse>;
    /// Returns main bank associated with the user if any.
    async fn get_main_bank(&self) -> ApiResult<GetMainBankResponse>;
}

#[derive(Debug)]
pub struct BankControllerImpl {
    user_service: UserService,
}

impl BankControllerImpl {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }
}

#[async_trait]
impl BankController for BankControllerImpl {
    async fn list_banks(&self) -> ApiResult<ListBanksResponse> {
        let ctx = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&CallContext::get().caller(), &ctx)?;

        Ok(ListBanksResponse {
            banks: user.banks.into_iter().map(UserBankDTO::from).collect(),
        })
    }

    async fn get_main_bank(&self) -> ApiResult<GetMainBankResponse> {
        let ctx = CallContext::get();
        let main_bank = self.user_service.get_main_bank(&ctx)?;

        Ok(GetMainBankResponse {
            bank: main_bank.map(UserBankDTO::from),
        })
    }
}

#[async_trait]
impl<T: BankController> BankController for WithLogs<T> {
    async fn list_banks(&self) -> ApiResult<ListBanksResponse> {
        let out = self.0.list_banks().await;

        print(format!("list_banks: {:?}", out));

        out
    }

    async fn get_main_bank(&self) -> ApiResult<GetMainBankResponse> {
        let out = self.0.get_main_bank().await;

        print(format!("main_bank: {:?}", out));

        out
    }
}
