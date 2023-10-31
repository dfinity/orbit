use super::{CanisterConfig, CanisterState, MAX_WASM_PAGES};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    Cell, DefaultMemoryImpl, RestrictedMemory,
};
use std::cell::RefCell;

pub type Memory = RestrictedMemory<DefaultMemoryImpl>;
pub type ConfigCell = Cell<CanisterState, Memory>;

pub const ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(1);
pub const ACCOUNT_IDENTITY_INDEX_MEMORY_ID: MemoryId = MemoryId::new(2);

thread_local! {
  /// Static configuration of the canister.
  static CONFIG: RefCell<ConfigCell> = RefCell::new(ConfigCell::init(config_memory(), CanisterState::Uninitialized)
    .expect("failed to initialize stable cell"));

  // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
  // return a memory that can be used by stable structures.
  static MEMORY_MANAGER: RefCell<MemoryManager<Memory>> =
      RefCell::new(MemoryManager::init(managed_memory()));
}

/// A helper function that executes a closure with the memory manager.
pub fn with_memory_manager<R>(f: impl FnOnce(&MemoryManager<Memory>) -> R) -> R {
    MEMORY_MANAGER.with(|cell| f(&cell.borrow()))
}

/// Reserve the first stable memory page for the configuration stable cell.
pub fn config_memory() -> Memory {
    RestrictedMemory::new(DefaultMemoryImpl::default(), 0..1)
}

/// A helper function to access the canister configuration.
pub fn canister_config() -> CanisterConfig {
    CONFIG.with(|m| m.borrow().get().get().clone())
}

/// All the memory after the initial config page is managed by the [MemoryManager].
pub fn managed_memory() -> Memory {
    RestrictedMemory::new(DefaultMemoryImpl::default(), 1..MAX_WASM_PAGES)
}

/// A helper function to write the canister configuration.
pub fn write_canister_config(config: CanisterConfig) {
    CONFIG.with(|cell| {
        cell.borrow_mut()
            .set(CanisterState::Initialized(config))
            .expect("failed to write canister config");
    });
}

#[cfg(test)]
pub mod test_utils {
    use super::write_canister_config;
    use crate::core::{ic_cdk::api::time, CanisterConfig};
    use candid::Principal;

    pub fn init_canister_config() {
        let config = CanisterConfig::new(Principal::anonymous(), time());
        write_canister_config(config);
    }
}
