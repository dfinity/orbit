use super::{AccountId, Blockchain, BlockchainStandard, Policy, UserId};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalOperation {
    Transfer(TransferOperation),
    EditAccount(EditAccountOperation),
    AddAccount(AddAccountOperation),
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
pub struct EditAccountOperation {
    pub account_id: AccountId,
    pub owners: Option<Vec<UserId>>,
    pub policies: Option<Vec<Policy>>,
    pub name: Option<String>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperation {
    /// The account id is only available after the operation is executed.
    pub id: Option<AccountId>,
    pub name: String,
    pub owners: Vec<UserId>,
    pub policies: Vec<Policy>,
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub metadata: Vec<(String, String)>,
}
