use super::{blockchain::BlockchainMapper, HelperMapper};
use crate::{
    models::{
        resource::{
            AccountResourceAction, CallExternalCanisterResourceTarget,
            ExecutionMethodResourceTarget, ExternalCanisterId, ExternalCanisterResourceAction,
            PermissionResourceAction, Resource, ResourceAction, ResourceId, SystemResourceAction,
            UserResourceAction,
        },
        Account, AccountKey, AddAccountOperation, AddAccountOperationInput,
        AddAddressBookEntryOperation, AddAddressBookEntryOperationInput, AddRequestPolicyOperation,
        AddRequestPolicyOperationInput, AddUserOperation, AddUserOperationInput, AddressBookEntry,
        CallExternalCanisterOperation, CallExternalCanisterOperationInput,
        CanisterExecutionAndValidationMethodPairInput, CanisterInstallMode,
        CanisterInstallModeArgs, CanisterMethod, CanisterReinstallModeArgs,
        CanisterUpgradeModeArgs, ChangeExternalCanisterOperation,
        ChangeExternalCanisterOperationInput, ConfigureExternalCanisterOperation,
        ConfigureExternalCanisterOperationKind, ConfigureExternalCanisterSettingsInput,
        CreateExternalCanisterOperation, CreateExternalCanisterOperationInput,
        CreateExternalCanisterOperationKind, CreateExternalCanisterOperationKindAddExisting,
        CreateExternalCanisterOperationKindCreateNew, CycleObtainStrategy,
        DefiniteCanisterSettingsInput, DisasterRecoveryCommittee, EditAccountOperation,
        EditAccountOperationInput, EditAddressBookEntryOperation, EditPermissionOperation,
        EditPermissionOperationInput, EditRequestPolicyOperation, EditRequestPolicyOperationInput,
        EditUserGroupOperation, EditUserOperation, EditUserOperationInput,
        ExternalCanisterCallPermission, ExternalCanisterCallPermissionMethodPairInput,
        ExternalCanisterCallRequestPoliciesMethodPairInput,
        ExternalCanisterCallRequestPolicyRuleInput, ExternalCanisterChangeCallPermissionsInput,
        ExternalCanisterChangeCallRequestPoliciesInput,
        ExternalCanisterChangeRequestPolicyRuleInput, ExternalCanisterPermissionsCreateInput,
        ExternalCanisterPermissionsUpdateInput, ExternalCanisterRequestPoliciesCreateInput,
        ExternalCanisterRequestPoliciesUpdateInput, FundExternalCanisterOperation,
        ManageSystemInfoOperation, ManageSystemInfoOperationInput, RemoveAddressBookEntryOperation,
        RemoveRequestPolicyOperation, RemoveRequestPolicyOperationInput, RemoveUserGroupOperation,
        RequestOperation, SetDisasterRecoveryOperation, SetDisasterRecoveryOperationInput,
        SystemUpgradeOperation, SystemUpgradeOperationInput, SystemUpgradeTarget,
        TransferOperation, User, WasmModuleExtraChunks,
    },
    repositories::{
        AccountRepository, AddressBookRepository, UserRepository, ACCOUNT_REPOSITORY,
        USER_GROUP_REPOSITORY,
    },
};
use orbit_essentials::repository::Repository;
use station_api::{
    AddAccountOperationDTO, AddAddressBookEntryOperationDTO, AddUserOperationDTO,
    CallExternalCanisterOperationDTO, CanisterMethodDTO, ChangeExternalCanisterOperationDTO,
    CreateExternalCanisterOperationDTO, EditAccountOperationDTO, EditAddressBookEntryOperationDTO,
    EditUserOperationDTO, NetworkDTO, RemoveAddressBookEntryOperationDTO, RequestOperationDTO,
    TransferOperationDTO,
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
            fee: self.fee,
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
                metadata: self.input.metadata.into_iter().map(Into::into).collect(),
                labels: self.input.labels,
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
            metadata: input.metadata.into_iter().map(Into::into).collect(),
            labels: input.labels,
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
                labels: operation.input.labels,
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
                cancel_pending_requests: operation.input.cancel_pending_requests,
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
            cancel_pending_requests: input.cancel_pending_requests,
        }
    }
}

impl From<SystemUpgradeTarget> for station_api::SystemUpgradeTargetDTO {
    fn from(value: SystemUpgradeTarget) -> Self {
        match value {
            SystemUpgradeTarget::UpgradeStation => {
                station_api::SystemUpgradeTargetDTO::UpgradeStation
            }
            SystemUpgradeTarget::UpgradeUpgrader => {
                station_api::SystemUpgradeTargetDTO::UpgradeUpgrader
            }
        }
    }
}

impl From<station_api::SystemUpgradeTargetDTO> for SystemUpgradeTarget {
    fn from(value: station_api::SystemUpgradeTargetDTO) -> Self {
        match value {
            station_api::SystemUpgradeTargetDTO::UpgradeStation => {
                SystemUpgradeTarget::UpgradeStation
            }
            station_api::SystemUpgradeTargetDTO::UpgradeUpgrader => {
                SystemUpgradeTarget::UpgradeUpgrader
            }
        }
    }
}

