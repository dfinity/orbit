use crate::{MetadataDTO, PaginationInput, SortDirection, TimestampRfc3339, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone, Eq, PartialEq)]
pub struct WasmModuleRegistryEntryDependencyDTO {
    pub name: String,
    pub version: String,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmModuleRegistryEntryValueDTO {
    pub wasm_artifact_id: UuidDTO,
    pub version: String,
    pub dependencies: Vec<WasmModuleRegistryEntryDependencyDTO>,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmModuleRegistryEntryValueInput {
    #[serde(with = "serde_bytes")]
    pub wasm_module: Vec<u8>,
    pub version: String,
    pub dependencies: Vec<WasmModuleRegistryEntryDependencyDTO>,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub enum RegistryEntryValueDTO {
    WasmModule(WasmModuleRegistryEntryValueDTO),
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub enum RegistryEntryValueInput {
    WasmModule(WasmModuleRegistryEntryValueInput),
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub enum RegistryEntryValueKindDTO {
    WasmModule,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
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

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct RegistryEntryInput {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub metadata: Vec<MetadataDTO>,
    pub value: RegistryEntryValueInput,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct RegistryEntryUpdateInput {
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub metadata: Option<Vec<MetadataDTO>>,
    pub value: Option<RegistryEntryValueInput>,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct GetRegistryEntryInput {
    pub id: UuidDTO,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct GetRegistryEntryResponse {
    pub entry: RegistryEntryDTO,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub enum SearchRegistryFilterKindDTO {
    Namespace(String),
    Name(String),
    Kind(RegistryEntryValueKindDTO),
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct SearchRegistryInput {
    pub filter_by: Vec<SearchRegistryFilterKindDTO>,
    pub sort_by: Option<RegistryEntrySortBy>,
    pub pagination: Option<PaginationInput>,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct SearchRegistryResponse {
    pub entries: Vec<RegistryEntryDTO>,
    pub total: u64,
    pub next_offset: Option<u64>,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct AddRegistryEntryInput {
    pub entry: RegistryEntryInput,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct AddRegistryEntryResponse {
    pub entry: RegistryEntryDTO,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct EditRegistryEntryInput {
    pub id: UuidDTO,
    pub entry: RegistryEntryUpdateInput,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct EditRegistryEntryResponse {
    pub entry: RegistryEntryDTO,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct DeleteRegistryEntryInput {
    pub id: UuidDTO,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct DeleteRegistryEntryResponse {
    pub entry: RegistryEntryDTO,
}

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone, Eq, PartialEq)]
pub enum RegistryEntrySortBy {
    CreatedAt(SortDirection),
    Version(SortDirection),
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct NextWasmModuleVersionInput {
    pub name: String,
    pub current_version: String,
}

#[derive(CandidType, Deserialize, serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct NextWasmModuleVersionResponse {
    pub entry: Option<RegistryEntryDTO>,
}
