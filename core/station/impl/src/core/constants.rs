use candid::Principal;
use uuid::Uuid;

pub const MAX_BYTE_SIZE_PRINCIPAL: u32 = std::mem::size_of::<Principal>() as u32;
pub const MAX_BYTE_SIZE_UUID: u32 = std::mem::size_of::<Uuid>() as u32;

/// Represents one gigabyte.
pub const GIB: u64 = 1 << 30;

/// Represents one mebibyte.
pub const MIB: u32 = 1 << 20;

/// The maximum memory size the canister can use for stable memory, currently set to 64GiB.
pub const MAX_STABLE_MEMORY_SIZE: u64 = 64 * GIB;

/// Canisters use 64KiB pages for Wasm memory, more details in the PR that introduced this constant:
/// - https://github.com/WebAssembly/design/pull/442#issuecomment-153203031
pub const WASM_PAGE_SIZE: u32 = 65536;

/// The number of Wasm pages reserved for the system state.
pub const SYSTEM_RESERVED_MEMORY_PAGES: u32 = MIB * 3 / WASM_PAGE_SIZE;

/// The number of bytes reserved for the system state.
pub const SYSTEM_RESERVED_MEMORY_BYTES: u32 = SYSTEM_RESERVED_MEMORY_PAGES * WASM_PAGE_SIZE;

/// The size of the stable memory bucket in WASM pages.
///
/// We use a bucket size of 1MiB to ensure that the default memory allocated to the canister is as small as possible,
/// this is due to the fact that this cansiter uses several MemoryIds to manage the stable memory similarly to to how
/// a database arranges data per table.
///
/// Currently a bucket size of 1MiB limits the canister to 32GiB of stable memory, which is more than enough for the
/// current use case, however, if the canister needs more memory in the future, `ic-stable-structures` will need to be
/// updated to support storing more buckets in a backwards compatible way.
pub const STABLE_MEMORY_BUCKET_SIZE: u16 = (MIB / WASM_PAGE_SIZE) as u16;

/// The maximum number of Wasm pages that we allow to use for the stable storage.
pub const MAX_WASM_PAGES: u64 = MAX_STABLE_MEMORY_SIZE / WASM_PAGE_SIZE as u64;

/// The number of seconds that the account balance is considered fresh.
pub const ACCOUNT_BALANCE_FRESHNESS_IN_MS: u64 = 3000;

/// The initial cycles balance to use when creating the upgrader canister.
pub const INITIAL_UPGRADER_CYCLES: u128 = 2_000_000_000_000;
