use super::{MAX_WASM_PAGES, STABLE_MEMORY_BUCKET_SIZE, SYSTEM_RESERVED_MEMORY_PAGES};
use crate::models::system::{SystemInfo, SystemState};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    Cell, DefaultMemoryImpl, RestrictedMemory,
};
use std::cell::RefCell;

pub type Memory = RestrictedMemory<DefaultMemoryImpl>;
pub type ConfigCell = Cell<SystemState, Memory>;

// Memory IDs for the main resources.
pub const USER_MEMORY_ID: MemoryId = MemoryId::new(1);
pub const ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(2);
pub const REQUEST_INDEX_MEMORY_ID: MemoryId = MemoryId::new(3); // new
pub const TRANSFER_MEMORY_ID: MemoryId = MemoryId::new(4);
pub const UNIQUE_INDEX_MEMORY_ID: MemoryId = MemoryId::new(5); // new
pub const TRANSFER_ACCOUNT_INDEX_MEMORY_ID: MemoryId = MemoryId::new(6);
pub const REQUEST_MEMORY_ID: MemoryId = MemoryId::new(7);
pub const ASSET_MEMORY_ID: MemoryId = MemoryId::new(8);
pub const NOTIFICATION_MEMORY_ID: MemoryId = MemoryId::new(11);
pub const NOTIFICATION_USER_INDEX_MEMORY_ID: MemoryId = MemoryId::new(12);
pub const TRANSFER_STATUS_INDEX_MEMORY_ID: MemoryId = MemoryId::new(13);
pub const USER_GROUP_MEMORY_ID: MemoryId = MemoryId::new(14);
pub const REQUEST_POLICIES_MEMORY_ID: MemoryId = MemoryId::new(16);
pub const PERMISSION_MEMORY_ID: MemoryId = MemoryId::new(17);
pub const USER_STATUS_GROUP_INDEX_MEMORY_ID: MemoryId = MemoryId::new(18);
pub const ADDRESS_BOOK_MEMORY_ID: MemoryId = MemoryId::new(19);
pub const REQUEST_RESOURCE_INDEX_MEMORY_ID: MemoryId = MemoryId::new(30);
pub const POLICY_RESOURCE_INDEX_MEMORY_ID: MemoryId = MemoryId::new(31);
pub const REQUEST_EVALUATION_RESULT_MEMORY_ID: MemoryId = MemoryId::new(32);
pub const EXTERNAL_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(33);
pub const NAMED_RULE_MEMORY_ID: MemoryId = MemoryId::new(34);

thread_local! {
  /// Static configuration of the canister.
  static CONFIG: RefCell<ConfigCell> = RefCell::new(ConfigCell::init(system_state_memory(), SystemState::Uninitialized)
    .expect("failed to initialize stable cell"));

  // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
  // return a memory that can be used by stable structures.
  pub static MEMORY_MANAGER: RefCell<MemoryManager<Memory>> =
      RefCell::new(MemoryManager::init_with_bucket_size(managed_memory(), STABLE_MEMORY_BUCKET_SIZE));
}

/// A helper function that executes a closure with the memory manager.
pub fn with_memory_manager<R>(f: impl FnOnce(&MemoryManager<Memory>) -> R) -> R {
    MEMORY_MANAGER.with(|cell| f(&cell.borrow()))
}

/// Reserve the first stable memory page for the configuration stable cell.
pub fn system_state_memory() -> Memory {
    RestrictedMemory::new(
        DefaultMemoryImpl::default(),
        0..(SYSTEM_RESERVED_MEMORY_PAGES as u64),
    )
}

/// All the memory after the initial config page is managed by the [MemoryManager].
pub fn managed_memory() -> Memory {
    RestrictedMemory::new(
        DefaultMemoryImpl::default(),
        (SYSTEM_RESERVED_MEMORY_PAGES as u64)..MAX_WASM_PAGES,
    )
}

/// A helper function to access the system information.
pub fn read_system_info() -> SystemInfo {
    CONFIG.with(|m| m.borrow().get().get().clone())
}

/// A helper function to write the system information to stable memory.
pub fn write_system_info(config: SystemInfo) {
    CONFIG.with(|cell| {
        cell.borrow_mut()
            .set(SystemState::Initialized(config))
            .expect("failed to write system information");
    });
}

// A helper function to read the system state, which can be uninitialized.
pub fn read_system_state() -> SystemState {
    CONFIG.with(|m| m.borrow().get().clone())
}
