use super::{canister_config, ic::caller, CanisterConfig};
use candid::Principal;

#[derive(Clone)]
pub struct CallContext {
    caller: Principal,
    canister_config: CanisterConfig,
}

impl Default for CallContext {
    fn default() -> Self {
        Self {
            caller: Principal::anonymous(),
            canister_config: CanisterConfig::default(),
        }
    }
}

impl CallContext {
    /// This method can only be used before any await has been called in the current call context,
    /// otherwise it will panic.
    pub fn get() -> Self {
        Self {
            caller: caller(),
            canister_config: canister_config(),
        }
    }

    pub fn caller(&self) -> Principal {
        self.caller
    }

    pub fn canister_config(&self) -> CanisterConfig {
        self.canister_config.clone()
    }
}
