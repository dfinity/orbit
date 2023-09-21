//! Bank services.
use crate::{
    core::ApiResult,
    transport::{GetMainBankResponse, ListBanksResponse},
};
use candid::candid_method;
use ic_cdk_macros::query;

#[candid_method(query)]
#[query(name = "list_banks")]
async fn list_banks() -> ApiResult<ListBanksResponse> {
    unimplemented!()
}

#[candid_method(query)]
#[query(name = "get_main_bank")]
async fn get_main_bank() -> ApiResult<GetMainBankResponse> {
    unimplemented!()
}
