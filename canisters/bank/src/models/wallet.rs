use super::{AccountId, Blockchain, BlockchainStandard, WalletBalance, WalletPolicy};
use crate::errors::WalletError;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// The wallet metadata key for the asset symbol;
pub const WALLET_METADATA_SYMBOL_KEY: &str = "symbol";

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
    /// The wallet address (e.g. `0x1234`, etc.)
    pub address: String,
    /// The blockchain standard (e.g. `native`, `icrc1`, `erc20`, etc.)
    pub standard: BlockchainStandard,
    /// The asset symbol (e.g. `ICP`, `ETH`, `BTC`, etc.)
    pub symbol: String,
    /// The asset decimals (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u32,
    /// The wallet name (e.g. `My Main Wallet`)
    pub name: Option<String>,
    /// The wallet owners, which are a list of account ids.
    ///
    /// If the wallet has no owners, it means that it is a system wallet and
    /// only admins of the system can operate on it.
    pub owners: Vec<AccountId>,
    /// The wallet balance, which is the amount of the asset that the wallet holds.
    pub balance: Option<WalletBalance>,
    /// The wallet policies, which define the rules for the wallet.
    pub policies: Vec<WalletPolicy>,
    /// The wallet metadata, which is a list of key-value pairs,
    /// where the key is unique and the first entry in the tuple,
    /// and the value is the second entry in the tuple.
    pub metadata: Vec<(String, String)>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WalletKey {
    /// The wallet id, which is a UUID.
    pub id: WalletId,
}

pub struct WalletValidator<'wallet> {
    wallet: &'wallet Wallet,
}

impl<'wallet> WalletValidator<'wallet> {
    pub const OWNERS_RANGE: (u8, u8) = (1, 10);
    pub const ADDRESS_RANGE: (u8, u8) = (1, 255);

    pub fn new(wallet: &'wallet Wallet) -> WalletValidator {
        WalletValidator { wallet }
    }

    pub fn validate_owners(&self) -> ModelValidatorResult<WalletError> {
        if (self.wallet.owners.len() < Self::OWNERS_RANGE.0 as usize)
            || (self.wallet.owners.len() > Self::OWNERS_RANGE.1 as usize)
        {
            return Err(WalletError::InvalidOwnersRange {
                min_owners: Self::ADDRESS_RANGE.0,
                max_owners: Self::ADDRESS_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate_address(&self) -> ModelValidatorResult<WalletError> {
        if (self.wallet.address.len() < Self::ADDRESS_RANGE.0 as usize)
            || (self.wallet.address.len() > Self::ADDRESS_RANGE.1 as usize)
        {
            return Err(WalletError::InvalidAddressLength {
                min_length: Self::ADDRESS_RANGE.0,
                max_length: Self::ADDRESS_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<WalletError> {
        self.validate_address()?;
        self.validate_owners()?;

        Ok(())
    }
}

impl ModelValidator<WalletError> for Wallet {
    fn validate(&self) -> ModelValidatorResult<WalletError> {
        WalletValidator::new(self).validate()
    }
}

impl Wallet {
    /// Creates a new wallet key from the given key components.
    pub fn key(id: WalletId) -> WalletKey {
        WalletKey { id }
    }

    pub fn as_key(&self) -> WalletKey {
        Self::key(self.id)
    }
}
