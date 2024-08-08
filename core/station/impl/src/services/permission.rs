use crate::{
    core::{
        utils::{paginated_items, PaginatedData, PaginatedItemsArgs},
        validation::{EnsureIdExists, EnsureUser, EnsureUserGroup},
    },
    models::{
        permission::{Allow, Permission},
        resource::Resource,
        EditPermissionOperationInput, User, UserGroup,
    },
    repositories::permission::{PermissionRepository, PERMISSION_REPOSITORY},
    services::{UserGroupService, UserService, USER_GROUP_SERVICE, USER_SERVICE},
};
use candid::CandidType;
use lazy_static::lazy_static;
use orbit_essentials::{api::ServiceResult, model::ModelKey};
use orbit_essentials::{model::ModelValidator, repository::Repository};
use station_api::ListPermissionsInput;
use std::{collections::HashSet, sync::Arc};

lazy_static! {
    pub static ref PERMISSION_SERVICE: Arc<PermissionService> = Arc::new(PermissionService::new(
        Arc::clone(&PERMISSION_REPOSITORY),
        Arc::clone(&USER_SERVICE),
        Arc::clone(&USER_GROUP_SERVICE)
    ));
}

#[derive(Clone, CandidType)]
pub struct PermissionDependenciesResponse {
    pub groups: Vec<UserGroup>,
    pub users: Vec<User>,
}

#[derive(Default, Debug)]
pub struct PermissionService {
    permission_repository: Arc<PermissionRepository>,
    user_service: Arc<UserService>,
    user_group_service: Arc<UserGroupService>,
}

impl PermissionService {
    pub const DEFAULT_POLICIES_LIMIT: u16 = 100;
    pub const MAX_LIST_POLICIES_LIMIT: u16 = 1000;

    pub fn new(
        permission_repository: Arc<PermissionRepository>,
        user_service: Arc<UserService>,
        user_group_service: Arc<UserGroupService>,
    ) -> Self {
        Self {
            permission_repository,
            user_service,
            user_group_service,
        }
    }

    /// Returns a permission for a given resource.
    ///
    /// If it does not exist, a default permission is returned.
    pub fn get_permission(&self, resource: &Resource) -> Permission {
        self.permission_repository
            .get(resource)
            .unwrap_or_else(|| Permission::new(Allow::default(), resource.clone()))
    }

    /// Removes a permission for a given resource, if it exists.
    pub fn remove_permission(&self, resource: &Resource) {
        self.permission_repository.remove(resource);
    }

    /// Edits a permission for a given resource.
    pub fn edit_permission(
        &self,
        input: EditPermissionOperationInput,
    ) -> ServiceResult<Permission> {
        input.resource.validate()?;

        let mut permission = self.get_permission(&input.resource);

        if let Some(scope) = input.auth_scope {
            permission.allow.auth_scope = scope;
        }
        if let Some(users) = input.users {
            EnsureUser::id_list_exists(&users)?;
            permission.allow.users = users;
        }
        if let Some(user_groups) = input.user_groups {
            EnsureUserGroup::id_list_exists(&user_groups)?;
            permission.allow.user_groups = user_groups;
        }

        self.permission_repository
            .insert(permission.key(), permission.to_owned());

        Ok(permission)
    }

