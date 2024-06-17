use crate::{
    errors::RegistryError,
    mappers::RegistryMapper,
    models::{RegistryEntry, RegistryEntryId, RegistryValue, WasmModuleRegistryValue, LATEST_TAG},
    repositories::{RegistryRepository, RegistryWhere, REGISTRY_REPOSITORY},
    services::{ArtifactService, ARTIFACT_SERVICE},
};
use control_panel_api::{
    RegistryEntryInput, RegistryEntryUpdateInput, SearchRegistryFilterKindDTO, SearchRegistryInput,
};
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ServiceResult,
    model::ModelValidator,
    pagination::{paginated_items, PaginatedData, PaginatedItemsArgs},
    repository::Repository,
};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref REGISTRY_SERVICE: Arc<RegistryService> = Arc::new(RegistryService::new(
        Arc::clone(&REGISTRY_REPOSITORY),
        Arc::clone(&ARTIFACT_SERVICE)
    ));
}

/// The registry service provides methods to manage registry entries.
#[derive(Default, Debug)]
pub struct RegistryService {
    registry_repository: Arc<RegistryRepository>,
    artifact_service: Arc<ArtifactService>,
}

impl RegistryService {
    pub const DEFAULT_SEARCH_LIMIT: u16 = 10;
    pub const MAX_SEARCH_LIMIT: u16 = 100;

    pub fn new(
        registry_repository: Arc<RegistryRepository>,
        artifact_service: Arc<ArtifactService>,
    ) -> Self {
        Self {
            registry_repository,
            artifact_service,
        }
    }

    /// Returns the registry entry by id.
    pub fn get(&self, registry_id: &RegistryEntryId) -> ServiceResult<RegistryEntry> {
        let entry =
            self.registry_repository
                .get(registry_id)
                .ok_or_else(|| RegistryError::NotFound {
                    id: Uuid::from_bytes(*registry_id).to_string(),
                })?;

        Ok(entry)
    }

    /// Returns all registry entries by the given name, if the name is not namespaced, the default namespace is used.
    pub fn search(
        &self,
        search: SearchRegistryInput,
    ) -> ServiceResult<PaginatedData<RegistryEntry>> {
        let mut where_clause = RegistryWhere::clause();

        search
            .filter_by
            .into_iter()
            .for_each(|filter| match filter {
                SearchRegistryFilterKindDTO::Namespace(namespace) => {
                    where_clause = where_clause.to_owned().and_namespace(&namespace);
                }
                SearchRegistryFilterKindDTO::Name(name) => {
                    let fullname = match name.starts_with(RegistryEntry::NAMESPACE_PREFIX) {
                        true => name,
                        false => format!(
                            "{}{}/{}",
                            RegistryEntry::NAMESPACE_PREFIX,
                            RegistryEntry::DEFAULT_NAMESPACE,
                            name
                        ),
                    };

                    where_clause = where_clause.to_owned().and_fullname(&fullname);
                }
                SearchRegistryFilterKindDTO::Kind(kind) => {
                    where_clause = where_clause.to_owned().and_kind(kind.into());
                }
            });

        let entry_ids = self.registry_repository.find_ids_where(where_clause, None);
        let paginated_ids = paginated_items(PaginatedItemsArgs {
            offset: search.pagination.to_owned().and_then(|p| p.offset),
            limit: search.pagination.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_SEARCH_LIMIT),
            max_limit: Some(Self::MAX_SEARCH_LIMIT),
            items: &entry_ids,
        })?;

