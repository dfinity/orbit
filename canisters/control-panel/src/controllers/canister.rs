//! Canister lifecycle hooks.
use crate::{
    core::{canister_config, write_canister_config, CanisterConfig},
    transport::{CanisterInit, DefaultBankInit},
};
use candid::Principal;
use ic_canister_core::cdk::api::time;
use ic_cdk_macros::{init, post_upgrade};

#[init]
async fn initialize(input: Option<CanisterInit>) {
    let init = input.unwrap_or_default();
    let shared_bank_canister = match init.default_bank {
        // todo: update shared bank canister to the correct one
        DefaultBankInit::InitSharedBankCanister => Principal::anonymous(),
        DefaultBankInit::SpecifiedBankCanister(canister) => canister,
    };
    let config = CanisterConfig::new(shared_bank_canister, time());

    write_canister_config(config);
}

#[post_upgrade]
async fn post_upgrade() {
    let current_config = canister_config();
    let updated_config = CanisterConfig::new(current_config.shared_bank_canister, time());

    write_canister_config(updated_config);
}
