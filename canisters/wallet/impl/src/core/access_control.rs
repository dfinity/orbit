use super::{
    evaluation::{
        Evaluate, ACCESS_CONTROL_DEFAULT_ACCESS_MATCHER, ACCESS_CONTROL_MATCHER, PROPOSAL_MATCHER,
        PROPOSAL_VOTE_RIGHTS_CRITERIA_EVALUATOR,
    },
    proposal::ProposalVoteRightsEvaluator,
    CallContext,
};
use crate::{
    core::ic_cdk::api::{print, trap},
    errors::{AccessControlError, EvaluateError, MatchError},
    models::{
        access_control::{
            AccessControlPolicy, AccountSpecifier, CanisterSettingsActionSpecifier,
            ChangeCanisterActionSpecifier, CommonActionSpecifier, ProposalActionSpecifier,
            ResourceSpecifier, ResourceType, TransferActionSpecifier, UserSpecifier,
        },
        specifier::{CommonSpecifier, Match},
        Account, Proposal, User, UserStatus, ADMIN_GROUP_ID,
    },
    repositories::{
        access_control::ACCESS_CONTROL_REPOSITORY, ACCOUNT_REPOSITORY, PROPOSAL_REPOSITORY,
        USER_REPOSITORY,
    },
};
use async_trait::async_trait;
use futures::{stream, StreamExt, TryStreamExt};
use ic_canister_core::repository::Repository;
use std::{collections::HashSet, sync::Arc};

pub struct AccessControlEvaluator<'ctx> {
    pub call_context: &'ctx CallContext,
    pub resource: ResourceSpecifier,
}

impl<'ctx> AccessControlEvaluator<'ctx> {
    pub fn new(
        call_context: &'ctx CallContext,
        resource: ResourceSpecifier,
    ) -> AccessControlEvaluator<'ctx> {
        AccessControlEvaluator {
            call_context,
            resource,
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
        };

        Ok(is_match)
    }
}

/// A matcher that checks if the policy is applicable to the given user,
/// this is not the caller, but the user that that the caller is trying to access.
pub struct AccessControlPolicyUserMatcher;

#[async_trait]
impl Match<(Arc<UserSpecifier>, AccessControlPolicy)> for AccessControlPolicyUserMatcher {
    async fn is_match(
        &self,
        v: (Arc<UserSpecifier>, AccessControlPolicy),
    ) -> Result<bool, MatchError> {
        let (requested_user, access_policy) = v;

        let mut requested_users = HashSet::new();
        let mut requested_user_group_ids = HashSet::new();
        if let CommonSpecifier::Id(user_ids) = requested_user.as_ref() {
            requested_users = user_ids
                .iter()
                .filter_map(|user_id| USER_REPOSITORY.get(&User::key(user_id.to_owned())))
                .collect();
        } else if let CommonSpecifier::Group(group_ids) = requested_user.as_ref() {
            requested_user_group_ids = group_ids.iter().cloned().collect();
        }

        if let ResourceSpecifier::Common(
            ResourceType::User,
            CommonActionSpecifier::Read(policy_user),
        )
        | ResourceSpecifier::Common(
            ResourceType::User,
            CommonActionSpecifier::Update(policy_user),
        )
        | ResourceSpecifier::Common(
            ResourceType::User,
            CommonActionSpecifier::Delete(policy_user),
        ) = access_policy.resource
        {
            let is_match = match policy_user {
                UserSpecifier::Any => true,
                UserSpecifier::Group(group_ids) => {
                    if requested_user_group_ids.is_empty() && requested_users.is_empty() {
                        // If the requested users and groups are empty, then by default we filter out the policy,
                        // otherwise this would allow access when requested for an empty list of users and groups.
                        return Ok(false);
                    }

                    let match_requested_groups = requested_user_group_ids
                        .iter()
                        .all(|group_id| group_ids.contains(group_id));

                    let match_requested_users = requested_users.iter().all(|requested_user| {
                        group_ids
                            .iter()
                            .any(|group_id| requested_user.groups.contains(group_id))
                    });

                    match_requested_groups && match_requested_users
                }
                UserSpecifier::Id(user_ids) => match requested_users.is_empty() {
                    // If the requested users is empty, then by default we filter out the policy, otherwise this would
                    // allow access when requested for an empty list of users.
                    true => false,
                    _ => requested_users.iter().all(|requested_user| {
                        user_ids.iter().any(|user_id| *user_id == requested_user.id)
                    }),
                },
            };

            return Ok(is_match);
        }

        Ok(false)
    }
}

