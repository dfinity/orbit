use candid::{CandidType, Deserialize};
use orbit_essentials::types::WasmModuleExtraChunks;

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UploadCanisterModulesInput {
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub upgrader_wasm_module: Option<Vec<u8>>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub station_wasm_module: Option<Vec<u8>>,
    pub station_wasm_module_extra_chunks: Option<Option<WasmModuleExtraChunks>>,
}
