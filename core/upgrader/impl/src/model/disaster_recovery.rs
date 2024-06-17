use candid::Principal;
use orbit_essentials::{
    storable,
    types::{Timestamp, UUID},
};
use uuid::Uuid;

use crate::utils::HelperMapper;

#[storable]
#[derive(Clone, Debug)]
pub enum InstallMode {
    /// Upgrade the wasm module.
    Upgrade,
    /// Reinstall the wasm module.
    Reinstall,
}

impl From<upgrader_api::InstallMode> for InstallMode {
    fn from(value: upgrader_api::InstallMode) -> Self {
        match value {
            upgrader_api::InstallMode::Upgrade => InstallMode::Upgrade,
            upgrader_api::InstallMode::Reinstall => InstallMode::Reinstall,
        }
    }
}

impl From<InstallMode> for upgrader_api::InstallMode {
    fn from(value: InstallMode) -> Self {
        match value {
            InstallMode::Upgrade => upgrader_api::InstallMode::Upgrade,
            InstallMode::Reinstall => upgrader_api::InstallMode::Reinstall,
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
    /// The SHA-256 hash of the wasm module.
    pub wasm_sha256: Vec<u8>,
    /// The install mode: upgrade or reinstall.
    pub install_mode: InstallMode,
    /// The install arguments.
    pub arg: Vec<u8>,
    /// Time in nanoseconds since the UNIX epoch when the request was submitted.
    pub submitted_at: Timestamp,
}

#[storable]
#[derive(Clone, Debug)]
pub enum RecoveryEvaluationResult {
    /// There are active recovery requests, but there is no quorum yet.
    Unmet,
    /// There is a consensus on the recovery requests.
    Met(StationRecoveryRequest),
}

#[storable]
#[derive(Clone, Debug, PartialEq)]
pub enum RecoveryStatus {
    /// There are no active recovery requests.
    Idle,
    /// There are active recovery requests, but there is no quorum yet.
    Unmet,
    /// There is a consensus on the recovery requests.
    InProgress,
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

#[storable]
#[derive(Clone, Debug)]
pub struct DisasterRecoveryCommittee {
    pub users: Vec<AdminUser>,
    pub quorum_percentage: u16,
}

impl From<upgrader_api::DisasterRecoveryCommittee> for DisasterRecoveryCommittee {
    fn from(value: upgrader_api::DisasterRecoveryCommittee) -> Self {
        DisasterRecoveryCommittee {
            users: value.users.into_iter().map(AdminUser::from).collect(),
            quorum_percentage: value.quorum_percentage,
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
            quorum_percentage: value.quorum_percentage,
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

#[storable]
#[derive(Clone, Debug)]
pub struct DisasterRecovery {
    pub accounts: Vec<Account>,
    pub committee: Option<DisasterRecoveryCommittee>,

    pub recovery_requests: Vec<StationRecoveryRequest>,
    pub recovery_status: RecoveryStatus,
    pub last_recovery_result: Option<RecoveryResult>,
}

impl Default for DisasterRecovery {
    fn default() -> Self {
        DisasterRecovery {
            accounts: vec![],
            committee: None,
            recovery_requests: vec![],
            recovery_status: RecoveryStatus::Idle,
            last_recovery_result: None,
        }
    }
}
