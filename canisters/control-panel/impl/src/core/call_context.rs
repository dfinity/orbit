use crate::core::ic_cdk::{
    api::{id as self_canister_id, is_controller},
    caller,
};
use candid::Principal;

#[derive(Clone, Debug)]
pub struct CallContext {
    _caller: Principal,
}

impl Default for CallContext {
    fn default() -> Self {
        Self {
            _caller: Principal::anonymous(),
        }
    }
}

impl CallContext {
    pub fn new(caller: Principal) -> Self {
        Self { _caller: caller }
    }

    /// This method can only be used before any await has been called in the current call context,
    /// otherwise it will panic.
    pub fn get() -> Self {
        Self { _caller: caller() }
    }

    pub fn caller(&self) -> Principal {
        self._caller
    }

    /// Checks if the caller is an admin.
    pub fn is_admin(&self) -> bool {
        self._caller == self_canister_id()
    }

    /// Checks if the caller is a controller.
    pub fn is_controller(&self) -> bool {
        is_controller(&self._caller)
    }
}
