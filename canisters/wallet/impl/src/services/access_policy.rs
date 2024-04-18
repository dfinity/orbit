use crate::{
    core::{
        utils::{paginated_items, PaginatedData, PaginatedItemsArgs},
        validation::{EnsureIdExists, EnsureUser, EnsureUserGroup},
    },
    models::{
        access_policy::{AccessPolicy, Allow},
        resource::Resource,
        EditAccessPolicyOperationInput, User, UserGroup,
    },
    repositories::access_policy::{AccessPolicyRepository, ACCESS_POLICY_REPOSITORY},
    services::{UserGroupService, UserService, USER_GROUP_SERVICE, USER_SERVICE},
};
use candid::CandidType;
use ic_canister_core::{api::ServiceResult, model::ModelKey};
use ic_canister_core::{model::ModelValidator, repository::Repository};
use lazy_static::lazy_static;
use std::{collections::HashSet, sync::Arc};
use wallet_api::ListAccessPoliciesInput;

lazy_static! {
    pub static ref ACCESS_POLICY_SERVICE: Arc<AccessPolicyService> =
        Arc::new(AccessPolicyService::new(
            Arc::clone(&ACCESS_POLICY_REPOSITORY),
            Arc::clone(&USER_SERVICE),
            Arc::clone(&USER_GROUP_SERVICE)
        ));
}

#[derive(Clone, CandidType)]
pub struct AccessPolicyDependenciesResponse {
    pub groups: Vec<UserGroup>,
    pub users: Vec<User>,
}

#[derive(Default, Debug)]
pub struct AccessPolicyService {
    access_policy_repository: Arc<AccessPolicyRepository>,
    user_service: Arc<UserService>,
    user_group_service: Arc<UserGroupService>,
}

impl AccessPolicyService {
    pub const DEFAULT_POLICIES_LIMIT: u16 = 100;
    pub const MAX_LIST_POLICIES_LIMIT: u16 = 1000;

    pub fn new(
        access_policy_repository: Arc<AccessPolicyRepository>,
        user_service: Arc<UserService>,
        user_group_service: Arc<UserGroupService>,
    ) -> Self {
        Self {
            access_policy_repository,
            user_service,
            user_group_service,
        }
    }

    pub fn get_access_policy(&self, resource: &Resource) -> AccessPolicy {
        self.access_policy_repository
            .get(resource)
            .unwrap_or_else(|| AccessPolicy::new(Allow::default(), resource.clone()))
    }

    pub async fn edit_access_policy(
        &self,
        input: EditAccessPolicyOperationInput,
    ) -> ServiceResult<AccessPolicy> {
        input.resource.validate()?;

        let mut access_policy = self.get_access_policy(&input.resource);

        if let Some(scope) = input.auth_scope {
            access_policy.allow.auth_scope = scope;
        }
        if let Some(users) = input.users {
            EnsureUser::id_list_exists(&users)?;
            access_policy.allow.users = users;
        }
        if let Some(user_groups) = input.user_groups {
            EnsureUserGroup::id_list_exists(&user_groups)?;
            access_policy.allow.user_groups = user_groups;
        }

        self.access_policy_repository
            .insert(access_policy.key(), access_policy.to_owned());

        Ok(access_policy)
    }