impl From<orbit_essentials::types::WasmModuleExtraChunks> for WasmModuleExtraChunks {
    fn from(input: orbit_essentials::types::WasmModuleExtraChunks) -> WasmModuleExtraChunks {
        WasmModuleExtraChunks {
            store_canister: input.store_canister,
            chunk_hashes_list: input.chunk_hashes_list,
            wasm_module_hash: input.wasm_module_hash,
        }
    }
}

impl From<WasmModuleExtraChunks> for orbit_essentials::types::WasmModuleExtraChunks {
    fn from(input: WasmModuleExtraChunks) -> orbit_essentials::types::WasmModuleExtraChunks {
        orbit_essentials::types::WasmModuleExtraChunks {
            store_canister: input.store_canister,
            chunk_hashes_list: input.chunk_hashes_list,
            wasm_module_hash: input.wasm_module_hash,
        }
    }
}

impl From<SystemUpgradeOperationInput> for station_api::SystemUpgradeOperationInput {
    fn from(input: SystemUpgradeOperationInput) -> station_api::SystemUpgradeOperationInput {
        station_api::SystemUpgradeOperationInput {
            target: input.target.into(),
            module: input.module,
            module_extra_chunks: input.module_extra_chunks.map(|c| c.into()),
            arg: input.arg,
        }
    }
}

impl From<station_api::SystemUpgradeOperationInput> for SystemUpgradeOperationInput {
    fn from(input: station_api::SystemUpgradeOperationInput) -> SystemUpgradeOperationInput {
        SystemUpgradeOperationInput {
            target: input.target.into(),
            module: input.module,
            module_extra_chunks: input.module_extra_chunks.map(|c| c.into()),
            arg: input.arg,
        }
    }
}

impl From<SystemUpgradeOperation> for station_api::SystemUpgradeOperationDTO {
    fn from(operation: SystemUpgradeOperation) -> station_api::SystemUpgradeOperationDTO {
        station_api::SystemUpgradeOperationDTO {
            target: operation.input.target.into(),
            module_checksum: hex::encode(operation.module_checksum),
            arg_checksum: operation.arg_checksum.map(hex::encode),
        }
    }
}

impl From<station_api::DisasterRecoveryCommitteeDTO> for DisasterRecoveryCommittee {
    fn from(value: station_api::DisasterRecoveryCommitteeDTO) -> Self {
        DisasterRecoveryCommittee {
            quorum: value.quorum,
            user_group_id: *HelperMapper::to_uuid(value.user_group_id)
                .expect("Invalid user group id")
                .as_bytes(),
        }
    }
}

impl From<station_api::CanisterInstallMode> for CanisterInstallMode {
    fn from(mode: station_api::CanisterInstallMode) -> Self {
        match mode {
            station_api::CanisterInstallMode::Install => {
                CanisterInstallMode::Install(CanisterInstallModeArgs {})
            }
            station_api::CanisterInstallMode::Reinstall => {
                CanisterInstallMode::Reinstall(CanisterReinstallModeArgs {})
            }
            station_api::CanisterInstallMode::Upgrade => {
                CanisterInstallMode::Upgrade(CanisterUpgradeModeArgs {})
            }
        }
    }
}

impl From<DisasterRecoveryCommittee> for station_api::DisasterRecoveryCommitteeDTO {
    fn from(value: DisasterRecoveryCommittee) -> Self {
        station_api::DisasterRecoveryCommitteeDTO {
            quorum: value.quorum,
            user_group_id: Uuid::from_bytes(value.user_group_id)
                .hyphenated()
                .to_string(),
        }
    }
}

impl From<CanisterInstallMode> for station_api::CanisterInstallMode {
    fn from(mode: CanisterInstallMode) -> Self {
        match mode {
            CanisterInstallMode::Install(CanisterInstallModeArgs {}) => {
                station_api::CanisterInstallMode::Install
            }
            CanisterInstallMode::Reinstall(CanisterReinstallModeArgs {}) => {
                station_api::CanisterInstallMode::Reinstall
            }
            CanisterInstallMode::Upgrade(CanisterUpgradeModeArgs {}) => {
                station_api::CanisterInstallMode::Upgrade
            }
        }
    }
}

impl From<station_api::SetDisasterRecoveryOperationInput> for SetDisasterRecoveryOperationInput {
    fn from(
        input: station_api::SetDisasterRecoveryOperationInput,
    ) -> SetDisasterRecoveryOperationInput {
        SetDisasterRecoveryOperationInput {
            committee: input.committee.map(|committee| committee.into()),
        }
    }
}
impl From<SetDisasterRecoveryOperation> for station_api::SetDisasterRecoveryOperationDTO {
    fn from(
        operation: SetDisasterRecoveryOperation,
    ) -> station_api::SetDisasterRecoveryOperationDTO {
        station_api::SetDisasterRecoveryOperationDTO {
            committee: operation.input.committee.map(|committee| committee.into()),
        }
    }
}

// ---

impl From<ChangeExternalCanisterOperationInput>
    for station_api::ChangeExternalCanisterOperationInput
{
    fn from(
        input: ChangeExternalCanisterOperationInput,
    ) -> station_api::ChangeExternalCanisterOperationInput {
        station_api::ChangeExternalCanisterOperationInput {
            canister_id: input.canister_id,
            mode: input.mode.into(),
            module: input.module,
            module_extra_chunks: input.module_extra_chunks.map(|c| c.into()),
            arg: input.arg,
        }
    }
}

