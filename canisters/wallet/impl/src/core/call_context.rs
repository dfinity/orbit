use crate::core::ic_cdk::{
    api::{id as self_canister_id, is_controller},
    caller,
};
use crate::models::User;
use crate::repositories::USER_REPOSITORY;
use candid::Principal;

#[derive(Clone, Debug)]
pub struct CallContext {
    caller: Principal,
    user: Option<User>,
}

impl Default for CallContext {
    fn default() -> Self {
        Self {
            caller: Principal::anonymous(),
            user: None,
        }
    }
}

impl CallContext {
    pub fn new(caller: Principal) -> Self {
        Self {
            caller,
            user: USER_REPOSITORY.find_by_identity(&caller),
        }
    }

    /// This method can only be used before any await has been called in the current call context,
    /// otherwise it will panic.
    pub fn get() -> Self {
        let caller = caller();

        Self {
            caller,
            user: USER_REPOSITORY.find_by_identity(&caller),
        }
    }

    pub fn caller(&self) -> Principal {
        self.caller
    }

    // return reference to user
    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn caller_is_controller_or_self(&self) -> bool {
        self.caller == self_canister_id() || is_controller(&self.caller)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::{user_test_utils::mock_user, ADMIN_GROUP_ID};
    use ic_canister_core::cdk::mocks::TEST_CANISTER_ID;
    use ic_canister_core::repository::Repository;

    #[test]
    fn check_caller_is_not_controller() {
        let caller = Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let call_context = CallContext::new(caller);
        assert!(!call_context.caller_is_controller_or_self());
    }

    #[test]
    fn check_caller_is_not_self() {
        let caller = Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![ADMIN_GROUP_ID.to_owned()];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let call_context = CallContext::new(caller);
        assert!(!call_context.caller_is_controller_or_self());
    }

    #[test]
    fn check_self_canister_call_is_true() {
        let call_context = CallContext::new(TEST_CANISTER_ID);
        assert!(call_context.caller_is_controller_or_self());
    }

    #[test]
    fn check_user_is_none() {
        let caller = Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap();
        let call_context = CallContext::new(caller);
        assert!(call_context.user().is_none());
    }

    #[test]
    fn check_user_is_some() {
        let caller = Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap();
        let mut user = mock_user();
        user.identities = vec![caller];
        user.groups = vec![ADMIN_GROUP_ID.to_owned()];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let call_context = CallContext::new(caller);
        assert!(call_context.user().is_some());
    }
}
