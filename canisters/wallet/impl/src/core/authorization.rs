use super::{
    evaluation::{Evaluate, PROPOSAL_MATCHER, PROPOSAL_VOTE_RIGHTS_CRITERIA_EVALUATOR},
    proposal::ProposalVoteRightsEvaluator,
    CallContext,
};
use crate::{
    errors::AuthorizationError,
    models::{
        access_policy::{
            AccountResourceAction, ChangeCanisterResourceAction, ProposalResourceAction, Resource,
            ResourceAction, ResourceId, SettingsResourceAction, UserResourceAction,
        },
        Account, ProposalKey, User, ADMIN_GROUP_ID,
    },
    repositories::{ACCOUNT_REPOSITORY, PROPOSAL_REPOSITORY},
    services::access_policy::ACCESS_POLICY_SERVICE,
};
use ic_canister_core::repository::Repository;

pub struct Authorization;

impl Authorization {
    pub fn is_allowed(ctx: &CallContext, resource: &Resource) -> bool {
        // If the call is made by the system, then the access is granted by default.
        if ctx.caller_is_controller_or_self() {
            return true;
        }

        // Gets the expanded list of resources.
        // e.g. if the resource is for account(1), then the list will expand to [account(1), account(any)]
        let resources = resource.to_expanded_list();

        // Checks if the caller has access to the requested resource.
        resources.iter().any(|resource| {
            let access_policy = ACCESS_POLICY_SERVICE.get_access_policy(resource).unwrap();

            // Checks if the resource is public, if so, then the access is granted.
            if access_policy.allowed_public() {
                return true;
            }

            if let Some(user) = ctx.user() {
                // If the user is not active, then the access is denied.
                if !user.is_active() {
                    return false;
                }

                // If the resource is available to authenticated users, then the access is granted.
                if access_policy.allowed_authenticated() {
                    return true;
                }

                // Validades if the user has access to the resource based on the default rules (non-policy based).
                if has_default_resource_access(user, resource) {
                    return true;
                }

                // Checks if the user has access to the resource based on the access policy.
                return access_policy.is_allowed(user);
            }

            false
        })
    }
}

/// Checks if the user had access to the resource based on default rules (non-policy based).
///
/// e.g. the user has access to their own user record, etc...
fn has_default_resource_access(user: &User, resource: &Resource) -> bool {
    match &resource {
        &Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(proposal_id))) => {
            if PROPOSAL_REPOSITORY.exists_voter(proposal_id, &user.id)
                || PROPOSAL_REPOSITORY.exists_proposer(proposal_id, &user.id)
            {
                return true;
            }

            if let Some(proposal) = PROPOSAL_REPOSITORY.get(&ProposalKey { id: *proposal_id }) {
                let validator = ProposalVoteRightsEvaluator {
                    proposal: &proposal,
                    voter_id: user.id,
                    proposal_matcher: PROPOSAL_MATCHER.to_owned(),
                    vote_rights_evaluator: PROPOSAL_VOTE_RIGHTS_CRITERIA_EVALUATOR.clone(),
                };

                validator.evaluate().unwrap_or(false)
            } else {
                false
            }
        }
        Resource::User(UserResourceAction::Read(ResourceId::Id(user_id))) => {
            // The user has access to their own user record.
            *user_id == user.id
        }
        Resource::Account(AccountResourceAction::Read(ResourceId::Id(account_id)))
        | Resource::Account(AccountResourceAction::Update(ResourceId::Id(account_id)))
        | Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(account_id))) => {
            // The user has access to their own account.
            if let Some(account) = ACCOUNT_REPOSITORY.get(&Account::key(account_id.to_owned())) {
                return account.owners.contains(&user.id);
            }

            false
        }
        Resource::ChangeCanister(ChangeCanisterResourceAction::Create)
        | Resource::User(UserResourceAction::Create)
        | Resource::Settings(SettingsResourceAction::Read)
        | Resource::Settings(SettingsResourceAction::ReadConfig)
        | Resource::UserGroup(ResourceAction::List)
        | Resource::UserGroup(ResourceAction::Read(_))
        | Resource::Account(AccountResourceAction::List) => {
            // admins have access to these resources by default
            user.groups.contains(ADMIN_GROUP_ID)
        }
        _ => false,
    }
}

