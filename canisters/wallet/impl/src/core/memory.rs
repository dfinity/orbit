use super::{CanisterConfig, CanisterState, MAX_WASM_PAGES};
use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    Cell, DefaultMemoryImpl, RestrictedMemory,
};
use std::cell::RefCell;

pub type Memory = RestrictedMemory<DefaultMemoryImpl>;
pub type ConfigCell = Cell<CanisterState, Memory>;

pub const USER_MEMORY_ID: MemoryId = MemoryId::new(1);
pub const ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(2);
pub const USER_IDENTITY_INDEX_MEMORY_ID: MemoryId = MemoryId::new(3);
pub const ACCOUNT_USER_INDEX_MEMORY_ID: MemoryId = MemoryId::new(4);
pub const TRANSFER_MEMORY_ID: MemoryId = MemoryId::new(5);
pub const PROPOSAL_EXPIRATION_TIME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(6);
pub const TRANSFER_ACCOUNT_INDEX_MEMORY_ID: MemoryId = MemoryId::new(7);
pub const PROPOSAL_MEMORY_ID: MemoryId = MemoryId::new(8);
pub const PROPOSAL_ACCOUNT_INDEX_MEMORY_ID: MemoryId = MemoryId::new(9);
pub const PROPOSAL_VOTER_INDEX_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const PROPOSAL_STATUS_INDEX_MEMORY_ID: MemoryId = MemoryId::new(11);
pub const PROPOSAL_SCHEDULED_INDEX_MEMORY_ID: MemoryId = MemoryId::new(12);
pub const NOTIFICATION_MEMORY_ID: MemoryId = MemoryId::new(13);
pub const NOTIFICATION_USER_INDEX_MEMORY_ID: MemoryId = MemoryId::new(14);
pub const TRANSFER_STATUS_INDEX_MEMORY_ID: MemoryId = MemoryId::new(15);
pub const USER_GROUP_MEMORY_ID: MemoryId = MemoryId::new(16);
pub const USER_GROUP_NAME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(17);
pub const PROPOSAL_POLICIES_MEMORY_ID: MemoryId = MemoryId::new(18);
pub const ACCESS_POLICY_MEMORY_ID: MemoryId = MemoryId::new(19);
pub const USER_STATUS_GROUP_INDEX_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const ADDRESS_BOOK_MEMORY_ID: MemoryId = MemoryId::new(21);
pub const ADDRESS_BOOK_INDEX_MEMORY_ID: MemoryId = MemoryId::new(22);
pub const PROPOSAL_PROPOSER_INDEX_MEMORY_ID: MemoryId = MemoryId::new(23);
pub const PROPOSAL_CREATION_TIME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(24);
pub const PROPOSAL_KEY_CREATION_TIME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(25);
pub const PROPOSAL_KEY_EXPIRATION_TIME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(26);
pub const PROPOSAL_SORT_INDEX_MEMORY_ID: MemoryId = MemoryId::new(27);
pub const PROPOSAL_STATUS_MODIFICATION_INDEX_MEMORY_ID: MemoryId = MemoryId::new(28);
pub const PROPOSAL_OPERATION_SPECIFIER_INDEX_MEMORY_ID: MemoryId = MemoryId::new(29);

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

/// A helper function to check if the canister is initialized.
pub fn is_canister_initialized() -> bool {
    CONFIG.with(|m| m.borrow().get().is_initialized())
}

/// A helper function to access the canister configuration and mutate it.
pub fn canister_config_mut() -> CanisterConfig {
    CONFIG.with(|m| m.borrow_mut().get().get().clone())
}

pub fn upgrader_canister_id() -> Principal {
    canister_config().upgrader_canister_id
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
