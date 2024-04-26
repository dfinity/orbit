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
pub mod validation;

#[cfg(not(test))]
pub use orbit_essentials::cdk as ic_cdk;
#[cfg(test)]
pub use orbit_essentials::cdk::mocks as ic_cdk;

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
pub mod proposal;
pub mod utils;

#[cfg(test)]
pub mod test_utils {
    use crate::core::write_system_info;
    use crate::models::system::SystemInfo;

    pub fn init_canister_system() -> SystemInfo {
        let system = SystemInfo::default();
        write_system_info(system.clone());

        system
    }
}
