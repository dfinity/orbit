use super::{
    evaluation::{Evaluate, REQUEST_APPROVE_RIGHTS_REQUEST_POLICY_RULE_EVALUATOR},
    request::RequestApprovalRightsEvaluator,
    CallContext,
};
use crate::{
    errors::AuthorizationError,
    models::{
        resource::{
            NotificationResourceAction, RequestResourceAction, Resource, ResourceId,
            UserResourceAction,
        },
        NotificationKey, User,
    },
    repositories::{NOTIFICATION_REPOSITORY, REQUEST_REPOSITORY},
    services::permission::PERMISSION_SERVICE,
};
use orbit_essentials::repository::Repository;

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
            let permission = PERMISSION_SERVICE.get_permission(resource);

            // Checks if the resource is public, if so, then the access is granted.
            if permission.allowed_public() {
                return true;
            }

            if let Some(user) = ctx.user() {
                // If the user is not active, then the access is denied.
                if !user.is_active() {
                    return false;
                }

                // If the resource is available to authenticated users, then the access is granted.
                if permission.allowed_authenticated() {
                    return true;
                }

                // Validades if the user has access to the resource based on the default rules (non-permission based).
                if has_default_resource_access(user, resource) {
                    return true;
                }

                // Checks if the user has access to the resource based on the system permissions.
                return permission.is_allowed(user);
            }

            false
        })
    }
}

/// Checks if the user had access to the resource based on default rules (non-permission based).
///
/// e.g. the user has access to their own user record, etc...
fn has_default_resource_access(user: &User, resource: &Resource) -> bool {
    match &resource {
        &Resource::Request(RequestResourceAction::Read(ResourceId::Id(request_id))) => {
            match REQUEST_REPOSITORY.find_indexed_fields_by_request_id(request_id) {
                None => false,
                Some(request) => {
                    if request.approved_by.iter().any(|id| *id == user.id)
                        || request.rejected_by.iter().any(|id| *id == user.id)
                        || request.requested_by == user.id
                    {
                        return true;
                    }

                    let validator = RequestApprovalRightsEvaluator::new(
                        REQUEST_APPROVE_RIGHTS_REQUEST_POLICY_RULE_EVALUATOR.clone(),
                        user.id,
                        &request,
                    );

                    validator.evaluate().unwrap_or(false)
                }
            }
        }

        Resource::User(UserResourceAction::Read(ResourceId::Id(user_id))) => {
            // The user has access to their own user record.
            *user_id == user.id
        }
        Resource::Notification(action) => {
            match action {
                // The user can always list notifications.
                NotificationResourceAction::List => true,
                // The user cannot update arbitrary notifications.
                NotificationResourceAction::Update(ResourceId::Any) => false,
                NotificationResourceAction::Update(ResourceId::Id(id)) => {
                    let key = NotificationKey { id: *id };
                    if let Some(notification) = NOTIFICATION_REPOSITORY.get(&key) {
                        // The user has access to the user's own notifications.
                        notification.target_user_id == user.id
                    } else {
                        false
                    }
                }
            }
        }
        _ => false,
    }
}

/// This function checks if the user has the required privilege to perform the given action.
///
/// It uses the permissions defined in the canister configuration.
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
            account_test_utils,
            permission::{Allow, Permission},
            resource::{AccountResourceAction, ResourceAction},
            user_group_test_utils,
            user_test_utils::{self, mock_user},
            UserGroup, UserStatus, ADMIN_GROUP_ID,
        },
        repositories::{permission::PERMISSION_REPOSITORY, USER_REPOSITORY},
    };
    use candid::Principal;
    use orbit_essentials::{model::ModelKey, repository::Repository};

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
        let permission = Permission::new(
            Allow::users(vec![test_context.finance_user.id]),
            Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
        );

        PERMISSION_REPOSITORY.insert(permission.key(), permission.to_owned());

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
        let admin_access = Permission::new(
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        PERMISSION_REPOSITORY.insert(admin_access.key(), admin_access.to_owned());

        let caller = Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap();
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
        let admin_access = Permission::new(
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        PERMISSION_REPOSITORY.insert(admin_access.key(), admin_access.to_owned());

        let caller = Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap();
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
        let admin_access = Permission::new(
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        PERMISSION_REPOSITORY.insert(admin_access.key(), admin_access.to_owned());

        let caller = Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap();
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
        let permission = Permission::new(
            Allow::user_groups(vec![test_context.finance_user_group.id]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        );

        PERMISSION_REPOSITORY.insert(permission.key(), permission.to_owned());

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
        let permission = Permission::new(
            Allow::user_groups(vec![test_context.finance_user_group.id]),
            Resource::Account(AccountResourceAction::Read(ResourceId::Id([1; 16]))),
        );

        account_test_utils::add_account(&[1; 16]);
        account_test_utils::add_account(&[2; 16]);

        PERMISSION_REPOSITORY.insert(permission.key(), permission.to_owned());

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
        let permission = Permission::new(
            Allow::users(vec![user.id]),
            Resource::User(UserResourceAction::Read(ResourceId::Id([1; 16]))),
        );

        PERMISSION_REPOSITORY.insert(permission.key(), permission.to_owned());

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
        let permission = Permission::new(
            Allow::users(vec![user.id]),
            Resource::User(UserResourceAction::Read(ResourceId::Any)),
        );

        PERMISSION_REPOSITORY.insert(permission.key(), permission.to_owned());

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
