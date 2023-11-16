use crate::{
    models::{AccountEditOperation, ProposalOperation, TransferOperation},
    transport::{
        AccountEditOperationDTO, NetworkDTO, ProposalOperationDTO, TransferMetadataDTO,
        TransferOperationDTO,
    },
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
                id: operation.network.clone(),
                name: operation.network.clone(),
            },
        }
    }
}

impl From<AccountEditOperation> for AccountEditOperationDTO {
    fn from(operation: AccountEditOperation) -> AccountEditOperationDTO {
        AccountEditOperationDTO {
            account_id: Uuid::from_bytes(operation.account_id)
                .hyphenated()
                .to_string(),
            name: operation.name,
            owners: operation.owners.map(|owners| {
                owners
                    .iter()
                    .map(|owner| Uuid::from_bytes(*owner).hyphenated().to_string())
                    .collect()
            }),
            policies: operation.policies.map(|policies| {
                policies
                    .iter()
                    .map(|policy| policy.clone().into())
                    .collect()
            }),
        }
    }
}

impl From<ProposalOperation> for ProposalOperationDTO {
    fn from(operation: ProposalOperation) -> ProposalOperationDTO {
        match operation {
            ProposalOperation::Transfer(operation) => {
                ProposalOperationDTO::Transfer(operation.into())
            }
            ProposalOperation::AccountEdit(operation) => {
                ProposalOperationDTO::AccountEdit(operation.into())
            }
        }
    }
}
