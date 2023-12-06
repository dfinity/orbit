use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(Clone, CandidType, Deserialize)]
pub enum UpgradeTarget {
    Wallet,
    Upgrader,
}

#[stable_object]
#[derive(Clone, CandidType, Deserialize)]
pub struct Upgrade {
    pub id: UUID,
    pub target: UpgradeTarget,
    pub module: Vec<u8>,
    pub checksum: Vec<u8>,
}
