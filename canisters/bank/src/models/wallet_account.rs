use super::{AccountId, WalletId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;

/// Represents an wallet account within the system.
#[stable_object(size = 256)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WalletAccount {
    /// The account id, which is a UUID.
    pub account_id: AccountId,
    /// The wallet associated with the account.
    pub wallet_id: WalletId,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WalletAccountKey {
    /// The account id, which is a UUID.
    pub account_id: AccountId,
    /// The wallet id, which is a UUID.
    pub wallet_id: WalletId,
}

impl WalletAccount {
    /// Creates a new wallet account key from the given key components.
    pub fn key(account_id: &AccountId, wallet_id: &WalletId) -> WalletAccountKey {
        WalletAccountKey {
            account_id: *account_id,
            wallet_id: *wallet_id,
        }
    }

    pub fn as_key(&self) -> WalletAccountKey {
        WalletAccount::key(&self.account_id, &self.wallet_id)
    }
}
