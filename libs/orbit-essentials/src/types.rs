use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WasmModuleExtraChunks {
    pub store_canister: Principal,
    #[serde(deserialize_with = "crate::deserialize::deserialize_vec_blob")]
    pub chunk_hashes_list: Vec<Vec<u8>>,
    #[serde(with = "serde_bytes")]
    pub wasm_module_hash: Vec<u8>,
}

/// A timestamp in nano seconds since epoch.
pub type Timestamp = u64;

/// A UUID that identifies objects within the system.
pub type UUID = [u8; 16];
