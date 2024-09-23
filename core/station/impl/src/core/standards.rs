use lazy_static::lazy_static;

use crate::models::{Blockchain, BlockchainStandard};

pub struct SupportedBlockchain {
    pub blockchain: Blockchain,
    pub supported_standards: Vec<BlockchainStandard>,
}

lazy_static! {
    pub static ref SUPPORTED_BLOCKCHAINS: Vec<SupportedBlockchain> = {
        vec![SupportedBlockchain {
            blockchain: Blockchain::InternetComputer,
            supported_standards: vec![BlockchainStandard::Native],
        }]
    };
}
