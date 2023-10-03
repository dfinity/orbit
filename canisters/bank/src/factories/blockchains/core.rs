use super::InternetComputer;
use crate::{
    errors::FactoryError,
    models::{Blockchain, BlockchainStandard, Transfer, Wallet},
};
use async_trait::async_trait;
use ic_canister_core::api::ApiError;
use num_bigint::BigUint;
use std::collections::HashMap;

pub type BlockchainApiResult<T> = Result<T, ApiError>;

#[derive(Clone, Debug, Hash)]
pub struct BlockchainTransactionFee {
    /// The fee to transfer tokens from one address to another.
    pub fee: BigUint,
    /// Depending on the blockchain, the fee can have different structures and include more options.
    ///
    /// This field is used to store the fee metadata in a key-value format when needed.
    pub metadata: Vec<(String, String)>,
}

impl BlockchainTransactionFee {
    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect()
    }
}

#[derive(Clone, Debug, Hash)]
pub struct BlockchainTransactioSubmitted {
    /// Depending on the blockchain, it returns details of the submitted transaction (e.g. block_height).
    pub details: Vec<(String, String)>,
}

impl BlockchainTransactioSubmitted {
    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.details
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect()
    }
}

#[async_trait]
pub trait BlockchainApi {
    /// Generates a new address for the given wallet.
    ///
    /// This address is used for token transfers.
    async fn generate_address(&self, wallet: &Wallet) -> Result<String, ApiError>;

    /// Returns the latest balance of the given wallet.
    async fn balance(&self, wallet: &Wallet) -> Result<BigUint, ApiError>;

    /// Returns the decimals of the given wallet.
    async fn decimals(&self, wallet: &Wallet) -> Result<u32, ApiError>;

    /// Returns the latest average transaction fee.
    async fn transaction_fee(&self, wallet: &Wallet) -> Result<BlockchainTransactionFee, ApiError>;

    /// Returns the default network.
    fn default_network(&self) -> String;

    /// Submits a transaction to the destination address.
    async fn submit_transaction(
        &self,
        wallet: &Wallet,
        transfer: &Transfer,
    ) -> Result<BlockchainTransactioSubmitted, ApiError>;
}

#[derive(Debug)]
pub struct BlockchainApiFactory {}

impl BlockchainApiFactory {
    pub fn build(
        blockchain: &Blockchain,
        standard: &BlockchainStandard,
    ) -> Result<Box<dyn BlockchainApi>, FactoryError> {
        match (blockchain, standard) {
            (Blockchain::InternetComputer, BlockchainStandard::Native) => {
                Ok(Box::new(InternetComputer::create()))
            }
            (blockchain, standard) => Err(FactoryError::UnsupportedBlockchainWallet {
                blockchain: blockchain.to_string(),
                standard: standard.to_string(),
            }),
        }
    }
}
