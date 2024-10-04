//! Core utility features for the control plane.

mod constants;
pub use constants::*;

mod memory;
pub use memory::*;

mod call_context;
pub use call_context::*;

mod config;
pub use config::*;

pub mod metrics;

pub mod middlewares;

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

#[cfg(test)]
pub mod test_utils {
    use super::write_canister_config;
    use crate::core::CanisterConfig;
    use candid::Principal;
    use uuid::Uuid;

    pub fn random_principal() -> Principal {
        let mut principal_id = [0u8; 29];
        Uuid::new_v4()
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, byte)| {
                principal_id[i] = *byte;
            });

        Principal::from_slice(&principal_id)
    }

    pub fn init_canister_config() {
        let config = CanisterConfig::new(Vec::new(), Vec::new(), None);
        write_canister_config(config);
    }
}
