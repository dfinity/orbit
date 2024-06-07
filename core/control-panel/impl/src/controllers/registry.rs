use crate::{
    core::middlewares::{call_context, use_canister_call_metric, use_is_authorized_admin},
    mappers::HelperMapper,
    services::{RegistryService, REGISTRY_SERVICE},
};
use control_panel_api::{
    AddRegistryEntryInput, AddRegistryEntryResponse, DeleteRegistryEntryInput,
    DeleteRegistryEntryResponse, EditRegistryEntryInput, EditRegistryEntryResponse,
    GetRegistryEntryInput, GetRegistryEntryResponse, SearchRegistryInput, SearchRegistryResponse,
};
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::{api::ApiResult, with_middleware};
use std::sync::Arc;

// Canister entrypoints for the controller.
#[query(name = "get_registry_entry")]
async fn get_registry_entry(input: GetRegistryEntryInput) -> ApiResult<GetRegistryEntryResponse> {
    CONTROLLER.get_registry_entry(input).await
}

#[query(name = "search_registry")]
async fn search_registry(input: SearchRegistryInput) -> ApiResult<SearchRegistryResponse> {
    CONTROLLER.search_registry(input).await
}

#[update(name = "add_registry_entry")]
async fn add_registry_entry(input: AddRegistryEntryInput) -> ApiResult<AddRegistryEntryResponse> {
    CONTROLLER.add_registry_entry(input).await
}

#[update(name = "edit_registry_entry")]
async fn edit_registry_entry(
    input: EditRegistryEntryInput,
) -> ApiResult<EditRegistryEntryResponse> {
    CONTROLLER.edit_registry_entry(input).await
}

#[update(name = "delete_registry_entry")]
async fn delete_registry_entry(
    input: DeleteRegistryEntryInput,
) -> ApiResult<DeleteRegistryEntryResponse> {
    CONTROLLER.delete_registry_entry(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: RegistryController =
        RegistryController::new(Arc::clone(&REGISTRY_SERVICE));
}

#[derive(Debug)]
pub struct RegistryController {
    registry_service: Arc<RegistryService>,
}

impl RegistryController {
    pub fn new(registry_service: Arc<RegistryService>) -> Self {
        Self { registry_service }
    }

    /// Returns the registry entry by id.
    pub async fn get_registry_entry(
        &self,
        input: GetRegistryEntryInput,
    ) -> ApiResult<GetRegistryEntryResponse> {
        let entry = self.registry_service.get(
            HelperMapper::to_uuid(input.id)
                .expect("Invalid registry id")
                .as_bytes(),
        )?;

        Ok(GetRegistryEntryResponse {
            entry: entry.into(),
        })
    }

    /// Searches the registry for entries.
    pub async fn search_registry(
        &self,
        input: SearchRegistryInput,
    ) -> ApiResult<SearchRegistryResponse> {
        let paginated_result = self.registry_service.search(input)?;

        Ok(SearchRegistryResponse {
            entries: paginated_result
                .items
                .into_iter()
                .map(|entry| entry.into())
                .collect(),
            total: paginated_result.total,
            next_offset: paginated_result.next_offset,
        })
    }

    /// Adds a new registry entry.
    #[with_middleware(guard = use_is_authorized_admin(&call_context()))]
    #[with_middleware(tail = use_canister_call_metric("add_registry_entry", &result))]
    pub async fn add_registry_entry(
        &self,
        input: AddRegistryEntryInput,
    ) -> ApiResult<AddRegistryEntryResponse> {
        let new_entry = self.registry_service.create(input.entry)?;

        Ok(AddRegistryEntryResponse {
            entry: new_entry.into(),
        })
    }

    /// Edits an existing registry entry.
    #[with_middleware(guard = use_is_authorized_admin(&call_context()))]
    #[with_middleware(tail = use_canister_call_metric("edit_registry_entry", &result))]
    pub async fn edit_registry_entry(
        &self,
        input: EditRegistryEntryInput,
    ) -> ApiResult<EditRegistryEntryResponse> {
        let entry_id = HelperMapper::to_uuid(input.id).expect("Invalid registry entry id");
        let edited_entry = self
            .registry_service
            .edit(entry_id.as_bytes(), input.entry)?;

        Ok(EditRegistryEntryResponse {
            entry: edited_entry.into(),
        })
    }

    /// Deletes an existing registry entry.
    #[with_middleware(guard = use_is_authorized_admin(&call_context()))]
    #[with_middleware(tail = use_canister_call_metric("delete_registry_entry", &result))]
    pub async fn delete_registry_entry(
        &self,
        input: DeleteRegistryEntryInput,
    ) -> ApiResult<DeleteRegistryEntryResponse> {
        let entry_id = HelperMapper::to_uuid(input.id).expect("Invalid registry entry id");
        let deleted_entry = self.registry_service.delete(entry_id.as_bytes())?;

        Ok(DeleteRegistryEntryResponse {
            entry: deleted_entry.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::RegistryError;
    use orbit_essentials::api::ApiError;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_registry_entry_not_found() {
        let entry_id = Uuid::new_v4().to_string();
        let result = CONTROLLER
            .get_registry_entry(GetRegistryEntryInput {
                id: entry_id.clone(),
            })
            .await;

        assert_eq!(
            result.unwrap_err(),
            ApiError::from(RegistryError::NotFound { id: entry_id })
        );
    }
}
