use crate::{models::Blockchain, types::BankAsset};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

thread_local! {
  // The list of assets that are supported by the bank canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
  static BANK_ASSETS: RefCell<HashSet<BankAsset>> =
      RefCell::new(vec![
        BankAsset {
          blockchain: Blockchain::InternetComputer,
          standards: Vec::new(),
          symbol: "ICP".to_string(),
          name: "Internet Computer".to_string(),
          decimals: 8,
          metadata: HashMap::new(),
        },
        BankAsset {
          blockchain: Blockchain::Bitcoin,
          standards: Vec::new(),
          symbol: "BTC".to_string(),
          name: "Bitcoin".to_string(),
          decimals: 8,
          metadata: HashMap::new(),
        },
        BankAsset {
          blockchain: Blockchain::Ethereum,
          standards: Vec::new(),
          symbol: "ETH".to_string(),
          name: "Ethereum".to_string(),
          decimals: 18,
          metadata: HashMap::new(),
        },
      ].into_iter().collect());
}

pub fn get_bank_assets() -> HashSet<BankAsset> {
    BANK_ASSETS.with(|bank_assets| bank_assets.borrow().clone())
}
