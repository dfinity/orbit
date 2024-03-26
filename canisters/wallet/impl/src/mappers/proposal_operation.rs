use super::{blockchain::BlockchainMapper, HelperMapper};
use crate::{
    models::{
        Account, AddAccountOperation, AddAccountOperationInput, AddAddressBookEntryOperation,
        AddAddressBookEntryOperationInput, AddProposalPolicyOperation,
        AddProposalPolicyOperationInput, AddUserOperation, AddUserOperationInput, AddressBookEntry,
        ChangeCanisterOperation, ChangeCanisterOperationInput, ChangeCanisterTarget,
        EditAccessPolicyOperation, EditAccessPolicyOperationInput, EditAccountOperation,
        EditAccountOperationInput, EditAddressBookEntryOperation, EditProposalPolicyOperation,
        EditProposalPolicyOperationInput, EditUserOperation, EditUserOperationInput,
        ProposalOperation, RemoveAddressBookEntryOperation, RemoveProposalPolicyOperation,
        RemoveProposalPolicyOperationInput, TransferOperation, User,
    },
    repositories::{
        AccountRepository, AddressBookRepository, UserRepository, USER_GROUP_REPOSITORY,
    },
};
use ic_canister_core::repository::Repository;
use uuid::Uuid;
use wallet_api::{
    AddAccountOperationDTO, AddAddressBookEntryOperationDTO, AddUserOperationDTO,
    ChangeCanisterOperationDTO, ChangeCanisterTargetDTO, EditAccountOperationDTO,
    EditAddressBookEntryOperationDTO, EditUserOperationDTO, NetworkDTO, ProposalOperationDTO,
    RemoveAddressBookEntryOperationDTO, TransferOperationDTO,
};

impl TransferOperation {
    pub fn to_dto(self, account: Option<Account>) -> TransferOperationDTO {
        TransferOperationDTO {
            from_account: account.map(|account| account.to_dto()),
            network: NetworkDTO {
                id: self.input.network.clone(),
                name: self.input.network.clone(),
            },
            input: wallet_api::TransferOperationInput {
                from_account_id: Uuid::from_bytes(self.input.from_account_id)
                    .hyphenated()
                    .to_string(),
                amount: self.input.amount,
                to: self.input.to,
                fee: self.input.fee,
                metadata: self.input.metadata.into_vec_dto(),
                network: Some(NetworkDTO {
                    id: self.input.network.clone(),
                    name: self.input.network.clone(),
                }),
            },
            transfer_id: self
                .transfer_id
                .map(|id| Uuid::from_bytes(id).hyphenated().to_string()),
        }
    }
}

