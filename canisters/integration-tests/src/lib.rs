#![cfg(test)]

use candid::Principal;
use pocket_ic::PocketIc;

mod control_panel_tests;
mod interfaces;
mod register_tests;
mod setup;
mod transfer_tests;
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
    pub upgrader: Principal,
    pub wallet: Principal,
}
