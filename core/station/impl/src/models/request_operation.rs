use super::{
    permission::{Allow, AuthScope},
    request_policy_rule::{RequestPolicyRule, RequestPolicyRuleInput},
    request_specifier::RequestSpecifier,
    resource::{Resource, ValidationMethodResourceTarget},
    AccountId, AddressBookEntryId, Blockchain, BlockchainStandard, ChangeMetadata,
    CycleObtainStrategy, DisasterRecoveryCommittee, ExternalCanisterCallPermission,
    ExternalCanisterState, MetadataItem, UserGroupId, UserId, UserStatus,
};
use crate::core::validation::EnsureExternalCanister;
use crate::errors::ValidationError;
use crate::models::Metadata;
use candid::Principal;
use orbit_essentials::cdk::api::management_canister::main::{self as mgmt};
use orbit_essentials::cmc::SubnetSelection;
use orbit_essentials::model::{ModelValidator, ModelValidatorResult};
use orbit_essentials::{storable, types::UUID};
use std::fmt::Display;

#[storable(skip_deserialize = true)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::VariantNames)]
#[strum(serialize_all = "PascalCase")]
pub enum RequestOperation {
    Transfer(TransferOperation),
    AddAccount(AddAccountOperation),
    EditAccount(EditAccountOperation),
    AddAddressBookEntry(AddAddressBookEntryOperation),
    EditAddressBookEntry(EditAddressBookEntryOperation),
    RemoveAddressBookEntry(RemoveAddressBookEntryOperation),
    AddUser(AddUserOperation),
    EditUser(EditUserOperation),
    EditPermission(EditPermissionOperation),
    AddUserGroup(AddUserGroupOperation),
    EditUserGroup(EditUserGroupOperation),
    RemoveUserGroup(RemoveUserGroupOperation),
    SystemUpgrade(SystemUpgradeOperation),
    ChangeExternalCanister(ChangeExternalCanisterOperation),
    ConfigureExternalCanister(ConfigureExternalCanisterOperation),
    CreateExternalCanister(CreateExternalCanisterOperation),
    CallExternalCanister(CallExternalCanisterOperation),
    FundExternalCanister(FundExternalCanisterOperation),
    SnapshotExternalCanister(SnapshotExternalCanisterOperation),
    RestoreExternalCanister(RestoreExternalCanisterOperation),
    PruneExternalCanister(PruneExternalCanisterOperation),
    AddRequestPolicy(AddRequestPolicyOperation),
    EditRequestPolicy(EditRequestPolicyOperation),
    RemoveRequestPolicy(RemoveRequestPolicyOperation),
    ManageSystemInfo(ManageSystemInfoOperation),
    SetDisasterRecovery(SetDisasterRecoveryOperation),
}

