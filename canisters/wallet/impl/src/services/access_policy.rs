use crate::{
    core::{
        authorization::Authorization,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    models::{
        access_policy::{
            AccessPolicy, AccessPolicyCallerPrivileges, AccessPolicyResourceAction, Allow,
            Resource, ResourceType, ResourceTypeId,
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
    ) -> ServiceResult<()> {
        let allow_level = match &input.access {
            ResourceAccess::Allow(allow) => allow.to_owned().into(),
            ResourceAccess::Deny(allow) => allow.to_owned(),
        };
        let policy = self
            .access_policy_repository
            .find_by_resource_and_allow_level(input.resource.clone(), allow_level);

        match input.access {
            ResourceAccess::Allow(allow) => {
                let mut policy =
                    policy.unwrap_or_else(|| AccessPolicy::new(allow.clone(), input.resource));
                policy.allow = allow;

                self.access_policy_repository
                    .insert(policy.key(), policy.clone());
            }
            ResourceAccess::Deny(_) => {
                if let Some(policy) = policy {
                    self.access_policy_repository.remove(&policy.key());
                }
            }
        };

        Ok(())
    }

    pub async fn get_caller_privileges_for_access_policy(
        &self,
        resource_type: &ResourceType,
        ctx: &CallContext,
    ) -> ServiceResult<AccessPolicyCallerPrivileges> {
        Ok(AccessPolicyCallerPrivileges {
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         core::ic_cdk::api::id as self_canister_id,
//         models::{
//             access_control::{
//                 access_control_test_utils::mock_access_policy, ProposalActionSpecifier,
//                 ResourceSpecifier, UserSpecifier,
//             },
//             criteria::Criteria,
//             proposal_policy_test_utils::mock_proposal_policy,
//             specifier::ProposalSpecifier,
//             user_group_test_utils::mock_user_group,
//             user_test_utils::mock_user,
//         },
//         repositories::{USER_GROUP_REPOSITORY, USER_REPOSITORY},
//     };

//     #[tokio::test]
//     async fn test_proposal_policy_operations() {
//         let service = POLICY_SERVICE.clone();
//         let policy = service
//             .add_proposal_policy(AddProposalPolicyOperationInput {
//                 specifier: ProposalSpecifier::AddAccount,
//                 criteria: Criteria::AutoAdopted,
//             })
//             .await;

//         assert!(policy.is_ok());

//         let policy = policy.unwrap();
//         let fetched_policy = service.get_proposal_policy(&policy.id).unwrap();

//         assert_eq!(fetched_policy.specifier, policy.specifier);
//         assert_eq!(fetched_policy.criteria, policy.criteria);

//         let policy = service
//             .edit_proposal_policy(EditProposalPolicyOperationInput {
//                 policy_id: policy.id,
//                 specifier: Some(ProposalSpecifier::AddAccount),
//                 criteria: Some(Criteria::AutoAdopted),
//             })
//             .await;

//         assert!(policy.is_ok());

//         let policy = policy.unwrap();
//         let updated_policy = service.get_proposal_policy(&policy.id).unwrap();

//         assert_eq!(updated_policy.specifier, policy.specifier);
//         assert_eq!(updated_policy.criteria, policy.criteria);
//     }

//     #[tokio::test]
//     async fn test_access_policy_operations() {
//         let service = POLICY_SERVICE.clone();
//         let policy = service
//             .add_access_policy(AddAccessPolicyOperationInput {
//                 user: UserSpecifier::Any,
//                 resource: ResourceSpecifier::Proposal(ProposalActionSpecifier::List),
//             })
//             .await;

//         assert!(policy.is_ok());

//         let policy = policy.unwrap();
//         let fetched_policy = service.get_access_policy(&policy.id).unwrap();

//         assert_eq!(fetched_policy.user, policy.user);
//         assert_eq!(fetched_policy.resource, policy.resource);

//         let policy = service
//             .edit_access_policy(EditAccessPolicyOperationInput {
//                 policy_id: policy.id,
//                 user: Some(UserSpecifier::Id(vec![[1; 16]])),
//                 resource: Some(ResourceSpecifier::Proposal(ProposalActionSpecifier::List)),
//             })
//             .await;

//         assert!(policy.is_ok());

//         let policy = policy.unwrap();
//         let updated_policy = service.get_access_policy(&policy.id).unwrap();

//         assert_eq!(updated_policy.user, policy.user);
//         assert_eq!(updated_policy.resource, policy.resource);
//     }

//     #[test]
//     fn test_get_proposal_policy_not_found() {
//         let service = POLICY_SERVICE.clone();
//         let result = service.get_proposal_policy(&[1; 16]);

//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_get_access_policy_not_found() {
//         let service = POLICY_SERVICE.clone();
//         let result = service.get_access_policy(&[1; 16]);

//         assert!(result.is_err());
//     }

//     #[test]
//     fn get_access_policy_dependencies_finds_users() {
//         let service = POLICY_SERVICE.clone();
//         let mut policy = mock_access_policy();
//         policy.user = UserSpecifier::Id(vec![[1; 16]]);
//         ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());
//         let mut user = mock_user();
//         user.id = [1; 16];
//         USER_REPOSITORY.insert(user.to_key(), user.to_owned());

//         let result = service
//             .get_access_policies_dependencies(&vec![policy.to_owned()])
//             .unwrap();

//         assert_eq!(result.users.len(), 1);
//         assert_eq!(result.groups.len(), 0);
//     }

//     #[test]
//     fn get_access_policy_dependencies_finds_groups() {
//         let service = POLICY_SERVICE.clone();
//         let mut policy = mock_access_policy();
//         policy.user = UserSpecifier::Group(vec![[1; 16]]);
//         ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());
//         let mut group = mock_user_group();
//         group.id = [1; 16];
//         USER_GROUP_REPOSITORY.insert(group.id, group.to_owned());

//         let result = service
//             .get_access_policies_dependencies(&vec![policy.to_owned()])
//             .unwrap();

//         assert_eq!(result.users.len(), 0);
//         assert_eq!(result.groups.len(), 1);
//     }

//     #[tokio::test]
//     async fn list_access_policies_should_use_offset_and_limit() {
//         for i in 0..50 {
//             let mut policy = mock_access_policy();
//             policy.id = [i; 16];
//             policy.user = UserSpecifier::Id(vec![[i; 16]]);
//             ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());
//         }

//         let input = ListAccessPoliciesInput {
//             offset: Some(15),
//             limit: Some(30),
//         };

//         let result = POLICY_SERVICE
//             .list_access_policies(input, &CallContext::new(self_canister_id()))
//             .await
//             .unwrap();
//         assert_eq!(result.items.len(), 30);
//         assert_eq!(result.next_offset, Some(45));
//     }

//     #[tokio::test]
//     async fn list_proposal_policies_should_use_offset_and_limit() {
//         for i in 0..50 {
//             let mut policy = mock_proposal_policy();
//             policy.id = [i; 16];
//             policy.specifier = ProposalSpecifier::AddAccount;
//             PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());
//         }

//         let input = ListProposalPoliciesInput {
//             offset: Some(15),
//             limit: Some(30),
//         };

//         let result = POLICY_SERVICE
//             .list_proposal_policies(input, &CallContext::new(self_canister_id()))
//             .await
//             .unwrap();
//         assert_eq!(result.items.len(), 30);
//         assert_eq!(result.next_offset, Some(45));
//     }

//     #[tokio::test]
//     async fn test_remove_access_policy() {
//         let service = POLICY_SERVICE.clone();
//         let policy = service
//             .add_access_policy(AddAccessPolicyOperationInput {
//                 user: UserSpecifier::Any,
//                 resource: ResourceSpecifier::Proposal(ProposalActionSpecifier::List),
//             })
//             .await
//             .unwrap();

//         assert!(service.get_access_policy(&policy.id).is_ok());

//         service.remove_access_policy(&policy.id).await.unwrap();

//         assert!(service.get_access_policy(&policy.id).is_err());
//     }

//     #[tokio::test]
//     async fn test_remove_proposal_policy() {
//         let service = POLICY_SERVICE.clone();
//         let policy = service
//             .add_proposal_policy(AddProposalPolicyOperationInput {
//                 specifier: ProposalSpecifier::AddAccount,
//                 criteria: Criteria::AutoAdopted,
//             })
//             .await
//             .unwrap();

//         assert!(service.get_proposal_policy(&policy.id).is_ok());

//         service.remove_proposal_policy(&policy.id).await.unwrap();

//         assert!(service.get_proposal_policy(&policy.id).is_err());
//     }
// }