/// This function checks if the user has the required access role to perform the given action.
///
/// It uses the access control policies defined in the canister configuration.
pub async fn evaluate_caller_access(
    ctx: &CallContext,
    resource: &Resource,
) -> Result<(), AuthorizationError> {
    let has_access = Authorization::is_allowed(ctx, resource);

    if !has_access {
        return Err(AuthorizationError::Unauthorized {
            resource: resource.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{
            access_policy::{AccessPolicy, Allow},
            account_test_utils, user_group_test_utils,
            user_test_utils::{self, mock_user},
            UserGroup, UserStatus, ADMIN_GROUP_ID,
        },
        repositories::{access_policy::ACCESS_POLICY_REPOSITORY, USER_REPOSITORY},
    };
    use candid::Principal;
    use ic_canister_core::{model::ModelKey, repository::Repository};

    struct TestContext {
        finance_user_group: UserGroup,
        finance_user: User,
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
            finance_user_group,
            finance_user,
        }
    }

    #[tokio::test]
    async fn inactive_user_has_no_access() {
        let mut test_context = setup();
        let policy = AccessPolicy::new(
            Allow::users(vec![test_context.finance_user.id]),
            Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
        );

        ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.to_owned());

        let ctx = CallContext::new(test_context.finance_user.identities[0]);

        assert!(Authorization::is_allowed(
            &ctx,
            &Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
        ));

        test_context.finance_user.status = UserStatus::Inactive;

        USER_REPOSITORY.insert(
            test_context.finance_user.to_key(),
            test_context.finance_user.clone(),
        );

        let ctx = CallContext::new(test_context.finance_user.identities[0]);

        assert!(!Authorization::is_allowed(
            &ctx,
            &Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
        ));
    }

    #[tokio::test]
    async fn fail_user_has_access_to_admin_resource() {
        let admin_access = AccessPolicy::new(
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        ACCESS_POLICY_REPOSITORY.insert(admin_access.key(), admin_access.to_owned());

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(caller);
        let has_access = Authorization::is_allowed(
            &ctx,
            &Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        assert!(!has_access);
    }

    #[tokio::test]
    async fn admin_user_has_access_to_admin_resource() {
        let admin_access = AccessPolicy::new(
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        ACCESS_POLICY_REPOSITORY.insert(admin_access.key(), admin_access.to_owned());

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![*ADMIN_GROUP_ID];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(caller);
        let has_access = Authorization::is_allowed(
            &ctx,
            &Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        assert!(has_access);
    }

    #[tokio::test]
    async fn user_has_access_to_admin_resource() {
        let admin_access = AccessPolicy::new(
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        ACCESS_POLICY_REPOSITORY.insert(admin_access.key(), admin_access.to_owned());

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![*ADMIN_GROUP_ID];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(caller);
        let has_access = Authorization::is_allowed(
            &ctx,
            &Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        assert!(has_access);
    }

    #[tokio::test]
    async fn group_with_read_access_should_not_have_other_access() {
        let test_context = setup();

        // add finance read access to address book
        let policy = AccessPolicy::new(
            Allow::user_groups(vec![test_context.finance_user_group.id]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.to_owned());

        assert!(Authorization::is_allowed(
            &CallContext::new(test_context.finance_user.identities[0]),
            &Resource::AddressBook(ResourceAction::Read(ResourceId::Any))
        ));
        assert!(!Authorization::is_allowed(
            &CallContext::new(test_context.finance_user.identities[0]),
            &Resource::AddressBook(ResourceAction::Update(ResourceId::Any))
        ));
        assert!(!Authorization::is_allowed(
            &CallContext::new(test_context.finance_user.identities[0]),
            &Resource::AddressBook(ResourceAction::Create)
        ));
        assert!(!Authorization::is_allowed(
            &CallContext::new(test_context.finance_user.identities[0]),
            &Resource::AddressBook(ResourceAction::Delete(ResourceId::Any))
        ));
    }

    #[tokio::test]
    async fn group_has_access_to_resource_by_id() {
        let test_context = setup();
        let policy = AccessPolicy::new(
            Allow::user_groups(vec![test_context.finance_user_group.id]),
            Resource::Account(AccountResourceAction::Read(ResourceId::Id([1; 16]))),
        );

        account_test_utils::add_account(&[1; 16]);
        account_test_utils::add_account(&[2; 16]);

        ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.to_owned());

        assert!(Authorization::is_allowed(
            &CallContext::new(test_context.finance_user.identities[0]),
            &Resource::Account(AccountResourceAction::Read(ResourceId::Id([1; 16])))
        ));
        assert!(!Authorization::is_allowed(
            &CallContext::new(test_context.finance_user.identities[0]),
            &Resource::Account(AccountResourceAction::Read(ResourceId::Id([2; 16])))
        ));
        assert!(!Authorization::is_allowed(
            &CallContext::new(test_context.finance_user.identities[0]),
            &Resource::Account(AccountResourceAction::Read(ResourceId::Any))
        ));
    }

    #[tokio::test]
    async fn user_has_access_to_resource_by_id() {
        let user = user_test_utils::add_user(&[1; 16]);
        let policy = AccessPolicy::new(
            Allow::users(vec![user.id]),
            Resource::User(UserResourceAction::Read(ResourceId::Id([1; 16]))),
        );

        ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.to_owned());

        assert!(Authorization::is_allowed(
            &CallContext::new(user.identities[0]),
            &Resource::User(UserResourceAction::Read(ResourceId::Id([1; 16])))
        ));
        assert!(!Authorization::is_allowed(
            &CallContext::new(user.identities[0]),
            &Resource::User(UserResourceAction::Read(ResourceId::Id([2; 16])))
        ));
        assert!(!Authorization::is_allowed(
            &CallContext::new(user.identities[0]),
            &Resource::User(UserResourceAction::Read(ResourceId::Any))
        ));
    }

    #[tokio::test]
    async fn user_has_access_to_any() {
        let user = user_test_utils::add_user(&[1; 16]);
        let policy = AccessPolicy::new(
            Allow::users(vec![user.id]),
            Resource::User(UserResourceAction::Read(ResourceId::Any)),
        );

        ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.to_owned());

        assert!(Authorization::is_allowed(
            &CallContext::new(user.identities[0]),
            &Resource::User(UserResourceAction::Read(ResourceId::Id([1; 16])))
        ));
        assert!(Authorization::is_allowed(
            &CallContext::new(user.identities[0]),
            &Resource::User(UserResourceAction::Read(ResourceId::Id([2; 16])))
        ));
        assert!(Authorization::is_allowed(
            &CallContext::new(user.identities[0]),
            &Resource::User(UserResourceAction::Read(ResourceId::Any))
        ));
    }
}