impl Display for RequestOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestOperation::Transfer(_) => write!(f, "transfer"),
            RequestOperation::AddAccount(_) => write!(f, "add_account"),
            RequestOperation::EditAccount(_) => write!(f, "edit_account"),
            RequestOperation::AddAddressBookEntry(_) => write!(f, "add_address_book_entry"),
            RequestOperation::EditAddressBookEntry(_) => write!(f, "edit_address_book_entry"),
            RequestOperation::RemoveAddressBookEntry(_) => write!(f, "remove_address_book_entry"),
            RequestOperation::AddUser(_) => write!(f, "add_user"),
            RequestOperation::EditUser(_) => write!(f, "edit_user"),
            RequestOperation::EditPermission(_) => write!(f, "edit_permission"),
            RequestOperation::AddUserGroup(_) => write!(f, "add_user_group"),
            RequestOperation::EditUserGroup(_) => write!(f, "adit_user_group"),
            RequestOperation::RemoveUserGroup(_) => write!(f, "remove_user_group"),
            RequestOperation::SystemUpgrade(_) => write!(f, "system_upgrade"),
            RequestOperation::ChangeExternalCanister(_) => write!(f, "change_external_canister"),
            RequestOperation::ConfigureExternalCanister(_) => {
                write!(f, "configure_external_canister")
            }
            RequestOperation::CreateExternalCanister(_) => write!(f, "create_external_canister"),
            RequestOperation::CallExternalCanister(_) => write!(f, "call_external_canister"),
            RequestOperation::FundExternalCanister(_) => write!(f, "fund_external_canister"),
            RequestOperation::SnapshotExternalCanister(_) => {
                write!(f, "snapshot_external_canister")
            }
            RequestOperation::RestoreExternalCanister(_) => {
                write!(f, "restore_external_canister")
            }
            RequestOperation::PruneExternalCanister(_) => {
                write!(f, "prune_external_canister")
            }
            RequestOperation::AddRequestPolicy(_) => write!(f, "add_request_policy"),
            RequestOperation::EditRequestPolicy(_) => write!(f, "edit_request_policy"),
            RequestOperation::RemoveRequestPolicy(_) => write!(f, "remove_request_policy"),
            RequestOperation::ManageSystemInfo(_) => write!(f, "manage_system_info"),
            RequestOperation::SetDisasterRecovery(_) => write!(f, "set_disaster_recovery"),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperation {
    pub transfer_id: Option<UUID>,
    pub input: TransferOperationInput,
    pub fee: Option<candid::Nat>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperationInput {
    pub from_account_id: AccountId,
    pub to: String,
    pub amount: candid::Nat,
    pub metadata: Metadata,
    pub network: String,
    pub fee: Option<candid::Nat>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperation {
    /// The account id is only available after the operation is executed.
    pub account_id: Option<AccountId>,
    pub input: AddAccountOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperationInput {
    pub name: String,
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub metadata: Metadata,
    pub read_permission: Allow,
    pub configs_permission: Allow,
    pub transfer_permission: Allow,
    pub configs_request_policy: Option<RequestPolicyRule>,
    pub transfer_request_policy: Option<RequestPolicyRule>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccountOperation {
    pub input: EditAccountOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccountOperationInput {
    pub account_id: AccountId,
    pub name: Option<String>,
    pub read_permission: Option<Allow>,
    pub configs_permission: Option<Allow>,
    pub transfer_permission: Option<Allow>,
    pub configs_request_policy: Option<RequestPolicyRuleInput>,
    pub transfer_request_policy: Option<RequestPolicyRuleInput>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAddressBookEntryOperation {
    /// The address book entry id is only available after the operation is executed.
    pub address_book_entry_id: Option<AddressBookEntryId>,
    pub input: AddAddressBookEntryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAddressBookEntryOperationInput {
    pub address_owner: String,
    pub address: String,
    pub blockchain: Blockchain,
    #[serde(default)]
    pub labels: Vec<String>,
    pub metadata: Vec<MetadataItem>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAddressBookEntryOperation {
    pub input: EditAddressBookEntryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAddressBookEntryOperationInput {
    pub address_book_entry_id: AddressBookEntryId,
    pub address_owner: Option<String>,
    pub change_metadata: Option<ChangeMetadata>,
    #[serde(default)]
    pub labels: Option<Vec<String>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAddressBookEntryOperation {
    pub input: RemoveAddressBookEntryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAddressBookEntryOperationInput {
    pub address_book_entry_id: AddressBookEntryId,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserOperation {
    pub user_id: Option<UUID>,
    pub input: AddUserOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserOperationInput {
    pub name: String,
    pub identities: Vec<Principal>,
    pub groups: Vec<UUID>,
    pub status: UserStatus,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperation {
    pub input: EditUserOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperationInput {
    pub user_id: UUID,
    pub name: Option<String>,
    pub identities: Option<Vec<Principal>>,
    pub groups: Option<Vec<UUID>>,
    pub status: Option<UserStatus>,
    pub cancel_pending_requests: Option<bool>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserGroupOperation {
    pub user_group_id: Option<UUID>,
    pub input: AddUserGroupOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserGroupOperationInput {
    pub name: String,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserGroupOperation {
    pub input: EditUserGroupOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserGroupOperationInput {
    pub user_group_id: UUID,
    pub name: String,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveUserGroupOperation {
    pub input: RemoveUserGroupOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveUserGroupOperationInput {
    pub user_group_id: UUID,
}

#[storable]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SystemUpgradeTarget {
    UpgradeStation,
    UpgradeUpgrader,
}

#[storable]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WasmModuleExtraChunks {
    pub store_canister: Principal,
    pub extra_chunks_key: String,
    pub wasm_module_hash: Vec<u8>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SystemUpgradeOperationInput {
    pub target: SystemUpgradeTarget,
    /// The module is only available while the operation is not finalized.
    pub module: Vec<u8>,
    pub module_extra_chunks: Option<WasmModuleExtraChunks>,
    pub arg: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SystemUpgradeOperation {
    pub module_checksum: Vec<u8>,
    pub arg_checksum: Option<Vec<u8>>,
    pub input: SystemUpgradeOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SetDisasterRecoveryOperation {
    pub input: SetDisasterRecoveryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanisterInstallModeArgs {}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanisterReinstallModeArgs {}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanisterUpgradeModeArgs {}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CanisterInstallMode {
    Install(CanisterInstallModeArgs),
    Reinstall(CanisterReinstallModeArgs),
    Upgrade(CanisterUpgradeModeArgs),
}

impl From<CanisterInstallMode> for mgmt::CanisterInstallMode {
    fn from(mode: CanisterInstallMode) -> Self {
        match mode {
            CanisterInstallMode::Install(_) => mgmt::CanisterInstallMode::Install,
            CanisterInstallMode::Reinstall(_) => mgmt::CanisterInstallMode::Reinstall,
            CanisterInstallMode::Upgrade(_) => mgmt::CanisterInstallMode::Upgrade(None),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SetDisasterRecoveryOperationInput {
    pub committee: Option<DisasterRecoveryCommittee>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub mode: CanisterInstallMode,
    pub module: Vec<u8>,
    pub module_extra_chunks: Option<WasmModuleExtraChunks>,
    pub arg: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeExternalCanisterOperation {
    pub module_checksum: Vec<u8>,
    pub arg_checksum: Option<Vec<u8>>,
    pub input: ChangeExternalCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterPermissionsCreateInput {
    pub read: Allow,
    pub change: Allow,
    pub calls: Vec<ExternalCanisterCallPermission>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterPermissionsUpdateInput {
    pub read: Option<Allow>,
    pub change: Option<Allow>,
    pub calls: Option<ExternalCanisterChangeCallPermissionsInput>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanisterExecutionAndValidationMethodPairInput {
    pub validation_method: ValidationMethodResourceTarget,
    pub execution_method: String,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallPermissionMethodPairInput {
    pub method_configuration: CanisterExecutionAndValidationMethodPairInput,
    pub allow: Option<Allow>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallPermissionExecMethodEntryInput {
    pub allow: Allow,
    pub validation_method: ValidationMethodResourceTarget,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallPermissionsExecMethodInput {
    pub execution_method: String,
    pub permissions: Vec<ExternalCanisterCallPermissionExecMethodEntryInput>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalCanisterChangeCallPermissionsInput {
    ReplaceAllBy(Vec<ExternalCanisterCallPermission>),
    OverrideSpecifiedByExecutionMethods(Vec<ExternalCanisterCallPermissionsExecMethodInput>),
    OverrideSpecifiedByExecutionValidationMethodPairs(
        Vec<ExternalCanisterCallPermissionMethodPairInput>,
    ),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallRequestPolicyRuleInput {
    pub policy_id: Option<UUID>,
    pub rule: RequestPolicyRule,
    pub validation_method: ValidationMethodResourceTarget,
    pub execution_method: String,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterChangeRequestPolicyRuleInput {
    pub policy_id: Option<UUID>,
    pub rule: RequestPolicyRule,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterRequestPoliciesCreateInput {
    pub change: Vec<ExternalCanisterChangeRequestPolicyRuleInput>,
    pub calls: Vec<ExternalCanisterCallRequestPolicyRuleInput>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterRequestPoliciesUpdateInput {
    pub change: Option<Vec<ExternalCanisterChangeRequestPolicyRuleInput>>,
    pub calls: Option<ExternalCanisterChangeCallRequestPoliciesInput>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallRequestPoliciesMethodPairInput {
    pub method_configuration: CanisterExecutionAndValidationMethodPairInput,
    pub policies: Vec<ExternalCanisterChangeRequestPolicyRuleInput>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallRequestPolicyRuleValidationInput {
    pub policy_id: Option<UUID>,
    pub rule: RequestPolicyRule,
    pub validation_method: ValidationMethodResourceTarget,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallRequestPoliciesExecMethodInput {
    pub execution_method: String,
    pub policies: Vec<ExternalCanisterCallRequestPolicyRuleValidationInput>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalCanisterChangeCallRequestPoliciesInput {
    ReplaceAllBy(Vec<ExternalCanisterCallRequestPolicyRuleInput>),
    RemoveByPolicyIds(Vec<UUID>),
    OverrideSpecifiedByExecutionMethods(Vec<ExternalCanisterCallRequestPoliciesExecMethodInput>),
    OverrideSpecifiedByExecutionValidationMethodPairs(
        Vec<ExternalCanisterCallRequestPoliciesMethodPairInput>,
    ),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CreateExternalCanisterOperationKindCreateNew {
    pub initial_cycles: Option<u64>,
    pub subnet_selection: Option<SubnetSelection>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CreateExternalCanisterOperationKindAddExisting {
    pub canister_id: Principal,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CreateExternalCanisterOperationKind {
    CreateNew(CreateExternalCanisterOperationKindCreateNew),
    AddExisting(CreateExternalCanisterOperationKindAddExisting),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CreateExternalCanisterOperationInput {
    pub kind: CreateExternalCanisterOperationKind,
    pub name: String,
    pub description: Option<String>,
    pub labels: Option<Vec<String>>,
    pub metadata: Option<Metadata>,
    pub permissions: ExternalCanisterPermissionsCreateInput,
    pub request_policies: ExternalCanisterRequestPoliciesCreateInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CreateExternalCanisterOperation {
    pub canister_id: Option<Principal>,
    pub input: CreateExternalCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FundExternalCanisterSendCyclesInput {
    pub cycles: u64,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FundExternalCanisterOperationKind {
    Send(FundExternalCanisterSendCyclesInput),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FundExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub kind: FundExternalCanisterOperationKind,
}

pub type FundExternalCanisterOperation = FundExternalCanisterOperationInput;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConfigureExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub kind: ConfigureExternalCanisterOperationKind,
}

pub type ConfigureExternalCanisterOperation = ConfigureExternalCanisterOperationInput;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConfigureExternalCanisterOperationKind {
    Settings(ConfigureExternalCanisterSettingsInput),
    SoftDelete,
    Delete,
    NativeSettings(DefiniteCanisterSettingsInput),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogVisibility {
    Public,
    Controllers,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DefiniteCanisterSettingsInput {
    pub controllers: Option<Vec<Principal>>,
    pub compute_allocation: Option<candid::Nat>,
    pub memory_allocation: Option<candid::Nat>,
    pub freezing_threshold: Option<candid::Nat>,
    pub reserved_cycles_limit: Option<candid::Nat>,
    pub log_visibility: Option<LogVisibility>,
    pub wasm_memory_limit: Option<candid::Nat>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ConfigureExternalCanisterSettingsInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub labels: Option<Vec<String>>,
    pub change_metadata: Option<ChangeMetadata>,
    pub state: Option<ExternalCanisterState>,
    pub permissions: Option<ExternalCanisterPermissionsUpdateInput>,
    pub request_policies: Option<ExternalCanisterRequestPoliciesUpdateInput>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanisterMethod {
    pub canister_id: Principal,
    pub method_name: String,
}

impl CanisterMethod {
    pub const WILDCARD: &'static str = "*";
}

impl ModelValidator<ValidationError> for CanisterMethod {
    fn validate(&self) -> ModelValidatorResult<ValidationError> {
        EnsureExternalCanister::is_external_canister(self.canister_id)?;

        Ok(())
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallExternalCanisterOperationInput {
    pub validation_method: Option<CanisterMethod>,
    pub execution_method: CanisterMethod,
    pub arg: Option<Vec<u8>>,
    pub execution_method_cycles: Option<u64>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallExternalCanisterOperation {
    pub input: CallExternalCanisterOperationInput,
    pub arg_checksum: Option<Vec<u8>>,
    pub arg_rendering: Option<String>,
    pub execution_method_reply: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SnapshotExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub replace_snapshot: Option<Vec<u8>>,
    pub force: bool,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SnapshotExternalCanisterOperation {
    pub snapshot_id: Option<Vec<u8>>,
    pub input: SnapshotExternalCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RestoreExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub snapshot_id: Vec<u8>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RestoreExternalCanisterOperation {
    pub input: RestoreExternalCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PruneExternalCanisterResource {
    Snapshot(Vec<u8>),
    ChunkStore,
    State,
}

impl Display for PruneExternalCanisterResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PruneExternalCanisterResource::Snapshot(snapshot_id) => {
                write!(f, "snapshot({})", hex::encode(snapshot_id))
            }
            PruneExternalCanisterResource::ChunkStore => write!(f, "chunk_store"),
            PruneExternalCanisterResource::State => write!(f, "state"),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PruneExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub prune: PruneExternalCanisterResource,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PruneExternalCanisterOperation {
    pub input: PruneExternalCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditPermissionOperationInput {
    pub resource: Resource,
    pub auth_scope: Option<AuthScope>,
    pub users: Option<Vec<UserId>>,
    pub user_groups: Option<Vec<UserGroupId>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditPermissionOperation {
    pub input: EditPermissionOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddRequestPolicyOperationInput {
    pub specifier: RequestSpecifier,
    pub rule: RequestPolicyRule,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddRequestPolicyOperation {
    pub policy_id: Option<UUID>,
    pub input: AddRequestPolicyOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditRequestPolicyOperationInput {
    pub policy_id: UUID,
    pub specifier: Option<RequestSpecifier>,
    pub rule: Option<RequestPolicyRule>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditRequestPolicyOperation {
    pub input: EditRequestPolicyOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveRequestPolicyOperationInput {
    pub policy_id: UUID,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveRequestPolicyOperation {
    pub input: RemoveRequestPolicyOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManageSystemInfoOperationInput {
    pub name: Option<String>,
    pub cycle_obtain_strategy: Option<CycleObtainStrategy>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManageSystemInfoOperation {
    pub input: ManageSystemInfoOperationInput,
}
