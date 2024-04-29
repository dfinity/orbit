use crate::{
    core::{
        ic_cdk::api::{time, trap},
        WASM_PAGE_SIZE,
    },
    errors::SystemError,
};
use candid::Principal;
use ic_stable_structures::{storable::Bound, Storable};
use orbit_essentials::types::{Timestamp, UUID};
use orbit_essentials::{model::ModelValidator, storable};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystemState {
    Uninitialized, // This state is only used between wasm module instantiation and init().
    Initialized(SystemInfo),
}

// Reserves 40 pages of memory for the system state in stable memory.
pub const SYSTEM_STATE_WASM_PAGES: u64 = 40;
pub const SYSTEM_STATE_MEMORY_SIZE: u32 = WASM_PAGE_SIZE * SYSTEM_STATE_WASM_PAGES as u32;

#[storable(size = SYSTEM_STATE_MEMORY_SIZE)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemInfo {
    /// The system name.
    name: String,
    /// Last time the canister was upgraded or initialized.
    last_upgrade_timestamp: Timestamp,
    /// An optionally pending change canister request.
    change_canister_request: Option<UUID>,
    /// The upgrader canister id that is allowed to upgrade this canister.
    upgrader_canister_id: Option<Principal>,
    /// The upgrader canister wasm module.
    upgrader_wasm_module: Option<Vec<u8>>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            name: "Station".to_string(),
            last_upgrade_timestamp: time(),
            change_canister_request: None,
            upgrader_canister_id: None,
            upgrader_wasm_module: None,
        }
    }
}

impl SystemInfo {
    pub const MAX_NAME_LENGTH: usize = 48;

    pub fn new(upgrader_canister_id: Principal, upgrader_wasm_module: Vec<u8>) -> Self {
        Self {
            upgrader_canister_id: Some(upgrader_canister_id),
            upgrader_wasm_module: Some(upgrader_wasm_module),
            ..Default::default()
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_last_upgrade_timestamp(&self) -> Timestamp {
        self.last_upgrade_timestamp
    }

    pub fn get_change_canister_request(&self) -> Option<&UUID> {
        self.change_canister_request.as_ref()
    }

    pub fn get_upgrader_canister_id(&self) -> &Principal {
        self.upgrader_canister_id
            .as_ref()
            .expect("upgrader_canister_id is not set")
    }

    pub fn get_upgrader_wasm_module(&self) -> &[u8] {
        self.upgrader_wasm_module
            .as_deref()
            .expect("upgrader_wasm_module is not set")
    }

    pub fn set_change_canister_request(&mut self, request: UUID) {
        self.change_canister_request = Some(request);
    }

    pub fn set_upgrader_canister_id(&mut self, canister_id: Principal) {
        self.upgrader_canister_id = Some(canister_id);
    }

    pub fn set_upgrader_wasm_module(&mut self, wasm_module: Vec<u8>) {
        self.upgrader_wasm_module = Some(wasm_module);
    }

    pub fn update_last_upgrade_timestamp(&mut self) {
        self.last_upgrade_timestamp = time();
    }

    pub fn clear_change_canister_request(&mut self) {
        self.change_canister_request = None;
    }
}

fn validate_name(name: &str) -> Result<(), SystemError> {
    if name.trim().is_empty() {
        return Err(SystemError::ValidationError {
            reason: "name is empty".to_string(),
        });
    }

    if name.len() > SystemInfo::MAX_NAME_LENGTH {
        return Err(SystemError::ValidationError {
            reason: "name is too long".to_string(),
        });
    }

    Ok(())
}

impl ModelValidator<SystemError> for SystemInfo {
    fn validate(&self) -> Result<(), SystemError> {
        validate_name(&self.name)?;

        Ok(())
    }
}

impl SystemState {
    pub fn get(&self) -> &SystemInfo {
        match &self {
            SystemState::Uninitialized => trap("canister not initialized"),
            SystemState::Initialized(info) => info,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, SystemState::Initialized(_))
    }
}

/// Adds serialization and deserialization support of SystemState to stable memory.
impl Storable for SystemState {
    fn to_bytes(&self) -> Cow<[u8]> {
        match &self {
            SystemState::Uninitialized => Cow::Borrowed(&[]),
            SystemState::Initialized(info) => info.to_bytes(),
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        if bytes.len() == 0 {
            return SystemState::Uninitialized;
        }
        SystemState::Initialized(SystemInfo::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: SYSTEM_STATE_MEMORY_SIZE,
        is_fixed_size: false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_name_validation() {
        let mut info = SystemInfo::default();
        assert!(info.validate().is_ok());

        info.set_name("test".to_string());
        assert!(info.validate().is_ok());

        info.set_name("".to_string());
        assert!(info.validate().is_err());

        info.set_name("a".repeat(SystemInfo::MAX_NAME_LENGTH + 1));
        assert!(info.validate().is_err());
    }
}
