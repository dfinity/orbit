use crate::{
    errors::RegistryError,
    models::{RegistryEntry, RegistryEntryId, RegistryValue},
    repositories::{
        ArtifactRepository, RegistryRepository, ARTIFACT_REPOSITORY, REGISTRY_REPOSITORY,
    },
};
use control_panel_api::{RegistryEntryInput, SearchRegistryInput};
use lazy_static::lazy_static;
use orbit_essentials::{api::ServiceResult, pagination::PaginatedData, repository::Repository};
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
        _search: SearchRegistryInput,
    ) -> ServiceResult<PaginatedData<RegistryEntry>> {
        unimplemented!()
    }

    /// Creates a new registry entry and returns it.
    pub fn create(&self, _input: RegistryEntryInput) -> ServiceResult<RegistryEntry> {
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
