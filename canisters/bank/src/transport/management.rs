use super::{AccountRoleDTO, WalletPolicyDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BankPermissionDTO {
    pub permission_id: String,
    pub access_roles: Vec<AccountRoleDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct BankCanisterInit {
    pub owners: Option<Vec<Principal>>,
    pub approval_threshold: Option<u8>,
    pub permissions: Option<Vec<BankPermissionDTO>>,
    pub wallet_policies: Option<Vec<WalletPolicyDTO>>,
}
