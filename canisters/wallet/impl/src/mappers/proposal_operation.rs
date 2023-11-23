use crate::{
    models::{
        Account, AddAccountOperation, EditAccountOperation, ProposalOperation, TransferOperation,
    },
    repositories::AccountRepository,
};
use ic_canister_core::repository::Repository;
use uuid::Uuid;
use wallet_api::{
    AddAccountOperationDTO, AddAccountOperationInput, EditAccountOperationDTO, NetworkDTO,
    ProposalOperationDTO, TransferMetadataDTO, TransferOperationDTO,
};

use super::{BlockchainMapper, HelperMapper};

impl TransferOperation {
    pub fn into_dto(self, account: Account) -> TransferOperationDTO {
        TransferOperationDTO {
            amount: self.amount,
            from_account: account.to_dto(),
            to: self.to,
            fee: self.fee,
            metadata: self
                .metadata
                .iter()
                .map(|(k, v)| TransferMetadataDTO {
                    key: k.to_string(),
                    value: v.to_string(),
                })
                .collect(),
            network: NetworkDTO {
                id: self.network.clone(),
                name: self.network.clone(),
            },
        }
    }
}

impl From<EditAccountOperation> for EditAccountOperationDTO {
    fn from(operation: EditAccountOperation) -> EditAccountOperationDTO {
        EditAccountOperationDTO {
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

impl AddAccountOperation {
    pub fn to_dto(self, account: Option<Account>) -> AddAccountOperationDTO {
        AddAccountOperationDTO {
            account: account.map(|account| account.to_dto()),
            name: self.name,
            owners: self
                .owners
                .iter()
                .map(|owner| Uuid::from_bytes(*owner).hyphenated().to_string())
                .collect(),
            policies: self
                .policies
                .iter()
                .map(|policy| policy.clone().into())
                .collect(),
            blockchain: self.blockchain.to_string(),
            standard: self.standard.to_string(),
            metadata: self.metadata,
        }
    }
}

impl From<AddAccountOperationInput> for AddAccountOperation {
    fn from(input: AddAccountOperationInput) -> AddAccountOperation {
        AddAccountOperation {
            id: None,
            name: input.name,
            owners: input
                .owners
                .iter()
                .map(|owner| {
                    *HelperMapper::to_uuid(owner.clone())
                        .expect("Invalid owner id")
                        .as_bytes()
                })
                .collect(),
            policies: input
                .policies
                .iter()
                .map(|policy| policy.clone().into())
                .collect(),
            blockchain: BlockchainMapper::to_blockchain(input.blockchain.clone())
                .expect("Invalid blockchain"),
            standard: BlockchainMapper::to_blockchain_standard(input.standard)
                .expect("Invalid blockchain standard"),
            metadata: input.metadata,
        }
    }
}

impl From<ProposalOperation> for ProposalOperationDTO {
    fn from(operation: ProposalOperation) -> ProposalOperationDTO {
        match operation {
            ProposalOperation::Transfer(operation) => {
                let account = AccountRepository::default()
                    .get(&Account::key(operation.from_account_id))
                    .expect("Account not found");

                ProposalOperationDTO::Transfer(Box::new(operation.into_dto(account)))
            }
            ProposalOperation::EditAccount(operation) => {
                ProposalOperationDTO::EditAccount(Box::new(operation.into()))
            }
            ProposalOperation::AddAccount(operation) => {
                let account = operation.id.map(|id| {
                    AccountRepository::default()
                        .get(&Account::key(id))
                        .expect("Account not found")
                });

                ProposalOperationDTO::AddAccount(Box::new(operation.to_dto(account)))
            }
        }
    }
}
