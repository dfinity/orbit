use crate::{
    errors::MapperError,
    models::{Blockchain, BlockchainStandard},
};
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct BlockchainMapper {}

impl BlockchainMapper {
    pub fn to_blockchain(blockchain: String) -> Result<Blockchain, MapperError> {
        let blockchain = Blockchain::from_str(blockchain.as_str())
            .map_err(|_| MapperError::UnknownBlockchain { blockchain })?;

        Ok(blockchain)
    }

    pub fn to_blockchain_standard(standard: String) -> Result<BlockchainStandard, MapperError> {
        let standard = BlockchainStandard::from_str(standard.as_str()).map_err(|_| {
            MapperError::UnknownBlockchainStandard {
                blockchain_standard: standard,
            }
        })?;

        Ok(standard)
    }
}
