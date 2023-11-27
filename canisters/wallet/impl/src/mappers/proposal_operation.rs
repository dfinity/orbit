use super::{BlockchainMapper, HelperMapper};
use crate::{
    models::{
        Account, AddAccountOperation, AddUserOperation, EditAccountOperation, EditUserOperation,
        ProposalOperation, TransferOperation, User,
    },
    repositories::{AccountRepository, UserRepository},
};
use ic_canister_core::repository::Repository;
use uuid::Uuid;
use wallet_api::{
    AddAccountOperationDTO, AddAccountOperationInput, AddUserOperationDTO, AddUserOperationInput,
    EditAccountOperationDTO, EditUserOperationDTO, EditUserOperationInput, NetworkDTO,
    ProposalOperationDTO, TransferMetadataDTO, TransferOperationDTO,
};

impl TransferOperation {
    pub fn to_dto(self, account: Account) -> TransferOperationDTO {
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

impl AddUserOperation {
    pub fn to_dto(self, user: Option<User>) -> AddUserOperationDTO {
        AddUserOperationDTO {
            user: user.map(|user| user.to_dto()),
            input: AddUserOperationInput {
                name: self.input.name,
                identities: self.input.identities,
                groups: self
                    .input
                    .groups
                    .iter()
                    .map(|group| Uuid::from_bytes(*group).hyphenated().to_string())
                    .collect(),
                status: self.input.status.into(),
            },
        }
    }
}

impl From<EditUserOperation> for EditUserOperationDTO {
    fn from(operation: EditUserOperation) -> EditUserOperationDTO {
        EditUserOperationDTO {
            input: EditUserOperationInput {
                id: Uuid::from_bytes(operation.input.user_id)
                    .hyphenated()
                    .to_string(),
                name: operation.input.name,
                identities: operation.input.identities,
                groups: operation.input.groups.map(|groups| {
                    groups
                        .iter()
                        .map(|group| Uuid::from_bytes(*group).hyphenated().to_string())
                        .collect()
                }),
                status: operation.input.status.map(Into::into),
            },
        }
    }
}

impl From<AddUserOperationInput> for crate::models::AddUserOperationInput {
    fn from(input: AddUserOperationInput) -> crate::models::AddUserOperationInput {
        crate::models::AddUserOperationInput {
            name: input.name,
            identities: input.identities,
            groups: input
                .groups
                .iter()
                .map(|group| {
                    *HelperMapper::to_uuid(group.clone())
                        .expect("Invalid group id")
                        .as_bytes()
                })
                .collect(),
            status: input.status.into(),
        }
    }
}

impl From<EditUserOperationInput> for crate::models::EditUserOperationInput {
    fn from(input: EditUserOperationInput) -> crate::models::EditUserOperationInput {
        crate::models::EditUserOperationInput {
            user_id: *HelperMapper::to_uuid(input.id)
                .expect("Invalid user id")
                .as_bytes(),
            name: input.name,
            identities: input.identities,
            groups: input.groups.map(|groups| {
                groups
                    .iter()
                    .map(|group| {
                        *HelperMapper::to_uuid(group.clone())
                            .expect("Invalid group id")
                            .as_bytes()
                    })
                    .collect()
            }),
            status: input.status.map(Into::into),
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

                ProposalOperationDTO::Transfer(Box::new(operation.to_dto(account)))
            }
            ProposalOperation::AddAccount(operation) => {
                let account = operation.id.map(|id| {
                    AccountRepository::default()
                        .get(&Account::key(id))
                        .expect("Account not found")
                });

                ProposalOperationDTO::AddAccount(Box::new(operation.to_dto(account)))
            }
            ProposalOperation::EditAccount(operation) => {
                ProposalOperationDTO::EditAccount(Box::new(operation.into()))
            }
            ProposalOperation::AddUser(operation) => {
                let user = operation.user_id.map(|id| {
                    UserRepository::default()
                        .get(&User::key(id))
                        .expect("User not found")
                });

                ProposalOperationDTO::AddUser(Box::new(operation.to_dto(user)))
            }
            ProposalOperation::EditUser(operation) => {
                ProposalOperationDTO::EditUser(Box::new(operation.into()))
            }
        }
    }
}
