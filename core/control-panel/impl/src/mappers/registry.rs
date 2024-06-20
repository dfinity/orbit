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
        // If the name starts with the namespace prefix and contains a slash, we assume that the
        // namespace is part of the name and extract it.
        //
        // The validation of the format is done at a later point when entry.validate() is called before saving.
        let mut name = input.name.clone();
        let mut namespace = RegistryEntry::DEFAULT_NAMESPACE.to_string();
        if name.starts_with(RegistryEntry::NAMESPACE_PREFIX) && name.contains('/') {
            let cloned_name = name.clone();
            let name_without_prefix =
                cloned_name.trim_start_matches(RegistryEntry::NAMESPACE_PREFIX);
            let mut parts = name_without_prefix.split('/');

            if let Some(found_namespace) = parts.next() {
                if let Some(found_name) = parts.next() {
                    name = found_name.to_string();
                    namespace = found_namespace.to_string();
                }
            }
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::registry_entry_test_utils::create_registry_entry;

    #[test]
    fn test_create_from_input_finds_namespace() {
        let mut entry = create_registry_entry();
        let input = control_panel_api::RegistryEntryInput {
            name: "@ns1/test".to_string(),
            description: "desc".to_string(),
            tags: vec![],
            categories: vec![],
            metadata: Default::default(),
            value: control_panel_api::RegistryEntryValueInput::WasmModule(
                control_panel_api::WasmModuleRegistryEntryValueInput {
                    version: "1.0.0".to_string(),
                    dependencies: vec![],
                    wasm_module: vec![],
                },
            ),
        };

        RegistryMapper::fill_from_create_input(&mut entry, &input);

        assert_eq!(entry.namespace, "ns1");
        assert_eq!(entry.name, "test");
    }

    #[test]
    fn test_create_from_input_uses_default_namespace_when_none_provided() {
        let mut entry = create_registry_entry();
        let input = control_panel_api::RegistryEntryInput {
            name: "test".to_string(),
            description: "desc".to_string(),
            tags: vec![],
            categories: vec![],
            metadata: Default::default(),
            value: control_panel_api::RegistryEntryValueInput::WasmModule(
                control_panel_api::WasmModuleRegistryEntryValueInput {
                    version: "1.0.0".to_string(),
                    dependencies: vec![],
                    wasm_module: vec![],
                },
            ),
        };

        RegistryMapper::fill_from_create_input(&mut entry, &input);

        assert_eq!(entry.namespace, RegistryEntry::DEFAULT_NAMESPACE);
        assert_eq!(entry.name, "test");
    }
}
