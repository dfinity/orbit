use crate::{
    errors::RegistryError,
    mappers::RegistryMapper,
    models::{RegistryEntry, RegistryEntryId, RegistryValue},
    repositories::{
        ArtifactRepository, RegistryRepository, RegistryWhere, ARTIFACT_REPOSITORY,
        REGISTRY_REPOSITORY,
    },
};
use control_panel_api::{RegistryEntryInput, SearchRegistryFilterKindDTO, SearchRegistryInput};
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ServiceResult,
    pagination::{paginated_items, PaginatedData, PaginatedItemsArgs},
    repository::Repository,
};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref REGISTRY_SERVICE: Arc<RegistryService> = Arc::new(RegistryService::new(
        Arc::clone(&REGISTRY_REPOSITORY),
        Arc::clone(&ARTIFACT_REPOSITORY)
    ));
}

/// The registry service provides methods to manage registry entries.
#[derive(Default, Debug)]
pub struct RegistryService {
    registry_repository: Arc<RegistryRepository>,
    artifact_repository: Arc<ArtifactRepository>,
}

impl RegistryService {
    pub const DEFAULT_SEARCH_LIMIT: u16 = 10;
    pub const MAX_SEARCH_LIMIT: u16 = 100;

    pub fn new(
        registry_repository: Arc<RegistryRepository>,
        artifact_repository: Arc<ArtifactRepository>,
    ) -> Self {
        Self {
            registry_repository,
            artifact_repository,
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
                        false => format!("@{}/{}", RegistryEntry::DEFAULT_NAMESPACE, name),
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

        unimplemented!()
    }

    /// Updates the registry entry and returns it.
    pub fn edit(
        &self,
        _registry_id: &RegistryEntryId,
        _input: RegistryEntryInput,
    ) -> ServiceResult<RegistryEntry> {
        unimplemented!()
    }

    /// Deletes the registry entry by id.
    pub fn delete(&self, registry_id: &RegistryEntryId) -> ServiceResult<RegistryEntry> {
        let registry = self.get(registry_id)?;

        match &registry.value {
            RegistryValue::WasmModule(wasm_module) => {
                self.artifact_repository
                    .remove(&wasm_module.wasm_artifact_id);
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
}
