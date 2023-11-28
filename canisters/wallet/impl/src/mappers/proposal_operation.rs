use super::{BlockchainMapper, HelperMapper};
use crate::{
    models::{
        Account, AddAccountOperation, AddUserOperation, EditAccountOperation, EditUserOperation,
        EditUserStatusOperation, ProposalOperation, TransferOperation, User,
    },
    repositories::{AccountRepository, UserRepository},
};
use ic_canister_core::repository::Repository;
use uuid::Uuid;
use wallet_api::{
    AddAccountOperationDTO, AddAccountOperationInput, AddUserOperationDTO, AddUserOperationInput,
    EditAccountOperationDTO, EditAccountOperationInput, EditUserOperationDTO,
    EditUserOperationInput, EditUserStatusOperationDTO, EditUserStatusOperationInput, NetworkDTO,
    ProposalOperationDTO, TransferMetadataDTO, TransferOperationDTO, TransferOperationInput,
};

impl TransferOperation {
    pub fn to_dto(self, account: Account) -> TransferOperationDTO {
        TransferOperationDTO {
            from_account: account.to_dto(),
            network: NetworkDTO {
                id: self.input.network.clone(),
                name: self.input.network.clone(),
            },
            input: TransferOperationInput {
                from_account_id: Uuid::from_bytes(account.id).hyphenated().to_string(),
                amount: self.input.amount,
                to: self.input.to,
                fee: self.input.fee,
                metadata: self
                    .input
                    .metadata
                    .iter()
                    .map(|(k, v)| TransferMetadataDTO {
                        key: k.to_string(),
                        value: v.to_string(),
                    })
                    .collect(),
                network: Some(NetworkDTO {
                    id: self.input.network.clone(),
                    name: self.input.network.clone(),
                }),
            },
        }
    }
}

impl AddAccountOperation {
    pub fn to_dto(self, account: Option<Account>) -> AddAccountOperationDTO {
        AddAccountOperationDTO {
            account: account.map(|account| account.to_dto()),
            input: AddAccountOperationInput {
                name: self.input.name,
                owners: self
                    .input
                    .owners
                    .iter()
                    .map(|owner| Uuid::from_bytes(*owner).hyphenated().to_string())
                    .collect(),
                policies: self
                    .input
                    .policies
                    .iter()
                    .map(|policy| policy.clone().into())
                    .collect(),
                blockchain: self.input.blockchain.to_string(),
                standard: self.input.standard.to_string(),
                metadata: self.input.metadata,
            },
        }
    }
}

impl From<AddAccountOperationDTO> for AddAccountOperation {
    fn from(operation: AddAccountOperationDTO) -> AddAccountOperation {
        AddAccountOperation {
            account_id: operation.account.map(|account| {
                *HelperMapper::to_uuid(account.id)
                    .expect("Invalid account id")
                    .as_bytes()
            }),
            input: operation.input.into(),
        }
    }
}

impl From<AddAccountOperationInput> for crate::models::AddAccountOperationInput {
    fn from(input: AddAccountOperationInput) -> crate::models::AddAccountOperationInput {
        crate::models::AddAccountOperationInput {
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
            input: EditAccountOperationInput {
                account_id: Uuid::from_bytes(operation.input.account_id)
                    .hyphenated()
                    .to_string(),
                name: operation.input.name,
                owners: operation.input.owners.map(|owners| {
                    owners
                        .iter()
                        .map(|owner| Uuid::from_bytes(*owner).hyphenated().to_string())
                        .collect()
                }),
                policies: operation.input.policies.map(|policies| {
                    policies
                        .iter()
                        .map(|policy| policy.clone().into())
                        .collect()
                }),
            },
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
        }
    }
}

impl From<EditUserStatusOperationInput> for crate::models::EditUserStatusOperationInput {
    fn from(input: EditUserStatusOperationInput) -> crate::models::EditUserStatusOperationInput {
        crate::models::EditUserStatusOperationInput {
            user_id: *HelperMapper::to_uuid(input.id)
                .expect("Invalid user id")
                .as_bytes(),
            status: input.status.into(),
        }
    }
}

impl From<EditUserStatusOperationDTO> for EditUserStatusOperation {
    fn from(operation: EditUserStatusOperationDTO) -> EditUserStatusOperation {
        EditUserStatusOperation {
            input: operation.input.into(),
        }
    }
}

impl From<EditUserStatusOperation> for EditUserStatusOperationDTO {
    fn from(operation: EditUserStatusOperation) -> EditUserStatusOperationDTO {
        EditUserStatusOperationDTO {
            input: operation.input.into(),
        }
    }
}

impl From<crate::models::EditUserStatusOperationInput> for EditUserStatusOperationInput {
    fn from(input: crate::models::EditUserStatusOperationInput) -> EditUserStatusOperationInput {
        EditUserStatusOperationInput {
            id: Uuid::from_bytes(input.user_id).hyphenated().to_string(),
            status: input.status.into(),
        }
    }
}

impl From<ProposalOperation> for ProposalOperationDTO {
    fn from(operation: ProposalOperation) -> ProposalOperationDTO {
        match operation {
            ProposalOperation::Transfer(operation) => {
                let account = AccountRepository::default()
                    .get(&Account::key(operation.input.from_account_id))
                    .expect("Account not found");

                ProposalOperationDTO::Transfer(Box::new(operation.to_dto(account)))
            }
            ProposalOperation::AddAccount(operation) => {
                let account = operation.account_id.map(|id| {
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
            ProposalOperation::EditUserStatus(operation) => {
                ProposalOperationDTO::EditUserStatus(Box::new(operation.into()))
            }
        }
    }
}