    /// Lists permissions with optional pagination.
    pub async fn list_permissions(
        &self,
        input: ListPermissionsInput,
    ) -> ServiceResult<PaginatedData<Permission>> {
        let policies = match input.resources {
            Some(resources) => resources
                .into_iter()
                .map(|r| self.get_permission(&r.into()))
                .collect::<_>(),
            None => self.permission_repository.list(),
        };

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.as_ref().and_then(|p| p.offset),
            limit: input.paginate.as_ref().and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_POLICIES_LIMIT),
            max_limit: Some(Self::MAX_LIST_POLICIES_LIMIT),
            items: &policies,
        })?;

        Ok(result)
    }

    pub fn get_permissions_dependencies(
        &self,
        permissions: &Vec<Permission>,
    ) -> ServiceResult<PermissionDependenciesResponse> {
        let mut user_ids = HashSet::new();
        let mut group_ids = HashSet::new();
        for permission in permissions {
            group_ids.extend(permission.allow.user_groups.clone());
            user_ids.extend(permission.allow.users.clone());
        }

        let mut groups = Vec::new();
        group_ids.iter().for_each(|id| {
            if let Ok(user_group) = self.user_group_service.get(id) {
                groups.push(user_group);
            }
        });

        let mut users = Vec::new();
        user_ids.iter().for_each(|id| {
            if let Ok(user) = self.user_service.get_user(id) {
                users.push(user);
            }
        });

        Ok(PermissionDependenciesResponse { groups, users })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::validation::disable_mock_resource_validation,
        models::{
            permission::{permission_test_utils::mock_permission, AuthScope},
            resource::{AccountResourceAction, RequestResourceAction, ResourceId},
            user_group_test_utils::mock_user_group,
            user_test_utils::mock_user,
        },
        repositories::{USER_GROUP_REPOSITORY, USER_REPOSITORY},
    };

    #[test]
    fn test_permission_operations() {
        let service = PERMISSION_SERVICE.clone();
        let result = service.edit_permission(EditPermissionOperationInput {
            auth_scope: Some(AuthScope::Authenticated),
            user_groups: None,
            users: None,
            resource: Resource::Request(RequestResourceAction::List),
        });

        assert!(result.is_ok());

        let policy = result.unwrap();
        assert!(policy.allowed_authenticated());

        let result = service.edit_permission(EditPermissionOperationInput {
            auth_scope: Some(AuthScope::Public),
            user_groups: None,
            users: None,
            resource: Resource::Request(RequestResourceAction::List),
        });

        assert!(result.is_ok());

        let policy = result.unwrap();

        assert!(policy.allowed_public());
    }

    #[test]
    fn test_get_default_policy() {
        let service = PERMISSION_SERVICE.clone();
        let result = service.get_permission(&Resource::Request(RequestResourceAction::List));

        assert_eq!(
            result,
            Permission::new(
                Allow::default(),
                Resource::Request(RequestResourceAction::List)
            )
        );
    }

    #[test]
    fn get_permission_dependencies_finds_users() {
        let service = PERMISSION_SERVICE.clone();
        let policy = Permission::new(
            Allow::users(vec![[1; 16]]),
            Resource::Request(RequestResourceAction::List),
        );
        PERMISSION_REPOSITORY.insert(policy.key(), policy.to_owned());
        let mut user = mock_user();
        user.id = [1; 16];
        USER_REPOSITORY.insert(user.to_key(), user.to_owned());

        let result = service
            .get_permissions_dependencies(&vec![policy.to_owned()])
            .unwrap();

        assert_eq!(result.users.len(), 1);
        assert_eq!(result.groups.len(), 0);
    }

    #[test]
    fn get_permission_dependencies_finds_groups() {
        let service = PERMISSION_SERVICE.clone();
        let policy = Permission::new(
            Allow::user_groups(vec![[1; 16]]),
            Resource::Request(RequestResourceAction::List),
        );
        PERMISSION_REPOSITORY.insert(policy.key(), policy.to_owned());
        let mut group = mock_user_group();
        group.id = [1; 16];
        USER_GROUP_REPOSITORY.insert(group.id, group.to_owned());

        let result = service
            .get_permissions_dependencies(&vec![policy.to_owned()])
            .unwrap();

        assert_eq!(result.users.len(), 0);
        assert_eq!(result.groups.len(), 1);
    }

    #[tokio::test]
    async fn list_permissions_should_use_offset_and_limit() {
        for _ in 0..20 {
            let policy = mock_permission();
            PERMISSION_REPOSITORY.insert(policy.key(), policy.to_owned());
        }

        let input = ListPermissionsInput {
            resources: None,
            paginate: Some(station_api::PaginationInput {
                offset: Some(5),
                limit: Some(10),
            }),
        };

        let result = PERMISSION_SERVICE.list_permissions(input).await.unwrap();
        assert_eq!(result.items.len(), 10);
        assert_eq!(result.next_offset, Some(15));
    }

    #[test]
    fn test_override_permission_auth_scope() {
        let service = PERMISSION_SERVICE.clone();
        let resource = Resource::Request(RequestResourceAction::List);
        let _ = service
            .edit_permission(EditPermissionOperationInput {
                auth_scope: Some(AuthScope::Public),
                user_groups: None,
                users: None,
                resource: resource.clone(),
            })
            .unwrap();

        assert!(service.get_permission(&resource).allowed_public());

        service
            .edit_permission(EditPermissionOperationInput {
                auth_scope: Some(AuthScope::Authenticated),
                user_groups: None,
                users: None,
                resource: resource.clone(),
            })
            .unwrap();

        assert!(service.get_permission(&resource).allowed_authenticated());
    }

    #[test]
    fn fail_edit_permission_invalid_ids() {
        let service = PermissionService::default();

        disable_mock_resource_validation();

        service
            .edit_permission(EditPermissionOperationInput {
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    [1; 16],
                ))),
                auth_scope: None,
                users: None,
                user_groups: None,
            })
            .expect_err("Should fail with invalid account ID");

        service
            .edit_permission(EditPermissionOperationInput {
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
                auth_scope: None,
                users: Some(vec![[1; 16]]),
                user_groups: None,
            })
            .expect_err("Should fail with invalid User ID");

        service
            .edit_permission(EditPermissionOperationInput {
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
                auth_scope: None,
                users: None,
                user_groups: Some(vec![[1; 16]]),
            })
            .expect_err("Should fail with invalid Group ID");
    }

    #[test]
    fn test_remove_permission() {
        let service = PERMISSION_SERVICE.clone();
        let resource = Resource::Request(RequestResourceAction::List);
        let _ = service
            .edit_permission(EditPermissionOperationInput {
                auth_scope: Some(AuthScope::Public),
                user_groups: None,
                users: None,
                resource: resource.clone(),
            })
            .unwrap();

        assert!(service.get_permission(&resource).allowed_public());

        service.remove_permission(&resource);

        assert_eq!(
            service.get_permission(&resource),
            Permission::new(Allow::default(), resource)
        );
    }
}