        Ok(PaginatedData {
            total: paginated_ids.total,
            next_offset: paginated_ids.next_offset,
            items: paginated_ids
                .items
                .into_iter()
                .flat_map(|id| match self.get(&id) {
                    Ok(entry) => Some(entry),
                    Err(_) => None,
                })
                .collect::<Vec<RegistryEntry>>(),
        })
    }

    /// Creates a new registry entry and returns it.
    pub fn create(&self, input: RegistryEntryInput) -> ServiceResult<RegistryEntry> {
        let mut entry = RegistryEntry::new();

        RegistryMapper::fill_from_create_input(&mut entry, &input);

        match &input.value {
            control_panel_api::RegistryEntryValueInput::WasmModule(module) => {
                let artifact_id = self.artifact_service.create(module.wasm_module.clone())?;

                entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
                    wasm_artifact_id: artifact_id,
                    version: module.version.clone(),
                    dependencies: module
                        .dependencies
                        .iter()
                        .map(|dep| dep.clone().into())
                        .collect(),
                });
            }
        };

        // There can only be one latest entry, so we need to remove the latest tag from the previous entry if it exists.
        if entry.tags.contains(&LATEST_TAG.to_string()) {
            let previous_latest = self.registry_repository.find_ids_where(
                RegistryWhere::clause()
                    .and_fullname(&entry.fullname())
                    .and_tags(vec![LATEST_TAG.to_string()]),
                None,
            );

            previous_latest
                .iter()
                .filter_map(|id| self.get(id).ok())
                .for_each(|mut e| {
                    e.tags.retain(|t| t != LATEST_TAG);
                    self.registry_repository.insert(e.id, e);
                });
        }

        entry.validate()?;

        self.registry_repository.insert(entry.id, entry.clone());

        Ok(entry)
    }

    /// Updates the registry entry and returns it.
    pub fn edit(
        &self,
        registry_id: &RegistryEntryId,
        input: RegistryEntryUpdateInput,
    ) -> ServiceResult<RegistryEntry> {
        let mut entry = self.get(registry_id)?;

        RegistryMapper::fill_from_update_input(&mut entry, &input);

        match (&input.value, &entry.value) {
            (
                Some(control_panel_api::RegistryEntryValueInput::WasmModule(module)),
                RegistryValue::WasmModule(current_module),
            ) => {
                self.artifact_service
                    .remove_by_id(&current_module.wasm_artifact_id)?;

                let artifact_id = self.artifact_service.create(module.wasm_module.clone())?;

                entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
                    wasm_artifact_id: artifact_id,
                    version: module.version.clone(),
                    dependencies: module
                        .dependencies
                        .iter()
                        .map(|dep| dep.clone().into())
                        .collect(),
                });
            }
            (None, _) => (),
        };

        entry.validate()?;

        self.registry_repository.insert(entry.id, entry.clone());

        Ok(entry)
    }

    /// Deletes the registry entry by id.
    pub fn delete(&self, registry_id: &RegistryEntryId) -> ServiceResult<RegistryEntry> {
        let registry = self.get(registry_id)?;

        match &registry.value {
            RegistryValue::WasmModule(wasm_module) => {
                self.artifact_service
                    .remove_by_id(&wasm_module.wasm_artifact_id)?;
            }
        };

        self.registry_repository.remove(registry_id);

        Ok(registry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        registry_entry_test_utils::create_registry_entry, WasmModuleRegistryValue,
    };
    use control_panel_api::RegistryEntryValueKindDTO;

    #[test]
    fn test_search_by_name_and_kind_is_none() {
        let search = SearchRegistryInput {
            filter_by: vec![
                SearchRegistryFilterKindDTO::Name("test".to_string()),
                SearchRegistryFilterKindDTO::Kind(RegistryEntryValueKindDTO::WasmModule),
            ],
            pagination: None,
        };

        let result = REGISTRY_SERVICE.search(search).unwrap();

        assert_eq!(result.total, 0);
        assert_eq!(result.items.len(), 0);
    }

    #[test]
    fn test_search_by_name_and_kind_is_some() {
        for i in 0..10 {
            let mut entry = create_registry_entry();
            entry.namespace = "orbit".to_string();
            entry.name = format!("module-{}", i);

            REGISTRY_REPOSITORY.insert(entry.id, entry.clone());
        }

        let search = SearchRegistryInput {
            filter_by: vec![
                SearchRegistryFilterKindDTO::Name("@orbit/module-2".to_string()),
                SearchRegistryFilterKindDTO::Kind(RegistryEntryValueKindDTO::WasmModule),
            ],
            pagination: None,
        };

        let result = REGISTRY_SERVICE.search(search).unwrap();

        assert_eq!(result.total, 1);
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].namespace, "orbit");
        assert_eq!(result.items[0].name, "module-2");
    }

    #[test]
    fn test_search_by_namespace_and_kind_is_none() {
        let search = SearchRegistryInput {
            filter_by: vec![
                SearchRegistryFilterKindDTO::Namespace("test".to_string()),
                SearchRegistryFilterKindDTO::Kind(RegistryEntryValueKindDTO::WasmModule),
            ],
            pagination: None,
        };

        let result = REGISTRY_SERVICE.search(search).unwrap();

        assert_eq!(result.total, 0);
        assert_eq!(result.items.len(), 0);
    }

    #[test]
    fn test_search_by_namespace_and_kind_is_some() {
        for i in 0..10 {
            let mut entry = create_registry_entry();
            entry.namespace = "test".to_string();
            entry.name = format!("module-{}", i);

            REGISTRY_REPOSITORY.insert(entry.id, entry.clone());
        }

        let search = SearchRegistryInput {
            filter_by: vec![
                SearchRegistryFilterKindDTO::Namespace("test".to_string()),
                SearchRegistryFilterKindDTO::Kind(RegistryEntryValueKindDTO::WasmModule),
            ],
            pagination: None,
        };

        let result = REGISTRY_SERVICE.search(search).unwrap();

        assert_eq!(result.total, 10);
        assert_eq!(result.items.len(), 10);
    }

    #[test]
    fn test_find_many_entries_with_same_name() {
        for i in 0..10 {
            let mut entry = create_registry_entry();
            entry.namespace = "test".to_string();
            entry.name = "module".to_string();
            entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
                wasm_artifact_id: *Uuid::new_v4().as_bytes(),
                version: format!("1.0.{}", i),
                dependencies: Vec::new(),
            });

            REGISTRY_REPOSITORY.insert(entry.id, entry.clone());
        }

        let search = SearchRegistryInput {
            filter_by: vec![
                SearchRegistryFilterKindDTO::Name("@test/module".to_string()),
                SearchRegistryFilterKindDTO::Kind(RegistryEntryValueKindDTO::WasmModule),
            ],
            pagination: None,
        };

        let result = REGISTRY_SERVICE.search(search).unwrap();

        assert_eq!(result.total, 10);
        assert_eq!(result.items.len(), 10);
    }

    #[test]
    fn test_duplicate_entry() {
        let mut entry = create_registry_entry();
        entry.name = "module".to_string();
        entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
        });

        REGISTRY_REPOSITORY.insert(entry.id, entry.clone());

        let input = RegistryEntryInput {
            name: entry.fullname(),
            description: "This is a test description for the module.".to_string(),
            tags: Vec::new(),
            categories: Vec::new(),
            metadata: Default::default(),
            value: control_panel_api::RegistryEntryValueInput::WasmModule(
                control_panel_api::WasmModuleRegistryEntryValueInput {
                    version: "1.0.0".to_string(),
                    wasm_module: [0, 1, 3].to_vec(),
                    dependencies: Vec::new(),
                },
            ),
        };

        let result = REGISTRY_SERVICE.create(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_latest_tag() {
        let mut entry = create_registry_entry();
        entry.name = "module".to_string();
        entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
        });
        entry.tags = vec![LATEST_TAG.to_string()];

        REGISTRY_REPOSITORY.insert(entry.id, entry.clone());

        let input = RegistryEntryInput {
            name: entry.fullname(),
            description: "This is a demo description for the module.".to_string(),
            tags: vec![LATEST_TAG.to_string()],
            categories: Vec::new(),
            metadata: Default::default(),
            value: control_panel_api::RegistryEntryValueInput::WasmModule(
                control_panel_api::WasmModuleRegistryEntryValueInput {
                    version: "1.0.1".to_string(),
                    wasm_module: [0, 1, 3].to_vec(),
                    dependencies: Vec::new(),
                },
            ),
        };

        let result = REGISTRY_SERVICE.create(input).unwrap();

        assert_eq!(result.tags, vec![LATEST_TAG.to_string()]);

        let latest = REGISTRY_REPOSITORY.find_ids_where(
            RegistryWhere::clause()
                .and_fullname(&entry.fullname())
                .and_tags(vec![LATEST_TAG.to_string()]),
            None,
        );

        assert_eq!(latest.len(), 1);
        assert_eq!(latest[0], result.id);
    }

    #[test]
    fn test_correctly_edits_entry() {
        let create_input = RegistryEntryInput {
            name: "module".to_string(),
            description: "This is a test description for the module.".to_string(),
            tags: Vec::new(),
            categories: Vec::new(),
            metadata: Default::default(),
            value: control_panel_api::RegistryEntryValueInput::WasmModule(
                control_panel_api::WasmModuleRegistryEntryValueInput {
                    version: "1.0.0".to_string(),
                    wasm_module: [0, 1].to_vec(),
                    dependencies: Vec::new(),
                },
            ),
        };

        let entry = REGISTRY_SERVICE.create(create_input).unwrap();

        let input = RegistryEntryUpdateInput {
            description: Some("This is a test description for the module.".to_string()),
            tags: Some(vec!["tag".to_string()]),
            categories: Some(vec!["category".to_string()]),
            metadata: Some(Default::default()),
            value: Some(control_panel_api::RegistryEntryValueInput::WasmModule(
                control_panel_api::WasmModuleRegistryEntryValueInput {
                    version: "1.0.1".to_string(),
                    wasm_module: [0, 1, 3].to_vec(),
                    dependencies: Vec::new(),
                },
            )),
        };

        let result = REGISTRY_SERVICE.edit(&entry.id, input).unwrap();

        assert_eq!(
            result.description,
            "This is a test description for the module."
        );
        assert_eq!(result.tags, vec!["tag".to_string()]);
        assert_eq!(result.categories, vec!["category".to_string()]);
        assert_eq!(result.metadata, Default::default());

        match result.value {
            RegistryValue::WasmModule(wasm_module) => {
                assert_eq!(wasm_module.version, "1.0.1");
                assert_eq!(wasm_module.dependencies.len(), 0);
            }
        }
    }
}
