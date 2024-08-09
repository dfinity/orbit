//! Core utility features for the canister.

mod assets;
pub use assets::*;

mod constants;
pub use constants::*;

mod memory;
pub use memory::*;

mod call_context;
pub use call_context::*;

pub mod middlewares;
pub mod observer;
pub mod validation;

#[cfg(not(test))]
pub use orbit_essentials::cdk as ic_cdk;
#[cfg(test)]
pub use orbit_essentials::cdk::mocks as ic_cdk;

#[cfg(all(not(test), not(feature = "canbench")))]
pub use orbit_essentials::timers as ic_timers;

#[cfg(any(test, feature = "canbench"))]
pub use orbit_essentials::timers::mocks as ic_timers;

#[cfg(not(test))]
pub use orbit_essentials::utils::generate_uuid_v4;
#[cfg(test)]
pub async fn generate_uuid_v4() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

pub mod authorization;
pub mod evaluation;
pub mod init;
pub mod metrics;
pub mod request;
pub mod utils;

#[cfg(test)]
pub mod test_utils {
    use crate::core::write_system_info;
    use crate::models::system::SystemInfo;
    use candid::Principal;

    pub const UPGRADER_CANISTER_ID: [u8; 29] = [25; 29];

    pub fn init_canister_system() -> SystemInfo {
        let mut system: SystemInfo = SystemInfo::default();
        system
            .set_upgrader_canister_id(Principal::from_slice(self::UPGRADER_CANISTER_ID.as_slice()));

        write_system_info(system.clone());

        system
    }
}
