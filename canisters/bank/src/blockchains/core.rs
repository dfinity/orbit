use super::InternetComputer;
use crate::{
    errors::{BlockchainApiError, FactoryError},
    models::{Blockchain, BlockchainStandard, Wallet},
};
use async_trait::async_trait;

pub type BlockchainApiResult<T> = Result<T, BlockchainApiError>;

#[async_trait]
pub trait BlockchainApi {
    /// Generates a new address for the given wallet.
    ///
    /// This address is used for token transfers.
    async fn generate_address(&self, wallet: &Wallet) -> Result<String, BlockchainApiError>;

    /// Returns the latest balance of the given wallet.
    async fn balance(&self, wallet: &Wallet) -> Result<u128, BlockchainApiError>;

    /// Returns the decimals of the given wallet.
    async fn decimals(&self, wallet: &Wallet) -> Result<u32, BlockchainApiError>;
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
