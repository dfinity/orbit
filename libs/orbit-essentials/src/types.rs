use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WasmModuleExtraChunks {
    pub store_canister: Principal,
    pub extra_chunks_key: String,
    #[serde(with = "serde_bytes")]
    pub wasm_module_hash: Vec<u8>,
}

/// A timestamp in nano seconds since epoch.
pub type Timestamp = u64;

/// A UUID that identifies objects within the system.
pub type UUID = [u8; 16];
