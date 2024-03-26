use crate::mappers::authorization::FetchAccountBalancesInputRef;
use crate::mappers::HelperMapper;
use crate::models::resource::{AccountResourceAction, Resource};
use crate::{
    core::middlewares::{authorize, call_context},
    services::AccountService,
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use wallet_api::{
    AccountCallerPrivilegesDTO, FetchAccountBalancesInput, FetchAccountBalancesResponse,
    GetAccountInput, GetAccountResponse, ListAccountsInput, ListAccountsResponse,
};

// Canister entrypoints for the controller.
#[query(name = "get_account")]
async fn get_account(input: GetAccountInput) -> ApiResult<GetAccountResponse> {
    CONTROLLER.get_account(input).await
}

#[query(name = "list_accounts")]
async fn list_accounts(input: ListAccountsInput) -> ApiResult<ListAccountsResponse> {
    CONTROLLER.list_accounts(input).await
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

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    async fn get_account(&self, input: GetAccountInput) -> ApiResult<GetAccountResponse> {
        let ctx = call_context();
        let account = self
            .account_service
            .get_account(HelperMapper::to_uuid(input.account_id)?.as_bytes())?;

        let privileges = self
            .account_service
            .get_caller_privileges_for_account(&account.id, &ctx)
            .await?;

        Ok(GetAccountResponse {
            account: account.to_dto(),
            privileges: privileges.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Account(AccountResourceAction::List)]))]
    async fn list_accounts(&self, input: ListAccountsInput) -> ApiResult<ListAccountsResponse> {
        let ctx = call_context();
        let result = self
            .account_service
            .list_accounts(input, Some(&ctx))
            .await?;

        let mut privileges = Vec::new();
        for account in &result.items {
            let account_privileges = self
                .account_service
                .get_caller_privileges_for_account(&account.id, &ctx)
                .await?;

            privileges.push(AccountCallerPrivilegesDTO::from(account_privileges));
        }

        Ok(ListAccountsResponse {
            accounts: result
                .items
                .into_iter()
                .map(|account| account.to_dto())
                .collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &FetchAccountBalancesInputRef(&input).to_resources()))]
    async fn fetch_account_balances(
        &self,
        input: FetchAccountBalancesInput,
    ) -> ApiResult<FetchAccountBalancesResponse> {
        let balances = self.account_service.fetch_account_balances(input).await?;

        Ok(FetchAccountBalancesResponse { balances })
    }
}
