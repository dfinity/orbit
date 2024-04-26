#![cfg(test)]

use candid::Principal;
use pocket_ic::PocketIc;

mod address_book;
mod change_canister_tests;
mod control_panel_tests;
mod cycles_monitor_tests;
mod interfaces;
mod register_tests;
mod setup;
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
    pub control_panel: Principal,
    pub station: Principal,
}
