//! Canister lifecycle hooks.
use super::AVAILABLE_TOKENS_USER_REGISTRATION;
use crate::core::ic_cdk::spawn;
use crate::core::metrics::recompute_all_metrics;
use crate::services::CANISTER_SERVICE;
use control_panel_api::UploadCanisterModulesInput;
use ic_cdk_macros::{init, post_upgrade};
use ic_cdk_timers::{set_timer, set_timer_interval};
use orbit_essentials::api::ApiResult;
use orbit_essentials::cdk::update;
use orbit_essentials::http::set_certified_data_for_skip_certification;
use std::time::Duration;

pub const MINUTE: u64 = 60;
pub const HOUR: u64 = 60 * MINUTE;
pub const DAY: u64 = 24 * HOUR;

pub const USER_REGISTRATION_RATE: u32 = 100;
pub const USER_REGISTRATION_LIMIT_PERIOD: Duration = Duration::from_secs(MINUTE);

#[update]
async fn upload_canister_modules(input: UploadCanisterModulesInput) -> ApiResult<()> {
    CANISTER_SERVICE.upload_canister_modules(input).await
}

fn init_timers_fn() {
    async fn initialize_rng_timer() {
        use orbit_essentials::utils::initialize_rng;
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
async fn initialize() {
    set_certified_data_for_skip_certification();
    init_timers_fn();

    CANISTER_SERVICE
        .init_canister()
        .await
        .expect("failed to initialize canister");
}

#[post_upgrade]
async fn post_upgrade() {
    set_certified_data_for_skip_certification();
    recompute_all_metrics();
    init_timers_fn();

    CANISTER_SERVICE
        .init_canister()
        .await
        .expect("failed to upgrade canister");
}
