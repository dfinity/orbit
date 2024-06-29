use candid::{CandidType, Deserialize, Principal};
use station_api::TimestampRfc3339;
pub use station_api::{MetadataDTO, UuidDTO};

#[derive(Clone, Debug, CandidType, serde::Serialize, Deserialize, PartialEq)]
pub struct UpgradeParams {
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub arg: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, serde::Serialize, Deserialize)]
pub struct InitArg {
    pub target_canister: Principal,
}

#[derive(Clone, Debug, CandidType, serde::Serialize, Deserialize)]
pub enum TriggerUpgradeError {
    NotController,
    Unauthorized,
    UnexpectedError(String),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct AdminUser {
    /// The user ID.
    pub id: UuidDTO,
    /// The name of the user (if any).
    pub name: String,
    /// The identities associated with the user.
    pub identities: Vec<Principal>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct DisasterRecoveryCommittee {
    /// The users that are able to request disaster recovery.
    pub users: Vec<AdminUser>,
    /// The quorum required to approve a disaster recovery request.
    pub quorum: u16,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    /// The account id, which is a UUID.
    pub id: UuidDTO,
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
    pub metadata: Vec<MetadataDTO>,
}

#[derive(Clone, Debug, CandidType)]
pub enum DisasterRecoveryError {
    Unauthorized,
}

#[derive(Clone, Debug, CandidType)]
pub struct IsCommitteeMemberResponse {
    pub is_committee_member: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GetDisasterRecoveryAccountsResponse {
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GetDisasterRecoveryCommitteeResponse {
    pub committee: Option<DisasterRecoveryCommittee>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct SetDisasterRecoveryCommitteeInput {
    pub committee: DisasterRecoveryCommittee,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct SetDisasterRecoveryAccountsInput {
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum InstallMode {
    /// Install the module.
    Install,
    /// Upgrade the module.
    Upgrade,
    /// Reinstall the module.
    Reinstall,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RequestDisasterRecoveryInput {
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub arg: Vec<u8>,

    pub install_mode: InstallMode,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct PaginationInput {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GetLogsInput {
    pub pagination: Option<PaginationInput>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct LogEntry {
    pub time: TimestampRfc3339,
    pub entry_type: String,
    pub message: String,
    pub data_json: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GetLogsResponse {
    pub logs: Vec<LogEntry>,
    pub next_offset: Option<u64>,
    pub total: u64,
}

#[derive(Clone, Debug, CandidType, serde::Serialize, Deserialize)]
pub enum TriggerUpgradeResponse {
    Ok,
    Err(TriggerUpgradeError),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StationRecoveryRequest {
    /// The user ID of the station.
    pub user_id: UuidDTO,
    /// The SHA-256 hash of the wasm module.
    pub wasm_sha256: Vec<u8>,
    /// The install mode: upgrade or reinstall.
    pub install_mode: InstallMode,
    /// The install arguments.
    pub arg: Vec<u8>,
    /// Time in nanoseconds since the UNIX epoch when the request was submitted.
    pub submitted_at: TimestampRfc3339,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum RecoveryStatus {
    /// There are no active recovery requests.
    Idle,
    /// There is a consensus on the recovery requests.
    InProgress { since: TimestampRfc3339 },
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RecoveryFailure {
    /// The reason for the recovery failure.
    pub reason: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum RecoveryResult {
    /// The recovery request was successful.
    Success,
    /// The recovery request failed.
    Failure(RecoveryFailure),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GetDisasterRecoveryStateResponse {
    pub committee: Option<DisasterRecoveryCommittee>,
    pub accounts: Vec<Account>,

    pub recovery_requests: Vec<StationRecoveryRequest>,
    pub recovery_status: RecoveryStatus,
    pub last_recovery_result: Option<RecoveryResult>,
}
