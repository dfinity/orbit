use crate::{
    models::{ProposalOperation, TransferOperation},
    transport::{NetworkDTO, ProposalOperationDTO, TransferMetadataDTO, TransferOperationDTO},
};
use uuid::Uuid;

impl From<TransferOperation> for TransferOperationDTO {
    fn from(operation: TransferOperation) -> TransferOperationDTO {
        TransferOperationDTO {
            amount: operation.amount,
            from_account_id: Uuid::from_bytes(operation.from_account_id)
                .hyphenated()
                .to_string(),
            to: operation.to,
            fee: operation.fee,
            metadata: operation
                .metadata
                .iter()
                .map(|(k, v)| TransferMetadataDTO {
                    key: k.to_string(),
                    value: v.to_string(),
                })
                .collect(),
            network: NetworkDTO {
                id: operation.network,
                name: operation.network,
            },
        }
    }
}

impl From<ProposalOperation> for ProposalOperationDTO {
    fn from(operation: ProposalOperation) -> ProposalOperationDTO {
        match operation {
            ProposalOperation::Transfer(operation) => {
                ProposalOperationDTO::Transfer(operation.into())
            }
        }
    }
}
