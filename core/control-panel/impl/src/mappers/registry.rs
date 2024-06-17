use super::HelperMapper;
use crate::models::{
    RegistryEntry, RegistryValue, RegistryValueKind, WasmModuleRegistryEntryDependency,
    WasmModuleRegistryValue,
};
use orbit_essentials::utils::timestamp_to_rfc3339;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RegistryMapper {}

impl RegistryMapper {
    pub fn fill_from_create_input(
        entry: &mut RegistryEntry,
        input: &control_panel_api::RegistryEntryInput,
    ) {
        let (namespace, name) = RegistryEntry::parse_namespace_and_name(&input.name);

        entry.namespace = namespace;
        entry.name = name;
        entry.description = input.description.to_string();
        entry.tags = input.tags.clone();
        entry.categories = input.categories.clone();
        entry.metadata = HelperMapper::from_metadata(input.metadata.clone());

        match &input.value {
            control_panel_api::RegistryEntryValueInput::WasmModule(wasm_module) => {
                entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
                    // a placeholder uuid that will be replaced later
                    wasm_artifact_id: [0; 16],
                    version: wasm_module.version.clone(),
                    dependencies: wasm_module
                        .dependencies
                        .iter()
                        .map(|dep| dep.clone().into())
                        .collect(),
                });
            }
        }
    }

    pub fn fill_from_update_input(
        entry: &mut RegistryEntry,
        input: &control_panel_api::RegistryEntryUpdateInput,
    ) {
        if let Some(description) = &input.description {
            entry.description = description.to_string();
        }

        if let Some(tags) = &input.tags {
            entry.tags = tags.clone();
        }

        if let Some(categories) = &input.categories {
            entry.categories = categories.clone();
        }

        if let Some(metadata) = &input.metadata {
            entry.metadata = HelperMapper::from_metadata(metadata.clone());
        }
    }
}

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
            dependencies: value.dependencies.into_iter().map(Into::into).collect(),
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
            name: entry.fullname(),
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

impl From<control_panel_api::RegistryEntryValueKindDTO> for RegistryValueKind {
    fn from(kind: control_panel_api::RegistryEntryValueKindDTO) -> Self {
        match kind {
            control_panel_api::RegistryEntryValueKindDTO::WasmModule => {
                RegistryValueKind::WasmModule
            }
        }
    }
}

impl From<control_panel_api::RegistryEntryValueInput> for RegistryValueKind {
    fn from(input: control_panel_api::RegistryEntryValueInput) -> Self {
        match input {
            control_panel_api::RegistryEntryValueInput::WasmModule(_) => {
                RegistryValueKind::WasmModule
            }
        }
    }
}

impl From<control_panel_api::WasmModuleRegistryEntryDependencyDTO>
    for WasmModuleRegistryEntryDependency
{
    fn from(dependency: control_panel_api::WasmModuleRegistryEntryDependencyDTO) -> Self {
        Self {
            name: dependency.name,
            version: dependency.version,
        }
    }
}

impl From<WasmModuleRegistryEntryDependency>
    for control_panel_api::WasmModuleRegistryEntryDependencyDTO
{
    fn from(value: WasmModuleRegistryEntryDependency) -> Self {
        Self {
            name: value.name,
            version: value.version,
        }
    }
}
