//! Canister lifecycle hooks.
use crate::{core::ic_cdk::api::trap, services::CANISTER_SERVICE};
use control_panel_api::CanisterInstall;
use ic_cdk_macros::{init, post_upgrade};

#[init]
async fn initialize(install: Option<CanisterInstall>) {
    if let Some(CanisterInstall::Init(input)) = install {
        return CANISTER_SERVICE
            .init_canister(input)
            .await
            .expect("failed to initialize canister");
    }

    trap("wrong install mode for canister");
}

#[post_upgrade]
async fn post_upgrade(install: Option<CanisterInstall>) {
    match install {
        Some(CanisterInstall::Upgrade(input)) => CANISTER_SERVICE
            .upgrade_canister(input.to_owned())
            .await
            .expect("failed to upgrade canister"),
        Some(_) => trap("wrong install mode for canister"),
        None => {}
    }
}