/// A matcher that checks if the policy is applicable to the given account.
pub struct AccessControlPolicyAccountMatcher;

#[async_trait]
impl Match<(Arc<AccountSpecifier>, AccessControlPolicy)> for AccessControlPolicyAccountMatcher {
    async fn is_match(
        &self,
        v: (Arc<AccountSpecifier>, AccessControlPolicy),
    ) -> Result<bool, MatchError> {
        let (requested_account, access_policy) = v;

        let mut requested_accounts = HashSet::new();
        let mut requested_account_group_ids = HashSet::new();
        if let CommonSpecifier::Id(account_ids) = requested_account.as_ref() {
            requested_accounts = account_ids
                .iter()
                .filter_map(|account_id| {
                    ACCOUNT_REPOSITORY.get(&Account::key(account_id.to_owned()))
                })
                .collect();
        } else if let CommonSpecifier::Group(group_ids) = requested_account.as_ref() {
            requested_account_group_ids = group_ids.iter().cloned().collect();
        }

        if let ResourceSpecifier::Common(
            ResourceType::Account,
            CommonActionSpecifier::Read(policy_account),
        )
        | ResourceSpecifier::Common(
            ResourceType::Account,
            CommonActionSpecifier::Update(policy_account),
        )
        | ResourceSpecifier::Common(
            ResourceType::Account,
            CommonActionSpecifier::Delete(policy_account),
        )
        | ResourceSpecifier::Transfer(TransferActionSpecifier::Create(policy_account))
        | ResourceSpecifier::Transfer(TransferActionSpecifier::Read(policy_account))
        | ResourceSpecifier::Transfer(TransferActionSpecifier::Delete(policy_account)) =
            access_policy.resource
        {
            let is_match = match policy_account {
                AccountSpecifier::Any => true,
                AccountSpecifier::Group(group_ids) => {
                    if requested_account_group_ids.is_empty() && requested_accounts.is_empty() {
                        // If the requested accounts and groups are empty, then by default we filter out the policy,
                        // otherwise this would allow access when requested for an empty list of accounts and groups.
                        return Ok(false);
                    }

                    let all_requested_groups_matched =
                        requested_account_group_ids.iter().all(|group_id| {
                            group_ids
                                .iter()
                                .any(|policy_group_id| policy_group_id == group_id)
                        });

                    all_requested_groups_matched
                }
                AccountSpecifier::Id(policy_account_ids) => match requested_accounts.is_empty() {
                    // If the requested accounts is empty, then by default we filter out the policy, otherwise this would
                    // allow access when requested for an empty list of accounts.
                    true => false,
                    _ => requested_accounts.iter().all(|requested_account| {
                        policy_account_ids
                            .iter()
                            .any(|account_id| *account_id == requested_account.id)
                    }),
                },
            };

            return Ok(is_match);
        }

        Ok(false)
    }
}

/// A matcher that checks if the caller has access to the given resource and access modifier.
pub struct AccessControlPolicyMatcher {
    pub user_matcher: Arc<dyn Match<(Arc<User>, AccessControlPolicy)>>,
    pub policy_user_matcher: Arc<dyn Match<(Arc<CommonSpecifier>, AccessControlPolicy)>>,
    pub policy_account_matcher: Arc<dyn Match<(Arc<CommonSpecifier>, AccessControlPolicy)>>,
}

