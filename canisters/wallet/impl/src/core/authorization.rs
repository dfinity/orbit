use super::CallContext;
use crate::{
    errors::AuthorizationError,
    models::{
        access_policy::{
            AccountResourceAction, AllowKey, ChangeCanisterResourceAction, ProposalResourceAction,
            Resource, ResourceAction, ResourceId, SettingsResourceAction, UserResourceAction,
        },
        Account, User, UserStatus, ADMIN_GROUP_ID,
    },
    repositories::{
        access_policy::ACCESS_POLICY_REPOSITORY, ACCOUNT_REPOSITORY, PROPOSAL_REPOSITORY,
        USER_REPOSITORY,
    },
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

        // Checks if the resource is public, if so, then the access is granted.
        if resources.iter().any(|resource| {
            ACCESS_POLICY_REPOSITORY.exists_by_resource_and_allowed_type(&resource, &AllowKey::Any)
        }) {
            return true;
        }

        // If the user is not active, then the access is denied.
        if !USER_REPOSITORY.exists_by_identity_and_status(&ctx.caller(), &UserStatus::Active) {
            return false;
        }

        // If the resource is available to authenticated users, then the access is granted.
        if resources.iter().any(|resource| {
            ACCESS_POLICY_REPOSITORY
                .exists_by_resource_and_allowed_type(&resource, &AllowKey::Authenticated)
        }) {
            return true;
        }

        let user = USER_REPOSITORY.find_by_identity(&ctx.caller()).unwrap();

        // Validades if the user has access to the resource based on the default rules (non-policy based).
        if resources
            .iter()
            .any(|resource| has_default_resource_access(&user, &resource))
        {
            return true;
        }

        if is_user_group_allowed(&user, &resources) || is_user_allowed(&user, &resources) {
            return true;
        }

        false
    }
}

/// Checks if the user is allowed to access any of the provided resources based on the
/// user groups associated with the user.
fn is_user_group_allowed(user: &User, resource: &Vec<Resource>) -> bool {
    for resource in resource {
        if let Some(policy) = ACCESS_POLICY_REPOSITORY
            .find_by_resource_and_allowed_type(&resource, &AllowKey::UserGroups)
        {
            if policy.is_allowed(&user) {
                return true;
            }
        }
    }

    false
}

