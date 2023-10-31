use super::{AccountId, Blockchain, BlockchainStandard, WalletBalance, WalletPolicy};
use crate::errors::WalletError;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::{collections::HashMap, hash::Hash};

/// The wallet metadata key for the asset symbol;
pub const WALLET_METADATA_SYMBOL_KEY: &str = "symbol";

/// The wallet id, which is a UUID.
pub type WalletId = UUID;

/// Represents a wallet in the system.
///
/// A wallet can be owned by one or more accounts and can only hold one type of asset,
/// which is defined by the blockchain, standard and symbol.
#[stable_object]
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

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WalletKey {
    /// The wallet id, which is a UUID.
    pub id: WalletId,
}

pub struct WalletValidator<'model> {
    wallet: &'model Wallet,
}

impl<'model> WalletValidator<'model> {
    pub const OWNERS_RANGE: (u8, u8) = (1, 10);
    pub const ADDRESS_RANGE: (u8, u8) = (1, 255);
    pub const SYMBOL_RANGE: (u8, u8) = (1, 8);
    pub const MAX_POLICIES: u8 = 10;
    pub const MAX_METADATA: u8 = 10;
    pub const MAX_METADATA_KEY_LEN: u8 = 24;
    pub const MAX_METADATA_VALUE_LEN: u8 = 255;

    pub fn new(wallet: &'model Wallet) -> WalletValidator {
        WalletValidator { wallet }
    }

    pub fn validate_policies(&self) -> ModelValidatorResult<WalletError> {
        if self.wallet.policies.len() > Self::MAX_POLICIES as usize {
            return Err(WalletError::ValidationError {
                info: format!(
                    "Wallet policies count exceeds the maximum allowed: {}",
                    Self::MAX_POLICIES
                ),
            });
        }

        Ok(())
    }

