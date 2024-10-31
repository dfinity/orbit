use std::fmt::Display;

use candid::Principal;
use ic_cdk::api::management_canister::main::CanisterInstallMode;
use orbit_essentials::{
    storable,
    types::{Timestamp, WasmModuleExtraChunks, UUID},
    utils::timestamp_to_rfc3339,
};
use uuid::Uuid;

use crate::utils::HelperMapper;

#[storable]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstallMode {
    /// Install the wasm module.
    Install,
    /// Upgrade the wasm module.
    Upgrade,
    /// Reinstall the wasm module.
    Reinstall,
}

impl Display for InstallMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallMode::Install => write!(f, "Install"),
            InstallMode::Reinstall => write!(f, "Reinstall"),
            InstallMode::Upgrade => write!(f, "Upgrade"),
        }
    }
}

impl From<upgrader_api::InstallMode> for InstallMode {
    fn from(value: upgrader_api::InstallMode) -> Self {
        match value {
            upgrader_api::InstallMode::Install => InstallMode::Install,
            upgrader_api::InstallMode::Upgrade => InstallMode::Upgrade,
            upgrader_api::InstallMode::Reinstall => InstallMode::Reinstall,
        }
    }
}

impl From<InstallMode> for upgrader_api::InstallMode {
    fn from(value: InstallMode) -> Self {
        match value {
            InstallMode::Install => upgrader_api::InstallMode::Install,
            InstallMode::Upgrade => upgrader_api::InstallMode::Upgrade,
            InstallMode::Reinstall => upgrader_api::InstallMode::Reinstall,
        }
    }
}

