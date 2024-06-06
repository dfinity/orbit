use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
pub use station_api::{MetadataDTO, UuidDTO};

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub struct UpgradeParams {
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub arg: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    pub target_canister: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
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
    pub users: Vec<AdminUser>,
    pub quorum_percentage: u16,
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
