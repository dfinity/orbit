use super::{BlockchainMapper, HelperMapper};
use crate::{
    models::{
        Account, AccountPoliciesInput, AddAccessPolicyOperation, AddAccessPolicyOperationInput,
        AddAccountOperation, AddProposalPolicyOperation, AddProposalPolicyOperationInput,
        AddUserOperation, ChangeCanisterOperation, ChangeCanisterTarget, EditAccessPolicyOperation,
        EditAccessPolicyOperationInput, EditAccountOperation, EditProposalPolicyOperation,
        EditProposalPolicyOperationInput, EditUserOperation, ProposalOperation,
        RemoveAccessPolicyOperation, RemoveAccessPolicyOperationInput,
        RemoveProposalPolicyOperation, RemoveProposalPolicyOperationInput, TransferOperation, User,
    },
    repositories::{
        access_control::ACCESS_CONTROL_REPOSITORY, policy::PROPOSAL_POLICY_REPOSITORY,
        AccountRepository, UserRepository, USER_GROUP_REPOSITORY,
    },
};
use ic_canister_core::repository::Repository;
use uuid::Uuid;
use wallet_api::{
    AddAccountOperationDTO, AddAccountOperationInput, AddUserOperationDTO, AddUserOperationInput,
    ChangeCanisterOperationDTO, ChangeCanisterOperationInput, ChangeCanisterTargetDTO,
    EditAccountOperationDTO, EditAccountOperationInput, EditUserOperationDTO,
    EditUserOperationInput, NetworkDTO, ProposalOperationDTO, TransferMetadataDTO,
    TransferOperationDTO, TransferOperationInput,
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

impl From<AccountPoliciesInput> for wallet_api::AccountPoliciesDTO {
    fn from(input: AccountPoliciesInput) -> wallet_api::AccountPoliciesDTO {
        wallet_api::AccountPoliciesDTO {
            transfer: input.transfer.map(|criteria| criteria.into()),
            edit: input.edit.map(|criteria| criteria.into()),
        }
    }
}

impl From<wallet_api::AccountPoliciesDTO> for AccountPoliciesInput {
    fn from(input: wallet_api::AccountPoliciesDTO) -> AccountPoliciesInput {
        AccountPoliciesInput {
            transfer: input.transfer.map(|criteria| criteria.into()),
            edit: input.edit.map(|criteria| criteria.into()),
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
                policies: self.input.policies.into(),
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
            policies: input.policies.into(),
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
                policies: operation.input.policies.map(|policies| policies.into()),
            },
        }
    }
}