impl From<station_api::ChangeExternalCanisterOperationInput>
    for ChangeExternalCanisterOperationInput
{
    fn from(
        input: station_api::ChangeExternalCanisterOperationInput,
    ) -> ChangeExternalCanisterOperationInput {
        ChangeExternalCanisterOperationInput {
            canister_id: input.canister_id,
            mode: input.mode.into(),
            module: input.module,
            module_extra_chunks: input.module_extra_chunks.map(|c| c.into()),
            arg: input.arg,
        }
    }
}

impl From<ChangeExternalCanisterOperation> for ChangeExternalCanisterOperationDTO {
    fn from(operation: ChangeExternalCanisterOperation) -> ChangeExternalCanisterOperationDTO {
        ChangeExternalCanisterOperationDTO {
            canister_id: operation.input.canister_id,
            mode: operation.input.mode.into(),
            module_checksum: hex::encode(operation.module_checksum),
            arg_checksum: operation.arg_checksum.map(hex::encode),
        }
    }
}

impl From<ConfigureExternalCanisterOperation>
    for station_api::ConfigureExternalCanisterOperationDTO
{
    fn from(
        operation: ConfigureExternalCanisterOperation,
    ) -> station_api::ConfigureExternalCanisterOperationDTO {
        station_api::ConfigureExternalCanisterOperationDTO {
            canister_id: operation.canister_id,
            kind: operation.kind.into(),
        }
    }
}

impl From<ConfigureExternalCanisterOperationKind>
    for station_api::ConfigureExternalCanisterOperationKindDTO
{
    fn from(
        kind: ConfigureExternalCanisterOperationKind,
    ) -> station_api::ConfigureExternalCanisterOperationKindDTO {
        match kind {
            ConfigureExternalCanisterOperationKind::Delete => {
                station_api::ConfigureExternalCanisterOperationKindDTO::Delete
            }
            ConfigureExternalCanisterOperationKind::SoftDelete => {
                station_api::ConfigureExternalCanisterOperationKindDTO::SoftDelete
            }
            ConfigureExternalCanisterOperationKind::Settings(input) => {
                station_api::ConfigureExternalCanisterOperationKindDTO::Settings(input.into())
            }
            ConfigureExternalCanisterOperationKind::NativeSettings(input) => {
                station_api::ConfigureExternalCanisterOperationKindDTO::NativeSettings(input.into())
            }
        }
    }
}

impl From<ConfigureExternalCanisterSettingsInput>
    for station_api::ConfigureExternalCanisterSettingsInput
{
    fn from(
        input: ConfigureExternalCanisterSettingsInput,
    ) -> station_api::ConfigureExternalCanisterSettingsInput {
        station_api::ConfigureExternalCanisterSettingsInput {
            name: input.name,
            description: input.description,
            labels: input.labels,
            state: input.state.map(Into::into),
            permissions: input.permissions.map(Into::into),
            request_policies: input.request_policies.map(Into::into),
        }
    }
}

impl From<DefiniteCanisterSettingsInput> for station_api::DefiniteCanisterSettingsInput {
    fn from(input: DefiniteCanisterSettingsInput) -> station_api::DefiniteCanisterSettingsInput {
        station_api::DefiniteCanisterSettingsInput {
            controllers: input.controllers,
            compute_allocation: input.compute_allocation,
            freezing_threshold: input.freezing_threshold,
            memory_allocation: input.memory_allocation,
            reserved_cycles_limit: input.reserved_cycles_limit,
        }
    }
}

impl From<station_api::ExternalCanisterRequestPoliciesUpdateInput>
    for ExternalCanisterRequestPoliciesUpdateInput
{
    fn from(
        input: station_api::ExternalCanisterRequestPoliciesUpdateInput,
    ) -> ExternalCanisterRequestPoliciesUpdateInput {
        ExternalCanisterRequestPoliciesUpdateInput {
            change: input
                .change
                .map(|change| change.into_iter().map(Into::into).collect()),
            calls: input.calls.map(Into::into),
        }
    }
}

impl From<ExternalCanisterRequestPoliciesUpdateInput>
    for station_api::ExternalCanisterRequestPoliciesUpdateInput
{
    fn from(
        input: ExternalCanisterRequestPoliciesUpdateInput,
    ) -> station_api::ExternalCanisterRequestPoliciesUpdateInput {
        station_api::ExternalCanisterRequestPoliciesUpdateInput {
            change: input
                .change
                .map(|change| change.into_iter().map(Into::into).collect()),
            calls: input.calls.map(Into::into),
        }
    }
}

