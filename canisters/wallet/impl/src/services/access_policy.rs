use crate::{
    core::{
        authorization::Authorization,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    models::{
        access_policy::{
            AccessPolicy, AccessPolicyCallerPrivileges, AccessPolicyResourceAction, Allow,
            Resource, ResourceTypeId,
        },
        indexes::access_policy_allow_level_index::AllowLevel,
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

    pub fn get_access_policy(&self, resource: &Resource) -> ServiceResult<AccessPolicy> {
        let access_policy = self
            .access_policy_repository
            .get(resource)
            .unwrap_or_else(|| AccessPolicy::new(Allow::default(), resource.clone()));

        Ok(access_policy)
    }

    pub async fn edit_access_policy(
        &self,
        input: EditAccessPolicyOperationInput,
    ) -> ServiceResult<AccessPolicy> {
        let mut access_policy = self.get_access_policy(&input.resource)?;

        match &input.access {
            ResourceAccess::Allow(allow) => {
                if let Some(user_groups) = &allow.user_groups {
                    access_policy.allow.user_groups = Some(user_groups.clone());
                }
                if let Some(users) = &allow.users {
                    access_policy.allow.users = Some(users.clone());
                }
                if let Some(authentication) = &allow.authentication {
                    access_policy.allow.authentication = Some(authentication.clone());
                }
            }
            ResourceAccess::Deny(level) => match level {
                AllowLevel::Any | AllowLevel::Authenticated => {
                    access_policy.allow.authentication = None;
                }
                AllowLevel::UserGroups => {
                    access_policy.allow.user_groups = None;
                }
                AllowLevel::Users => {
                    access_policy.allow.users = None;
                }
            },
        };

        self.access_policy_repository
            .insert(access_policy.key(), access_policy.to_owned());

        Ok(access_policy)
    }

    pub async fn get_caller_privileges_for_access_policy(
        &self,
        resource: &Resource,
        ctx: &CallContext,
    ) -> ServiceResult<AccessPolicyCallerPrivileges> {
        Ok(AccessPolicyCallerPrivileges {
            resource: resource.clone(),
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::AccessPolicy(AccessPolicyResourceAction::Edit(
                    ResourceTypeId::Resource(resource.to_type()),
                )),
            ),
        })
    }

    pub async fn list_access_policies(
        &self,
        input: ListAccessPoliciesInput,
        ctx: &CallContext,
    ) -> ServiceResult<PaginatedData<AccessPolicy>> {
        let mut policies = match input.resources {
            Some(resources) => resources
                .into_iter()
                .map(|r| self.get_access_policy(&r.into()))
                .collect::<Result<Vec<_>, _>>()?,
            None => self.access_policy_repository.list(),
        };

        retain_accessible_resources(ctx, &mut policies, |policy| {
            Resource::AccessPolicy(AccessPolicyResourceAction::Read(ResourceTypeId::Resource(
                policy.resource.to_type(),
            )))
        });

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
            if let Some(user_groups) = &policy.allow.user_groups {
                group_ids.extend(user_groups);
            }

            if let Some(users) = &policy.allow.users {
                user_ids.extend(users);
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
                access: ResourceAccess::Allow(Allow::authenticated()),
                resource: Resource::Proposal(ProposalResourceAction::List),
            })
            .await;

        assert!(result.is_ok());

        let policy = result.unwrap();
        assert!(policy.allowed_authenticated());

        let result = service
            .edit_access_policy(EditAccessPolicyOperationInput {
                access: ResourceAccess::Allow(Allow::any()),
                resource: Resource::Proposal(ProposalResourceAction::List),
            })
            .await;

        assert!(result.is_ok());

        let policy = result.unwrap();

        assert!(policy.allowed_any());
    }

    #[test]
    fn test_get_default_policy() {
        let service = ACCESS_POLICY_SERVICE.clone();
        let result = service.get_access_policy(&Resource::Proposal(ProposalResourceAction::List));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
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
                access: ResourceAccess::Allow(Allow::any()),
                resource: resource.clone(),
            })
            .await
            .unwrap();

        assert_eq!(
            service.get_access_policy(&resource).unwrap().allow,
            Allow::any()
        );

        service
            .edit_access_policy(EditAccessPolicyOperationInput {
                access: ResourceAccess::Deny(AllowLevel::Any),
                resource: resource.clone(),
            })
            .await
            .unwrap();

        assert!(!service.get_access_policy(&resource).unwrap().allowed_any());
    }
}
