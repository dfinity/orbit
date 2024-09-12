use crate::AssetDTO;
use candid::{CandidType, Deserialize};

/// The capabilities of the canister.
#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct CapabilitiesDTO {
    /// The name of the canister.
    pub name: String,
    /// The current version of the canister.
    pub version: String,
    /// The list of assets that are supported by the canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub supported_assets: Vec<AssetDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct CapabilitiesResponse {
    pub capabilities: CapabilitiesDTO,
}