impl AddUserOperation {
    pub fn to_dto(self, user: Option<User>) -> AddUserOperationDTO {
        AddUserOperationDTO {
            user: user.map(|user| user.into()),
            input: AddUserOperationInput {
                name: self.input.name,
                identities: self.input.identities,
                unconfirmed_identities: self.input.unconfirmed_identities,
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
                unconfirmed_identities: operation.input.unconfirmed_identities,
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
            unconfirmed_identities: input.unconfirmed_identities,
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
            unconfirmed_identities: input.unconfirmed_identities,
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

impl From<ChangeCanisterTarget> for ChangeCanisterTargetDTO {
    fn from(value: ChangeCanisterTarget) -> Self {
        match value {
            ChangeCanisterTarget::UpgradeWallet => ChangeCanisterTargetDTO::UpgradeWallet,
            ChangeCanisterTarget::UpgradeUpgrader => ChangeCanisterTargetDTO::UpgradeUpgrader,
            ChangeCanisterTarget::InstallCanister(canister_id) => {
                ChangeCanisterTargetDTO::InstallCanister(canister_id)
            }
            ChangeCanisterTarget::UpgradeCanister(canister_id) => {
                ChangeCanisterTargetDTO::UpgradeCanister(canister_id)
            }
        }
    }
}

impl From<ChangeCanisterTargetDTO> for ChangeCanisterTarget {
    fn from(value: ChangeCanisterTargetDTO) -> Self {
        match value {
            ChangeCanisterTargetDTO::UpgradeWallet => ChangeCanisterTarget::UpgradeWallet,
            ChangeCanisterTargetDTO::UpgradeUpgrader => ChangeCanisterTarget::UpgradeUpgrader,
            ChangeCanisterTargetDTO::InstallCanister(canister_id) => {
                ChangeCanisterTarget::InstallCanister(canister_id)
            }
            ChangeCanisterTargetDTO::UpgradeCanister(canister_id) => {
                ChangeCanisterTarget::UpgradeCanister(canister_id)
            }
        }
    }
}

impl From<crate::models::ChangeCanisterOperationInput> for ChangeCanisterOperationInput {
    fn from(input: crate::models::ChangeCanisterOperationInput) -> ChangeCanisterOperationInput {
        ChangeCanisterOperationInput {
            target: input.target.into(),
            module: input.module,
            arg: input.arg,
            checksum: input.checksum,
        }
    }
}

impl From<ChangeCanisterOperationInput> for crate::models::ChangeCanisterOperationInput {
    fn from(input: ChangeCanisterOperationInput) -> crate::models::ChangeCanisterOperationInput {
        crate::models::ChangeCanisterOperationInput {
            target: input.target.into(),
            module: input.module,
            arg: input.arg,
            checksum: input.checksum,
        }
    }
}

impl From<ChangeCanisterOperation> for ChangeCanisterOperationDTO {
    fn from(operation: ChangeCanisterOperation) -> ChangeCanisterOperationDTO {
        ChangeCanisterOperationDTO {
            input: operation.input.into(),
        }
    }
}

impl From<AddAccessPolicyOperationInput> for wallet_api::AddAccessPolicyOperationInput {
    fn from(input: AddAccessPolicyOperationInput) -> wallet_api::AddAccessPolicyOperationInput {
        wallet_api::AddAccessPolicyOperationInput {
            user: input.user.into(),
            resource: input.resource.into(),
        }
    }
}

impl From<wallet_api::AddAccessPolicyOperationInput> for AddAccessPolicyOperationInput {
    fn from(input: wallet_api::AddAccessPolicyOperationInput) -> AddAccessPolicyOperationInput {
        AddAccessPolicyOperationInput {
            user: input.user.into(),
            resource: input.resource.into(),
        }
    }
}

impl From<AddAccessPolicyOperation> for wallet_api::AddAccessPolicyOperationDTO {
    fn from(operation: AddAccessPolicyOperation) -> wallet_api::AddAccessPolicyOperationDTO {
        wallet_api::AddAccessPolicyOperationDTO {
            policy: operation.policy_id.map(|id| {
                ACCESS_CONTROL_REPOSITORY
                    .get(&id)
                    .expect("policy id not found")
                    .into()
            }),
            input: operation.input.into(),
        }
    }
}

impl From<EditAccessPolicyOperationInput> for wallet_api::EditAccessPolicyOperationInput {
    fn from(input: EditAccessPolicyOperationInput) -> wallet_api::EditAccessPolicyOperationInput {
        wallet_api::EditAccessPolicyOperationInput {
            policy_id: Uuid::from_bytes(input.policy_id).hyphenated().to_string(),
            user: input.user.map(|user| user.into()),
            resource: input.resource.map(|resource| resource.into()),
        }
    }
}

impl From<wallet_api::EditAccessPolicyOperationInput> for EditAccessPolicyOperationInput {
    fn from(input: wallet_api::EditAccessPolicyOperationInput) -> EditAccessPolicyOperationInput {
        EditAccessPolicyOperationInput {
            policy_id: *HelperMapper::to_uuid(input.policy_id)
                .expect("Invalid policy id")
                .as_bytes(),
            user: input.user.map(|user| user.into()),
            resource: input.resource.map(|resource| resource.into()),
        }
    }
}

impl From<EditAccessPolicyOperation> for wallet_api::EditAccessPolicyOperationDTO {
    fn from(operation: EditAccessPolicyOperation) -> wallet_api::EditAccessPolicyOperationDTO {
        wallet_api::EditAccessPolicyOperationDTO {
            input: operation.input.into(),
        }
    }
}

impl From<RemoveAccessPolicyOperationInput> for wallet_api::RemoveAccessPolicyOperationInput {
    fn from(
        input: RemoveAccessPolicyOperationInput,
    ) -> wallet_api::RemoveAccessPolicyOperationInput {
        wallet_api::RemoveAccessPolicyOperationInput {
            policy_id: Uuid::from_bytes(input.policy_id).hyphenated().to_string(),
        }
    }
}

impl From<wallet_api::RemoveAccessPolicyOperationInput> for RemoveAccessPolicyOperationInput {
    fn from(
        input: wallet_api::RemoveAccessPolicyOperationInput,
    ) -> RemoveAccessPolicyOperationInput {
        RemoveAccessPolicyOperationInput {
            policy_id: *HelperMapper::to_uuid(input.policy_id)
                .expect("Invalid policy id")
                .as_bytes(),
        }
    }
}

impl From<RemoveAccessPolicyOperation> for wallet_api::RemoveAccessPolicyOperationDTO {
    fn from(operation: RemoveAccessPolicyOperation) -> wallet_api::RemoveAccessPolicyOperationDTO {
        wallet_api::RemoveAccessPolicyOperationDTO {
            input: operation.input.into(),
        }
    }
}

impl From<AddProposalPolicyOperationInput> for wallet_api::AddProposalPolicyOperationInput {
    fn from(input: AddProposalPolicyOperationInput) -> wallet_api::AddProposalPolicyOperationInput {
        wallet_api::AddProposalPolicyOperationInput {
            specifier: input.specifier.into(),
            criteria: input.criteria.into(),
        }
    }
}

impl From<wallet_api::AddProposalPolicyOperationInput> for AddProposalPolicyOperationInput {
    fn from(input: wallet_api::AddProposalPolicyOperationInput) -> AddProposalPolicyOperationInput {
        AddProposalPolicyOperationInput {
            specifier: input.specifier.into(),
            criteria: input.criteria.into(),
        }
    }
}

impl From<AddProposalPolicyOperation> for wallet_api::AddProposalPolicyOperationDTO {
    fn from(operation: AddProposalPolicyOperation) -> wallet_api::AddProposalPolicyOperationDTO {
        wallet_api::AddProposalPolicyOperationDTO {
            policy: operation.policy_id.map(|id| {
                PROPOSAL_POLICY_REPOSITORY
                    .get(&id)
                    .expect("policy id not found")
                    .into()
            }),
            input: operation.input.into(),
        }
    }
}

impl From<EditProposalPolicyOperationInput> for wallet_api::EditProposalPolicyOperationInput {
    fn from(
        input: EditProposalPolicyOperationInput,
    ) -> wallet_api::EditProposalPolicyOperationInput {
        wallet_api::EditProposalPolicyOperationInput {
            policy_id: Uuid::from_bytes(input.policy_id).hyphenated().to_string(),
            specifier: input.specifier.map(|specifier| specifier.into()),
            criteria: input.criteria.map(|criteria| criteria.into()),
        }
    }
}

impl From<wallet_api::EditProposalPolicyOperationInput> for EditProposalPolicyOperationInput {
    fn from(
        input: wallet_api::EditProposalPolicyOperationInput,
    ) -> EditProposalPolicyOperationInput {
        EditProposalPolicyOperationInput {
            policy_id: *HelperMapper::to_uuid(input.policy_id)
                .expect("Invalid policy id")
                .as_bytes(),
            specifier: input.specifier.map(|specifier| specifier.into()),
            criteria: input.criteria.map(|criteria| criteria.into()),
        }
    }
}

impl From<EditProposalPolicyOperation> for wallet_api::EditProposalPolicyOperationDTO {
    fn from(operation: EditProposalPolicyOperation) -> wallet_api::EditProposalPolicyOperationDTO {
        wallet_api::EditProposalPolicyOperationDTO {
            input: operation.input.into(),
        }
    }
}

impl From<RemoveProposalPolicyOperationInput> for wallet_api::RemoveProposalPolicyOperationInput {
    fn from(
        input: RemoveProposalPolicyOperationInput,
    ) -> wallet_api::RemoveProposalPolicyOperationInput {
        wallet_api::RemoveProposalPolicyOperationInput {
            policy_id: Uuid::from_bytes(input.policy_id).hyphenated().to_string(),
        }
    }
}

impl From<wallet_api::RemoveProposalPolicyOperationInput> for RemoveProposalPolicyOperationInput {
    fn from(
        input: wallet_api::RemoveProposalPolicyOperationInput,
    ) -> RemoveProposalPolicyOperationInput {
        RemoveProposalPolicyOperationInput {
            policy_id: *HelperMapper::to_uuid(input.policy_id)
                .expect("Invalid policy id")
                .as_bytes(),
        }
    }
}

impl From<RemoveProposalPolicyOperation> for wallet_api::RemoveProposalPolicyOperationDTO {
    fn from(
        operation: RemoveProposalPolicyOperation,
    ) -> wallet_api::RemoveProposalPolicyOperationDTO {
        wallet_api::RemoveProposalPolicyOperationDTO {
            input: operation.input.into(),
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
            ProposalOperation::AddUserGroup(operation) => {
                let user_group = operation.user_group_id.map(|id| {
                    USER_GROUP_REPOSITORY
                        .get(&id)
                        .expect("User group not found")
                });

                ProposalOperationDTO::AddUserGroup(Box::new(operation.to_dto(user_group)))
            }
            ProposalOperation::EditUserGroup(operation) => {
                ProposalOperationDTO::EditUserGroup(Box::new(operation.into()))
            }
            ProposalOperation::RemoveUserGroup(operation) => {
                ProposalOperationDTO::RemoveUserGroup(Box::new(operation.into()))
            }
            ProposalOperation::ChangeCanister(operation) => {
                ProposalOperationDTO::ChangeCanister(Box::new(operation.into()))
            }
            ProposalOperation::AddAccessPolicy(operation) => {
                ProposalOperationDTO::AddAccessPolicy(Box::new(operation.into()))
            }
            ProposalOperation::EditAccessPolicy(operation) => {
                ProposalOperationDTO::EditAccessPolicy(Box::new(operation.into()))
            }
            ProposalOperation::RemoveAccessPolicy(operation) => {
                ProposalOperationDTO::RemoveAccessPolicy(Box::new(operation.into()))
            }
            ProposalOperation::AddProposalPolicy(operation) => {
                ProposalOperationDTO::AddProposalPolicy(Box::new(operation.into()))
            }
            ProposalOperation::EditProposalPolicy(operation) => {
                ProposalOperationDTO::EditProposalPolicy(Box::new(operation.into()))
            }
            ProposalOperation::RemoveProposalPolicy(operation) => {
                ProposalOperationDTO::RemoveProposalPolicy(Box::new(operation.into()))
            }
        }
    }
}
