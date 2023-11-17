#![cfg(test)]

use candid::Principal;
use pocket_ic::PocketIc;

mod env;
mod interfaces;
mod register_tests;
mod setup;
mod utils;

pub struct TestEnv {
    pub env: PocketIc,
    pub canister_ids: CanisterIds,
    pub minter: Principal,
    pub controller: Principal,
}

#[derive(Debug)]
pub struct CanisterIds {
    pub control_panel: Principal,
    pub upgrader: Principal,
    pub wallet: Principal,
    pub icp_ledger: Principal,
    pub icp_index: Principal,
}
