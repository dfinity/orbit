use super::canister_config;
use crate::{
    core::ic_cdk::{
        api::{id as self_canister_id, is_controller, trap},
        caller,
    },
    models::ADMIN_GROUP_ID,
};
use crate::{models::AccessRole, repositories::UserRepository};
use candid::Principal;

#[derive(Clone, Debug)]
pub struct CallContext {
    caller: Principal,
}

impl Default for CallContext {
    fn default() -> Self {
        Self {
            caller: Principal::anonymous(),
        }
    }
}

impl CallContext {
    pub fn new(caller: Principal) -> Self {
        Self { caller }
    }

    /// This method can only be used before any await has been called in the current call context,
    /// otherwise it will panic.
    pub fn get() -> Self {
        Self { caller: caller() }
    }

    pub fn caller(&self) -> Principal {
        self.caller
    }

    pub fn caller_is_controller_or_self(&self) -> bool {
        self.caller == self_canister_id() || is_controller(&self.caller)
    }

    /// Checks if the caller is an admin.
    pub fn is_admin(&self) -> bool {
        if self.caller == self_canister_id() || is_controller(&self.caller) {
            return true;
        }

        let user: Option<crate::models::User> =
            UserRepository::default().find_by_identity(&self.caller);

        match user {
            Some(user) => user.groups.contains(ADMIN_GROUP_ID),
            None => false,
        }
    }

    /// Checks if the caller has the required access role to perform the given action.
    pub fn check_access(&self, permission: &str) {
        if !self.is_admin() {
            check_access(permission, self.caller.to_owned())
        }
    }
}

/// This function checks if the user has the required access role to perform the given action.
fn check_access(permission: &str, caller: Principal) {
    let permissions = canister_config().permissions;
    let permission = permissions
        .iter()
        .find(|p| p.permission_id == permission)
        .unwrap_or_else(|| trap(format!("Permission {} not found", permission).as_str()));

    if permission.access_roles.contains(&AccessRole::Guest) {
        return;
    }

    let user = UserRepository::default()
        .find_by_identity(&caller)
        .unwrap_or_else(|| {
            trap(
                format!(
                    "Access denied for user with principal `{}`",
                    caller.to_text()
                )
                .as_str(),
            )
        });

    if !user.groups.contains(ADMIN_GROUP_ID) {
        // TODO: Add validation once resource access control is integrated with the new permission model
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{core::test_utils, models::user_test_utils::mock_user};
    use ic_canister_core::{cdk::mocks::TEST_CANISTER_ID, repository::Repository};

    #[test]
    fn check_caller_is_not_admin() {
        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![];

        let user_repository = UserRepository::default();
        user_repository.insert(user.to_key(), user.clone());

        let call_context = CallContext::new(caller);
        assert!(!call_context.is_admin());
    }

    #[test]
    fn check_caller_is_admin() {
        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![ADMIN_GROUP_ID.to_owned()];

        let user_repository = UserRepository::default();
        user_repository.insert(user.to_key(), user.clone());

        let call_context = CallContext::new(caller);
        assert!(call_context.is_admin());
    }

    #[test]
    fn check_self_canister_call_is_admin() {
        let call_context = CallContext::new(TEST_CANISTER_ID);
        assert!(call_context.is_admin());
    }

    #[test]
    fn admin_should_have_access_to_all() {
        let canister_config = test_utils::init_canister_config();
        let admin_permission = canister_config
            .permissions
            .iter()
            .find(|p| p.access_roles.contains(&AccessRole::Admin))
            .unwrap_or_else(|| panic!("Permission with admin requirement not found"));

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![ADMIN_GROUP_ID.to_owned()];

        let user_repository = UserRepository::default();
        user_repository.insert(user.to_key(), user.clone());

        let call_context = CallContext::new(caller);
        call_context.check_access(admin_permission.permission_id.as_str());
    }

    // TODO: Add again once resource access control is integrated with the new permission model
    //
    // #[test]
    // #[should_panic]
    // fn fail_user_has_access_to_admin_permission() {
    //     let canister_config = test_utils::init_canister_config();
    //     let admin_permission = canister_config
    //         .permissions
    //         .iter()
    //         .find(|p| p.access_roles.contains(&AccessRole::Admin))
    //         .unwrap_or_else(|| panic!("Permission with admin requirement not found"));

    //     let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
    //     let mut user = mock_user();
    //     user.identities = vec![caller];

    //     let user_repository = UserRepository::default();
    //     user_repository.insert(user.to_key(), user.clone());

    //     let call_context = CallContext::new(caller);
    //     call_context.check_access(admin_permission.permission_id.as_str());
    // }

    #[test]
    fn any_user_has_access_to_guest_permission() {
        let canister_config = test_utils::init_canister_config();
        let guest_permission = canister_config
            .permissions
            .iter()
            .find(|p| p.access_roles.contains(&AccessRole::Guest))
            .unwrap_or_else(|| panic!("Permission with guest requirement not found"));

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let call_context = CallContext::new(caller);
        call_context.check_access(guest_permission.permission_id.as_str());
    }
}
