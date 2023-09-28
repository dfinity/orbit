use candid::{CandidType, Deserialize, Principal};

pub type BankAccountIdDTO = String;
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
pub enum CreateWalletInputOwnersItemDTO {
    #[serde(rename = "Principal")]
    Principal_(Principal),
    #[serde(rename = "AccountID")]
    AccountId(BankAccountIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateWalletInput {
    pub owners: Vec<CreateWalletInputOwnersItemDTO>,
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
pub struct GetWalletBalanceInput {
    pub wallet_id: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct WalletBalanceDTO {
    pub wallet_id: String,
    pub balance: candid::Nat,
    pub decimals: u32,
    pub last_update_timestamp: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct GetWalletBalanceResponse {
    pub balance: WalletBalanceDTO,
}
