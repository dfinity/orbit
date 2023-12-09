use super::{
    evaluation::{Evaluate, ACCESS_CONTROL_MATCHER},
    CallContext,
};
use crate::{
    core::ic_cdk::api::print,
    errors::{AccessControlError, EvaluateError, MatchError},
    models::{
        access_control::{
            AccessControlPolicy, AccessModifier, ResourceIdentifier, ResourceSpecifier,
        },
        specifier::{Match, UserSpecifier},
        User, UserId,
    },
    repositories::{access_control::ACCESS_CONTROL_REPOSITORY, USER_REPOSITORY},
};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use ic_canister_core::repository::Repository;
use std::{collections::HashSet, sync::Arc};

pub struct AccessControlEvaluator<'ctx> {
    pub call_context: &'ctx CallContext,
    pub resource: ResourceSpecifier,
    pub access_modifier: AccessModifier,
}

impl<'ctx> AccessControlEvaluator<'ctx> {
    pub fn new(
        call_context: &'ctx CallContext,
        resource: ResourceSpecifier,
        access_modifier: AccessModifier,
    ) -> AccessControlEvaluator<'ctx> {
        AccessControlEvaluator {
            call_context,
            resource,
            access_modifier,
        }
    }
}

/// A matcher that checks if the user has access to the given policy.
pub struct AccessControlUserMatcher;

#[async_trait]
impl Match<(Arc<User>, AccessControlPolicy)> for AccessControlUserMatcher {
    async fn is_match(&self, v: (Arc<User>, AccessControlPolicy)) -> Result<bool, MatchError> {
        let (caller, access_policy) = v;
        let is_match = match &access_policy.user {
            UserSpecifier::Group(allowed_groups) => allowed_groups.iter().any(|group| {
                caller
                    .groups
                    .iter()
                    .any(|caller_group| caller_group == group)
            }),
            UserSpecifier::Id(allowed_users) => allowed_users
                .iter()
                .any(|allowed_user_id| caller.id == *allowed_user_id),
            UserSpecifier::Any => true,
            _ => false,
        };

        Ok(is_match)
    }
}

/// A matcher that checks if the policy is applicable to the given account.
pub struct AccessControlResourceAccountMatcher;

/// A matcher that checks if the policy is applicable to the given address.
pub struct AccessControlResourceTransferAddressMatcher;

/// A matcher that checks if the policy is applicable to the given user,
/// this is not the caller, but the user that that the caller is trying to access.
pub struct AccessControlResourceUserMatcher;

#[async_trait]
impl Match<(Arc<UserSpecifier>, AccessControlPolicy)> for AccessControlResourceUserMatcher {
    async fn is_match(
        &self,
        v: (Arc<UserSpecifier>, AccessControlPolicy),
    ) -> Result<bool, MatchError> {
        let (requested_user, access_policy) = v;

        let mut requested_users = HashSet::new();
        let mut requested_user_group_ids = HashSet::new();
        if let UserSpecifier::Id(user_ids) = requested_user.as_ref() {
            requested_users = user_ids
                .iter()
                .filter_map(|user_id| USER_REPOSITORY.get(&User::key(user_id.to_owned())))
                .collect();
        } else if let UserSpecifier::Group(group_ids) = requested_user.as_ref() {
            requested_user_group_ids = group_ids.iter().cloned().collect();
        }

        if let ResourceSpecifier::User(policy_user) | ResourceSpecifier::UserStatus(policy_user) =
            access_policy.resource
        {
            let is_match = match policy_user {
                UserSpecifier::Any => true,
                UserSpecifier::Group(group_ids) => {
                    let match_requested_groups = requested_user_group_ids.iter().all(|group_id| {
                        group_ids
                            .iter()
                            .any(|policy_group_id| policy_group_id == group_id)
                    });

                    let match_requested_users = requested_users.iter().all(|requested_user| {
                        group_ids.iter().any(|group_id| {
                            requested_user
                                .groups
                                .iter()
                                .any(|requested_user_group_id| requested_user_group_id == group_id)
                        })
                    });

                    match_requested_groups && match_requested_users
                }
                UserSpecifier::Id(user_ids) => requested_users.iter().all(|requested_user| {
                    user_ids.iter().any(|user_id| *user_id == requested_user.id)
                }),
                _ => false,
            };

            return Ok(is_match);
        }

        Ok(false)
    }
}

/// A matcher that checks if the caller has access to the given resource and access modifier.
pub struct AccessControlMatcher {
    pub user_matcher: Arc<dyn Match<(Arc<User>, AccessControlPolicy)>>,
    pub user_resource_matcher: Arc<dyn Match<(Arc<UserSpecifier>, AccessControlPolicy)>>,
}

