//! # Control panel canister
//!
//! The control panel canister is responsible for providing helper functions for the Orbit Wallet user interface.
pub mod types;

use ic_cdk_macros::{query, update};
use types::{
    AccountInfoResult, ManageAccountInput, ManageAccountResult, RegisterAccountInput,
    RegisterAccountResult,
};

#[update(name = "register_account")]
async fn register_account(input: RegisterAccountInput) -> RegisterAccountResult {
    println!("input name = {:?}", input.name);
    println!("input main_bank = {:?}", input.main_bank);
    unimplemented!()
}

#[update(name = "manage_account")]
async fn manage_account(input: ManageAccountInput) -> ManageAccountResult {
    println!("input name = {:?}", input.name);
    println!("input identities = {:?}", input.identities);
    println!("input use_shared_bank = {:?}", input.use_shared_bank);
    println!("input bank = {:?}", input.bank);
    unimplemented!()
}

#[query(name = "account_info")]
async fn account_info() -> AccountInfoResult {
    println!("account info called");
    unimplemented!()
}
