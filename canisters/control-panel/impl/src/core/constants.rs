use candid::Principal;
use uuid::Uuid;

pub const MAX_BYTE_SIZE_PRINCIPAL: u32 = std::mem::size_of::<Principal>() as u32;
pub const MAX_BYTE_SIZE_UUID: u32 = std::mem::size_of::<Uuid>() as u32;

/// Represents one gigabyte.
pub const GIB: u64 = 1 << 30;

/// The maximum memory size the canister can use for stable memory, currently set to 32GiB.
pub const MAX_STABLE_MEMORY_SIZE: u64 = 32 * GIB;

/// Canisters use 64KiB pages for Wasm memory, more details in the PR that introduced this constant:
///
/// - https://github.com/WebAssembly/design/pull/442#issuecomment-153203031
pub const WASM_PAGE_SIZE: u32 = 65536;

/// The pages to reserve for the canister configuration.
pub const CANISTER_CONFIG_TOTAL_MEMORY_PAGES: u32 = 160;

/// The maximum size of the stable memory that can be used for the canister configuration.
///
/// This reserves 10MiB for the canister configuration state, giving enough space to grow in the future.
pub const CANISTER_CONFIG_STATE_SIZE: u32 = WASM_PAGE_SIZE * CANISTER_CONFIG_TOTAL_MEMORY_PAGES;

/// The maximum number of Wasm pages that we allow to use for the stable storage.
pub const MAX_WASM_PAGES: u64 = MAX_STABLE_MEMORY_SIZE / WASM_PAGE_SIZE as u64;

/// The initial cycle balance to set for new wallet canisters.
pub const INITIAL_WALLET_CYCLES: u128 = 1_000_000_000_000;
