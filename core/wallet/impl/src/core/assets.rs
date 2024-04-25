use crate::models::{Blockchain, BlockchainStandard, Metadata, WalletAsset};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  /// The list of assets that are supported by the wallet canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
  pub static WALLET_ASSETS: RefCell<HashSet<WalletAsset>> =
      RefCell::new(vec![
        WalletAsset {
          blockchain: Blockchain::InternetComputer,
          standard: BlockchainStandard::Native,
          symbol: "ICP".to_string(),
          name: "Internet Computer".to_string(),
          metadata: Metadata::default(),
        },
      ].into_iter().collect());
}
