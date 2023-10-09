use crate::{
    models::{AccountId, Wallet, WalletId},
    repositories::WalletRepository,
};
use candid::{CandidType, Deserialize};
use ic_canister_core::repository::Repository;
use ic_canister_macros::stable_object;

#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WalletAccountIndex {
    /// The account id, which is a UUID.
    pub account_id: AccountId,
    /// The wallet id, which is a UUID.
    pub wallet_id: WalletId,
}

impl WalletAccountIndex {
    pub fn to_wallet(&self) -> Wallet {
        WalletRepository::default()
            .get(&Wallet::key(self.wallet_id))
            .expect("Wallet not found")
    }
}

#[derive(Clone, Debug)]
pub struct WalletAccountIndexCriteria {
    pub account_id: WalletId,
}

impl Wallet {
    pub fn to_index_by_accounts(&self) -> Vec<WalletAccountIndex> {
        self.owners
            .iter()
            .map(|owner| WalletAccountIndex {
                account_id: owner.to_owned(),
                wallet_id: self.id,
            })
            .collect::<Vec<WalletAccountIndex>>()
    }
}
