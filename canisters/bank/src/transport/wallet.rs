use super::AccountIdDTO;
use candid::{CandidType, Deserialize};

pub type WalletIdDTO = String;
pub type UuidDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct WalletDTO {
    pub id: WalletIdDTO,
    pub owners: Vec<UuidDTO>,
    pub name: Option<String>,
    pub address: String,
    pub blockchain: String,
    pub standard: String,
    pub symbol: String,
    pub decimals: u32,
    pub balance: Option<WalletBalanceInfoDTO>,
    pub policies: Vec<WalletPolicyDTO>,
    pub metadata: Vec<(String, String)>,
    pub last_modification_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ApprovalThresholdPolicyDTO {
    VariableThreshold(u8),
    FixedThreshold(u8),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum WalletPolicyDTO {
    #[serde(rename = "approval_threshold")]
    ApprovalThreshold(ApprovalThresholdPolicyDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateWalletInput {
    pub owners: Vec<AccountIdDTO>,
    pub name: Option<String>,
    pub blockchain: String,
    pub standard: String,
    pub policies: Vec<WalletPolicyDTO>,
    pub metadata: Option<Vec<(String, String)>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateWalletResponse {
    pub wallet: WalletDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetWalletInput {
    pub wallet_id: WalletIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetWalletResponse {
    pub wallet: WalletDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct FetchWalletBalancesInput {
    pub wallet_ids: Vec<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct WalletBalanceDTO {
    pub wallet_id: String,
    pub balance: candid::Nat,
    pub decimals: u32,
    pub last_update_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct WalletBalanceInfoDTO {
    pub balance: candid::Nat,
    pub decimals: u32,
    pub last_update_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct FetchWalletBalancesResponse {
    pub balances: Vec<WalletBalanceDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct WalletListItemDTO {
    pub id: WalletIdDTO,
    pub address: String,
    pub asset_symbol: String,
    pub asset_name: Option<String>,
    pub name: Option<String>,
    pub decimals: u32,
    pub balance: Option<WalletBalanceInfoDTO>,
    pub nr_owners: u8,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListWalletResponse {
    pub wallets: Vec<WalletListItemDTO>,
}