impl From<station_api::ExternalCanisterChangeCallRequestPoliciesInput>
    for ExternalCanisterChangeCallRequestPoliciesInput
{
    fn from(
        input: station_api::ExternalCanisterChangeCallRequestPoliciesInput,
    ) -> ExternalCanisterChangeCallRequestPoliciesInput {
        match input {
            station_api::ExternalCanisterChangeCallRequestPoliciesInput::ReplaceAllBy(input) => {
                ExternalCanisterChangeCallRequestPoliciesInput::ReplaceAllBy(input.into_iter().map(Into::into).collect())
            }
            station_api::ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionMethods(input) => {
                ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionMethods(input.into_iter().map(Into::into).collect())
            }
            station_api::ExternalCanisterChangeCallRequestPoliciesInput::RemoveByPolicyIds(ids) => {
                ExternalCanisterChangeCallRequestPoliciesInput::RemoveByPolicyIds(ids.into_iter().map(|id| *HelperMapper::to_uuid(id).expect("Invalid policy id").as_bytes()).collect())
            }
            station_api::ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionValidationMethodPairs(input) => {
                ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionValidationMethodPairs(input.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<ExternalCanisterChangeCallRequestPoliciesInput>
    for station_api::ExternalCanisterChangeCallRequestPoliciesInput
{
    fn from(
        input: ExternalCanisterChangeCallRequestPoliciesInput,
    ) -> station_api::ExternalCanisterChangeCallRequestPoliciesInput {
        match input {
            ExternalCanisterChangeCallRequestPoliciesInput::ReplaceAllBy(input) => {
                station_api::ExternalCanisterChangeCallRequestPoliciesInput::ReplaceAllBy(input.into_iter().map(Into::into).collect())
            }
            ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionMethods(input) => {
                station_api::ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionMethods(input.into_iter().map(Into::into).collect())
            }
            ExternalCanisterChangeCallRequestPoliciesInput::RemoveByPolicyIds(ids) => {
                station_api::ExternalCanisterChangeCallRequestPoliciesInput::RemoveByPolicyIds(ids.into_iter().map(|id| Uuid::from_bytes(id).hyphenated().to_string()).collect())
            }
            ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionValidationMethodPairs(input) => {
                station_api::ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionValidationMethodPairs(input.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<station_api::ExternalCanisterCallPermissionDTO> for ExternalCanisterCallPermission {
    fn from(
        input: station_api::ExternalCanisterCallPermissionDTO,
    ) -> ExternalCanisterCallPermission {
        ExternalCanisterCallPermission {
            allow: input.allow.into(),
            validation_method: input.validation_method.into(),
            execution_method: input.execution_method,
        }
    }
}

impl From<ExternalCanisterCallPermission> for station_api::ExternalCanisterCallPermissionDTO {
    fn from(
        input: ExternalCanisterCallPermission,
    ) -> station_api::ExternalCanisterCallPermissionDTO {
        station_api::ExternalCanisterCallPermissionDTO {
            allow: input.allow.into(),
            validation_method: input.validation_method.into(),
            execution_method: input.execution_method,
        }
    }
}

impl From<station_api::ExternalCanisterPermissionsCreateInput>
    for ExternalCanisterPermissionsCreateInput
{
    fn from(
        input: station_api::ExternalCanisterPermissionsCreateInput,
    ) -> ExternalCanisterPermissionsCreateInput {
        ExternalCanisterPermissionsCreateInput {
            read: input.read.into(),
            change: input.change.into(),
            calls: input.calls.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ExternalCanisterPermissionsCreateInput>
    for station_api::ExternalCanisterPermissionsCreateInput
{
    fn from(
        input: ExternalCanisterPermissionsCreateInput,
    ) -> station_api::ExternalCanisterPermissionsCreateInput {
        station_api::ExternalCanisterPermissionsCreateInput {
            read: input.read.into(),
            change: input.change.into(),
            calls: input.calls.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<station_api::ExternalCanisterPermissionsUpdateInput>
    for ExternalCanisterPermissionsUpdateInput
{
    fn from(
        input: station_api::ExternalCanisterPermissionsUpdateInput,
    ) -> ExternalCanisterPermissionsUpdateInput {
        ExternalCanisterPermissionsUpdateInput {
            read: input.read.map(|read| read.into()),
            change: input.change.map(|change| change.into()),
            calls: input.calls.map(Into::into),
        }
    }
}

impl From<ExternalCanisterPermissionsUpdateInput>
    for station_api::ExternalCanisterPermissionsUpdateInput
{
    fn from(
        input: ExternalCanisterPermissionsUpdateInput,
    ) -> station_api::ExternalCanisterPermissionsUpdateInput {
        station_api::ExternalCanisterPermissionsUpdateInput {
            read: input.read.map(|read| read.into()),
            change: input.change.map(|change| change.into()),
            calls: input.calls.map(Into::into),
        }
    }
}

impl From<station_api::CanisterExecutionAndValidationMethodPairDTO>
    for CanisterExecutionAndValidationMethodPairInput
{
    fn from(
        input: station_api::CanisterExecutionAndValidationMethodPairDTO,
    ) -> CanisterExecutionAndValidationMethodPairInput {
        CanisterExecutionAndValidationMethodPairInput {
            execution_method: input.execution_method,
            validation_method: input.validation_method.into(),
        }
    }
}

impl From<CanisterExecutionAndValidationMethodPairInput>
    for station_api::CanisterExecutionAndValidationMethodPairDTO
{
    fn from(
        input: CanisterExecutionAndValidationMethodPairInput,
    ) -> station_api::CanisterExecutionAndValidationMethodPairDTO {
        station_api::CanisterExecutionAndValidationMethodPairDTO {
            execution_method: input.execution_method,
            validation_method: input.validation_method.into(),
        }
    }
}

impl From<station_api::ExternalCanisterCallRequestPoliciesMethodPairInput>
    for ExternalCanisterCallRequestPoliciesMethodPairInput
{
    fn from(
        input: station_api::ExternalCanisterCallRequestPoliciesMethodPairInput,
    ) -> ExternalCanisterCallRequestPoliciesMethodPairInput {
        ExternalCanisterCallRequestPoliciesMethodPairInput {
            method_configuration: input.method_configuration.into(),
            policies: input.policies.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ExternalCanisterCallRequestPoliciesMethodPairInput>
    for station_api::ExternalCanisterCallRequestPoliciesMethodPairInput
{
    fn from(
        input: ExternalCanisterCallRequestPoliciesMethodPairInput,
    ) -> station_api::ExternalCanisterCallRequestPoliciesMethodPairInput {
        station_api::ExternalCanisterCallRequestPoliciesMethodPairInput {
            method_configuration: input.method_configuration.into(),
            policies: input.policies.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<station_api::ExternalCanisterCallPermissionMethodPairInput>
    for ExternalCanisterCallPermissionMethodPairInput
{
    fn from(
        input: station_api::ExternalCanisterCallPermissionMethodPairInput,
    ) -> ExternalCanisterCallPermissionMethodPairInput {
        ExternalCanisterCallPermissionMethodPairInput {
            method_configuration: input.method_configuration.into(),
            allow: input.allow.map(Into::into),
        }
    }
}

impl From<ExternalCanisterCallPermissionMethodPairInput>
    for station_api::ExternalCanisterCallPermissionMethodPairInput
{
    fn from(
        input: ExternalCanisterCallPermissionMethodPairInput,
    ) -> station_api::ExternalCanisterCallPermissionMethodPairInput {
        station_api::ExternalCanisterCallPermissionMethodPairInput {
            method_configuration: input.method_configuration.into(),
            allow: input.allow.map(Into::into),
        }
    }
}

impl From<station_api::ExternalCanisterChangeCallPermissionsInput>
    for ExternalCanisterChangeCallPermissionsInput
{
    fn from(
        input: station_api::ExternalCanisterChangeCallPermissionsInput,
    ) -> ExternalCanisterChangeCallPermissionsInput {
        match input {
            station_api::ExternalCanisterChangeCallPermissionsInput::ReplaceAllBy(input) => {
                ExternalCanisterChangeCallPermissionsInput::ReplaceAllBy(
                    input.into_iter().map(Into::into).collect(),
                )
            }
            station_api::ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionMethods(
                input,
            ) => ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionMethods(
                input.into_iter().map(Into::into).collect(),
            ),
            station_api::ExternalCanisterChangeCallPermissionsInput::RemoveByExecutionMethods(methods) => {
                ExternalCanisterChangeCallPermissionsInput::RemoveByExecutionMethods(
                    methods.into_iter().map(Into::into).collect(),
                )
            }
            station_api::ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionValidationMethodPairs(
                input,
            ) => ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionValidationMethodPairs(
                input.into_iter().map(Into::into).collect(),
            ),
        }
    }
}

impl From<ExternalCanisterChangeCallPermissionsInput>
    for station_api::ExternalCanisterChangeCallPermissionsInput
{
    fn from(
        input: ExternalCanisterChangeCallPermissionsInput,
    ) -> station_api::ExternalCanisterChangeCallPermissionsInput {
        match input {
            ExternalCanisterChangeCallPermissionsInput::ReplaceAllBy(input) => {
                station_api::ExternalCanisterChangeCallPermissionsInput::ReplaceAllBy(
                    input.into_iter().map(Into::into).collect(),
                )
            }
            ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionMethods(input) => {
                station_api::ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionMethods(
                    input.into_iter().map(Into::into).collect(),
                )
            }
            ExternalCanisterChangeCallPermissionsInput::RemoveByExecutionMethods(methods) => {
                station_api::ExternalCanisterChangeCallPermissionsInput::RemoveByExecutionMethods(
                    methods.into_iter().map(Into::into).collect(),
                )
            }
            ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionValidationMethodPairs(input) => {
                station_api::ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionValidationMethodPairs(
                    input.into_iter().map(Into::into).collect(),
                )
            }
        }
    }
}

impl From<station_api::ExternalCanisterCallRequestPolicyRuleInput>
    for ExternalCanisterCallRequestPolicyRuleInput
{
    fn from(
        input: station_api::ExternalCanisterCallRequestPolicyRuleInput,
    ) -> ExternalCanisterCallRequestPolicyRuleInput {
        ExternalCanisterCallRequestPolicyRuleInput {
            policy_id: input.policy_id.map(|policy_id| {
                *HelperMapper::to_uuid(policy_id)
                    .expect("Invalid policy id format")
                    .as_bytes()
            }),
            rule: input.rule.into(),
            validation_method: input.validation_method.into(),
            execution_method: input.execution_method,
        }
    }
}

impl From<ExternalCanisterCallRequestPolicyRuleInput>
    for station_api::ExternalCanisterCallRequestPolicyRuleInput
{
    fn from(
        input: ExternalCanisterCallRequestPolicyRuleInput,
    ) -> station_api::ExternalCanisterCallRequestPolicyRuleInput {
        station_api::ExternalCanisterCallRequestPolicyRuleInput {
            policy_id: input
                .policy_id
                .map(|policy_id| Uuid::from_bytes(policy_id).hyphenated().to_string()),
            rule: input.rule.into(),
            validation_method: input.validation_method.into(),
            execution_method: input.execution_method,
        }
    }
}

impl From<station_api::ExternalCanisterChangeRequestPolicyRuleInput>
    for ExternalCanisterChangeRequestPolicyRuleInput
{
    fn from(
        input: station_api::ExternalCanisterChangeRequestPolicyRuleInput,
    ) -> ExternalCanisterChangeRequestPolicyRuleInput {
        ExternalCanisterChangeRequestPolicyRuleInput {
            policy_id: input.policy_id.map(|policy_id| {
                *HelperMapper::to_uuid(policy_id)
                    .expect("Invalid policy id format")
                    .as_bytes()
            }),
            rule: input.rule.into(),
        }
    }
}

impl From<ExternalCanisterChangeRequestPolicyRuleInput>
    for station_api::ExternalCanisterChangeRequestPolicyRuleInput
{
    fn from(
        input: ExternalCanisterChangeRequestPolicyRuleInput,
    ) -> station_api::ExternalCanisterChangeRequestPolicyRuleInput {
        station_api::ExternalCanisterChangeRequestPolicyRuleInput {
            policy_id: input
                .policy_id
                .map(|policy_id| Uuid::from_bytes(policy_id).hyphenated().to_string()),
            rule: input.rule.into(),
        }
    }
}

impl From<station_api::ExternalCanisterRequestPoliciesCreateInput>
    for ExternalCanisterRequestPoliciesCreateInput
{
    fn from(
        input: station_api::ExternalCanisterRequestPoliciesCreateInput,
    ) -> ExternalCanisterRequestPoliciesCreateInput {
        ExternalCanisterRequestPoliciesCreateInput {
            change: input.change.into_iter().map(Into::into).collect(),
            calls: input.calls.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ExternalCanisterRequestPoliciesCreateInput>
    for station_api::ExternalCanisterRequestPoliciesCreateInput
{
    fn from(
        input: ExternalCanisterRequestPoliciesCreateInput,
    ) -> station_api::ExternalCanisterRequestPoliciesCreateInput {
        station_api::ExternalCanisterRequestPoliciesCreateInput {
            change: input.change.into_iter().map(Into::into).collect(),
            calls: input.calls.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<station_api::CreateExternalCanisterOperationKindCreateNewDTO>
    for CreateExternalCanisterOperationKindCreateNew
{
    fn from(
        input: station_api::CreateExternalCanisterOperationKindCreateNewDTO,
    ) -> CreateExternalCanisterOperationKindCreateNew {
        CreateExternalCanisterOperationKindCreateNew {
            initial_cycles: input.initial_cycles,
        }
    }
}

impl From<CreateExternalCanisterOperationKindCreateNew>
    for station_api::CreateExternalCanisterOperationKindCreateNewDTO
{
    fn from(
        input: CreateExternalCanisterOperationKindCreateNew,
    ) -> station_api::CreateExternalCanisterOperationKindCreateNewDTO {
        station_api::CreateExternalCanisterOperationKindCreateNewDTO {
            initial_cycles: input.initial_cycles,
        }
    }
}

impl From<station_api::CreateExternalCanisterOperationKindAddExistingDTO>
    for CreateExternalCanisterOperationKindAddExisting
{
    fn from(
        input: station_api::CreateExternalCanisterOperationKindAddExistingDTO,
    ) -> CreateExternalCanisterOperationKindAddExisting {
        CreateExternalCanisterOperationKindAddExisting {
            canister_id: input.canister_id,
        }
    }
}

impl From<CreateExternalCanisterOperationKindAddExisting>
    for station_api::CreateExternalCanisterOperationKindAddExistingDTO
{
    fn from(
        input: CreateExternalCanisterOperationKindAddExisting,
    ) -> station_api::CreateExternalCanisterOperationKindAddExistingDTO {
        station_api::CreateExternalCanisterOperationKindAddExistingDTO {
            canister_id: input.canister_id,
        }
    }
}

impl From<station_api::CreateExternalCanisterOperationKindDTO>
    for CreateExternalCanisterOperationKind
{
    fn from(
        input: station_api::CreateExternalCanisterOperationKindDTO,
    ) -> CreateExternalCanisterOperationKind {
        match input {
            station_api::CreateExternalCanisterOperationKindDTO::CreateNew(kind) => {
                CreateExternalCanisterOperationKind::CreateNew(kind.into())
            }
            station_api::CreateExternalCanisterOperationKindDTO::AddExisting(kind) => {
                CreateExternalCanisterOperationKind::AddExisting(kind.into())
            }
        }
    }
}

impl From<CreateExternalCanisterOperationKind>
    for station_api::CreateExternalCanisterOperationKindDTO
{
    fn from(
        input: CreateExternalCanisterOperationKind,
    ) -> station_api::CreateExternalCanisterOperationKindDTO {
        match input {
            CreateExternalCanisterOperationKind::CreateNew(kind) => {
                station_api::CreateExternalCanisterOperationKindDTO::CreateNew(kind.into())
            }
            CreateExternalCanisterOperationKind::AddExisting(kind) => {
                station_api::CreateExternalCanisterOperationKindDTO::AddExisting(kind.into())
            }
        }
    }
}

impl From<CreateExternalCanisterOperationInput>
    for station_api::CreateExternalCanisterOperationInput
{
    fn from(
        input: CreateExternalCanisterOperationInput,
    ) -> station_api::CreateExternalCanisterOperationInput {
        station_api::CreateExternalCanisterOperationInput {
            kind: input.kind.into(),
            name: input.name,
            description: input.description,
            labels: input.labels,
            permissions: input.permissions.into(),
            request_policies: input.request_policies.into(),
        }
    }
}

impl From<station_api::CreateExternalCanisterOperationInput>
    for CreateExternalCanisterOperationInput
{
    fn from(
        input: station_api::CreateExternalCanisterOperationInput,
    ) -> CreateExternalCanisterOperationInput {
        CreateExternalCanisterOperationInput {
            kind: input.kind.into(),
            name: input.name,
            description: input.description,
            labels: input.labels,
            permissions: input.permissions.into(),
            request_policies: input.request_policies.into(),
        }
    }
}

impl From<CreateExternalCanisterOperation> for CreateExternalCanisterOperationDTO {
    fn from(operation: CreateExternalCanisterOperation) -> CreateExternalCanisterOperationDTO {
        CreateExternalCanisterOperationDTO {
            canister_id: operation.canister_id,
            input: operation.input.into(),
        }
    }
}

impl From<CanisterMethod> for CanisterMethodDTO {
    fn from(canister_method: CanisterMethod) -> CanisterMethodDTO {
        CanisterMethodDTO {
            canister_id: canister_method.canister_id,
            method_name: canister_method.method_name,
        }
    }
}

impl From<CanisterMethodDTO> for CanisterMethod {
    fn from(canister_method: CanisterMethodDTO) -> CanisterMethod {
        CanisterMethod {
            canister_id: canister_method.canister_id,
            method_name: canister_method.method_name,
        }
    }
}

impl From<CallExternalCanisterOperationInput> for station_api::CallExternalCanisterOperationInput {
    fn from(
        input: CallExternalCanisterOperationInput,
    ) -> station_api::CallExternalCanisterOperationInput {
        station_api::CallExternalCanisterOperationInput {
            validation_method: input.validation_method.map(|m| m.into()),
            execution_method: input.execution_method.into(),
            arg: input.arg,
            execution_method_cycles: input.execution_method_cycles,
        }
    }
}

impl From<station_api::CallExternalCanisterOperationInput> for CallExternalCanisterOperationInput {
    fn from(
        input: station_api::CallExternalCanisterOperationInput,
    ) -> CallExternalCanisterOperationInput {
        CallExternalCanisterOperationInput {
            validation_method: input.validation_method.map(|m| m.into()),
            execution_method: input.execution_method.into(),
            arg: input.arg,
            execution_method_cycles: input.execution_method_cycles,
        }
    }
}

impl From<CallExternalCanisterOperation> for CallExternalCanisterOperationDTO {
    fn from(operation: CallExternalCanisterOperation) -> CallExternalCanisterOperationDTO {
        CallExternalCanisterOperationDTO {
            validation_method: operation.input.validation_method.map(|m| m.into()),
            execution_method: operation.input.execution_method.into(),
            arg_checksum: operation.arg_checksum.map(hex::encode),
            arg_rendering: operation.arg_rendering,
            execution_method_cycles: operation.input.execution_method_cycles,
            execution_method_reply: operation.execution_method_reply,
            // By default this field is not set to avoid having responses that could be too large
            arg: None,
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

impl From<station_api::CycleObtainStrategyDTO> for CycleObtainStrategy {
    fn from(value: station_api::CycleObtainStrategyDTO) -> Self {
        match value {
            station_api::CycleObtainStrategyDTO::Disabled => CycleObtainStrategy::Disabled,
            station_api::CycleObtainStrategyDTO::MintFromNativeToken { account_id, .. } => {
                CycleObtainStrategy::MintFromNativeToken {
                    account_id: *HelperMapper::to_uuid(account_id)
                        .expect("Invalid account id")
                        .as_bytes(),
                }
            }
        }
    }
}

impl From<CycleObtainStrategy> for station_api::CycleObtainStrategyDTO {
    fn from(value: CycleObtainStrategy) -> Self {
        match value {
            CycleObtainStrategy::Disabled => station_api::CycleObtainStrategyDTO::Disabled,
            CycleObtainStrategy::MintFromNativeToken { account_id } => {
                station_api::CycleObtainStrategyDTO::MintFromNativeToken {
                    account_name: ACCOUNT_REPOSITORY
                        .get(&AccountKey { id: account_id })
                        .map(|a| a.name),
                    account_id: Uuid::from_bytes(account_id).hyphenated().to_string(),
                }
            }
        }
    }
}

impl From<station_api::CycleObtainStrategyInput> for CycleObtainStrategy {
    fn from(value: station_api::CycleObtainStrategyInput) -> Self {
        match value {
            station_api::CycleObtainStrategyInput::Disabled => CycleObtainStrategy::Disabled,
            station_api::CycleObtainStrategyInput::MintFromNativeToken { account_id, .. } => {
                CycleObtainStrategy::MintFromNativeToken {
                    account_id: *HelperMapper::to_uuid(account_id)
                        .expect("Invalid account id")
                        .as_bytes(),
                }
            }
        }
    }
}

impl From<CycleObtainStrategy> for station_api::CycleObtainStrategyInput {
    fn from(value: CycleObtainStrategy) -> Self {
        match value {
            CycleObtainStrategy::Disabled => station_api::CycleObtainStrategyInput::Disabled,
            CycleObtainStrategy::MintFromNativeToken { account_id } => {
                station_api::CycleObtainStrategyInput::MintFromNativeToken {
                    account_id: Uuid::from_bytes(account_id).hyphenated().to_string(),
                }
            }
        }
    }
}

impl From<ManageSystemInfoOperationInput> for station_api::ManageSystemInfoOperationInput {
    fn from(input: ManageSystemInfoOperationInput) -> station_api::ManageSystemInfoOperationInput {
        station_api::ManageSystemInfoOperationInput {
            name: input.name,
            cycle_obtain_strategy: input.cycle_obtain_strategy.map(|strategy| strategy.into()),
        }
    }
}

impl From<station_api::ManageSystemInfoOperationInput> for ManageSystemInfoOperationInput {
    fn from(input: station_api::ManageSystemInfoOperationInput) -> ManageSystemInfoOperationInput {
        ManageSystemInfoOperationInput {
            name: input.name,
            cycle_obtain_strategy: input.cycle_obtain_strategy.map(|strategy| strategy.into()),
        }
    }
}

impl From<ManageSystemInfoOperation> for station_api::ManageSystemInfoOperationDTO {
    fn from(operation: ManageSystemInfoOperation) -> station_api::ManageSystemInfoOperationDTO {
        station_api::ManageSystemInfoOperationDTO {
            input: operation.input.into(),
        }
    }
}

impl From<station_api::ManageSystemInfoOperationDTO> for ManageSystemInfoOperation {
    fn from(operation: station_api::ManageSystemInfoOperationDTO) -> ManageSystemInfoOperation {
        ManageSystemInfoOperation {
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
            RequestOperation::SystemUpgrade(operation) => {
                RequestOperationDTO::SystemUpgrade(Box::new(operation.into()))
            }
            RequestOperation::SetDisasterRecovery(operation) => {
                RequestOperationDTO::SetDisasterRecovery(Box::new(operation.into()))
            }
            RequestOperation::ChangeExternalCanister(operation) => {
                RequestOperationDTO::ChangeExternalCanister(Box::new(operation.into()))
            }
            RequestOperation::FundExternalCanister(operation) => {
                RequestOperationDTO::FundExternalCanister(Box::new(operation.into()))
            }
            RequestOperation::ConfigureExternalCanister(operation) => {
                RequestOperationDTO::ConfigureExternalCanister(Box::new(operation.into()))
            }
            RequestOperation::CreateExternalCanister(operation) => {
                RequestOperationDTO::CreateExternalCanister(Box::new(operation.into()))
            }
            RequestOperation::CallExternalCanister(operation) => {
                RequestOperationDTO::CallExternalCanister(Box::new(operation.into()))
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
            RequestOperation::ManageSystemInfo(operation) => {
                RequestOperationDTO::ManageSystemInfo(Box::new(operation.into()))
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
            RequestOperation::SetDisasterRecovery(_) | RequestOperation::SystemUpgrade(_) => {
                vec![Resource::System(SystemResourceAction::Upgrade)]
            }
            RequestOperation::ChangeExternalCanister(ChangeExternalCanisterOperation {
                input,
                ..
            }) => {
                vec![
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                        ExternalCanisterId::Any,
                    )),
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                        ExternalCanisterId::Canister(input.canister_id),
                    )),
                ]
            }
            RequestOperation::ConfigureExternalCanister(ConfigureExternalCanisterOperation {
                canister_id,
                ..
            }) => {
                vec![
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                        ExternalCanisterId::Any,
                    )),
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                        ExternalCanisterId::Canister(*canister_id),
                    )),
                ]
            }
            RequestOperation::FundExternalCanister(FundExternalCanisterOperation {
                canister_id,
                ..
            }) => {
                vec![
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Fund(
                        ExternalCanisterId::Any,
                    )),
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Fund(
                        ExternalCanisterId::Canister(*canister_id),
                    )),
                ]
            }
            RequestOperation::CreateExternalCanister(CreateExternalCanisterOperation {
                ..
            }) => {
                vec![Resource::ExternalCanister(
                    ExternalCanisterResourceAction::Create,
                )]
            }
            RequestOperation::CallExternalCanister(CallExternalCanisterOperation {
                input, ..
            }) => {
                vec![
                    // Any canister with any method
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                        CallExternalCanisterResourceTarget {
                            validation_method: input.validation_method.clone().into(),
                            execution_method: ExecutionMethodResourceTarget::Any,
                        },
                    )),
                    // A specific canister with any execution method
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                        CallExternalCanisterResourceTarget {
                            validation_method: input.validation_method.clone().into(),
                            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                CanisterMethod {
                                    canister_id: input.execution_method.canister_id,
                                    method_name: CanisterMethod::WILDCARD.to_string(),
                                },
                            ),
                        },
                    )),
                    // A specific canister with a specific execution method
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                        CallExternalCanisterResourceTarget {
                            validation_method: input.validation_method.clone().into(),
                            execution_method: input.execution_method.clone().into(),
                        },
                    )),
                ]
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
            RequestOperation::ManageSystemInfo(_) => {
                vec![Resource::System(SystemResourceAction::ManageSystemInfo)]
            }
        }
    }
}
