use candid::Principal;
use uuid::Uuid;

pub const MAX_BYTE_SIZE_PRINCIPAL: u32 = std::mem::size_of::<Principal>() as u32;
pub const MAX_BYTE_SIZE_UUID: u32 = std::mem::size_of::<Uuid>() as u32;

/// Represents one gigabyte.
pub const GIB: u64 = 1 << 30;

/// The maximum memory size the canister can use for stable memory, currently set to 32GiB.
pub const MAX_STABLE_MEMORY_SIZE: u64 = 32 * GIB;

/// Canisters use 64KiB pages for Wasm memory, more details in the PR that introduced this constant:
/// - https://github.com/WebAssembly/design/pull/442#issuecomment-153203031
pub const WASM_PAGE_SIZE: u32 = 65536;

/// The maximum number of Wasm pages that we allow to use for the stable storage.
pub const MAX_WASM_PAGES: u64 = MAX_STABLE_MEMORY_SIZE / WASM_PAGE_SIZE as u64;

/// The number of seconds that the account balance is considered fresh.
pub const ACCOUNT_BALANCE_FRESHNESS_IN_MS: u64 = 15000;

/// The initial cycles balance to use when creating the upgrader canister.
pub const INITIAL_UPGRADER_CYCLES: u128 = 250_000_000_000;

/// The NNS Root canister id added to wallet and upgrader canisters as a recovery method.
pub const NNS_ROOT_CANISTER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 3, 1, 1]);

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn nns_root_canister_id_match_string_representation() {
        let nns_text_canister_id = Principal::from_str("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap();

        assert_eq!(NNS_ROOT_CANISTER_ID, nns_text_canister_id);
    }
}