/// Checks if the user is allowed to access the any of the provided resources, based on the
/// specific user id associated with the resource access policy.
fn is_user_allowed(user: &User, resource: &Vec<Resource>) -> bool {
    for resource in resource {
        if let Some(policy) =
            ACCESS_POLICY_REPOSITORY.find_by_resource_and_allowed_type(&resource, &AllowKey::Users)
        {
            if policy.is_allowed(&user) {
                return true;
            }
        }
    }

    false
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

            // TODO: add check if the user has voting rights on the proposal.

            false
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
            // todo: add resource name
            resource: "".to_string(),
        });
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::models::{
//         access_control::{
//             access_control_test_utils::mock_access_policy, AddressBookActionSpecifier,
//         },
//         account_test_utils::{self},
//         user_group_test_utils,
//         user_test_utils::{self, mock_user},
//         UserGroup, ADMIN_GROUP_ID,
//     };
//     use candid::Principal;
//     use ic_canister_core::repository::Repository;

//     struct TestContext {
//         admin_user_group: UserGroup,
//         finance_user_group: UserGroup,
//         finance_user: User,
//         hr_user: User,
//     }

//     fn setup() -> TestContext {
//         let admin_user_group = UserGroup {
//             id: *ADMIN_GROUP_ID,
//             name: "Admin".to_string(),
//             last_modification_timestamp: 0,
//         };
//         let finance_user_group = user_group_test_utils::add_group("finance");
//         let hr_user_group = user_group_test_utils::add_group("hr");
//         let mut admin_user = mock_user();
//         admin_user.id = [0; 16];
//         admin_user.identities = vec![Principal::from_slice(&[1; 29])];
//         admin_user.groups = vec![admin_user_group.id];
//         let mut finance_user = mock_user();
//         finance_user.id = [2; 16];
//         finance_user.identities = vec![Principal::from_slice(&[2; 29])];
//         finance_user.groups = vec![finance_user_group.id];
//         let mut hr_user = mock_user();
//         hr_user.id = [3; 16];
//         hr_user.identities = vec![Principal::from_slice(&[3; 29])];
//         hr_user.groups = vec![hr_user_group.id];

//         USER_REPOSITORY.insert(admin_user.to_key(), admin_user.to_owned());
//         USER_REPOSITORY.insert(finance_user.to_key(), finance_user.to_owned());
//         USER_REPOSITORY.insert(hr_user.to_key(), hr_user.to_owned());

//         TestContext {
//             admin_user_group,
//             finance_user_group,
//             finance_user,
//             hr_user,
//         }
//     }

//     #[tokio::test]
//     async fn inactive_user_has_no_access() {
//         let mut test_context = setup();
//         let mut policy = mock_access_policy();
//         policy.id = [10; 16];
//         policy.user = UserSpecifier::Id(vec![test_context.finance_user.id]);
//         policy.resource = ResourceSpecifier::Common(
//             ResourceType::Account,
//             CommonActionSpecifier::Read(CommonSpecifier::Any),
//         );

//         ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

//         let ctx = CallContext::new(test_context.finance_user.identities[0]);

//         assert!(evaluate_caller_access(
//             &ctx,
//             &ResourceSpecifier::Common(
//                 ResourceType::Account,
//                 CommonActionSpecifier::Read(CommonSpecifier::Any),
//             ),
//         )
//         .await
//         .is_ok());

//         test_context.finance_user.status = UserStatus::Inactive;

//         USER_REPOSITORY.insert(
//             test_context.finance_user.to_key(),
//             test_context.finance_user.clone(),
//         );

//         assert!(evaluate_caller_access(
//             &ctx,
//             &ResourceSpecifier::Common(
//                 ResourceType::Account,
//                 CommonActionSpecifier::Read(CommonSpecifier::Any),
//             ),
//         )
//         .await
//         .is_err());
//     }

//     #[tokio::test]
//     async fn fail_user_has_access_to_admin_resource() {
//         let mut admin_access = mock_access_policy();
//         admin_access.user = UserSpecifier::Group(vec![*ADMIN_GROUP_ID]);
//         admin_access.resource = ResourceSpecifier::Common(
//             ResourceType::AddressBook,
//             AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//         );

//         ACCESS_CONTROL_REPOSITORY.insert(admin_access.id, admin_access.to_owned());

//         let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
//         let mut user = mock_user();
//         user.identities = vec![caller];
//         user.groups = vec![];

//         USER_REPOSITORY.insert(user.to_key(), user.clone());

//         let ctx = CallContext::new(caller);
//         let has_access = evaluate_caller_access(
//             &ctx,
//             &ResourceSpecifier::Common(
//                 ResourceType::AddressBook,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//             ),
//         )
//         .await;

//         assert!(has_access.is_err());
//     }

//     #[tokio::test]
//     async fn admin_user_has_access_to_admin_resource() {
//         let mut admin_access = mock_access_policy();
//         admin_access.user = UserSpecifier::Group(vec![*ADMIN_GROUP_ID]);
//         admin_access.resource = ResourceSpecifier::Common(
//             ResourceType::AddressBook,
//             AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//         );

//         ACCESS_CONTROL_REPOSITORY.insert(admin_access.id, admin_access.to_owned());

//         let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
//         let mut user = mock_user();
//         user.identities = vec![caller];
//         user.groups = vec![*ADMIN_GROUP_ID];

//         USER_REPOSITORY.insert(user.to_key(), user.clone());

//         let ctx = CallContext::new(caller);
//         let has_access = evaluate_caller_access(
//             &ctx,
//             &ResourceSpecifier::Common(
//                 ResourceType::AddressBook,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//             ),
//         )
//         .await;

//         assert!(has_access.is_ok());
//     }

//     #[tokio::test]
//     async fn user_has_access_to_admin_resource() {
//         let mut admin_access = mock_access_policy();
//         admin_access.user = UserSpecifier::Group(vec![*ADMIN_GROUP_ID]);
//         admin_access.resource = ResourceSpecifier::Common(
//             ResourceType::AddressBook,
//             AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//         );

//         ACCESS_CONTROL_REPOSITORY.insert(admin_access.id, admin_access.to_owned());

//         let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
//         let mut user = mock_user();
//         user.identities = vec![caller];
//         user.groups = vec![*ADMIN_GROUP_ID];

//         USER_REPOSITORY.insert(user.to_key(), user.clone());

//         let ctx = CallContext::new(caller);
//         let has_access = evaluate_caller_access(
//             &ctx,
//             &ResourceSpecifier::Common(
//                 ResourceType::AddressBook,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//             ),
//         )
//         .await;

//         assert!(has_access.is_ok());
//     }

//     #[tokio::test]
//     async fn group_with_read_access_should_not_have_other_access() {
//         let test_context = setup();

//         // add finance read access to address book
//         let mut policy = mock_access_policy();
//         policy.id = [10; 16];
//         policy.user = UserSpecifier::Group(vec![test_context.finance_user_group.id]);
//         policy.resource = ResourceSpecifier::Common(
//             ResourceType::AddressBook,
//             AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//         );

//         ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.finance_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::AddressBook,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//             )
//         )
//         .await
//         .is_ok());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.finance_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::AddressBook,
//                 AddressBookActionSpecifier::Update(CommonSpecifier::Any),
//             )
//         )
//         .await
//         .is_err());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.finance_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::AddressBook,
//                 AddressBookActionSpecifier::Delete(CommonSpecifier::Any),
//             )
//         )
//         .await
//         .is_err());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.finance_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::AddressBook,
//                 AddressBookActionSpecifier::Create,
//             )
//         )
//         .await
//         .is_err());
//     }

//     #[tokio::test]
//     async fn group_has_access_to_resource_by_id() {
//         let test_context = setup();

//         // group should have acess
//         let mut policy = mock_access_policy();
//         policy.id = [10; 16];
//         policy.user = UserSpecifier::Group(vec![test_context.finance_user_group.id]);
//         policy.resource = ResourceSpecifier::Common(
//             ResourceType::Account,
//             AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16]])),
//         );

//         account_test_utils::add_account(&[1; 16]);
//         account_test_utils::add_account(&[2; 16]);

//         ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.finance_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::Account,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16],])),
//             )
//         )
//         .await
//         .is_ok());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.finance_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::Account,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[2; 16],])),
//             )
//         )
//         .await
//         .is_err());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.finance_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::Account,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//             )
//         )
//         .await
//         .is_err());
//     }

//     #[tokio::test]
//     async fn user_has_access_to_resource_by_id() {
//         let test_context = setup();

//         // group should have acess
//         let mut policy = mock_access_policy();
//         policy.id = [10; 16];
//         policy.user = UserSpecifier::Id(vec![test_context.hr_user.id]);
//         policy.resource = ResourceSpecifier::Common(
//             ResourceType::User,
//             AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16]])),
//         );

//         user_test_utils::add_user(&[1; 16]);
//         user_test_utils::add_user(&[2; 16]);

//         ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.hr_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::User,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16]])),
//             )
//         )
//         .await
//         .is_ok());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.hr_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::User,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[2; 16]])),
//             )
//         )
//         .await
//         .is_err());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.hr_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::User,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//             )
//         )
//         .await
//         .is_err());
//     }

//     #[tokio::test]
//     async fn user_has_access_to_any() {
//         let test_context = setup();

//         // group should have acess
//         let mut policy = mock_access_policy();
//         policy.id = [10; 16];
//         policy.user = UserSpecifier::Id(vec![test_context.hr_user.id]);
//         policy.resource = ResourceSpecifier::Common(
//             ResourceType::User,
//             AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//         );

//         user_test_utils::add_user(&[1; 16]);
//         user_test_utils::add_user(&[2; 16]);

//         ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());

//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.hr_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::User,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[1; 16]])),
//             )
//         )
//         .await
//         .is_ok());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.hr_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::User,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Id(vec![[2; 16]])),
//             )
//         )
//         .await
//         .is_ok());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.hr_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::User,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Group(vec![
//                     test_context.admin_user_group.id
//                 ])),
//             )
//         )
//         .await
//         .is_ok());
//         assert!(evaluate_caller_access(
//             &CallContext::new(test_context.hr_user.identities[0]),
//             &ResourceSpecifier::Common(
//                 ResourceType::User,
//                 AddressBookActionSpecifier::Read(CommonSpecifier::Any),
//             )
//         )
//         .await
//         .is_ok());
//     }
// }
