use lazy_static::lazy_static;

use crate::models::{Blockchain, BlockchainStandard};

pub struct StandardData {
    pub standard: BlockchainStandard,
    pub required_metadata_fields: Vec<String>,
    pub supported_operations: Vec<StandardOperation>,
}

pub enum StandardOperation {
    Balance,
    Transfer,
    ListTransfers,
}
impl std::fmt::Display for StandardOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StandardOperation::Balance => write!(f, "balance"),
            StandardOperation::Transfer => write!(f, "transfer"),
            StandardOperation::ListTransfers => write!(f, "list_transfers"),
        }
    }
}

pub struct SupportedBlockchain {
    pub blockchain: Blockchain,
    pub supported_standards: Vec<StandardData>,
}

lazy_static! {
    pub static ref SUPPORTED_BLOCKCHAINS: Vec<SupportedBlockchain> = {
        vec![SupportedBlockchain {
            blockchain: Blockchain::InternetComputer,
            supported_standards: vec![StandardData {
                standard: BlockchainStandard::Native,
                required_metadata_fields: vec![
                    "ledger_canister_id".to_string(),
                    "index_canister_id".to_string(),
                ],
                supported_operations: vec![
                    StandardOperation::Balance,
                    StandardOperation::Transfer,
                    StandardOperation::ListTransfers,
                ],
            }],
        }]
    };
}
