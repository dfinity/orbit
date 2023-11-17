use crate::{
    models::{Account, AccountEditOperation, ProposalOperation, TransferOperation},
    repositories::AccountRepository,
    transport::{
        AccountEditOperationDTO, NetworkDTO, ProposalOperationDTO, TransferMetadataDTO,
        TransferOperationDTO,
    },
};
use ic_canister_core::repository::Repository;
use uuid::Uuid;

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
                let account = AccountRepository::default()
                    .get(&Account::key(operation.from_account_id))
                    .expect("Account not found");

                ProposalOperationDTO::Transfer(operation.into_dto(account))
            }
            ProposalOperation::AccountEdit(operation) => {
                ProposalOperationDTO::AccountEdit(operation.into())
            }
        }
    }
}
