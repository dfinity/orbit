use crate::{
    errors::RegistryError,
    models::{Registry, RegistryId},
    repositories::{RegistryRepository, REGISTRY_REPOSITORY},
};
use lazy_static::lazy_static;
use orbit_essentials::{api::ServiceResult, repository::Repository};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref REGISTRY_SERVICE: Arc<RegistryService> =
        Arc::new(RegistryService::new(Arc::clone(&REGISTRY_REPOSITORY)));
}

/// The registry service provides methods to manage registry entries.
#[derive(Default, Debug)]
pub struct RegistryService {
    registry_repository: Arc<RegistryRepository>,
}

impl RegistryService {
    pub fn new(registry_repository: Arc<RegistryRepository>) -> Self {
        Self {
            registry_repository,
        }
    }

    /// Returns the registry entry by id.
    pub fn get(&self, registry_id: &RegistryId) -> ServiceResult<Registry> {
        let entry =
            self.registry_repository
                .get(registry_id)
                .ok_or_else(|| RegistryError::NotFound {
                    id: Uuid::from_bytes(*registry_id).to_string(),
                })?;

        Ok(entry)
    }

    /// Creates a new registry entry and returns it.
    pub fn create(&self) -> ServiceResult<Registry> {
        todo!()
    }

    /// Updates the registry entry and returns it.
    pub fn edit(&self, registry_id: &RegistryId) -> ServiceResult<Registry> {
        let _entry = self.get(registry_id)?;

        todo!();
    }

    /// Deletes the registry entry by id.
    pub fn delete(&self, registry_id: &RegistryId) -> ServiceResult<()> {
        self.registry_repository.remove(registry_id);

        Ok(())
    }

    /// Returns all registry entries by the given name, if the name is not namespaced, 
    /// the default namespace is used.
    pub fn find_by_name(&self, name: &str) -> ServiceResult<Vec<Registry>> {
        let entries = self
            .registry_repository
            .find_by_fullname(&Registry::format_fullname(name));

        Ok(entries)
    }
}
