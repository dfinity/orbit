use super::{AccountId, Blockchain, BlockchainStandard, WalletPolicy};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;

/// The wallet id, which is a UUID.
pub type WalletId = UUID;

/// Represents a wallet in the system.
///
/// A wallet can be owned by one or more accounts and can only hold one type of asset,
/// which is defined by the blockchain, standard and symbol.
#[stable_object(size = 1024)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Wallet {
    /// The wallet id, which is a UUID.
    pub id: WalletId,
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: Blockchain,
    /// The blockchain standard (e.g. `icrc1`, `erc20`, etc.)
    ///
    /// If not set, it means that the wallet holds a native token.
    pub standard: Option<BlockchainStandard>,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    /// The wallet owners, which are a list of account ids.
    ///
    /// If the wallet has no owners, it means that it is a system wallet and
    /// only admins of the system can operate on it.
    pub owners: Vec<AccountId>,
    /// The wallet name (e.g. `My Main Wallet`)
    pub name: Option<String>,
    /// The wallet policies, which define the rules for the wallet.
    pub policies: Vec<WalletPolicy>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}
