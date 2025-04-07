use super::{
    permission::{Allow, AuthScope},
    request_policy_rule::{RequestPolicyRule, RequestPolicyRuleInput},
    request_specifier::RequestSpecifier,
    resource::{Resource, ValidationMethodResourceTarget},
    AccountAsset, AccountId, AddressBookEntryId, AddressFormat, Asset, AssetId, Blockchain,
    ChangeMetadata, CycleObtainStrategy, DisasterRecoveryCommittee, ExternalCanisterCallPermission,
    ExternalCanisterState, MetadataItem, NamedRuleId, TokenStandard, UserGroupId, UserId,
    UserStatus,
};
use crate::core::validation::{
    EnsureAccount, EnsureAddressBookEntry, EnsureAsset, EnsureExternalCanister, EnsureIdExists,
    EnsureNamedRule, EnsureRequestPolicy, EnsureUser, EnsureUserGroup,
};
use crate::errors::{ExternalCanisterValidationError, ValidationError};
use crate::models::resource::ExecutionMethodResourceTarget;
use crate::models::Metadata;
use candid::Principal;
use orbit_essentials::cdk::api::management_canister::main::{self as mgmt};
use orbit_essentials::cmc::SubnetSelection;
use orbit_essentials::model::{ContextualModel, ModelValidator, ModelValidatorResult};
use orbit_essentials::{storable, types::UUID};
use std::{collections::HashSet, fmt::Display};

