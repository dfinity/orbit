//! Bank services.
use crate::{
    core::{CallContext, WithCallContext},
    services::AccountService,
    transport::{AccountBankDTO, GetMainBankResponse, ListBanksResponse},
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::query;

#[query(name = "list_banks")]
async fn list_banks() -> ApiResult<ListBanksResponse> {
    let account = AccountService::with_call_context(CallContext::get())
        .get_account_by_identity(&CallContext::get().caller())?;

    Ok(ListBanksResponse {
        banks: account
            .banks
            .into_iter()
            .map(AccountBankDTO::from)
            .collect(),
    })
}

#[query(name = "get_main_bank")]
async fn get_main_bank() -> ApiResult<GetMainBankResponse> {
    let main_bank = AccountService::with_call_context(CallContext::get()).get_main_bank()?;

    Ok(GetMainBankResponse {
        bank: main_bank.map(AccountBankDTO::from),
    })
}
