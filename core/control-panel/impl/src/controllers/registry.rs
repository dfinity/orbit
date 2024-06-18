use crate::{
    core::middlewares::{call_context, use_canister_call_metric, use_is_authorized_admin},
    mappers::HelperMapper,
    services::{RegistryService, REGISTRY_SERVICE},
};
use control_panel_api::{
    AddRegistryEntryInput, AddRegistryEntryResponse, DeleteRegistryEntryInput,
    DeleteRegistryEntryResponse, EditRegistryEntryInput, EditRegistryEntryResponse,
    GetRegistryEntryInput, GetRegistryEntryResponse, NextModuleVersionInput,
    NextModuleVersionResponse, SearchRegistryInput, SearchRegistryResponse,
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

#[query(name = "next_module_version")]
async fn next_module_version(
    input: NextModuleVersionInput,
) -> ApiResult<NextModuleVersionResponse> {
    CONTROLLER.next_module_version(input).await
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

    /// Returns the next package version for the given package name and current version, if any.
    pub async fn next_module_version(
        &self,
        input: NextModuleVersionInput,
    ) -> ApiResult<NextModuleVersionResponse> {
        let entry = self
            .registry_service
            .find_next_version(&input.name, &input.current_version)?;

        Ok(NextModuleVersionResponse {
            entry: entry.map(|e| e.into()),
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
    use crate::core::ic_cdk::{api::id as self_canister_id, set_caller};
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

    #[tokio::test]
    #[should_panic]
    async fn test_add_registry_entry_not_allowed() {
        let _ = CONTROLLER
            .add_registry_entry(control_panel_api::AddRegistryEntryInput {
                entry: control_panel_api::RegistryEntryInput {
                    name: "test".to_string(),
                    description: "This is a demo description for the package.".to_string(),
                    categories: vec![],
                    tags: vec![],
                    metadata: vec![],
                    value: control_panel_api::RegistryEntryValueInput::WasmModule(
                        control_panel_api::WasmModuleRegistryEntryValueInput {
                            version: "0.1.0".to_string(),
                            dependencies: vec![],
                            wasm_module: vec![],
                        },
                    ),
                },
            })
            .await;
    }

    #[tokio::test]
    async fn test_add_registry_entry_is_allowed() {
        set_caller(self_canister_id());

        let result = CONTROLLER
            .add_registry_entry(control_panel_api::AddRegistryEntryInput {
                entry: control_panel_api::RegistryEntryInput {
                    name: "test".to_string(),
                    description: "This is a demo description for the package.".to_string(),
                    categories: vec![],
                    tags: vec![],
                    metadata: vec![],
                    value: control_panel_api::RegistryEntryValueInput::WasmModule(
                        control_panel_api::WasmModuleRegistryEntryValueInput {
                            version: "0.1.0".to_string(),
                            dependencies: vec![],
                            wasm_module: vec![1, 2, 3],
                        },
                    ),
                },
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_edit_registry_entry_not_found() {
        set_caller(self_canister_id());

        let entry_id = Uuid::new_v4().to_string();
        let result = CONTROLLER
            .edit_registry_entry(EditRegistryEntryInput {
                id: entry_id.clone(),
                entry: control_panel_api::RegistryEntryUpdateInput {
                    description: None,
                    categories: Some(vec!["test".to_string()]),
                    tags: None,
                    metadata: None,
                    value: None,
                },
            })
            .await;

        assert_eq!(
            result.unwrap_err(),
            ApiError::from(RegistryError::NotFound { id: entry_id })
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn test_edit_registry_entry_not_allowed() {
        let _ = CONTROLLER
            .edit_registry_entry(control_panel_api::EditRegistryEntryInput {
                id: Uuid::new_v4().to_string(),
                entry: control_panel_api::RegistryEntryUpdateInput {
                    description: None,
                    categories: Some(vec!["test".to_string()]),
                    tags: None,
                    metadata: None,
                    value: None,
                },
            })
            .await;
    }

    #[tokio::test]
    async fn test_edit_registry_entry_allowed() {
        set_caller(self_canister_id());

        let response = CONTROLLER
            .add_registry_entry(control_panel_api::AddRegistryEntryInput {
                entry: control_panel_api::RegistryEntryInput {
                    name: "test".to_string(),
                    description: "This is a demo description for the package.".to_string(),
                    categories: vec![],
                    tags: vec![],
                    metadata: vec![],
                    value: control_panel_api::RegistryEntryValueInput::WasmModule(
                        control_panel_api::WasmModuleRegistryEntryValueInput {
                            version: "0.1.0".to_string(),
                            dependencies: vec![],
                            wasm_module: vec![1, 2, 3],
                        },
                    ),
                },
            })
            .await
            .unwrap();

        assert!(response.entry.categories.is_empty());

        let edited = CONTROLLER
            .edit_registry_entry(control_panel_api::EditRegistryEntryInput {
                id: response.entry.id,
                entry: control_panel_api::RegistryEntryUpdateInput {
                    description: None,
                    categories: Some(vec!["test".to_string()]),
                    tags: None,
                    metadata: None,
                    value: None,
                },
            })
            .await
            .unwrap();

        assert_eq!(edited.entry.categories, vec!["test".to_string()]);
    }

    #[tokio::test]
    async fn test_delete_registry_entry_not_found() {
        set_caller(self_canister_id());

        let entry_id = Uuid::new_v4().to_string();
        let result = CONTROLLER
            .delete_registry_entry(DeleteRegistryEntryInput {
                id: entry_id.clone(),
            })
            .await;

        assert_eq!(
            result.unwrap_err(),
            ApiError::from(RegistryError::NotFound { id: entry_id })
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn test_delete_registry_entry_not_allowed() {
        let _ = CONTROLLER
            .delete_registry_entry(control_panel_api::DeleteRegistryEntryInput {
                id: Uuid::new_v4().to_string(),
            })
            .await;
    }

    #[tokio::test]
    async fn test_delete_registry_entry_allowed() {
        set_caller(self_canister_id());

        let response = CONTROLLER
            .add_registry_entry(control_panel_api::AddRegistryEntryInput {
                entry: control_panel_api::RegistryEntryInput {
                    name: "test".to_string(),
                    description: "This is a demo description for the package.".to_string(),
                    categories: vec![],
                    tags: vec![],
                    metadata: vec![],
                    value: control_panel_api::RegistryEntryValueInput::WasmModule(
                        control_panel_api::WasmModuleRegistryEntryValueInput {
                            version: "0.1.0".to_string(),
                            dependencies: vec![],
                            wasm_module: vec![1, 2, 3],
                        },
                    ),
                },
            })
            .await
            .unwrap();

        let _ = CONTROLLER
            .delete_registry_entry(control_panel_api::DeleteRegistryEntryInput {
                id: response.entry.id,
            })
            .await
            .unwrap();
    }
}