#[storable]
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
    MonitorExternalCanister(MonitorExternalCanisterOperation),
    SnapshotExternalCanister(SnapshotExternalCanisterOperation),
    RestoreExternalCanister(RestoreExternalCanisterOperation),
    PruneExternalCanister(PruneExternalCanisterOperation),
    AddRequestPolicy(AddRequestPolicyOperation),
    EditRequestPolicy(EditRequestPolicyOperation),
    RemoveRequestPolicy(RemoveRequestPolicyOperation),
    ManageSystemInfo(ManageSystemInfoOperation),
    SetDisasterRecovery(SetDisasterRecoveryOperation),
    AddAsset(AddAssetOperation),
    EditAsset(EditAssetOperation),
    RemoveAsset(RemoveAssetOperation),
    AddNamedRule(AddNamedRuleOperation),
    EditNamedRule(EditNamedRuleOperation),
    RemoveNamedRule(RemoveNamedRuleOperation),
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
            RequestOperation::MonitorExternalCanister(_) => write!(f, "monitor_external_canister"),
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
            RequestOperation::AddAsset(_) => write!(f, "add_asset"),
            RequestOperation::EditAsset(_) => write!(f, "edit_asset"),
            RequestOperation::RemoveAsset(_) => write!(f, "remove_asset"),
            RequestOperation::AddNamedRule(_) => write!(f, "add_named_rule"),
            RequestOperation::EditNamedRule(_) => write!(f, "edit_named_rule"),
            RequestOperation::RemoveNamedRule(_) => write!(f, "remove_named_rule"),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAssetOperation {
    pub asset_id: Option<AssetId>,
    pub input: AddAssetOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAssetOperationInput {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub metadata: Metadata,
    pub blockchain: Blockchain,
    pub standards: Vec<TokenStandard>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAssetOperation {
    pub input: EditAssetOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAssetOperationInput {
    pub asset_id: AssetId,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub change_metadata: Option<ChangeMetadata>,
    pub blockchain: Option<Blockchain>,
    pub standards: Option<Vec<TokenStandard>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAssetOperation {
    pub input: RemoveAssetOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAssetOperationInput {
    pub asset_id: AssetId,
}

#[storable(skip_deserialize = true)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperation {
    pub transfer_id: Option<UUID>,
    pub input: TransferOperationInput,
    pub asset: Asset,
    pub fee: Option<candid::Nat>,
}

#[storable(skip_deserialize = true)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperationInput {
    pub from_account_id: AccountId,
    pub from_asset_id: AssetId,
    pub with_standard: TokenStandard,
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

#[storable(skip_deserialize = true)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperationInput {
    pub name: String,
    pub assets: Vec<AssetId>,
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
pub enum ChangeAssets {
    ReplaceWith {
        assets: Vec<AssetId>,
    },
    Change {
        add_assets: Vec<AssetId>,
        remove_assets: Vec<AssetId>,
    },
}

impl ChangeAssets {
    pub fn apply(&self, assets: &mut Vec<AccountAsset>) {
        match self {
            ChangeAssets::ReplaceWith { assets: new_assets } => {
                *assets = new_assets
                    .iter()
                    .map(|asset_id| AccountAsset {
                        asset_id: *asset_id,
                        balance: None,
                    })
                    .collect();
            }
            ChangeAssets::Change {
                add_assets,
                remove_assets,
            } => {
                let existing_assets: HashSet<_> = assets.iter().map(|a| a.asset_id).collect();
                for asset_id in add_assets {
                    if !existing_assets.contains(asset_id) {
                        assets.push(AccountAsset {
                            asset_id: *asset_id,
                            balance: None,
                        });
                    }
                }

                assets.retain(|a| !remove_assets.contains(&a.asset_id));
            }
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccountOperationInput {
    pub account_id: AccountId,
    pub change_assets: Option<ChangeAssets>,
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

#[storable(skip_deserialize = true)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAddressBookEntryOperationInput {
    pub address_owner: String,
    pub address: String,
    pub address_format: AddressFormat,
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
    #[serde(with = "serde_bytes")]
    pub wasm_module_hash: Vec<u8>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SystemUpgradeOperationInput {
    pub target: SystemUpgradeTarget,
    /// The module is only available while the operation is not finalized.
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
    pub module_extra_chunks: Option<WasmModuleExtraChunks>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub arg: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SystemUpgradeOperation {
    #[serde(with = "serde_bytes")]
    pub module_checksum: Vec<u8>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
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
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
    pub module_extra_chunks: Option<WasmModuleExtraChunks>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub arg: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeExternalCanisterOperation {
    #[serde(with = "serde_bytes")]
    pub module_checksum: Vec<u8>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
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
pub struct MonitoringExternalCanisterEstimatedRuntimeInput {
    pub fund_runtime_secs: u64,
    pub fallback_min_cycles: u128,
    pub min_runtime_secs: u64,
    pub fallback_fund_cycles: u128,
    pub max_runtime_cycles_fund: u128,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonitoringExternalCanisterCyclesThresholdInput {
    pub fund_cycles: u128,
    pub min_cycles: u128,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MonitorExternalCanisterStrategy {
    Always(u128),
    BelowThreshold(MonitoringExternalCanisterCyclesThresholdInput),
    BelowEstimatedRuntime(MonitoringExternalCanisterEstimatedRuntimeInput),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonitorExternalCanisterStartInput {
    pub funding_strategy: MonitorExternalCanisterStrategy,
    pub cycle_obtain_strategy: Option<CycleObtainStrategy>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MonitorExternalCanisterOperationKind {
    Start(MonitorExternalCanisterStartInput),
    Stop,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonitorExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub kind: MonitorExternalCanisterOperationKind,
}

pub type MonitorExternalCanisterOperation = MonitorExternalCanisterOperationInput;

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
    AllowedViewers(Vec<Principal>),
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
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub arg: Option<Vec<u8>>,
    pub execution_method_cycles: Option<u64>,
}

impl ModelValidator<ValidationError> for CallExternalCanisterOperationInput {
    fn validate(&self) -> ModelValidatorResult<ValidationError> {
        // Disallows the wildcard execution method
        if self.execution_method.method_name == CanisterMethod::WILDCARD {
            return Err(ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::ValidationError {
                    info: "Wildcard execution method is not allowed.".to_string(),
                },
            ));
        }

        // Validate both methods
        let validation_method_target: ValidationMethodResourceTarget =
            self.validation_method.clone().into();
        validation_method_target.validate()?;
        let execution_method_target: ExecutionMethodResourceTarget =
            self.execution_method.clone().into();
        execution_method_target.validate()?;

        Ok(())
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallExternalCanisterOperation {
    pub input: CallExternalCanisterOperationInput,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub arg_checksum: Option<Vec<u8>>,
    pub arg_rendering: Option<String>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub execution_method_reply: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SnapshotExternalCanisterOperationInput {
    pub canister_id: Principal,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub replace_snapshot: Option<Vec<u8>>,
    pub force: bool,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SnapshotExternalCanisterOperation {
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub snapshot_id: Option<Vec<u8>>,
    pub input: SnapshotExternalCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RestoreExternalCanisterOperationInput {
    pub canister_id: Principal,
    #[serde(with = "serde_bytes")]
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
    #[serde(with = "serde_bytes")]
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

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddNamedRuleOperationInput {
    pub name: String,
    pub description: Option<String>,
    pub rule: RequestPolicyRule,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddNamedRuleOperation {
    pub named_rule_id: Option<NamedRuleId>,
    pub input: AddNamedRuleOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveNamedRuleOperation {
    pub input: RemoveNamedRuleOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveNamedRuleOperationInput {
    pub named_rule_id: NamedRuleId,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditNamedRuleOperation {
    pub input: EditNamedRuleOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditNamedRuleOperationInput {
    pub named_rule_id: NamedRuleId,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub rule: Option<RequestPolicyRule>,
}

impl ModelValidator<ValidationError> for RequestOperation {
    fn validate(&self) -> ModelValidatorResult<ValidationError> {
        match self {
            RequestOperation::ManageSystemInfo(_) => (),
            RequestOperation::Transfer(op) => {
                EnsureAccount::id_exists(&op.input.from_account_id)?;
                EnsureAsset::id_exists(&op.input.from_asset_id)?;
            }
            RequestOperation::AddAccount(op) => {
                op.input.read_permission.validate()?;
                op.input.configs_permission.validate()?;
                op.input.transfer_permission.validate()?;

                if let Some(policy_rule) = &op.input.transfer_request_policy {
                    policy_rule.validate()?;
                }

                if let Some(policy_rule) = &op.input.configs_request_policy {
                    policy_rule.validate()?;
                }
            }
            RequestOperation::EditAccount(op) => {
                EnsureAccount::id_exists(&op.input.account_id)?;

                if let Some(allow) = &op.input.read_permission {
                    allow.validate()?;
                }

                if let Some(allow) = &op.input.configs_permission {
                    allow.validate()?;
                }

                if let Some(allow) = &op.input.transfer_permission {
                    allow.validate()?;
                }

                if let Some(RequestPolicyRuleInput::Set(criteria)) =
                    &op.input.configs_request_policy
                {
                    criteria.validate()?;
                }

                if let Some(RequestPolicyRuleInput::Set(policy_rule)) =
                    &op.input.transfer_request_policy
                {
                    policy_rule.validate()?;
                }

                if let Some(ChangeAssets::ReplaceWith { assets }) = &op.input.change_assets {
                    EnsureAsset::id_list_exists(assets)?;
                }

                if let Some(ChangeAssets::Change {
                    add_assets,
                    remove_assets,
                }) = &op.input.change_assets
                {
                    EnsureAsset::id_list_exists(add_assets)?;
                    EnsureAsset::id_list_exists(remove_assets)?;
                }
            }
            RequestOperation::AddAddressBookEntry(_) => (),
            RequestOperation::EditAddressBookEntry(op) => {
                EnsureAddressBookEntry::id_exists(&op.input.address_book_entry_id)?;
            }
            RequestOperation::RemoveAddressBookEntry(op) => {
                EnsureAddressBookEntry::id_exists(&op.input.address_book_entry_id)?;
            }
            RequestOperation::AddUser(op) => {
                EnsureUserGroup::id_list_exists(&op.input.groups)?;
            }
            RequestOperation::EditUser(op) => {
                EnsureUser::id_exists(&op.input.user_id)?;

                if let Some(group_ids) = &op.input.groups {
                    EnsureUserGroup::id_list_exists(group_ids)?;
                }
            }
            RequestOperation::EditPermission(op) => {
                op.input.resource.validate()?;

                if let Some(user_ids) = &op.input.users {
                    EnsureUser::id_list_exists(user_ids)?;
                }

                if let Some(group_ids) = &op.input.user_groups {
                    EnsureUserGroup::id_list_exists(group_ids)?;
                }
            }
            RequestOperation::AddUserGroup(_) => (),
            RequestOperation::EditUserGroup(op) => {
                EnsureUserGroup::id_exists(&op.input.user_group_id)?;
            }
            RequestOperation::RemoveUserGroup(ok) => {
                EnsureUserGroup::id_exists(&ok.input.user_group_id)?;
            }
            RequestOperation::SystemUpgrade(_) => (),
            RequestOperation::ChangeExternalCanister(op) => {
                let canister_id = op.input.canister_id;
                EnsureExternalCanister::is_external_canister(canister_id)?;
            }
            RequestOperation::ConfigureExternalCanister(op) => {
                let canister_id = op.canister_id;
                EnsureExternalCanister::is_external_canister(canister_id)?;
                if let ConfigureExternalCanisterOperationKind::Settings(settings) = &op.kind {
                    if let Some(updated_request_policies) = &settings.request_policies {
                        ContextualModel::new(updated_request_policies.clone(), canister_id)
                            .validate()?;
                    }
                }
            }
            RequestOperation::FundExternalCanister(op) => {
                let canister_id = op.canister_id;
                EnsureExternalCanister::is_external_canister(canister_id)?;
            }
            RequestOperation::MonitorExternalCanister(op) => {
                let canister_id = op.canister_id;
                EnsureExternalCanister::is_external_canister(canister_id)?;
            }
            RequestOperation::CreateExternalCanister(op) => {
                op.input.validate()?;
            }
            RequestOperation::CallExternalCanister(op) => {
                op.input.validate()?;
            }
            RequestOperation::SnapshotExternalCanister(op) => {
                let canister_id = op.input.canister_id;
                EnsureExternalCanister::is_external_canister(canister_id)?;
            }
            RequestOperation::RestoreExternalCanister(op) => {
                let canister_id = op.input.canister_id;
                EnsureExternalCanister::is_external_canister(canister_id)?;
            }
            RequestOperation::PruneExternalCanister(op) => {
                let canister_id = op.input.canister_id;
                EnsureExternalCanister::is_external_canister(canister_id)?;
            }
            RequestOperation::AddRequestPolicy(op) => {
                op.input.specifier.validate()?;
                op.input.rule.validate()?;
            }
            RequestOperation::EditRequestPolicy(op) => {
                EnsureRequestPolicy::id_exists(&op.input.policy_id)?;

                if let Some(specifier) = &op.input.specifier {
                    specifier.validate()?;
                }

                if let Some(policy_rule) = &op.input.rule {
                    policy_rule.validate()?;
                }
            }
            RequestOperation::RemoveRequestPolicy(op) => {
                EnsureRequestPolicy::id_exists(&op.input.policy_id)?;
            }
            RequestOperation::SetDisasterRecovery(op) => {
                if let Some(committee) = &op.input.committee {
                    EnsureUserGroup::id_exists(&committee.user_group_id)?;
                }
            }
            RequestOperation::AddAsset(_) => (),
            RequestOperation::EditAsset(op) => {
                EnsureAsset::id_exists(&op.input.asset_id)?;
            }
            RequestOperation::RemoveAsset(op) => {
                EnsureAsset::id_exists(&op.input.asset_id)?;
            }
            RequestOperation::AddNamedRule(_) => (),
            RequestOperation::EditNamedRule(op) => {
                EnsureNamedRule::id_exists(&op.input.named_rule_id)?;
            }
            RequestOperation::RemoveNamedRule(op) => {
                EnsureNamedRule::id_exists(&op.input.named_rule_id)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::validation::disable_mock_resource_validation;
    use crate::core::write_system_info;
    use crate::errors::ExternalCanisterValidationError;
    use crate::models::asset_test_utils::mock_asset;
    use crate::models::permission::Allow;
    use crate::models::{
        Account, AccountKey, AddAccountOperationInput, AddAssetOperationInput, AddUserOperation,
        AddUserOperationInput, Blockchain, Metadata, SystemInfo, TokenStandard, TransferOperation,
        TransferOperationInput,
    };
    use crate::repositories::ACCOUNT_REPOSITORY;
    use crate::services::{AccountService, AssetService};
    use orbit_essentials::repository::Repository;

    use super::ChangeAssets;

    #[test]
    fn test_change_assets() {
        let mut assets: Vec<AccountAsset> = [[3; 16], [9; 16], [10; 16], [11; 16], [13; 16]]
            .into_iter()
            .map(|id| AccountAsset {
                asset_id: id,
                balance: None,
            })
            .collect();

        ChangeAssets::Change {
            // 3 already exists, should not be added twice
            add_assets: vec![[0; 16], [1; 16], [2; 16], [3; 16]],
            // 12 doesn't exist, should not be in an issue
            remove_assets: vec![[10; 16], [11; 16], [12; 16]],
        }
        .apply(&mut assets);

        assert_eq!(assets.len(), 5 + 3 - 2);

        assert!(!assets.iter().any(|a| a.asset_id == [10; 16]));
        assert!(!assets.iter().any(|a| a.asset_id == [11; 16]));
        assert!(!assets.iter().any(|a| a.asset_id == [12; 16]));

        assert!(assets.iter().any(|a| a.asset_id == [0; 16]));
        assert!(assets.iter().any(|a| a.asset_id == [1; 16]));
        assert!(assets.iter().any(|a| a.asset_id == [2; 16]));
        assert!(assets.iter().any(|a| a.asset_id == [3; 16]));

        assert_eq!(assets.iter().filter(|a| a.asset_id == [3; 16]).count(), 1);
    }

    #[tokio::test]
    async fn test_request_operation_is_valid() {
        disable_mock_resource_validation();

        let asset = AssetService::default()
            .create(
                AddAssetOperationInput {
                    name: "a".to_owned(),
                    symbol: "a".to_owned(),
                    decimals: 0,
                    metadata: Metadata::default(),
                    blockchain: Blockchain::InternetComputer,
                    standards: vec![TokenStandard::InternetComputerNative],
                },
                None,
            )
            .expect("Failed to create asset");

        let account_service = AccountService::default();
        let account = account_service
            .create_account(
                AddAccountOperationInput {
                    name: "a".to_owned(),
                    assets: vec![asset.id],
                    metadata: Metadata::default(),
                    read_permission: Allow::default(),
                    configs_permission: Allow::default(),
                    transfer_permission: Allow::default(),
                    configs_request_policy: None,
                    transfer_request_policy: None,
                },
                None,
            )
            .await
            .expect("Failed to create account");

        let operation = RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,

            input: TransferOperationInput {
                network: "mainnet".to_string(),
                amount: 1u64.into(),
                fee: None,
                metadata: Metadata::default(),
                to: "0x1234".to_string(),
                from_account_id: account.id,
                from_asset_id: asset.id,
                with_standard: TokenStandard::InternetComputerNative,
            },
            asset,
        });

        assert!(operation.validate().is_ok());
    }

    #[tokio::test]
    async fn fail_request_operation_with_invalid_id() {
        disable_mock_resource_validation();

        RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,
            input: TransferOperationInput {
                network: "mainnet".to_string(),
                amount: 1u64.into(),
                fee: None,
                metadata: Metadata::default(),
                to: "0x1234".to_string(),
                from_account_id: [0; 16],
                from_asset_id: [0; 16],
                with_standard: TokenStandard::InternetComputerNative,
            },
            asset: mock_asset(),
        })
        .validate()
        .expect_err("Invalid account id should fail");

        RequestOperation::AddUser(AddUserOperation {
            user_id: None,
            input: AddUserOperationInput {
                name: "user-1".to_string(),
                identities: vec![],
                groups: vec![[1; 16]],
                status: crate::models::UserStatus::Active,
            },
        })
        .validate()
        .expect_err("Invalid user group id should fail");

        RequestOperation::EditUserGroup(crate::models::EditUserGroupOperation {
            input: crate::models::EditUserGroupOperationInput {
                user_group_id: [0; 16],
                name: "a".to_owned(),
            },
        })
        .validate()
        .expect_err("Invalid user group id should fail");
        RequestOperation::RemoveUserGroup(crate::models::RemoveUserGroupOperation {
            input: crate::models::RemoveUserGroupOperationInput {
                user_group_id: [0; 16],
            },
        })
        .validate()
        .expect_err("Invalid user group id should fail");

        RequestOperation::AddRequestPolicy(crate::models::AddRequestPolicyOperation {
            policy_id: None,
            input: crate::models::AddRequestPolicyOperationInput {
                specifier: crate::models::request_specifier::RequestSpecifier::EditUser(
                    crate::models::resource::ResourceIds::Ids(vec![[1; 16]]),
                ),
                rule: crate::models::request_policy_rule::RequestPolicyRule::AutoApproved,
            },
        })
        .validate()
        .expect_err("Invalid request specifier should fail");

        RequestOperation::EditRequestPolicy(crate::models::EditRequestPolicyOperation {
            input: crate::models::EditRequestPolicyOperationInput {
                policy_id: [0; 16],
                specifier: None,
                rule: None,
            },
        })
        .validate()
        .expect_err("Invalid request policy id should fail");

        RequestOperation::RemoveRequestPolicy(crate::models::RemoveRequestPolicyOperation {
            input: crate::models::RemoveRequestPolicyOperationInput { policy_id: [0; 16] },
        })
        .validate()
        .expect_err("Invalid request policy id should fail");

        RequestOperation::AddAccount(crate::models::AddAccountOperation {
            account_id: None,
            input: crate::models::AddAccountOperationInput {
                name: "a".to_owned(),
                assets: vec![],
                metadata: Metadata::default(),
                read_permission: Allow {
                    auth_scope: crate::models::permission::AuthScope::Restricted,
                    users: vec![[1; 16]],
                    user_groups: vec![],
                },
                configs_permission: Allow::default(),
                transfer_permission: Allow::default(),
                configs_request_policy: None,
                transfer_request_policy: None,
            },
        })
        .validate()
        .expect_err("Invalid user id should fail");

        RequestOperation::EditAccount(crate::models::EditAccountOperation {
            input: crate::models::EditAccountOperationInput {
                account_id: [0; 16],
                change_assets: None,
                read_permission: None,
                configs_permission: None,
                transfer_permission: None,
                configs_request_policy: None,
                transfer_request_policy: None,
                name: None,
            },
        })
        .validate()
        .expect_err("Invalid account id should fail");

        ACCOUNT_REPOSITORY.insert(
            AccountKey { id: [0; 16] },
            Account {
                id: [0; 16],
                name: "a".to_owned(),
                seed: [0; 16],
                assets: vec![],
                addresses: vec![],
                metadata: Metadata::default(),
                transfer_request_policy_id: None,
                configs_request_policy_id: None,
                last_modification_timestamp: 0,
            },
        );

        RequestOperation::EditAccount(crate::models::EditAccountOperation {
            input: crate::models::EditAccountOperationInput {
                account_id: [0; 16],
                change_assets: Some(ChangeAssets::ReplaceWith {
                    assets: vec![[0; 16]],
                }),
                read_permission: None,
                configs_permission: None,
                transfer_permission: None,
                configs_request_policy: None,
                transfer_request_policy: None,
                name: None,
            },
        })
        .validate()
        .expect_err("Invalid asset id should fail");

        ACCOUNT_REPOSITORY.clear();

        RequestOperation::EditAddressBookEntry(crate::models::EditAddressBookEntryOperation {
            input: crate::models::EditAddressBookEntryOperationInput {
                address_book_entry_id: [0; 16],
                address_owner: None,
                change_metadata: None,
                labels: None,
            },
        })
        .validate()
        .expect_err("Invalid address book entry id should fail");

        RequestOperation::RemoveAddressBookEntry(crate::models::RemoveAddressBookEntryOperation {
            input: crate::models::RemoveAddressBookEntryOperationInput {
                address_book_entry_id: [0; 16],
            },
        })
        .validate()
        .expect_err("Invalid address book entry id should fail");

        RequestOperation::EditUser(crate::models::EditUserOperation {
            input: crate::models::EditUserOperationInput {
                user_id: [0; 16],
                groups: None,
                name: None,
                identities: None,
                status: None,
                cancel_pending_requests: None,
            },
        })
        .validate()
        .expect_err("Invalid user id should fail");

        RequestOperation::EditPermission(crate::models::EditPermissionOperation {
            input: crate::models::EditPermissionOperationInput {
                resource: crate::models::resource::Resource::Account(
                    crate::models::resource::AccountResourceAction::Read(
                        crate::models::resource::ResourceId::Id([0; 16]),
                    ),
                ),
                users: None,
                user_groups: None,
                auth_scope: None,
            },
        })
        .validate()
        .expect_err("Invalid resource id should fail");
    }

    #[tokio::test]
    async fn fail_request_operation_with_non_external_canister() {
        let upgrader_id = candid::Principal::from_slice(&[42; 29]);
        let regular_canister = candid::Principal::from_slice(&[64; 29]);

        let mut system_info = SystemInfo::default();
        system_info.set_upgrader_canister_id(upgrader_id);
        write_system_info(system_info);

        let err = RequestOperation::CreateExternalCanister(
            crate::models::CreateExternalCanisterOperation {
                input: crate::models::CreateExternalCanisterOperationInput {
                    kind: crate::models::CreateExternalCanisterOperationKind::AddExisting(
                        crate::models::CreateExternalCanisterOperationKindAddExisting {
                            canister_id: upgrader_id,
                        },
                    ),
                    name: "canister".to_string(),
                    description: None,
                    labels: None,
                    metadata: None,
                    permissions: crate::models::ExternalCanisterPermissionsCreateInput {
                        read: Allow {
                            auth_scope: crate::models::permission::AuthScope::Public,
                            users: vec![],
                            user_groups: vec![],
                        },
                        change: Allow {
                            auth_scope: crate::models::permission::AuthScope::Public,
                            users: vec![],
                            user_groups: vec![],
                        },
                        calls: vec![],
                    },
                    request_policies: crate::models::ExternalCanisterRequestPoliciesCreateInput {
                        change: vec![],
                        calls: vec![],
                    },
                },
                canister_id: None,
            },
        )
        .validate()
        .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));

        let err = RequestOperation::ChangeExternalCanister(
            crate::models::ChangeExternalCanisterOperation {
                input: crate::models::ChangeExternalCanisterOperationInput {
                    canister_id: upgrader_id,
                    mode: crate::models::CanisterInstallMode::Upgrade(
                        crate::models::CanisterUpgradeModeArgs {},
                    ),
                    module: vec![],
                    module_extra_chunks: None,
                    arg: None,
                },
                module_checksum: vec![],
                arg_checksum: None,
            },
        )
        .validate()
        .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));

        let err = RequestOperation::ConfigureExternalCanister(
            crate::models::ConfigureExternalCanisterOperation {
                canister_id: upgrader_id,
                kind: ConfigureExternalCanisterOperationKind::Delete,
            },
        )
        .validate()
        .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));

        let err =
            RequestOperation::FundExternalCanister(crate::models::FundExternalCanisterOperation {
                canister_id: upgrader_id,
                kind: crate::models::FundExternalCanisterOperationKind::Send(
                    crate::models::FundExternalCanisterSendCyclesInput {
                        cycles: 100_000_000_000,
                    },
                ),
            })
            .validate()
            .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));

        let err = RequestOperation::MonitorExternalCanister(
            crate::models::MonitorExternalCanisterOperation {
                canister_id: upgrader_id,
                kind: crate::models::MonitorExternalCanisterOperationKind::Stop,
            },
        )
        .validate()
        .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));

        let err = RequestOperation::SnapshotExternalCanister(
            crate::models::SnapshotExternalCanisterOperation {
                input: crate::models::SnapshotExternalCanisterOperationInput {
                    canister_id: upgrader_id,
                    replace_snapshot: None,
                    force: false,
                },
                snapshot_id: None,
            },
        )
        .validate()
        .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));

        let err = RequestOperation::RestoreExternalCanister(
            crate::models::RestoreExternalCanisterOperation {
                input: crate::models::RestoreExternalCanisterOperationInput {
                    canister_id: upgrader_id,
                    snapshot_id: vec![],
                },
            },
        )
        .validate()
        .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));

        let err = RequestOperation::PruneExternalCanister(
            crate::models::PruneExternalCanisterOperation {
                input: crate::models::PruneExternalCanisterOperationInput {
                    canister_id: upgrader_id,
                    prune: crate::models::PruneExternalCanisterResource::State,
                },
            },
        )
        .validate()
        .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));

        let err =
            RequestOperation::CallExternalCanister(crate::models::CallExternalCanisterOperation {
                input: crate::models::CallExternalCanisterOperationInput {
                    validation_method: None,
                    execution_method: crate::models::CanisterMethod {
                        canister_id: upgrader_id,
                        method_name: "bar".to_string(),
                    },
                    arg: None,
                    execution_method_cycles: None,
                },
                arg_checksum: None,
                arg_rendering: None,
                execution_method_reply: None,
            })
            .validate()
            .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));

        let err =
            RequestOperation::CallExternalCanister(crate::models::CallExternalCanisterOperation {
                input: crate::models::CallExternalCanisterOperationInput {
                    validation_method: Some(crate::models::CanisterMethod {
                        canister_id: upgrader_id,
                        method_name: "foo".to_string(),
                    }),
                    execution_method: crate::models::CanisterMethod {
                        canister_id: regular_canister,
                        method_name: "bar".to_string(),
                    },
                    arg: None,
                    execution_method_cycles: None,
                },
                arg_checksum: None,
                arg_rendering: None,
                execution_method_reply: None,
            })
            .validate()
            .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::ExternalCanisterValidationError(
                ExternalCanisterValidationError::InvalidExternalCanister { .. }
            )
        ));
    }
}
