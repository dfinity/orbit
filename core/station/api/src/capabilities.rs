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
    /// The list of blockchains and standards that are supported by the canister (e.g. `ethereum`, `bitcoin`, `icp`, etc.)
    pub supported_blockchains: Vec<SupportedBlockchainDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct CapabilitiesResponse {
    pub capabilities: CapabilitiesDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SupportedBlockchainDTO {
    pub blockchain: String,
    pub supported_standards: Vec<StandardDataDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct StandardDataDTO {
    pub standard: String,
    pub required_metadata_fields: Vec<String>,
    pub supported_operations: Vec<String>,
}
