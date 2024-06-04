use super::{CanisterConfig, CanisterState, CANISTER_CONFIG_TOTAL_MEMORY_PAGES, MAX_WASM_PAGES};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    Cell, DefaultMemoryImpl, RestrictedMemory,
};
use std::{cell::RefCell, thread::LocalKey};

pub type Memory = RestrictedMemory<DefaultMemoryImpl>;
pub type ConfigCell = Cell<CanisterState, Memory>;
pub type LocalRef<T> = &'static LocalKey<RefCell<T>>;

pub const USER_MEMORY_ID: MemoryId = MemoryId::new(1);
pub const USER_IDENTITY_INDEX_MEMORY_ID: MemoryId = MemoryId::new(2);
pub const USER_STATUS_INDEX_MEMORY_ID: MemoryId = MemoryId::new(3);
pub const ARTIFACT_MEMORY_ID: MemoryId = MemoryId::new(4);
pub const ARTIFACT_HASH_INDEX_MEMORY_ID: MemoryId = MemoryId::new(5);

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
    RestrictedMemory::new(
        DefaultMemoryImpl::default(),
        0..CANISTER_CONFIG_TOTAL_MEMORY_PAGES as u64,
    )
}

/// A helper function to access the canister configuration.
pub fn canister_config() -> Option<CanisterConfig> {
    CONFIG.with(|m| m.borrow().get().get().cloned())
}

/// All the memory after the initial config page is managed by the [MemoryManager].
pub fn managed_memory() -> Memory {
    RestrictedMemory::new(
        DefaultMemoryImpl::default(),
        CANISTER_CONFIG_TOTAL_MEMORY_PAGES as u64..MAX_WASM_PAGES,
    )
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
mod tests {
    use super::*;

    #[test]
    fn test_canister_config() {
        let config = CanisterConfig::new(Vec::new(), Vec::new());
        write_canister_config(config.clone());
        assert_eq!(canister_config(), Some(config));
    }

    #[test]
    fn test_update_canister_config() {
        let config = CanisterConfig::new(Vec::new(), Vec::new());
        write_canister_config(config.clone());
        let new_config = CanisterConfig::new(vec![1], vec![2]);
        write_canister_config(new_config.clone());
        assert_eq!(canister_config(), Some(new_config));
    }
}
