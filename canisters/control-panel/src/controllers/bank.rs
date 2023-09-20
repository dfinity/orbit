//! Bank services.
use crate::{
    core::ApiResult,
    transport::{GetMainBankResponse, ListBanksResponse, ManageBanksInput, ManageBanksResponse},
};
use candid::candid_method;
use ic_cdk_macros::{query, update};

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

#[candid_method(update)]
#[update(name = "manage_banks")]
async fn manage_banks(_input: ManageBanksInput) -> ApiResult<ManageBanksResponse> {
    unimplemented!()
}
