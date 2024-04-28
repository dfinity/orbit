use super::{blockchain::BlockchainMapper, HelperMapper};
use crate::{
    models::{
        resource::{
            AccountResourceAction, ChangeCanisterResourceAction, PermissionResourceAction,
            Resource, ResourceAction, ResourceId, UserResourceAction,
        },
        Account, AddAccountOperation, AddAccountOperationInput, AddAddressBookEntryOperation,
        AddAddressBookEntryOperationInput, AddRequestPolicyOperation,
        AddRequestPolicyOperationInput, AddUserOperation, AddUserOperationInput, AddressBookEntry,
        ChangeCanisterOperation, ChangeCanisterOperationInput, ChangeCanisterTarget,
        EditAccountOperation, EditAccountOperationInput, EditAddressBookEntryOperation,
        EditPermissionOperation, EditPermissionOperationInput, EditRequestPolicyOperation,
        EditRequestPolicyOperationInput, EditUserGroupOperation, EditUserOperation,
        EditUserOperationInput, RemoveAddressBookEntryOperation, RemoveRequestPolicyOperation,
        RemoveRequestPolicyOperationInput, RemoveUserGroupOperation, RequestOperation,
        TransferOperation, User,
    },
    repositories::{
        AccountRepository, AddressBookRepository, UserRepository, USER_GROUP_REPOSITORY,
    },
};
use orbit_essentials::repository::Repository;
use station_api::{
    AddAccountOperationDTO, AddAddressBookEntryOperationDTO, AddUserOperationDTO,
    ChangeCanisterOperationDTO, ChangeCanisterTargetDTO, EditAccountOperationDTO,
    EditAddressBookEntryOperationDTO, EditUserOperationDTO, NetworkDTO,
    RemoveAddressBookEntryOperationDTO, RequestOperationDTO, TransferOperationDTO,
};
use uuid::Uuid;

