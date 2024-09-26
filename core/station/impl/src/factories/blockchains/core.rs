use super::InternetComputer;
use crate::{
    errors::FactoryError,
    models::{
        Account, AccountAddress, AccountSeed, Asset, Blockchain, Metadata, TokenStandard, Transfer,
    },
};
use async_trait::async_trait;
use num_bigint::BigUint;
use orbit_essentials::api::ApiError;
use std::collections::HashMap;

pub const TRANSACTION_SUBMITTED_DETAILS_BLOCK_HEIGHT_KEY: &str = "block_height";
pub const TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY: &str = "transaction_hash";

pub type BlockchainApiResult<T> = Result<T, ApiError>;

#[derive(Clone, Debug, Hash)]
pub struct BlockchainTransactionFee {
    /// The fee to transfer tokens from one address to another.
    pub fee: BigUint,
    /// Depending on the blockchain, the fee can have different structures and include more options.
    ///
    /// This field is used to store the fee metadata in a key-value format when needed.
    pub metadata: Metadata,
}

impl BlockchainTransactionFee {
    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata.map()
    }
}

#[derive(Clone, Debug, Hash)]
pub struct BlockchainTransactionSubmitted {
    /// Depending on the blockchain, it returns details of the submitted transaction (e.g. block_height).
    pub details: Vec<(String, String)>,
}

impl BlockchainTransactionSubmitted {
    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.details
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect()
    }
}

#[async_trait]
pub trait BlockchainApi: Send + Sync {
    /// Generates a new address for the given account.
    ///
    /// This address is used for token transfers.
    async fn generate_address(&self, seed: &AccountSeed) -> Result<Vec<AccountAddress>, ApiError>;

    /// Returns the latest balance of the given account.
    async fn balance(&self, asset: &Asset, address: &AccountAddress) -> Result<BigUint, ApiError>;

    /// Returns the decimals of the given account.
    async fn decimals(&self, account: &Account) -> Result<u32, ApiError>;

    /// Returns the latest average transaction fee.
    async fn transaction_fee(
        &self,
        account: &Account,
    ) -> Result<BlockchainTransactionFee, ApiError>;

    /// Returns the default network.
    fn default_network(&self) -> String;

    /// Submits a transaction to the destination address.
    async fn submit_transaction(
        &self,
        account: &Account,
        transfer: &Transfer,
    ) -> Result<BlockchainTransactionSubmitted, ApiError>;
}

#[derive(Debug)]
pub struct BlockchainApiFactory {}

impl BlockchainApiFactory {
    pub fn build(
        blockchain: &Blockchain,
        standard: &TokenStandard,
    ) -> Result<Box<dyn BlockchainApi>, FactoryError> {
        match (blockchain, standard) {
            (Blockchain::InternetComputer, TokenStandard::InternetComputerNative) => {
                Ok(Box::new(InternetComputer::create()))
            }
            (blockchain, standard) => Err(FactoryError::UnsupportedBlockchainAccount {
                blockchain: blockchain.to_string(),
                standard: standard.to_string(),
            }),
        }
    }
}
