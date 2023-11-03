//! Bank services.
use crate::{
    core::{CallContext, WithCallContext},
    services::UserService,
    transport::{GetMainBankResponse, ListBanksResponse, UserBankDTO},
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::query;

#[query(name = "list_banks")]
async fn list_banks() -> ApiResult<ListBanksResponse> {
    let user = UserService::with_call_context(CallContext::get())
        .get_user_by_identity(&CallContext::get().caller())?;

    Ok(ListBanksResponse {
        banks: user.banks.into_iter().map(UserBankDTO::from).collect(),
    })
}

#[query(name = "get_main_bank")]
async fn get_main_bank() -> ApiResult<GetMainBankResponse> {
    let main_bank = UserService::with_call_context(CallContext::get()).get_main_bank()?;

    Ok(GetMainBankResponse {
        bank: main_bank.map(UserBankDTO::from),
    })
}
