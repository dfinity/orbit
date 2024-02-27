/// The maximum number of instructions that can be executed in a single query call.
pub const QUERY_INSTRUCTIONS_LIMIT: u64 = 5_000_000_000;

/// The maximum number of instructions that can be executed in a single update call
/// with DTS enabled (Deterministic Time Slicing).
pub const UPDATE_INSTRUCTIONS_LIMIT: u64 = 20_000_000_000;

/// The 95th percentile of the maximum number of instructions that can be executed in a single query call.
pub const QUERY_INSTRUCTIONS_LIMIT_P95: u64 = QUERY_INSTRUCTIONS_LIMIT * 19 / 20;

/// The 95th percentile of the maximum number of instructions that can be executed in a single update call.
pub const UPDATE_INSTRUCTIONS_LIMIT_P95: u64 = UPDATE_INSTRUCTIONS_LIMIT * 19 / 20;

/// Canisters use 64KiB pages for Wasm memory, more details in the PR that introduced this constant:
///
/// - https://github.com/WebAssembly/design/pull/442#issuecomment-153203031
pub const WASM_PAGE_SIZE: u32 = 65536;