impl AddAccountOperation {
    pub fn to_dto(self, account: Option<Account>) -> AddAccountOperationDTO {
        AddAccountOperationDTO {
            account: account.map(|account: Account| account.to_dto()),
            input: wallet_api::AddAccountOperationInput {
                name: self.input.name,
                blockchain: self.input.blockchain.to_string(),
                standard: self.input.standard.to_string(),
                metadata: self.input.metadata.into_vec_dto(),
                read_access_policy: self.input.read_access_policy.into(),
                transfer_access_policy: self.input.transfer_access_policy.into(),
                update_access_policy: self.input.update_access_policy.into(),
                transfer_approval_policy: self.input.transfer_approval_policy.map(Into::into),
                update_approval_policy: self.input.update_approval_policy.map(Into::into),
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

impl From<wallet_api::AddAccountOperationInput> for AddAccountOperationInput {
    fn from(input: wallet_api::AddAccountOperationInput) -> AddAccountOperationInput {
        AddAccountOperationInput {
            name: input.name,
            blockchain: BlockchainMapper::to_blockchain(input.blockchain.clone())
                .expect("Invalid blockchain"),
            standard: BlockchainMapper::to_blockchain_standard(input.standard)
                .expect("Invalid blockchain standard"),
            metadata: input.metadata.into(),
            read_access_policy: input.read_access_policy.into(),
            update_access_policy: input.update_access_policy.into(),
            transfer_access_policy: input.transfer_access_policy.into(),
            transfer_approval_policy: input.transfer_approval_policy.map(Into::into),
            update_approval_policy: input.update_approval_policy.map(Into::into),
        }
    }
}

impl From<EditAccountOperation> for EditAccountOperationDTO {
    fn from(operation: EditAccountOperation) -> EditAccountOperationDTO {
        EditAccountOperationDTO {
            input: wallet_api::EditAccountOperationInput {
                account_id: Uuid::from_bytes(operation.input.account_id)
                    .hyphenated()
                    .to_string(),
                name: operation.input.name,
                read_access_policy: operation
                    .input
                    .read_access_policy
                    .map(|policy| policy.into()),
                transfer_access_policy: operation
                    .input
                    .transfer_access_policy
                    .map(|policy| policy.into()),
                update_access_policy: operation
                    .input
                    .update_access_policy
                    .map(|policy| policy.into()),
                transfer_approval_policy: operation
                    .input
                    .transfer_approval_policy
                    .map(|policy| policy.into()),
                update_approval_policy: operation
                    .input
                    .update_approval_policy
                    .map(|policy| policy.into()),
            },
        }
    }
}

impl From<wallet_api::EditAccountOperationInput> for EditAccountOperationInput {
    fn from(input: wallet_api::EditAccountOperationInput) -> EditAccountOperationInput {
        EditAccountOperationInput {
            account_id: *HelperMapper::to_uuid(input.account_id)
                .expect("Invalid account id")
                .as_bytes(),
            name: input.name,
            read_access_policy: input.read_access_policy.map(|policy| policy.into()),
            transfer_access_policy: input.transfer_access_policy.map(|policy| policy.into()),
            update_access_policy: input.update_access_policy.map(|policy| policy.into()),
            transfer_approval_policy: input.transfer_approval_policy.map(|policy| policy.into()),
            update_approval_policy: input.update_approval_policy.map(|policy| policy.into()),
        }
    }
}

impl AddAddressBookEntryOperation {
    pub fn to_dto(
        self,
        address_book_entry: Option<AddressBookEntry>,
    ) -> AddAddressBookEntryOperationDTO {
        AddAddressBookEntryOperationDTO {
            address_book_entry: address_book_entry
                .map(|address_book_entry| address_book_entry.to_dto()),
            input: wallet_api::AddAddressBookEntryOperationInput {
                address_owner: self.input.address_owner,
                address: self.input.address,
                blockchain: self.input.blockchain.to_string(),
                standard: self.input.standard.to_string(),
                metadata: self.input.metadata.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl From<wallet_api::AddAddressBookEntryOperationInput> for AddAddressBookEntryOperationInput {
    fn from(
        input: wallet_api::AddAddressBookEntryOperationInput,
    ) -> AddAddressBookEntryOperationInput {
        AddAddressBookEntryOperationInput {
            address_owner: input.address_owner,
            address: input.address,
            blockchain: BlockchainMapper::to_blockchain(input.blockchain.clone())
                .expect("Invalid blockchain"),
            standard: BlockchainMapper::to_blockchain_standard(input.standard)
                .expect("Invalid blockchain standard"),
            metadata: input.metadata.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<EditAddressBookEntryOperation> for EditAddressBookEntryOperationDTO {
    fn from(operation: EditAddressBookEntryOperation) -> EditAddressBookEntryOperationDTO {
        EditAddressBookEntryOperationDTO {
            input: wallet_api::EditAddressBookEntryOperationInput {
                address_book_entry_id: Uuid::from_bytes(operation.input.address_book_entry_id)
                    .hyphenated()
                    .to_string(),
                address_owner: operation.input.address_owner,
                change_metadata: operation
                    .input
                    .change_metadata
                    .map(|change_metadata| change_metadata.into()),
            },
        }
    }
}

impl From<RemoveAddressBookEntryOperation> for RemoveAddressBookEntryOperationDTO {
    fn from(operation: RemoveAddressBookEntryOperation) -> RemoveAddressBookEntryOperationDTO {
        RemoveAddressBookEntryOperationDTO {
            input: wallet_api::RemoveAddressBookEntryOperationInput {
                address_book_entry_id: Uuid::from_bytes(operation.input.address_book_entry_id)
                    .hyphenated()
                    .to_string(),
            },
        }
    }
}

impl AddUserOperation {
    pub fn to_dto(self, user: Option<User>) -> AddUserOperationDTO {
        AddUserOperationDTO {
            user: user.map(|user| user.into()),
            input: wallet_api::AddUserOperationInput {
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
            input: wallet_api::EditUserOperationInput {
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
                status: operation.input.status.map(|status| status.into()),
            },
        }
    }
}

impl From<wallet_api::AddUserOperationInput> for AddUserOperationInput {
    fn from(input: wallet_api::AddUserOperationInput) -> AddUserOperationInput {
        AddUserOperationInput {
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

impl From<wallet_api::EditUserOperationInput> for EditUserOperationInput {
    fn from(input: wallet_api::EditUserOperationInput) -> EditUserOperationInput {
        EditUserOperationInput {
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
            status: input.status.map(|status| status.into()),
        }
    }
}

impl From<ChangeCanisterTarget> for ChangeCanisterTargetDTO {
    fn from(value: ChangeCanisterTarget) -> Self {
        match value {
            ChangeCanisterTarget::UpgradeWallet => ChangeCanisterTargetDTO::UpgradeWallet,
            ChangeCanisterTarget::UpgradeUpgrader => ChangeCanisterTargetDTO::UpgradeUpgrader,
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
            ChangeCanisterTargetDTO::UpgradeCanister(canister_id) => {
                ChangeCanisterTarget::UpgradeCanister(canister_id)
            }
        }
    }
}

impl From<ChangeCanisterOperationInput> for wallet_api::ChangeCanisterOperationInput {
    fn from(input: ChangeCanisterOperationInput) -> wallet_api::ChangeCanisterOperationInput {
        wallet_api::ChangeCanisterOperationInput {
            target: input.target.into(),
            module: input.module,
            arg: input.arg,
        }
    }
}

impl From<wallet_api::ChangeCanisterOperationInput> for ChangeCanisterOperationInput {
    fn from(input: wallet_api::ChangeCanisterOperationInput) -> ChangeCanisterOperationInput {
        ChangeCanisterOperationInput {
            target: input.target.into(),
            module: input.module,
            arg: input.arg,
        }
    }
}

impl From<ChangeCanisterOperation> for ChangeCanisterOperationDTO {
    fn from(operation: ChangeCanisterOperation) -> ChangeCanisterOperationDTO {
        ChangeCanisterOperationDTO {
            target: operation.input.target.into(),
            module_checksum: hex::encode(operation.module_checksum),
            arg_checksum: operation.arg_checksum.map(hex::encode),
        }
    }
}

impl From<EditAccessPolicyOperationInput> for wallet_api::EditAccessPolicyOperationInput {
    fn from(input: EditAccessPolicyOperationInput) -> wallet_api::EditAccessPolicyOperationInput {
        wallet_api::EditAccessPolicyOperationInput {
            auth_scope: input.auth_scope.map(|auth_scope| auth_scope.into()),
            user_groups: input.user_groups.map(|ids| {
                ids.iter()
                    .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                    .collect::<Vec<String>>()
            }),
            users: input.users.map(|ids| {
                ids.iter()
                    .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                    .collect()
            }),
            resource: input.resource.into(),
        }
    }
}

impl From<wallet_api::EditAccessPolicyOperationInput> for EditAccessPolicyOperationInput {
    fn from(input: wallet_api::EditAccessPolicyOperationInput) -> EditAccessPolicyOperationInput {
        EditAccessPolicyOperationInput {
            auth_scope: input.auth_scope.map(|auth_scope| auth_scope.into()),
            user_groups: input.user_groups.map(|ids| {
                ids.into_iter()
                    .map(|id| {
                        *HelperMapper::to_uuid(id)
                            .expect("Invalid user group id")
                            .as_bytes()
                    })
                    .collect()
            }),
            users: input.users.map(|ids| {
                ids.into_iter()
                    .map(|id| {
                        *HelperMapper::to_uuid(id)
                            .expect("Invalid user id")
                            .as_bytes()
                    })
                    .collect()
            }),
            resource: input.resource.into(),
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
            policy_id: operation
                .policy_id
                .map(|id| Uuid::from_bytes(id).hyphenated().to_string()),
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
                    .get(&Account::key(operation.input.from_account_id));

                ProposalOperationDTO::Transfer(Box::new(operation.to_dto(account)))
            }
            ProposalOperation::AddAccount(operation) => {
                let account = operation
                    .account_id
                    .and_then(|id| AccountRepository::default().get(&Account::key(id)));

                ProposalOperationDTO::AddAccount(Box::new(operation.to_dto(account)))
            }
            ProposalOperation::EditAccount(operation) => {
                ProposalOperationDTO::EditAccount(Box::new(operation.into()))
            }
            ProposalOperation::AddAddressBookEntry(operation) => {
                let address_book_entry = operation.address_book_entry_id.and_then(|id| {
                    AddressBookRepository::default().get(&AddressBookEntry::key(id))
                });

                ProposalOperationDTO::AddAddressBookEntry(Box::new(
                    operation.to_dto(address_book_entry),
                ))
            }
            ProposalOperation::EditAddressBookEntry(operation) => {
                ProposalOperationDTO::EditAddressBookEntry(Box::new(operation.into()))
            }
            ProposalOperation::RemoveAddressBookEntry(operation) => {
                ProposalOperationDTO::RemoveAddressBookEntry(Box::new(operation.into()))
            }
            ProposalOperation::AddUser(operation) => {
                let user = operation
                    .user_id
                    .and_then(|id| UserRepository::default().get(&User::key(id)));

                ProposalOperationDTO::AddUser(Box::new(operation.to_dto(user)))
            }
            ProposalOperation::EditUser(operation) => {
                ProposalOperationDTO::EditUser(Box::new(operation.into()))
            }
            ProposalOperation::AddUserGroup(operation) => {
                let user_group = operation
                    .user_group_id
                    .and_then(|id| USER_GROUP_REPOSITORY.get(&id));

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
            ProposalOperation::EditAccessPolicy(operation) => {
                ProposalOperationDTO::EditAccessPolicy(Box::new(operation.into()))
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
