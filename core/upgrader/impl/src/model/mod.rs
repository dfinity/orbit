mod disaster_recovery;
mod logging;

pub use disaster_recovery::*;
pub use logging::*;

use crate::STABLE_MEMORY_VERSION;
use candid::Principal;
use orbit_essentials::storable;

#[storable]
pub struct State {
    pub target_canister: Principal,
    pub disaster_recovery: DisasterRecovery,
    pub stable_memory_version: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            target_canister: Principal::anonymous(),
            disaster_recovery: Default::default(),
            stable_memory_version: STABLE_MEMORY_VERSION,
        }
    }
}