#[async_trait]
impl Match<(User, ResourceSpecifier)> for AccessControlPolicyMatcher {
    async fn is_match(&self, v: (User, ResourceSpecifier)) -> Result<bool, MatchError> {
        let (caller, requested_resource) = v;
        let policies = ACCESS_CONTROL_REPOSITORY.find_by_resource(&requested_resource);

        // Filter policies based on the resource specifier, e.g. if the resource is for account_id = 1, then only
        // policies that include account_id = 1 are kept in the list of policies.
        let filtered_policies = match requested_resource {
            ResourceSpecifier::Common(
                ResourceType::Account,
                CommonActionSpecifier::Read(account),
            )
            | ResourceSpecifier::Common(
                ResourceType::Account,
                CommonActionSpecifier::Update(account),
            )
            | ResourceSpecifier::Common(
                ResourceType::Account,
                CommonActionSpecifier::Delete(account),
            )
            | ResourceSpecifier::Transfer(TransferActionSpecifier::Create(account))
            | ResourceSpecifier::Transfer(TransferActionSpecifier::Read(account))
            | ResourceSpecifier::Transfer(TransferActionSpecifier::Delete(account)) => {
                let requested_account = &Arc::new(account);
                stream::iter(policies.iter())
                    .filter_map(|policy| async move {
                        match self
                            .policy_account_matcher
                            .is_match((requested_account.to_owned(), policy.to_owned()))
                            .await
                        {
                            Ok(true) => Some(policy.to_owned()),
                            Ok(false) => None,
                            Err(e) => {
                                print(format!("Failed policy account matcher: {:?}", e));

                                None
                            }
                        }
                    })
                    .collect()
                    .await
            }
            ResourceSpecifier::Common(ResourceType::User, CommonActionSpecifier::Read(user))
            | ResourceSpecifier::Common(ResourceType::User, CommonActionSpecifier::Update(user))
            | ResourceSpecifier::Common(ResourceType::User, CommonActionSpecifier::Delete(user)) => {
                let requested_user = &Arc::new(user);
                stream::iter(policies.iter())
                    .filter_map(|policy| async move {
                        match self
                            .policy_user_matcher
                            .is_match((requested_user.to_owned(), policy.to_owned()))
                            .await
                        {
                            Ok(true) => Some(policy.to_owned()),
                            Ok(false) => None,
                            Err(e) => {
                                print(format!("Failed user resource matcher: {:?}", e));

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

        let caller_arc: &Arc<User> = &Arc::new(caller);

        for policy in &filtered_policies {
            match self
                .user_matcher
                .is_match((caller_arc.to_owned(), policy.to_owned()))
                .await
            {
                Ok(true) => return Ok(true),
                Ok(false) => continue,
                Err(e) => {
                    print(format!(
                        "Failed to match access control for caller: {:?}",
                        e
                    ));

                    continue;
                }
            }
        }

        Ok(false)
    }
}

pub struct AccessControlDefaultAccessMatcher;

#[async_trait]
impl Match<(User, ResourceSpecifier)> for AccessControlDefaultAccessMatcher {
    async fn is_match(&self, v: (User, ResourceSpecifier)) -> Result<bool, MatchError> {
        let (caller, requested_resource) = v;

        let is_match = match &requested_resource {
            ResourceSpecifier::Proposal(ProposalActionSpecifier::Read(CommonSpecifier::Id(
                ids,
            ))) => {
                let proposals = ids
                    .iter()
                    .filter_map(|id| PROPOSAL_REPOSITORY.get(&Proposal::key(id.to_owned())))
                    .collect::<Vec<_>>();

                if proposals.len() != ids.len() {
                    // If the number of proposals found is not equal to the number of requested proposals, then
                    // the caller does not have access to all of them.
                    return Ok(false);
                }

                let evaluations = stream::iter(proposals.iter())
                    .then(|proposal| async move {
                        // This is added to always allow view access to proposal owners and voters.
                        if proposal.proposed_by == caller.id
                            || proposal.voters().iter().any(|owner| owner == &caller.id)
                        {
                            return Ok(true);
                        }

                        // This is added to always allow view access to anyone that has voting rights on the proposal.
                        let validator = ProposalVoteRightsEvaluator {
                            proposal,
                            voter_id: caller.id,
                            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
                            vote_rights_evaluator: PROPOSAL_VOTE_RIGHTS_CRITERIA_EVALUATOR.clone(),
                        };

                        validator.evaluate().await
                    })
                    .try_collect::<Vec<bool>>()
                    .await
                    .map_err(|e| MatchError::UnexpectedError(e.into()))?;

                evaluations.iter().all(|evaluation| *evaluation)
            }
            ResourceSpecifier::Common(
                ResourceType::User,
                CommonActionSpecifier::Read(CommonSpecifier::Id(ids)),
            ) => {
                let users = ids
                    .iter()
                    .filter_map(|id| USER_REPOSITORY.get(&User::key(id.to_owned())))
                    .collect::<Vec<_>>();

                if users.len() != ids.len() {
                    // If the number of users found is not equal to the number of requested users, then
                    // the caller does not have access to all of them.
                    return Ok(false);
                }

                users.iter().all(|user| user.id == caller.id)
            }
            ResourceSpecifier::Common(
                ResourceType::Account,
                CommonActionSpecifier::Read(CommonSpecifier::Id(ids)),
            )
            | ResourceSpecifier::Common(
                ResourceType::Account,
                CommonActionSpecifier::Update(CommonSpecifier::Id(ids)),
            ) => {
                let accounts = ids
                    .iter()
                    .filter_map(|id| ACCOUNT_REPOSITORY.get(&Account::key(id.to_owned())))
                    .collect::<Vec<_>>();

                if accounts.len() != ids.len() {
                    // If the number of accounts found is not equal to the number of requested accounts, then
                    // the caller does not have access to all of them.
                    return Ok(false);
                }

                accounts
                    .iter()
                    .all(|account| account.owners.contains(&caller.id))
            }
            ResourceSpecifier::Transfer(TransferActionSpecifier::Read(CommonSpecifier::Id(
                account_ids,
            )))
            | ResourceSpecifier::Transfer(TransferActionSpecifier::Create(CommonSpecifier::Id(
                account_ids,
            ))) => {
                let accounts = account_ids
                    .iter()
                    .map(|account_id| {
                        ACCOUNT_REPOSITORY
                            .get(&Account::key(*account_id))
                            .unwrap_or_else(|| trap("Failed to get accounts"))
                    })
                    .collect::<Vec<Account>>();

                accounts
                    .iter()
                    .all(|account| account.owners.contains(&caller.id))
            }
            ResourceSpecifier::ChangeCanister(ChangeCanisterActionSpecifier::Create)
            | ResourceSpecifier::Common(ResourceType::User, CommonActionSpecifier::Create)
            | ResourceSpecifier::CanisterSettings(CanisterSettingsActionSpecifier::Read)
            | ResourceSpecifier::CanisterSettings(CanisterSettingsActionSpecifier::ReadConfig)
            | ResourceSpecifier::Common(ResourceType::UserGroup, CommonActionSpecifier::List)
            | ResourceSpecifier::Common(ResourceType::UserGroup, CommonActionSpecifier::Read(_))
            | ResourceSpecifier::Common(ResourceType::Account, CommonActionSpecifier::List) => {
                // admins have access to these resources by default
                caller.groups.contains(ADMIN_GROUP_ID)
            }
            _ => false,
        };

        Ok(is_match)
    }
}

#[async_trait]
impl Evaluate<bool> for AccessControlEvaluator<'_> {
    async fn evaluate(&self) -> Result<bool, EvaluateError> {
        if self.call_context.caller_is_controller_or_self() {
            // If the call is made by the system, then the access is granted by default.
            return Ok(true);
        }

        let user = USER_REPOSITORY
            .find_by_identity(&self.call_context.caller())
            .ok_or(EvaluateError::Failed {
                reason: "User not found".to_string(),
            })?;

        let is_user_active = user.status == UserStatus::Active;
        if !is_user_active {
            return Ok(false);
        }

        let is_resource_owner = ACCESS_CONTROL_DEFAULT_ACCESS_MATCHER
            .is_match((user.to_owned(), self.resource.to_owned()))
            .await
            .map_err(|e| EvaluateError::UnexpectedError(e.into()))?;

        // If the user is the owner of the resource, then the access is granted by default.
        if is_resource_owner {
            return Ok(true);
        }

        let is_match = ACCESS_CONTROL_MATCHER
            .is_match((user.to_owned(), self.resource.to_owned()))
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
) -> Result<(), AccessControlError> {
    let evaluator = AccessControlEvaluator::new(ctx, resource.to_owned());
    let has_access = evaluator
        .evaluate()
        .await
        .map_err(|e| AccessControlError::UnexpectedError(e.into()))?;

    if !has_access {
        return Err(AccessControlError::Unauthorized {
            resource: resource.to_key(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        access_control::{
            access_control_test_utils::mock_access_policy, AddressBookActionSpecifier,
        },
        account_test_utils::{self},
        user_group_test_utils,
        user_test_utils::{self, mock_user},
        UserGroup, ADMIN_GROUP_ID,
    };
    use candid::Principal;
    use ic_canister_core::repository::Repository;

    struct TestContext {
        admin_user_group: UserGroup,
        finance_user_group: UserGroup,
        finance_user: User,
        hr_user: User,
    }

    fn setup() -> TestContext {
        let admin_user_group = UserGroup {
            id: *ADMIN_GROUP_ID,
            name: "Admin".to_string(),
            last_modification_timestamp: 0,
        };
        let finance_user_group = user_group_test_utils::add_group("finance");
        let hr_user_group = user_group_test_utils::add_group("hr");
        let mut admin_user = mock_user();
        admin_user.id = [0; 16];
        admin_user.identities = vec![Principal::from_slice(&[1; 29])];
        admin_user.groups = vec![admin_user_group.id];
        let mut finance_user = mock_user();
        finance_user.id = [2; 16];
        finance_user.identities = vec![Principal::from_slice(&[2; 29])];
        finance_user.groups = vec![finance_user_group.id];
        let mut hr_user = mock_user();
        hr_user.id = [3; 16];
        hr_user.identities = vec![Principal::from_slice(&[3; 29])];
        hr_user.groups = vec![hr_user_group.id];

        USER_REPOSITORY.insert(admin_user.to_key(), admin_user.to_owned());
        USER_REPOSITORY.insert(finance_user.to_key(), finance_user.to_owned());
        USER_REPOSITORY.insert(hr_user.to_key(), hr_user.to_owned());

        TestContext {
            admin_user_group,
            finance_user_group,
            finance_user,
            hr_user,
        }
    }

    #[tokio::test]
    async fn inactive_user_has_no_access() {
        let mut test_context = setup();
        let mut policy = mock_access_policy();
        policy.id = [10; 16];
        policy.user = UserSpecifier::Id(vec![test_context.finance_user.id]);
        policy.resource = ResourceSpecifier::Common(
            ResourceType::Account,
            CommonActionSpecifier::Read(CommonSpecifier::Any),
        );

        ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

        let ctx = CallContext::new(test_context.finance_user.identities[0]);

        assert!(evaluate_caller_access(
            &ctx,
            &ResourceSpecifier::Common(
                ResourceType::Account,
                CommonActionSpecifier::Read(CommonSpecifier::Any),
            ),
        )
        .await
        .is_ok());

        test_context.finance_user.status = UserStatus::Inactive;

        USER_REPOSITORY.insert(
            test_context.finance_user.to_key(),
            test_context.finance_user.clone(),
        );

        assert!(evaluate_caller_access(
            &ctx,
            &ResourceSpecifier::Common(
                ResourceType::Account,
                CommonActionSpecifier::Read(CommonSpecifier::Any),
            ),
        )
        .await
        .is_err());
    }

    #[tokio::test]
    async fn fail_user_has_access_to_admin_resource() {
        let mut admin_access = mock_access_policy();
        admin_access.user = UserSpecifier::Group(vec![*ADMIN_GROUP_ID]);
        admin_access.resource = ResourceSpecifier::Common(
            ResourceType::AddressBook,
            AddressBookActionSpecifier::Read(CommonSpecifier::Any),
        );

        ACCESS_CONTROL_REPOSITORY.insert(admin_access.id, admin_access.to_owned());

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(caller);
        let has_access = evaluate_caller_access(
            &ctx,
            &ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Read(CommonSpecifier::Any),
            ),
        )
        .await;

        assert!(has_access.is_err());
    }

    #[tokio::test]
    async fn admin_user_has_access_to_admin_resource() {
        let mut admin_access = mock_access_policy();
        admin_access.user = UserSpecifier::Group(vec![*ADMIN_GROUP_ID]);
        admin_access.resource = ResourceSpecifier::Common(
            ResourceType::AddressBook,
            AddressBookActionSpecifier::Read(CommonSpecifier::Any),
        );

        ACCESS_CONTROL_REPOSITORY.insert(admin_access.id, admin_access.to_owned());

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![*ADMIN_GROUP_ID];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(caller);
        let has_access = evaluate_caller_access(
            &ctx,
            &ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Read(CommonSpecifier::Any),
            ),
        )
        .await;

        assert!(has_access.is_ok());
    }

    #[tokio::test]
    async fn user_has_access_to_admin_resource() {
        let mut admin_access = mock_access_policy();
        admin_access.user = UserSpecifier::Group(vec![*ADMIN_GROUP_ID]);
        admin_access.resource = ResourceSpecifier::Common(
            ResourceType::AddressBook,
            AddressBookActionSpecifier::Read(CommonSpecifier::Any),
        );

        ACCESS_CONTROL_REPOSITORY.insert(admin_access.id, admin_access.to_owned());

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![*ADMIN_GROUP_ID];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(caller);
        let has_access = evaluate_caller_access(
            &ctx,
            &ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Read(CommonSpecifier::Any),
            ),
        )
        .await;

        assert!(has_access.is_ok());
    }

    #[tokio::test]
    async fn group_with_read_access_should_not_have_other_access() {
        let test_context = setup();

        // add finance read access to address book
        let mut policy = mock_access_policy();
        policy.id = [10; 16];
        policy.user = UserSpecifier::Group(vec![test_context.finance_user_group.id]);
        policy.resource = ResourceSpecifier::Common(
            ResourceType::AddressBook,
            AddressBookActionSpecifier::Read(CommonSpecifier::Any),
        );

        ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

        assert!(evaluate_caller_access(
            &CallContext::new(test_context.finance_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Read(CommonSpecifier::Any),
            )
        )
        .await
        .is_ok());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.finance_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Update(CommonSpecifier::Any),
            )
        )
        .await
        .is_err());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.finance_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Delete(CommonSpecifier::Any),
            )
        )
        .await
        .is_err());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.finance_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::AddressBook,
                AddressBookActionSpecifier::Create,
            )
        )
        .await
        .is_err());
    }

    #[tokio::test]
    async fn group_has_access_to_resource_by_id() {
        let test_context = setup();

        // group should have acess
        let mut policy = mock_access_policy();
        policy.id = [10; 16];
        policy.user = UserSpecifier::Group(vec![test_context.finance_user_group.id]);
        policy.resource = ResourceSpecifier::Common(
            ResourceType::Account,
            AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16]])),
        );

        account_test_utils::add_account(&[1; 16]);
        account_test_utils::add_account(&[2; 16]);

        ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

        assert!(evaluate_caller_access(
            &CallContext::new(test_context.finance_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::Account,
                AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16],])),
            )
        )
        .await
        .is_ok());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.finance_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::Account,
                AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[2; 16],])),
            )
        )
        .await
        .is_err());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.finance_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::Account,
                AddressBookActionSpecifier::Read(CommonSpecifier::Any),
            )
        )
        .await
        .is_err());
    }

    #[tokio::test]
    async fn user_has_access_to_resource_by_id() {
        let test_context = setup();

        // group should have acess
        let mut policy = mock_access_policy();
        policy.id = [10; 16];
        policy.user = UserSpecifier::Id(vec![test_context.hr_user.id]);
        policy.resource = ResourceSpecifier::Common(
            ResourceType::User,
            AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16]])),
        );

        user_test_utils::add_user(&[1; 16]);
        user_test_utils::add_user(&[2; 16]);

        ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

        assert!(evaluate_caller_access(
            &CallContext::new(test_context.hr_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::User,
                AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16]])),
            )
        )
        .await
        .is_ok());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.hr_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::User,
                AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[2; 16]])),
            )
        )
        .await
        .is_err());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.hr_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::User,
                AddressBookActionSpecifier::Read(CommonSpecifier::Any),
            )
        )
        .await
        .is_err());
    }

    #[tokio::test]
    async fn user_has_access_to_any() {
        let test_context = setup();

        // group should have acess
        let mut policy = mock_access_policy();
        policy.id = [10; 16];
        policy.user = UserSpecifier::Id(vec![test_context.hr_user.id]);
        policy.resource = ResourceSpecifier::Common(
            ResourceType::User,
            AddressBookActionSpecifier::Read(CommonSpecifier::Any),
        );

        user_test_utils::add_user(&[1; 16]);
        user_test_utils::add_user(&[2; 16]);

        ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

        assert!(evaluate_caller_access(
            &CallContext::new(test_context.hr_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::User,
                AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16]])),
            )
        )
        .await
        .is_ok());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.hr_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::User,
                AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[2; 16]])),
            )
        )
        .await
        .is_ok());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.hr_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::User,
                AddressBookActionSpecifier::Read(CommonSpecifier::Group(vec![
                    test_context.admin_user_group.id
                ])),
            )
        )
        .await
        .is_ok());
        assert!(evaluate_caller_access(
            &CallContext::new(test_context.hr_user.identities[0]),
            &ResourceSpecifier::Common(
                ResourceType::User,
                AddressBookActionSpecifier::Read(CommonSpecifier::Any),
            )
        )
        .await
        .is_ok());
    }
}