impl From<InstallMode> for CanisterInstallMode {
    fn from(val: InstallMode) -> Self {
        match val {
            InstallMode::Install => CanisterInstallMode::Install,
            InstallMode::Upgrade => CanisterInstallMode::Upgrade(None),
            InstallMode::Reinstall => CanisterInstallMode::Reinstall,
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct StationRecoveryRequest {
    /// The user ID of the station.
    pub user_id: UUID,
    /// The wasm module to be installed.
    pub wasm_module: Vec<u8>,
    /// Optional extra chunks of the wasm module to be installed.
    pub wasm_module_extra_chunks: Option<WasmModuleExtraChunks>,
    /// The SHA-256 hash of the wasm module.
    pub wasm_sha256: Vec<u8>,
    /// The install mode: upgrade or reinstall.
    pub install_mode: InstallMode,
    /// The install arguments.
    pub arg: Vec<u8>,
    /// The SHA-256 hash of the install arguments.
    pub arg_sha256: Vec<u8>,
    /// Time in nanoseconds since the UNIX epoch when the request was submitted.
    pub submitted_at: Timestamp,
}

impl From<StationRecoveryRequest> for upgrader_api::StationRecoveryRequest {
    fn from(value: StationRecoveryRequest) -> Self {
        upgrader_api::StationRecoveryRequest {
            user_id: Uuid::from_bytes(value.user_id).hyphenated().to_string(),
            wasm_sha256: value.wasm_sha256,
            install_mode: upgrader_api::InstallMode::from(value.install_mode),
            arg: value.arg,
            submitted_at: timestamp_to_rfc3339(&value.submitted_at),
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub enum RecoveryEvaluationResult {
    /// There are active recovery requests, but there is no quorum yet.
    Unmet,
    /// There is a consensus on the recovery requests.
    Met(Box<StationRecoveryRequest>),
}

#[storable]
#[derive(Clone, Debug, PartialEq)]
pub enum RecoveryStatus {
    /// There are no active recovery requests.
    Idle,
    /// There is a consensus on the recovery requests.
    InProgress { since: Timestamp },
}

impl From<RecoveryStatus> for upgrader_api::RecoveryStatus {
    fn from(value: RecoveryStatus) -> Self {
        match value {
            RecoveryStatus::Idle => upgrader_api::RecoveryStatus::Idle,
            RecoveryStatus::InProgress { since } => upgrader_api::RecoveryStatus::InProgress {
                since: timestamp_to_rfc3339(&since),
            },
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct RecoveryFailure {
    /// The reason for the recovery failure.
    pub reason: String,
}

#[storable]
#[derive(Clone, Debug)]
pub enum RecoveryResult {
    /// The recovery request was successful.
    Success,
    /// The recovery request failed.
    Failure(RecoveryFailure),
}

impl From<RecoveryResult> for upgrader_api::RecoveryResult {
    fn from(value: RecoveryResult) -> Self {
        match value {
            RecoveryResult::Success => upgrader_api::RecoveryResult::Success,
            RecoveryResult::Failure(failure) => {
                upgrader_api::RecoveryResult::Failure(upgrader_api::RecoveryFailure {
                    reason: failure.reason,
                })
            }
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct DisasterRecoveryCommittee {
    pub users: Vec<AdminUser>,
    pub quorum: u16,
}

impl From<upgrader_api::DisasterRecoveryCommittee> for DisasterRecoveryCommittee {
    fn from(value: upgrader_api::DisasterRecoveryCommittee) -> Self {
        DisasterRecoveryCommittee {
            users: value.users.into_iter().map(AdminUser::from).collect(),
            quorum: value.quorum,
        }
    }
}

impl From<DisasterRecoveryCommittee> for upgrader_api::DisasterRecoveryCommittee {
    fn from(value: DisasterRecoveryCommittee) -> Self {
        upgrader_api::DisasterRecoveryCommittee {
            users: value
                .users
                .into_iter()
                .map(upgrader_api::AdminUser::from)
                .collect(),
            quorum: value.quorum,
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct Metadata {
    pub key: String,
    pub value: String,
}

impl From<upgrader_api::MetadataDTO> for Metadata {
    fn from(value: upgrader_api::MetadataDTO) -> Self {
        Metadata {
            key: value.key,
            value: value.value,
        }
    }
}

impl From<Metadata> for upgrader_api::MetadataDTO {
    fn from(value: Metadata) -> Self {
        upgrader_api::MetadataDTO {
            key: value.key,
            value: value.value,
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct AdminUser {
    /// The user ID.
    pub id: UUID,
    /// The name of the user (if any).
    pub name: String,
    /// The identities associated with the user.
    pub identities: Vec<Principal>,
}

impl AdminUser {
    pub fn to_summary(&self) -> String {
        format!("{}[{}]", self.name, Uuid::from_bytes(self.id).hyphenated())
    }
}

impl From<upgrader_api::AdminUser> for AdminUser {
    fn from(value: upgrader_api::AdminUser) -> Self {
        AdminUser {
            id: *HelperMapper::to_uuid(value.id)
                .expect("Invalid user ID")
                .as_bytes(),
            name: value.name,
            identities: value.identities,
        }
    }
}

impl From<AdminUser> for upgrader_api::AdminUser {
    fn from(value: AdminUser) -> Self {
        upgrader_api::AdminUser {
            id: Uuid::from_bytes(value.id).hyphenated().to_string(),
            name: value.name,
            identities: value.identities,
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct Asset {
    /// The asset id, which is a UUID.
    pub id: UUID,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    /// The number of decimal places that the asset supports (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u32,
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: String,
    // The asset standard that is supported (e.g. `erc20`, `native`, etc.), canonically
    // represented as a lowercase string with spaces replaced with underscores.
    pub standards: Vec<String>,
    /// The account metadata, which is a list of key-value pairs,
    /// where the key is unique and the first entry in the tuple,
    /// and the value is the second entry in the tuple.
    pub metadata: Vec<Metadata>,
}

impl From<upgrader_api::Asset> for Asset {
    fn from(value: upgrader_api::Asset) -> Self {
        Asset {
            id: *HelperMapper::to_uuid(value.id)
                .expect("Invalid asset ID")
                .as_bytes(),
            name: value.name,
            symbol: value.symbol,
            decimals: value.decimals,
            blockchain: value.blockchain,
            standards: value.standards,
            metadata: value.metadata.into_iter().map(Metadata::from).collect(),
        }
    }
}

impl From<Asset> for upgrader_api::Asset {
    fn from(value: Asset) -> Self {
        upgrader_api::Asset {
            id: Uuid::from_bytes(value.id).hyphenated().to_string(),
            name: value.name,
            symbol: value.symbol,
            decimals: value.decimals,
            blockchain: value.blockchain,
            standards: value.standards,
            metadata: value
                .metadata
                .into_iter()
                .map(upgrader_api::MetadataDTO::from)
                .collect(),
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct Account {
    /// The account id, which is a UUID.
    pub id: UUID,
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: String,
    /// The account address (e.g. `0x1234`, etc.)
    pub address: String,
    /// The blockchain standard (e.g. `native`, `icrc1`, `erc20`, etc.)
    pub standard: String,
    /// The asset symbol (e.g. `ICP`, `ETH`, `BTC`, etc.)
    pub symbol: String,
    /// The asset decimals (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u32,
    /// The account name (e.g. `My Main Account`)
    pub name: String,
    /// The account metadata, which is a list of key-value pairs,
    /// where the key is unique and the first entry in the tuple,
    /// and the value is the second entry in the tuple.
    pub metadata: Vec<Metadata>,
}

type AccountSeed = [u8; 16];
#[storable]
#[derive(Clone, Debug)]
pub struct MultiAssetAccount {
    /// The account id, which is a UUID.
    pub id: UUID,
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub name: String,
    /// The address generation seed.
    pub seed: AccountSeed,
    /// Assets
    pub assets: Vec<UUID>,
    /// The account metadata, which is a list of key-value pairs,
    /// where the key is unique and the first entry in the tuple,
    /// and the value is the second entry in the tuple.
    pub metadata: Vec<Metadata>,
}

impl From<upgrader_api::Account> for Account {
    fn from(value: upgrader_api::Account) -> Self {
        Account {
            id: *HelperMapper::to_uuid(value.id)
                .expect("Invalid account ID")
                .as_bytes(),
            blockchain: value.blockchain,
            address: value.address,
            standard: value.standard,
            symbol: value.symbol,
            decimals: value.decimals,
            name: value.name,
            metadata: value.metadata.into_iter().map(Metadata::from).collect(),
        }
    }
}

impl From<Account> for upgrader_api::Account {
    fn from(value: Account) -> Self {
        upgrader_api::Account {
            id: Uuid::from_bytes(value.id).hyphenated().to_string(),
            blockchain: value.blockchain,
            address: value.address,
            standard: value.standard,
            symbol: value.symbol,
            decimals: value.decimals,
            name: value.name,
            metadata: value
                .metadata
                .into_iter()
                .map(upgrader_api::MetadataDTO::from)
                .collect(),
        }
    }
}

impl From<upgrader_api::MultiAssetAccount> for MultiAssetAccount {
    fn from(value: upgrader_api::MultiAssetAccount) -> Self {
        MultiAssetAccount {
            id: *HelperMapper::to_uuid(value.id)
                .expect("Invalid account ID")
                .as_bytes(),
            assets: value
                .assets
                .into_iter()
                .map(|asset_id| {
                    *HelperMapper::to_uuid(asset_id)
                        .expect("Invalid asset ID")
                        .as_bytes()
                })
                .collect(),
            seed: value.seed,
            name: value.name,
            metadata: value.metadata.into_iter().map(Metadata::from).collect(),
        }
    }
}

impl From<MultiAssetAccount> for upgrader_api::MultiAssetAccount {
    fn from(value: MultiAssetAccount) -> Self {
        upgrader_api::MultiAssetAccount {
            id: Uuid::from_bytes(value.id).hyphenated().to_string(),
            name: value.name,
            seed: value.seed,
            assets: value
                .assets
                .into_iter()
                .map(|asset_id| Uuid::from_bytes(asset_id).hyphenated().to_string())
                .collect(),
            metadata: value
                .metadata
                .into_iter()
                .map(upgrader_api::MetadataDTO::from)
                .collect(),
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct DisasterRecovery {
    pub accounts: Vec<Account>,

    #[serde(default)]
    pub multi_asset_accounts: Vec<MultiAssetAccount>,
    #[serde(default)]
    pub assets: Vec<Asset>,

    pub committee: Option<DisasterRecoveryCommittee>,

    pub recovery_requests: Vec<StationRecoveryRequest>,
    pub recovery_status: RecoveryStatus,
    pub last_recovery_result: Option<RecoveryResult>,
}

impl Default for DisasterRecovery {
    fn default() -> Self {
        DisasterRecovery {
            accounts: vec![],
            multi_asset_accounts: vec![],
            assets: vec![],
            committee: None,
            recovery_requests: vec![],
            recovery_status: RecoveryStatus::Idle,
            last_recovery_result: None,
        }
    }
}

impl From<DisasterRecovery> for upgrader_api::GetDisasterRecoveryStateResponse {
    fn from(value: DisasterRecovery) -> Self {
        upgrader_api::GetDisasterRecoveryStateResponse {
            accounts: value
                .accounts
                .into_iter()
                .map(upgrader_api::Account::from)
                .collect(),

            multi_asset_accounts: value
                .multi_asset_accounts
                .into_iter()
                .map(upgrader_api::MultiAssetAccount::from)
                .collect(),
            assets: value
                .assets
                .into_iter()
                .map(upgrader_api::Asset::from)
                .collect(),

            committee: value
                .committee
                .map(upgrader_api::DisasterRecoveryCommittee::from),
            recovery_requests: value
                .recovery_requests
                .into_iter()
                .map(upgrader_api::StationRecoveryRequest::from)
                .collect(),
            recovery_status: value.recovery_status.into(),
            last_recovery_result: value.last_recovery_result.map(|r| r.into()),
        }
    }
}

#[cfg(test)]
pub mod test {
    use candid::Principal;

    use crate::model::{Asset, MultiAssetAccount};

    use super::{Account, AdminUser, DisasterRecoveryCommittee};

    pub fn mock_committee() -> DisasterRecoveryCommittee {
        DisasterRecoveryCommittee {
            users: vec![
                AdminUser {
                    id: [1; 16],
                    name: "admin_user_1".to_owned(),
                    identities: vec![Principal::from_slice(&[1; 29])],
                },
                AdminUser {
                    id: [2; 16],
                    name: "admin_user_2".to_owned(),
                    identities: vec![Principal::from_slice(&[2; 29])],
                },
                AdminUser {
                    id: [3; 16],
                    name: "admin_user_3".to_owned(),
                    identities: vec![Principal::from_slice(&[3; 29])],
                },
            ],
            quorum: 2,
        }
    }

    pub fn mock_accounts() -> Vec<Account> {
        vec![
            Account {
                id: [1; 16],
                blockchain: "icp".to_owned(),
                address: "0x1234".to_owned(),
                standard: "native".to_owned(),
                symbol: "ICP".to_owned(),
                decimals: 8,
                name: "Main Account".to_owned(),
                metadata: vec![],
            },
            Account {
                id: [2; 16],
                blockchain: "eth".to_owned(),
                address: "0x5678".to_owned(),
                standard: "erc20".to_owned(),
                symbol: "ETH".to_owned(),
                decimals: 18,
                name: "Secondary Account".to_owned(),
                metadata: vec![],
            },
        ]
    }

    pub fn mock_multi_asset_accounts() -> Vec<MultiAssetAccount> {
        vec![
            MultiAssetAccount {
                id: [1; 16],
                assets: vec![[1; 16], [2; 16]],
                seed: [0; 16],
                name: "Main Account".to_owned(),
                metadata: vec![],
            },
            MultiAssetAccount {
                id: [2; 16],
                assets: vec![[1; 16]],
                seed: [0; 16],
                name: "Secondary Account".to_owned(),
                metadata: vec![],
            },
        ]
    }

    pub fn mock_assets() -> Vec<Asset> {
        vec![
            Asset {
                id: [1; 16],
                name: "Internet Computer".to_owned(),
                symbol: "ICP".to_owned(),
                decimals: 8,
                blockchain: "icp".to_owned(),
                standards: vec!["icp_native".to_owned()],
                metadata: vec![],
            },
            Asset {
                id: [2; 16],
                name: "Ethereum".to_owned(),
                symbol: "ETH".to_owned(),
                decimals: 18,
                blockchain: "eth".to_owned(),
                standards: vec!["erc20".to_owned()],
                metadata: vec![],
            },
        ]
    }
}
