use candid::Principal;
use ic_canister_core::cdk::{api::id as self_canister_id, caller};

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
    /// This method can only be used before any await has been called in the current call context,
    /// otherwise it will panic.
    pub fn get() -> Self {
        Self { caller: caller() }
    }

    pub fn caller(&self) -> Principal {
        self.caller
    }

    /// Checks if the caller is an admin.
    pub fn is_admin(&self) -> bool {
        self.caller == self_canister_id()
    }
}

pub trait WithCallContext {
    fn with_call_context(call_context: CallContext) -> Self;
}
