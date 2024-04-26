use crate::models::{Asset, Blockchain, BlockchainStandard, Metadata};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  /// The list of assets that are supported by the canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
  pub static ASSETS: RefCell<HashSet<Asset>> =
      RefCell::new(vec![
        Asset {
          blockchain: Blockchain::InternetComputer,
          standard: BlockchainStandard::Native,
          symbol: "ICP".to_string(),
          name: "Internet Computer".to_string(),
          metadata: Metadata::default(),
        },
      ].into_iter().collect());
}