impl TransferOperation {
    pub fn to_dto(self, account: Option<Account>) -> TransferOperationDTO {
        TransferOperationDTO {
            from_account: account.map(|account| account.to_dto()),
            network: NetworkDTO {
                id: self.input.network.clone(),
                name: self.input.network.clone(),
            },
            input: station_api::TransferOperationInput {
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
            input: station_api::AddAccountOperationInput {
                name: self.input.name,
                blockchain: self.input.blockchain.to_string(),
                standard: self.input.standard.to_string(),
                metadata: self.input.metadata.into_vec_dto(),
                read_permission: self.input.read_permission.into(),
                transfer_permission: self.input.transfer_permission.into(),
                configs_permission: self.input.configs_permission.into(),
                transfer_request_policy: self.input.transfer_request_policy.map(Into::into),
                configs_request_policy: self.input.configs_request_policy.map(Into::into),
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

impl From<station_api::AddAccountOperationInput> for AddAccountOperationInput {
    fn from(input: station_api::AddAccountOperationInput) -> AddAccountOperationInput {
        AddAccountOperationInput {
            name: input.name,
            blockchain: BlockchainMapper::to_blockchain(input.blockchain.clone())
                .expect("Invalid blockchain"),
            standard: BlockchainMapper::to_blockchain_standard(input.standard)
                .expect("Invalid blockchain standard"),
            metadata: input.metadata.into(),
            read_permission: input.read_permission.into(),
            configs_permission: input.configs_permission.into(),
            transfer_permission: input.transfer_permission.into(),
            transfer_request_policy: input.transfer_request_policy.map(Into::into),
            configs_request_policy: input.configs_request_policy.map(Into::into),
        }
    }
}

impl From<EditAccountOperation> for EditAccountOperationDTO {
    fn from(operation: EditAccountOperation) -> EditAccountOperationDTO {
        EditAccountOperationDTO {
            input: station_api::EditAccountOperationInput {
                account_id: Uuid::from_bytes(operation.input.account_id)
                    .hyphenated()
                    .to_string(),
                name: operation.input.name,
                read_permission: operation.input.read_permission.map(|policy| policy.into()),
                transfer_permission: operation
                    .input
                    .transfer_permission
                    .map(|policy| policy.into()),
                configs_permission: operation
                    .input
                    .configs_permission
                    .map(|policy| policy.into()),
                transfer_request_policy: operation
                    .input
                    .transfer_request_policy
                    .map(|policy| policy.into()),
                configs_request_policy: operation
                    .input
                    .configs_request_policy
                    .map(|policy| policy.into()),
            },
        }
    }
}

impl From<station_api::EditAccountOperationInput> for EditAccountOperationInput {
    fn from(input: station_api::EditAccountOperationInput) -> EditAccountOperationInput {
        EditAccountOperationInput {
            account_id: *HelperMapper::to_uuid(input.account_id)
                .expect("Invalid account id")
                .as_bytes(),
            name: input.name,
            read_permission: input.read_permission.map(|policy| policy.into()),
            transfer_permission: input.transfer_permission.map(|policy| policy.into()),
            configs_permission: input.configs_permission.map(|policy| policy.into()),
            transfer_request_policy: input.transfer_request_policy.map(|policy| policy.into()),
            configs_request_policy: input.configs_request_policy.map(|policy| policy.into()),
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
            input: station_api::AddAddressBookEntryOperationInput {
                address_owner: self.input.address_owner,
                address: self.input.address,
                blockchain: self.input.blockchain.to_string(),
                standard: self.input.standard.to_string(),
                metadata: self.input.metadata.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl From<station_api::AddAddressBookEntryOperationInput> for AddAddressBookEntryOperationInput {
    fn from(
        input: station_api::AddAddressBookEntryOperationInput,
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
            input: station_api::EditAddressBookEntryOperationInput {
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
            input: station_api::RemoveAddressBookEntryOperationInput {
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
            input: station_api::AddUserOperationInput {
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
            input: station_api::EditUserOperationInput {
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

impl From<station_api::AddUserOperationInput> for AddUserOperationInput {
    fn from(input: station_api::AddUserOperationInput) -> AddUserOperationInput {
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

impl From<station_api::EditUserOperationInput> for EditUserOperationInput {
    fn from(input: station_api::EditUserOperationInput) -> EditUserOperationInput {
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
            ChangeCanisterTarget::UpgradeStation => ChangeCanisterTargetDTO::UpgradeStation,
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
            ChangeCanisterTargetDTO::UpgradeStation => ChangeCanisterTarget::UpgradeStation,
            ChangeCanisterTargetDTO::UpgradeUpgrader => ChangeCanisterTarget::UpgradeUpgrader,
            ChangeCanisterTargetDTO::UpgradeCanister(canister_id) => {
                ChangeCanisterTarget::UpgradeCanister(canister_id)
            }
        }
    }
}

impl From<ChangeCanisterOperationInput> for station_api::ChangeCanisterOperationInput {
    fn from(input: ChangeCanisterOperationInput) -> station_api::ChangeCanisterOperationInput {
        station_api::ChangeCanisterOperationInput {
            target: input.target.into(),
            module: input.module,
            arg: input.arg,
        }
    }
}

impl From<station_api::ChangeCanisterOperationInput> for ChangeCanisterOperationInput {
    fn from(input: station_api::ChangeCanisterOperationInput) -> ChangeCanisterOperationInput {
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

impl From<EditPermissionOperationInput> for station_api::EditPermissionOperationInput {
    fn from(input: EditPermissionOperationInput) -> station_api::EditPermissionOperationInput {
        station_api::EditPermissionOperationInput {
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

impl From<station_api::EditPermissionOperationInput> for EditPermissionOperationInput {
    fn from(input: station_api::EditPermissionOperationInput) -> EditPermissionOperationInput {
        EditPermissionOperationInput {
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

impl From<EditPermissionOperation> for station_api::EditPermissionOperationDTO {
    fn from(operation: EditPermissionOperation) -> station_api::EditPermissionOperationDTO {
        station_api::EditPermissionOperationDTO {
            input: operation.input.into(),
        }
    }
}

impl From<AddRequestPolicyOperationInput> for station_api::AddRequestPolicyOperationInput {
    fn from(input: AddRequestPolicyOperationInput) -> station_api::AddRequestPolicyOperationInput {
        station_api::AddRequestPolicyOperationInput {
            specifier: input.specifier.into(),
            rule: input.rule.into(),
        }
    }
}

impl From<station_api::AddRequestPolicyOperationInput> for AddRequestPolicyOperationInput {
    fn from(input: station_api::AddRequestPolicyOperationInput) -> AddRequestPolicyOperationInput {
        AddRequestPolicyOperationInput {
            specifier: input.specifier.into(),
            rule: input.rule.into(),
        }
    }
}

impl From<AddRequestPolicyOperation> for station_api::AddRequestPolicyOperationDTO {
    fn from(operation: AddRequestPolicyOperation) -> station_api::AddRequestPolicyOperationDTO {
        station_api::AddRequestPolicyOperationDTO {
            policy_id: operation
                .policy_id
                .map(|id| Uuid::from_bytes(id).hyphenated().to_string()),
            input: operation.input.into(),
        }
    }
}

impl From<EditRequestPolicyOperationInput> for station_api::EditRequestPolicyOperationInput {
    fn from(
        input: EditRequestPolicyOperationInput,
    ) -> station_api::EditRequestPolicyOperationInput {
        station_api::EditRequestPolicyOperationInput {
            policy_id: Uuid::from_bytes(input.policy_id).hyphenated().to_string(),
            specifier: input.specifier.map(|specifier| specifier.into()),
            rule: input.rule.map(|rule| rule.into()),
        }
    }
}

impl From<station_api::EditRequestPolicyOperationInput> for EditRequestPolicyOperationInput {
    fn from(
        input: station_api::EditRequestPolicyOperationInput,
    ) -> EditRequestPolicyOperationInput {
        EditRequestPolicyOperationInput {
            policy_id: *HelperMapper::to_uuid(input.policy_id)
                .expect("Invalid policy id")
                .as_bytes(),
            specifier: input.specifier.map(|specifier| specifier.into()),
            rule: input.rule.map(|rule| rule.into()),
        }
    }
}

impl From<EditRequestPolicyOperation> for station_api::EditRequestPolicyOperationDTO {
    fn from(operation: EditRequestPolicyOperation) -> station_api::EditRequestPolicyOperationDTO {
        station_api::EditRequestPolicyOperationDTO {
            input: operation.input.into(),
        }
    }
}

impl From<RemoveRequestPolicyOperationInput> for station_api::RemoveRequestPolicyOperationInput {
    fn from(
        input: RemoveRequestPolicyOperationInput,
    ) -> station_api::RemoveRequestPolicyOperationInput {
        station_api::RemoveRequestPolicyOperationInput {
            policy_id: Uuid::from_bytes(input.policy_id).hyphenated().to_string(),
        }
    }
}

impl From<station_api::RemoveRequestPolicyOperationInput> for RemoveRequestPolicyOperationInput {
    fn from(
        input: station_api::RemoveRequestPolicyOperationInput,
    ) -> RemoveRequestPolicyOperationInput {
        RemoveRequestPolicyOperationInput {
            policy_id: *HelperMapper::to_uuid(input.policy_id)
                .expect("Invalid policy id")
                .as_bytes(),
        }
    }
}

impl From<RemoveRequestPolicyOperation> for station_api::RemoveRequestPolicyOperationDTO {
    fn from(
        operation: RemoveRequestPolicyOperation,
    ) -> station_api::RemoveRequestPolicyOperationDTO {
        station_api::RemoveRequestPolicyOperationDTO {
            input: operation.input.into(),
        }
    }
}

impl From<RequestOperation> for RequestOperationDTO {
    fn from(operation: RequestOperation) -> RequestOperationDTO {
        match operation {
            RequestOperation::Transfer(operation) => {
                let account = AccountRepository::default()
                    .get(&Account::key(operation.input.from_account_id));

                RequestOperationDTO::Transfer(Box::new(operation.to_dto(account)))
            }
            RequestOperation::AddAccount(operation) => {
                let account = operation
                    .account_id
                    .and_then(|id| AccountRepository::default().get(&Account::key(id)));

                RequestOperationDTO::AddAccount(Box::new(operation.to_dto(account)))
            }
            RequestOperation::EditAccount(operation) => {
                RequestOperationDTO::EditAccount(Box::new(operation.into()))
            }
            RequestOperation::AddAddressBookEntry(operation) => {
                let address_book_entry = operation.address_book_entry_id.and_then(|id| {
                    AddressBookRepository::default().get(&AddressBookEntry::key(id))
                });

                RequestOperationDTO::AddAddressBookEntry(Box::new(
                    operation.to_dto(address_book_entry),
                ))
            }
            RequestOperation::EditAddressBookEntry(operation) => {
                RequestOperationDTO::EditAddressBookEntry(Box::new(operation.into()))
            }
            RequestOperation::RemoveAddressBookEntry(operation) => {
                RequestOperationDTO::RemoveAddressBookEntry(Box::new(operation.into()))
            }
            RequestOperation::AddUser(operation) => {
                let user = operation
                    .user_id
                    .and_then(|id| UserRepository::default().get(&User::key(id)));

                RequestOperationDTO::AddUser(Box::new(operation.to_dto(user)))
            }
            RequestOperation::EditUser(operation) => {
                RequestOperationDTO::EditUser(Box::new(operation.into()))
            }
            RequestOperation::AddUserGroup(operation) => {
                let user_group = operation
                    .user_group_id
                    .and_then(|id| USER_GROUP_REPOSITORY.get(&id));

                RequestOperationDTO::AddUserGroup(Box::new(operation.to_dto(user_group)))
            }
            RequestOperation::EditUserGroup(operation) => {
                RequestOperationDTO::EditUserGroup(Box::new(operation.into()))
            }
            RequestOperation::RemoveUserGroup(operation) => {
                RequestOperationDTO::RemoveUserGroup(Box::new(operation.into()))
            }
            RequestOperation::ChangeCanister(operation) => {
                RequestOperationDTO::ChangeCanister(Box::new(operation.into()))
            }
            RequestOperation::EditPermission(operation) => {
                RequestOperationDTO::EditPermission(Box::new(operation.into()))
            }
            RequestOperation::AddRequestPolicy(operation) => {
                RequestOperationDTO::AddRequestPolicy(Box::new(operation.into()))
            }
            RequestOperation::EditRequestPolicy(operation) => {
                RequestOperationDTO::EditRequestPolicy(Box::new(operation.into()))
            }
            RequestOperation::RemoveRequestPolicy(operation) => {
                RequestOperationDTO::RemoveRequestPolicy(Box::new(operation.into()))
            }
        }
    }
}

impl RequestOperation {
    pub fn to_resources(&self) -> Vec<Resource> {
        match self {
            RequestOperation::AddAccount(_) => {
                vec![Resource::Account(AccountResourceAction::Create)]
            }
            RequestOperation::AddAddressBookEntry(_) => {
                vec![Resource::AddressBook(ResourceAction::Create)]
            }
            RequestOperation::AddUser(_) => vec![Resource::User(UserResourceAction::Create)],
            RequestOperation::AddUserGroup(_) => vec![Resource::UserGroup(ResourceAction::Create)],

            RequestOperation::AddRequestPolicy(_) => {
                vec![Resource::RequestPolicy(ResourceAction::Create)]
            }
            RequestOperation::EditPermission(_) => {
                vec![Resource::Permission(PermissionResourceAction::Update)]
            }

            RequestOperation::Transfer(transfer) => {
                vec![
                    Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                        transfer.input.from_account_id,
                    ))),
                    Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
                ]
            }

            RequestOperation::EditAccount(EditAccountOperation { input }) => {
                vec![
                    Resource::Account(AccountResourceAction::Update(ResourceId::Id(
                        input.account_id,
                    ))),
                    Resource::Account(AccountResourceAction::Update(ResourceId::Any)),
                ]
            }
            RequestOperation::EditAddressBookEntry(EditAddressBookEntryOperation {
                input, ..
            }) => {
                vec![
                    Resource::AddressBook(ResourceAction::Update(ResourceId::Id(
                        input.address_book_entry_id,
                    ))),
                    Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
                ]
            }
            RequestOperation::RemoveAddressBookEntry(RemoveAddressBookEntryOperation { input }) => {
                vec![
                    Resource::AddressBook(ResourceAction::Delete(ResourceId::Id(
                        input.address_book_entry_id,
                    ))),
                    Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
                ]
            }
            RequestOperation::EditUser(EditUserOperation { input }) => {
                vec![
                    Resource::User(UserResourceAction::Update(ResourceId::Id(input.user_id))),
                    Resource::User(UserResourceAction::Update(ResourceId::Any)),
                ]
            }
            RequestOperation::EditUserGroup(EditUserGroupOperation { input }) => {
                vec![
                    Resource::UserGroup(ResourceAction::Update(ResourceId::Id(
                        input.user_group_id,
                    ))),
                    Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
                ]
            }
            RequestOperation::RemoveUserGroup(RemoveUserGroupOperation { input }) => {
                vec![
                    Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(
                        input.user_group_id,
                    ))),
                    Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
                ]
            }
            RequestOperation::ChangeCanister(_) => {
                vec![Resource::ChangeCanister(
                    ChangeCanisterResourceAction::Create,
                )]
            }
            RequestOperation::EditRequestPolicy(EditRequestPolicyOperation { input }) => {
                vec![
                    Resource::RequestPolicy(ResourceAction::Update(ResourceId::Id(
                        input.policy_id,
                    ))),
                    Resource::RequestPolicy(ResourceAction::Update(ResourceId::Any)),
                ]
            }
            RequestOperation::RemoveRequestPolicy(RemoveRequestPolicyOperation { input }) => {
                vec![
                    Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Id(
                        input.policy_id,
                    ))),
                    Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Any)),
                ]
            }
        }
    }
}
