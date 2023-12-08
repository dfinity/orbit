use crate::repositories::UserRepository;
use crate::{
    core::ic_cdk::{
        api::{id as self_canister_id, is_controller},
        caller,
    },
    models::ADMIN_GROUP_ID,
};
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
        if self.caller_is_controller_or_self() {
            return true;
        }

        let user: Option<crate::models::User> =
            UserRepository::default().find_by_identity(&self.caller);

        match user {
            Some(user) => user.groups.contains(ADMIN_GROUP_ID),
            None => false,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::user_test_utils::mock_user;
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
}
