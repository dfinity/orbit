use crate::{
    core::ic_cdk::{
        api::{id as self_canister_id, is_controller},
        caller,
    },
    errors::{CanisterError, UserError},
    models::User,
    repositories::USER_REPOSITORY,
};
use candid::Principal;

#[derive(Clone, Debug)]
pub struct CallContext {
    _caller: Principal,
    user: Option<User>,
}

impl Default for CallContext {
    fn default() -> Self {
        Self {
            _caller: Principal::anonymous(),
            user: None,
        }
    }
}

impl CallContext {
    pub fn new(caller: Principal) -> Self {
        Self {
            _caller: caller,
            user: USER_REPOSITORY.find_by_identity(&caller),
        }
    }

    /// This method can only be used before any await has been called in the current call context,
    /// otherwise it will panic.
    pub fn get() -> Self {
        let caller = caller();

        Self {
            _caller: caller,
            user: USER_REPOSITORY.find_by_identity(&caller),
        }
    }

    pub fn user(&self) -> Result<User, UserError> {
        self.user
            .clone()
            .ok_or(UserError::AssociatedUserIdentityNotFound {
                identity: self._caller.to_string(),
            })
    }

    pub fn caller(&self) -> Principal {
        self._caller
    }

    /// Checks if the caller is the current canister.
    pub fn is_self(&self) -> bool {
        self._caller == self_canister_id()
    }

    /// Checks if the caller is a controller.
    pub fn is_controller(&self) -> bool {
        is_controller(&self._caller)
    }

    /// Checks if the caller is an admin.
    pub fn is_admin(&self) -> bool {
        self.is_self() || self.is_controller()
    }

    /// Checks if the caller is the asset canister.
    pub fn assert_is_admin(&self, scope: &str) -> Result<(), CanisterError> {
        if !self.is_admin() {
            return Err(CanisterError::Forbidden {
                method: scope.to_string(),
            });
        }

        Ok(())
    }
}
