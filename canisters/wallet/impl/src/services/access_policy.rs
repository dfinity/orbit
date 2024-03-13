use crate::{
    core::{
        authorization::Authorization,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    models::{
        access_policy::{
            AccessPolicy, AccessPolicyCallerPrivileges, AccessPolicyKey,
            AccessPolicyResourceAction, Allow, AllowLevel, Resource, ResourceType, ResourceTypeId,
        },
        EditAccessPolicyOperationInput, ResourceAccess, User, UserGroup,
    },
    repositories::access_policy::{AccessPolicyRepository, ACCESS_POLICY_REPOSITORY},
    services::{UserGroupService, UserService, USER_GROUP_SERVICE, USER_SERVICE},
};
use candid::CandidType;
use ic_canister_core::repository::Repository;
use ic_canister_core::{api::ServiceResult, model::ModelKey};
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

    pub fn get_access_policy(&self, resource: &Resource) -> ServiceResult<Vec<AccessPolicy>> {
        let policies = self
            .access_policy_repository
            .find_by_resource(resource.clone());

        Ok(policies)
    }

    pub async fn edit_access_policy(
        &self,
        input: EditAccessPolicyOperationInput,
    ) -> ServiceResult<Vec<AccessPolicy>> {
        let allow_level = match &input.access {
            ResourceAccess::Allow(allow) => allow.to_owned().into(),
            ResourceAccess::Deny(allow) => allow.to_owned(),
        };
        let policy = self
            .access_policy_repository
            .find_by_resource_and_allow_level(input.resource.clone(), allow_level);

        match input.access {
            ResourceAccess::Allow(allow) => {
                let mut policy = policy
                    .unwrap_or_else(|| AccessPolicy::new(allow.clone(), input.resource.to_owned()));
                policy.allow = allow;

                if policy.allow == Allow::Any {
                    self.access_policy_repository.remove(&AccessPolicyKey {
                        resource: input.resource.clone(),
                        allow_level: AllowLevel::Authenticated,
                    });
                } else if policy.allow == Allow::Authenticated {
                    self.access_policy_repository.remove(&AccessPolicyKey {
                        resource: input.resource.clone(),
                        allow_level: AllowLevel::Any,
                    });
                }

                self.access_policy_repository
                    .insert(policy.key(), policy.clone());
            }
            ResourceAccess::Deny(_) => {
                if let Some(policy) = policy {
                    self.access_policy_repository.remove(&policy.key());
                }
            }
        };

        let updated_policies = self
            .access_policy_repository
            .find_by_resource(input.resource);

        Ok(updated_policies)
    }

    pub async fn get_caller_privileges_for_access_policy(
        &self,
        resource_type: &ResourceType,
        ctx: &CallContext,
    ) -> ServiceResult<AccessPolicyCallerPrivileges> {
        Ok(AccessPolicyCallerPrivileges {
            resource_type: resource_type.clone(),
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::AccessPolicy(AccessPolicyResourceAction::Edit(
                    ResourceTypeId::Resource(resource_type.clone()),
                )),
            ),
        })
    }

    pub async fn list_access_policies(
        &self,
        input: ListAccessPoliciesInput,
        ctx: &CallContext,
    ) -> ServiceResult<PaginatedData<AccessPolicy>> {
        let mut policies = self.access_policy_repository.list();

        retain_accessible_resources(ctx, &mut policies, |policy| {
            Resource::AccessPolicy(AccessPolicyResourceAction::Read(ResourceTypeId::Resource(
                policy.resource.to_type(),
            )))
        });

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.offset,
            limit: input.limit,
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
            match &policy.allow {
                Allow::Users(ids) => user_ids.extend(ids),
                Allow::UserGroups(ids) => group_ids.extend(ids),
                Allow::Authenticated | Allow::Any => {}
            }
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
        core::ic_cdk::api::id as self_canister_id,
        models::{
            access_policy::{access_policy_test_utils::mock_access_policy, ProposalResourceAction},
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
                access: ResourceAccess::Allow(Allow::Authenticated),
                resource: Resource::Proposal(ProposalResourceAction::List),
            })
            .await;

        assert!(result.is_ok());

        let policies = result.unwrap();
        let policy = policies.first().unwrap();

        assert_eq!(policy.allow, Allow::Authenticated);

        let result = service
            .edit_access_policy(EditAccessPolicyOperationInput {
                access: ResourceAccess::Allow(Allow::Any),
                resource: Resource::Proposal(ProposalResourceAction::List),
            })
            .await;

        assert!(result.is_ok());

        let policies = result.unwrap();

        assert_eq!(policies.len(), 1);
        assert_eq!(policies.first().unwrap().allow, Allow::Any);
    }

    #[test]
    fn test_get_empty_list_of_policies() {
        let service = ACCESS_POLICY_SERVICE.clone();
        let result = service.get_access_policy(&Resource::Proposal(ProposalResourceAction::List));

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn get_access_policy_dependencies_finds_users() {
        let service = ACCESS_POLICY_SERVICE.clone();
        let policy = AccessPolicy::new(
            Allow::Users(vec![[1; 16]]),
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
            Allow::UserGroups(vec![[1; 16]]),
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
            offset: Some(5),
            limit: Some(10),
        };

        let result = ACCESS_POLICY_SERVICE
            .list_access_policies(input, &CallContext::new(self_canister_id()))
            .await
            .unwrap();
        assert_eq!(result.items.len(), 10);
        assert_eq!(result.next_offset, Some(15));
    }

    #[tokio::test]
    async fn test_remove_access_policy() {
        let service = ACCESS_POLICY_SERVICE.clone();
        let resource = Resource::Proposal(ProposalResourceAction::List);
        let _ = service
            .edit_access_policy(EditAccessPolicyOperationInput {
                access: ResourceAccess::Allow(Allow::Any),
                resource: resource.clone(),
            })
            .await
            .unwrap();

        assert!(!service.get_access_policy(&resource).unwrap().is_empty());

        service
            .edit_access_policy(EditAccessPolicyOperationInput {
                access: ResourceAccess::Deny(AllowLevel::Any),
                resource: resource.clone(),
            })
            .await
            .unwrap();

        assert!(service.get_access_policy(&resource).unwrap().is_empty());
    }
}
