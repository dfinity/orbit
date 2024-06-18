use crate::{MetadataDTO, PaginationInput, TimestampRfc3339, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmModuleRegistryEntryValueDTO {
    pub wasm_artifact_id: UuidDTO,
    pub version: String,
    pub dependencies: Vec<UuidDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmModuleRegistryEntryValueInput {
    #[serde(with = "serde_bytes")]
    pub wasm_module: Vec<u8>,
    pub version: String,
    pub dependencies: Vec<UuidDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum RegistryEntryValueDTO {
    WasmModule(WasmModuleRegistryEntryValueDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum RegistryEntryValueInput {
    WasmModule(WasmModuleRegistryEntryValueInput),
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegistryEntryDTO {
    pub id: UuidDTO,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub metadata: Vec<MetadataDTO>,
    pub value: RegistryEntryValueDTO,
    pub created_at: TimestampRfc3339,
    pub updated_at: Option<TimestampRfc3339>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegistryEntryInput {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub metadata: Vec<MetadataDTO>,
    pub value: RegistryEntryValueInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetRegistryEntryInput {
    pub id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetRegistryEntryResponse {
    pub entry: RegistryEntryDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum SearchRegistryFilterKindDTO {
    Name(String),
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SearchRegistryInput {
    pub filter_by: Vec<SearchRegistryFilterKindDTO>,
    pub pagination: Option<PaginationInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SearchRegistryResponse {
    pub entries: Vec<RegistryEntryDTO>,
    pub total: u64,
    pub next_offset: Option<u64>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AddRegistryEntryInput {
    pub entry: RegistryEntryInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AddRegistryEntryResponse {
    pub entry: RegistryEntryDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EditRegistryEntryInput {
    pub id: UuidDTO,
    pub entry: RegistryEntryInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EditRegistryEntryResponse {
    pub entry: RegistryEntryDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeleteRegistryEntryInput {
    pub id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeleteRegistryEntryResponse {
    pub entry: RegistryEntryDTO,
}
