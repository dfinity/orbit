use crate::mappers::HelperMapper;
use crate::models::access_control::{AccountActionSpecifier, ResourceSpecifier, ResourceType};
use crate::{
    core::middlewares::{authorize, call_context},
    services::AccountService,
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use wallet_api::{
    FetchAccountBalancesInput, FetchAccountBalancesResponse, GetAccountInput, GetAccountResponse,
    ListAccountResponse,
};

// Canister entrypoints for the controller.
#[query(name = "get_account")]
async fn get_account(input: GetAccountInput) -> ApiResult<GetAccountResponse> {
    CONTROLLER.get_account(input).await
}

#[query(name = "list_accounts")]
async fn list_accounts() -> ApiResult<ListAccountResponse> {
    CONTROLLER.list_accounts().await
}

#[update(name = "fetch_account_balances")]
async fn fetch_account_balances(
    input: FetchAccountBalancesInput,
) -> ApiResult<FetchAccountBalancesResponse> {
    CONTROLLER.fetch_account_balances(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: AccountController = AccountController::new(AccountService::default());
}

#[derive(Debug)]
pub struct AccountController {
    account_service: AccountService,
}

impl AccountController {
    pub fn new(account_service: AccountService) -> Self {
        Self { account_service }
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn get_account(&self, input: GetAccountInput) -> ApiResult<GetAccountResponse> {
        let account = self
            .account_service
            .get_account(HelperMapper::to_uuid(input.account_id)?.as_bytes())?
            .to_dto();

        Ok(GetAccountResponse { account })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::Common(ResourceType::Account, AccountActionSpecifier::List)],
        is_async = true
    )]
    async fn list_accounts(&self) -> ApiResult<ListAccountResponse> {
        let ctx = call_context();
        let owner_identity = ctx.caller();

        let accounts = self
            .account_service
            .list_accounts(owner_identity, &ctx)?
            .iter()
            .map(|account| account.to_dto())
            .collect();

        Ok(ListAccountResponse { accounts })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn fetch_account_balances(
        &self,
        input: FetchAccountBalancesInput,
    ) -> ApiResult<FetchAccountBalancesResponse> {
        let balances = self.account_service.fetch_account_balances(input).await?;

        Ok(FetchAccountBalancesResponse { balances })
    }
}