    pub fn validate_metadata(&self) -> ModelValidatorResult<WalletError> {
        if self.wallet.metadata.len() > Self::MAX_METADATA as usize {
            return Err(WalletError::ValidationError {
                info: format!(
                    "Wallet metadata count exceeds the maximum allowed: {}",
                    Self::MAX_METADATA
                ),
            });
        }

        for (key, value) in self.wallet.metadata.iter() {
            if key.len() > Self::MAX_METADATA_KEY_LEN as usize {
                return Err(WalletError::ValidationError {
                    info: format!(
                        "Wallet metadata key length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_KEY_LEN
                    ),
                });
            }

            if value.len() > Self::MAX_METADATA_VALUE_LEN as usize {
                return Err(WalletError::ValidationError {
                    info: format!(
                        "Wallet metadata value length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_VALUE_LEN
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn validate_symbol(&self) -> ModelValidatorResult<WalletError> {
        if (self.wallet.symbol.len() < Self::SYMBOL_RANGE.0 as usize)
            || (self.wallet.symbol.len() > Self::SYMBOL_RANGE.1 as usize)
        {
            return Err(WalletError::ValidationError {
                info: format!(
                    "Wallet symbol length must be between {} and {}",
                    Self::SYMBOL_RANGE.0,
                    Self::SYMBOL_RANGE.1
                ),
            });
        }

        Ok(())
    }

    pub fn validate_owners(&self) -> ModelValidatorResult<WalletError> {
        if (self.wallet.owners.len() < Self::OWNERS_RANGE.0 as usize)
            || (self.wallet.owners.len() > Self::OWNERS_RANGE.1 as usize)
        {
            return Err(WalletError::InvalidOwnersRange {
                min_owners: Self::OWNERS_RANGE.0,
                max_owners: Self::OWNERS_RANGE.1,
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
        self.validate_policies()?;
        self.validate_metadata()?;
        self.validate_symbol()?;
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

    pub fn to_key(&self) -> WalletKey {
        Self::key(self.id)
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ApprovalThresholdPolicy;

    #[test]
    fn fail_policies_validation() {
        let mut wallet = fake_wallet();
        wallet.policies =
            vec![
                WalletPolicy::ApprovalThreshold(ApprovalThresholdPolicy::FixedThreshold(1),);
                WalletValidator::MAX_POLICIES as usize + 1
            ];

        let result = WalletValidator::new(&wallet).validate_policies();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::ValidationError {
                info: "Wallet policies count exceeds the maximum allowed: 10".to_string()
            }
        );
    }

    #[test]
    fn test_policies_validation() {
        let mut wallet = fake_wallet();
        wallet.policies =
            vec![
                WalletPolicy::ApprovalThreshold(ApprovalThresholdPolicy::FixedThreshold(1),);
                WalletValidator::MAX_POLICIES as usize - 1
            ];

        let result = WalletValidator::new(&wallet).validate_policies();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_metadata_validation_too_many() {
        let mut wallet = fake_wallet();
        wallet.metadata =
            vec![("a".repeat(25), "b".repeat(25)); WalletValidator::MAX_METADATA as usize + 1];

        let result = WalletValidator::new(&wallet).validate_metadata();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::ValidationError {
                info: "Wallet metadata count exceeds the maximum allowed: 10".to_string()
            }
        );
    }

    #[test]
    fn test_metadata_validation() {
        let mut wallet = fake_wallet();
        wallet.metadata =
            vec![("a".repeat(24), "b".repeat(24)); WalletValidator::MAX_METADATA as usize - 1];

        let result = WalletValidator::new(&wallet).validate_metadata();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_symbol_validation_too_short() {
        let mut wallet = fake_wallet();
        wallet.symbol = "a".repeat(0);

        let result = WalletValidator::new(&wallet).validate_symbol();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::ValidationError {
                info: "Wallet symbol length must be between 1 and 8".to_string()
            }
        );
    }

    #[test]
    fn fail_symbol_validation_too_long() {
        let mut wallet = fake_wallet();
        wallet.symbol = "a".repeat(9);

        let result = WalletValidator::new(&wallet).validate_symbol();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::ValidationError {
                info: "Wallet symbol length must be between 1 and 8".to_string()
            }
        );
    }

    #[test]
    fn test_symbol_validation() {
        let mut wallet = fake_wallet();
        wallet.symbol = "a".to_string();

        let result = WalletValidator::new(&wallet).validate_symbol();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_address_too_short() {
        let mut wallet = fake_wallet();
        wallet.address = "".to_string();

        let result = WalletValidator::new(&wallet).validate_address();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::InvalidAddressLength {
                min_length: 1,
                max_length: 255
            }
        );
    }

    #[test]
    fn fail_address_too_long() {
        let mut wallet = fake_wallet();
        wallet.address = "a".repeat(256);

        let result = WalletValidator::new(&wallet).validate_address();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::InvalidAddressLength {
                min_length: 1,
                max_length: 255
            }
        );
    }

    #[test]
    fn test_address_validation() {
        let mut wallet = fake_wallet();
        wallet.address = "a".to_string();

        let result = WalletValidator::new(&wallet).validate_address();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_owners_too_many_entries() {
        let mut wallet = fake_wallet();
        wallet.owners = vec![[0; 16]; WalletValidator::OWNERS_RANGE.1 as usize + 1];

        let result = WalletValidator::new(&wallet).validate_owners();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::InvalidOwnersRange {
                min_owners: 1,
                max_owners: 10
            }
        );
    }

    #[test]
    fn fail_owners_too_little_entries() {
        let mut wallet = fake_wallet();
        wallet.owners = vec![[0; 16]; WalletValidator::OWNERS_RANGE.0 as usize - 1];

        let result = WalletValidator::new(&wallet).validate_owners();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::InvalidOwnersRange {
                min_owners: 1,
                max_owners: 10
            }
        );
    }

    #[test]
    fn test_owners_validation() {
        let mut wallet = fake_wallet();
        wallet.owners = vec![[0; 16]];

        let result = WalletValidator::new(&wallet).validate_owners();

        assert!(result.is_ok());
    }

    fn fake_wallet() -> Wallet {
        Wallet {
            id: [0; 16],
            address: "0x1234".to_string(),
            balance: None,
            blockchain: Blockchain::InternetComputer,
            decimals: 0u32,
            name: None,
            owners: vec![],
            policies: vec![],
            standard: BlockchainStandard::Native,
            last_modification_timestamp: 0,
            metadata: vec![
                ("a".repeat(24), "b".repeat(24));
                WalletValidator::MAX_METADATA as usize - 1
            ],
            symbol: "ICP".to_string(),
        }
    }
}