    pub async fn list_access_policies(
        &self,
        input: ListAccessPoliciesInput,
    ) -> ServiceResult<PaginatedData<AccessPolicy>> {
        let policies = match input.resources {
            Some(resources) => resources
                .into_iter()
                .map(|r| self.get_access_policy(&r.into()))
                .collect::<_>(),
            None => self.access_policy_repository.list(),
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

    pub fn get_access_policies_dependencies(
        &self,
        policies: &Vec<AccessPolicy>,
    ) -> ServiceResult<AccessPolicyDependenciesResponse> {
        let mut user_ids = HashSet::new();
        let mut group_ids = HashSet::new();
        for policy in policies {
            group_ids.extend(policy.allow.user_groups.clone());
            user_ids.extend(policy.allow.users.clone());
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

        Ok(AccessPolicyDependenciesResponse { groups, users })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::validation::disable_mock_resource_validation,
        models::{
            access_policy::{access_policy_test_utils::mock_access_policy, AuthScope},
            resource::{AccountResourceAction, ProposalResourceAction, ResourceId},
            user_group_test_utils::mock_user_group,
            user_test_utils::mock_user,
        },
        repositories::{USER_GROUP_REPOSITORY, USER_REPOSITORY},
    };

    #[tokio::test]
    async fn test_access_policy_operations() {
        let service = ACCESS_POLICY_SERVICE.clone();
        let result = service
            .edit_access_policy(EditAccessPolicyOperationInput {
                auth_scope: Some(AuthScope::Authenticated),
                user_groups: None,
                users: None,
                resource: Resource::Proposal(ProposalResourceAction::List),
            })
            .await;

        assert!(result.is_ok());

        let policy = result.unwrap();
        assert!(policy.allowed_authenticated());

        let result = service
            .edit_access_policy(EditAccessPolicyOperationInput {
                auth_scope: Some(AuthScope::Public),
                user_groups: None,
                users: None,
                resource: Resource::Proposal(ProposalResourceAction::List),
            })
            .await;

        assert!(result.is_ok());

        let policy = result.unwrap();

        assert!(policy.allowed_public());
    }

    #[test]
    fn test_get_default_policy() {
        let service = ACCESS_POLICY_SERVICE.clone();
        let result = service.get_access_policy(&Resource::Proposal(ProposalResourceAction::List));

        assert_eq!(
            result,
            AccessPolicy::new(
                Allow::default(),
                Resource::Proposal(ProposalResourceAction::List)
            )
        );
    }

    #[test]
    fn get_access_policy_dependencies_finds_users() {
        let service = ACCESS_POLICY_SERVICE.clone();
        let policy = AccessPolicy::new(
            Allow::users(vec![[1; 16]]),
            Resource::Proposal(ProposalResourceAction::List),
        );
        ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.to_owned());
        let mut user = mock_user();
        user.id = [1; 16];
        USER_REPOSITORY.insert(user.to_key(), user.to_owned());

        let result = service
            .get_access_policies_dependencies(&vec![policy.to_owned()])
            .unwrap();

        assert_eq!(result.users.len(), 1);
        assert_eq!(result.groups.len(), 0);
    }

    #[test]
    fn get_access_policy_dependencies_finds_groups() {
        let service = ACCESS_POLICY_SERVICE.clone();
        let policy = AccessPolicy::new(
            Allow::user_groups(vec![[1; 16]]),
            Resource::Proposal(ProposalResourceAction::List),
        );
        ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.to_owned());
        let mut group = mock_user_group();
        group.id = [1; 16];
        USER_GROUP_REPOSITORY.insert(group.id, group.to_owned());

        let result = service
            .get_access_policies_dependencies(&vec![policy.to_owned()])
            .unwrap();

        assert_eq!(result.users.len(), 0);
        assert_eq!(result.groups.len(), 1);
    }

    #[tokio::test]
    async fn list_access_policies_should_use_offset_and_limit() {
        for _ in 0..20 {
            let policy = mock_access_policy();
            ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.to_owned());
        }

        let input = ListAccessPoliciesInput {
            resources: None,
            paginate: Some(wallet_api::PaginationInput {
                offset: Some(5),
                limit: Some(10),
            }),
        };

        let result = ACCESS_POLICY_SERVICE
            .list_access_policies(input)
            .await
            .unwrap();
        assert_eq!(result.items.len(), 10);
        assert_eq!(result.next_offset, Some(15));
    }

    #[tokio::test]
    async fn test_override_access_policy_auth_scope() {
        let service = ACCESS_POLICY_SERVICE.clone();
        let resource = Resource::Proposal(ProposalResourceAction::List);
        let _ = service
            .edit_access_policy(EditAccessPolicyOperationInput {
                auth_scope: Some(AuthScope::Public),
                user_groups: None,
                users: None,
                resource: resource.clone(),
            })
            .await
            .unwrap();

        assert!(service.get_access_policy(&resource).allowed_public());

        service
            .edit_access_policy(EditAccessPolicyOperationInput {
                auth_scope: Some(AuthScope::Authenticated),
                user_groups: None,
                users: None,
                resource: resource.clone(),
            })
            .await
            .unwrap();

        assert!(service.get_access_policy(&resource).allowed_authenticated());
    }

    #[tokio::test]
    async fn fail_edit_access_policy_invalid_ids() {
        let service = AccessPolicyService::default();

        disable_mock_resource_validation();

        service
            .edit_access_policy(EditAccessPolicyOperationInput {
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    [1; 16],
                ))),
                auth_scope: None,
                users: None,
                user_groups: None,
            })
            .await
            .expect_err("Should fail with invalid account ID");

        service
            .edit_access_policy(EditAccessPolicyOperationInput {
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
                auth_scope: None,
                users: Some(vec![[1; 16]]),
                user_groups: None,
            })
            .await
            .expect_err("Should fail with invalid User ID");

        service
            .edit_access_policy(EditAccessPolicyOperationInput {
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
                auth_scope: None,
                users: None,
                user_groups: Some(vec![[1; 16]]),
            })
            .await
            .expect_err("Should fail with invalid Group ID");
    }
}
