use super::HelperMapper;
use crate::models::{RegistryEntry, RegistryValue, RegistryValueKind, WasmModuleRegistryValue};
use orbit_essentials::utils::timestamp_to_rfc3339;
use uuid::Uuid;

impl From<&RegistryValue> for RegistryValueKind {
    fn from(value: &RegistryValue) -> Self {
        match value {
            RegistryValue::WasmModule(_) => RegistryValueKind::WasmModule,
        }
    }
}

impl From<WasmModuleRegistryValue> for control_panel_api::WasmModuleRegistryEntryValueDTO {
    fn from(value: WasmModuleRegistryValue) -> Self {
        Self {
            wasm_artifact_id: Uuid::from_bytes(value.wasm_artifact_id).to_string(),
            version: value.version,
            dependencies: value
                .dependencies
                .into_iter()
                .map(|dependency_id| Uuid::from_bytes(dependency_id).to_string())
                .collect(),
        }
    }
}

impl From<RegistryValue> for control_panel_api::RegistryEntryValueDTO {
    fn from(value: RegistryValue) -> Self {
        match value {
            RegistryValue::WasmModule(wasm_module) => {
                control_panel_api::RegistryEntryValueDTO::WasmModule(wasm_module.into())
            }
        }
    }
}

impl From<RegistryEntry> for control_panel_api::RegistryEntryDTO {
    fn from(entry: RegistryEntry) -> Self {
        Self {
            id: Uuid::from_bytes(entry.id).to_string(),
            name: entry.name,
            description: entry.description,
            tags: entry.tags,
            categories: entry.categories,
            metadata: HelperMapper::to_metadata(entry.metadata),
            value: entry.value.into(),
            created_at: timestamp_to_rfc3339(&entry.created_at),
            updated_at: entry.updated_at.map(|ts| timestamp_to_rfc3339(&ts)),
        }
    }
}