#[async_trait]
impl Match<(User, ResourceSpecifier, AccessModifier)> for AccessControlMatcher {
    async fn is_match(
        &self,
        v: (User, ResourceSpecifier, AccessModifier),
    ) -> Result<bool, MatchError> {
        let (caller, resource, required_access) = v;
        let policies =
            ACCESS_CONTROL_REPOSITORY.find_by_resource_and_access(&resource, &required_access);

        if policies.is_empty() {
            // If there is no policy for the given resource and access modifier, then the access is denied by default.
            return Ok(false);
        }

        // Filter policies based on the resource specifier, e.g. if the resource is for account_id = 1, then only
        // policies that include account_id = 1 are kept in the list of policies.
        let filtered_policies = match resource {
            ResourceSpecifier::Account(account) => policies,
            ResourceSpecifier::Transfer(account, address) => policies,
            ResourceSpecifier::User(user) | ResourceSpecifier::UserStatus(user) => {
                let requested_user = &Arc::new(user);
                stream::iter(policies.iter())
                    .filter_map(|policy| async move {
                        match self
                            .user_resource_matcher
                            .is_match((requested_user.to_owned(), policy.to_owned()))
                            .await
                        {
                            Ok(true) => Some(policy.to_owned()),
                            Ok(false) => None,
                            Err(e) => {
                                print(&format!("Failed user resource matcher: {:?}", e));

                                None
                            }
                        }
                    })
                    .collect()
                    .await
            }
            // Resources that are whole sets of data (e.g. address book, etc...) do not have a specific id,
            // so all policies are kept.
            _ => policies,
        };

        let caller_arc = &Arc::new(caller);
        let is_match = stream::iter(filtered_policies.iter())
            .then(|policy| async move {
                self.user_matcher
                    .is_match((caller_arc.to_owned(), policy.to_owned()))
                    .await
            })
            .filter_map(|result| async move {
                match result {
                    Ok(is_match) => Some(is_match),
                    Err(e) => {
                        print(&format!(
                            "Failed to match access control for caller: {:?}",
                            e
                        ));

                        None
                    }
                }
            })
            .any(|is_match| async move { is_match })
            .await;

        return Ok(is_match);
    }
}

#[async_trait]
impl Evaluate<bool> for AccessControlEvaluator<'_> {
    async fn evaluate(&self) -> Result<bool, EvaluateError> {
        if self.call_context.caller_is_controller_or_self() {
            // If the call is made by the system, then the access is granted by default.
            return Ok(true);
        }

        let is_match = ACCESS_CONTROL_MATCHER
            .is_match((
                USER_REPOSITORY
                    .find_by_identity(&self.call_context.caller())
                    .ok_or(EvaluateError::Failed {
                        reason: "User not found".to_string(),
                    })?,
                self.resource.to_owned(),
                self.access_modifier.to_owned(),
            ))
            .await
            .map_err(|e| EvaluateError::UnexpectedError(e.into()))?;

        Ok(is_match)
    }
}

/// This function checks if the user has the required access role to perform the given action.
///
/// It uses the access control policies defined in the canister configuration.
pub async fn evaluate_caller_access(
    ctx: &CallContext,
    resource: &ResourceSpecifier,
    access_modifier: &AccessModifier,
) -> Result<(), AccessControlError> {
    let evaluator =
        AccessControlEvaluator::new(ctx, resource.to_owned(), access_modifier.to_owned());
    let has_access = evaluator
        .evaluate()
        .await
        .map_err(|e| AccessControlError::UnexpectedError(e.into()))?;

    if !has_access {
        return Err(AccessControlError::Unauthorized {
            resource: resource.to_string(),
            access_modifier: access_modifier.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        models::{
            access_control::access_control_test_utils::mock_access_policy,
            user_test_utils::mock_user, ADMIN_GROUP_ID,
        },
        repositories::UserRepository,
    };
    use candid::Principal;
    use ic_canister_core::repository::Repository;

    #[tokio::test]
    async fn fail_user_has_access_to_admin_resource() {
        let mut admin_access = mock_access_policy();
        admin_access.user = UserSpecifier::Group(vec![*ADMIN_GROUP_ID]);
        admin_access.access = AccessModifier::All;
        admin_access.resource = ResourceSpecifier::AddressBook;

        ACCESS_CONTROL_REPOSITORY.insert(admin_access.id, admin_access.to_owned());

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![];

        let user_repository = UserRepository::default();
        user_repository.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(caller);
        let has_access =
            evaluate_caller_access(&ctx, &ResourceSpecifier::AddressBook, &AccessModifier::All)
                .await;

        assert!(has_access.is_err());
    }

    #[tokio::test]
    async fn admin_user_has_access_to_admin_resource() {
        let mut admin_access = mock_access_policy();
        admin_access.user = UserSpecifier::Group(vec![*ADMIN_GROUP_ID]);
        admin_access.access = AccessModifier::All;
        admin_access.resource = ResourceSpecifier::AddressBook;

        ACCESS_CONTROL_REPOSITORY.insert(admin_access.id, admin_access.to_owned());

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![*ADMIN_GROUP_ID];

        let user_repository = UserRepository::default();
        user_repository.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(caller);
        let has_access =
            evaluate_caller_access(&ctx, &ResourceSpecifier::AddressBook, &AccessModifier::All)
                .await;

        assert!(has_access.is_ok());
    }
}
