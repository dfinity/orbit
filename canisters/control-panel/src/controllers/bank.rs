//! Bank services.
use crate::{
    core::{ApiResult, CallContext},
    services::AccountService,
    transport::{GetMainBankResponse, ListBanksResponse},
};
use candid::candid_method;
use ic_cdk_macros::query;

#[candid_method(query)]
#[query(name = "list_banks")]
async fn list_banks() -> ApiResult<ListBanksResponse> {
    let ctx = CallContext::get();
    let account_service = AccountService::default();
    let account_details = account_service.get_account_details(&ctx.caller()).await?;

    Ok(ListBanksResponse {
        banks: account_details.banks,
    })
}

#[candid_method(query)]
#[query(name = "get_main_bank")]
async fn get_main_bank() -> ApiResult<GetMainBankResponse> {
    let ctx = CallContext::get();
    let account_service = AccountService::default();
    let main_bank = account_service.get_account_main_bank(&ctx.caller()).await?;

    Ok(GetMainBankResponse { bank: main_bank })
}
