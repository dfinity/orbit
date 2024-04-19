//! Canister lifecycle hooks.
use super::AVAILABLE_TOKENS_USER_REGISTRATION;
use crate::core::ic_cdk::spawn;
use crate::{core::ic_cdk::api::trap, services::CANISTER_SERVICE};
use control_panel_api::CanisterInstall;
use ic_cdk_macros::{init, post_upgrade};
use ic_cdk_timers::{set_timer, set_timer_interval};
use std::time::Duration;

pub const MINUTE: u64 = 60;
pub const HOUR: u64 = 60 * MINUTE;
pub const DAY: u64 = 24 * HOUR;

pub const USER_REGISTRATION_RATE: u32 = 100;
pub const USER_REGISTRATION_LIMIT_PERIOD: Duration = Duration::from_secs(MINUTE);

fn init_timers_fn() {
    async fn initialize_rng_timer() {
        use ic_canister_core::utils::initialize_rng;
        if let Err(e) = initialize_rng().await {
            ic_cdk::print(format!("initializing rng failed: {}", e));
            ic_cdk_timers::set_timer(std::time::Duration::from_secs(60), move || {
                spawn(initialize_rng_timer())
            });
        }
    }

    set_timer(std::time::Duration::from_millis(0), move || {
        spawn(initialize_rng_timer())
    });

    set_timer_interval(
        USER_REGISTRATION_LIMIT_PERIOD / USER_REGISTRATION_RATE,
        || {
            AVAILABLE_TOKENS_USER_REGISTRATION.with(|ts| {
                let mut ts = ts.borrow_mut();

                if *ts < USER_REGISTRATION_RATE {
                    *ts += 1;
                }
            });
        },
    );
}

#[init]
async fn initialize(install: Option<CanisterInstall>) {
    init_timers_fn();

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
    init_timers_fn();

    match install {
        Some(CanisterInstall::Upgrade(input)) => CANISTER_SERVICE
            .upgrade_canister(input)
            .await
            .expect("failed to upgrade canister"),
        Some(_) => trap("wrong install mode for canister"),
        None => {}
    }
}
