use super::WASM_PAGE_SIZE;
use crate::{
    core::ic_cdk::api::{time, trap},
    models::{Blockchain, BlockchainStandard, WalletAsset},
};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{HashMap, HashSet},
};

thread_local! {
  /// The list of assets that are supported by the wallet canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
  pub static WALLET_ASSETS: RefCell<HashSet<WalletAsset>> =
      RefCell::new(vec![
        WalletAsset {
          blockchain: Blockchain::InternetComputer,
          standards: vec![BlockchainStandard::Native],
          symbol: "ICP".to_string(),
          name: "Internet Computer".to_string(),
          metadata: HashMap::new(),
        },
      ].into_iter().collect());
}

#[stable_object(size = WASM_PAGE_SIZE)]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterConfig {
    /// Last time the canister was upgraded or initialized.
    pub last_upgrade_timestamp: Timestamp,
    /// The default users of the canister.
    pub owners: Vec<Principal>,
    /// An optionally pending upgrade proposal.
    pub upgrade_proposal: Option<UUID>,
    /// The upgrader canister id that is allowed to upgrade this canister.
    pub upgrader_canister_id: Principal,
}

impl Default for CanisterConfig {
    fn default() -> Self {
        Self {
            last_upgrade_timestamp: time(),
            owners: vec![],
            upgrade_proposal: None,
            upgrader_canister_id: Principal::management_canister(),
        }
    }
}

impl CanisterConfig {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;

    /// The maximum size of the CanisterConfig in stable memory.
    pub const MAX_BYTE_SIZE: u32 = WASM_PAGE_SIZE;

    /// If this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE - Self::MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP;
}

/// Configuration state of the canister.
pub enum CanisterState {
    Uninitialized, // This state is only used between wasm module initialization and init().
    Initialized(CanisterConfig),
}

impl CanisterState {
    pub fn get(&self) -> &CanisterConfig {
        match &self {
            CanisterState::Uninitialized => trap("canister not initialized"),
            CanisterState::Initialized(config) => config,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, CanisterState::Initialized(_))
    }
}

/// Adds serialization and deserialization support of CanisterState to stable memory.
impl Storable for CanisterState {
    fn to_bytes(&self) -> Cow<[u8]> {
        match &self {
            CanisterState::Uninitialized => Cow::Borrowed(&[]),
            CanisterState::Initialized(config) => config.to_bytes(),
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        if bytes.len() == 0 {
            return CanisterState::Uninitialized;
        }
        CanisterState::Initialized(CanisterConfig::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: WASM_PAGE_SIZE,
        is_fixed_size: false,
    };
}

#[cfg(test)]
pub mod test_utils {
    use crate::core::{write_canister_config, CanisterConfig};

    pub fn init_canister_config() -> CanisterConfig {
        let config = CanisterConfig::default();
        write_canister_config(config.clone());

        config
    }
}
