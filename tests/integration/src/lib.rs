#![cfg(test)]

use candid::Principal;
use pocket_ic::PocketIc;

mod address_book_tests;
mod asset_tests;
mod control_panel_tests;
mod cycles_monitor_tests;
mod dfx_orbit;
mod disaster_recovery_tests;
mod external_canister_tests;
mod http;
mod interfaces;
mod migration_tests;
mod notification;
mod register_tests;
mod setup;
mod test_data;
mod transfer_tests;
mod upgrade_station_tests;
mod utils;

pub struct TestEnv {
    pub env: PocketIc,
    pub canister_ids: CanisterIds,
    pub controller: Principal,
    pub minter: Principal,
}

#[derive(Debug)]
pub struct CanisterIds {
    pub icp_ledger: Principal,
    pub icp_index: Principal,
    pub cycles_minting_canister: Principal,
    pub control_panel: Principal,
    pub station: Principal,
}
