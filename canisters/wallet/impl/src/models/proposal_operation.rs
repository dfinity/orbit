use super::{AccountId, Policy, UserId};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalOperation {
    Transfer(TransferOperation),
    AccountEdit(AccountEditOperation),
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperation {
    pub from_account_id: AccountId,
    pub to: String,
    pub amount: candid::Nat,
    pub metadata: Vec<(String, String)>,
    pub network: String,
    pub fee: Option<candid::Nat>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountEditOperation {
    pub account_id: AccountId,
    pub owners: Option<Vec<UserId>>,
    pub policies: Option<Vec<Policy>>,
    pub name: Option<String>,
}
