use super::{AccountId, Blockchain, BlockchainStandard, Policy, UserId, UserStatus};
use candid::{CandidType, Deserialize, Principal};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalOperation {
    Transfer(TransferOperation),
    AddAccount(AddAccountOperation),
    EditAccount(EditAccountOperation),
    AddUser(AddUserOperation),
    EditUser(EditUserOperation),
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
pub struct AddUserOperationInput {
    pub name: Option<String>,
    pub identities: Vec<Principal>,
    pub groups: Vec<String>,
    pub status: UserStatus,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserOperation {
    pub user_id: Option<UUID>,
    pub input: AddUserOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperationInput {
    pub user_id: UUID,
    pub name: Option<String>,
    pub identities: Option<Vec<Principal>>,
    pub groups: Option<Vec<String>>,
    pub status: Option<UserStatus>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperation {
    pub input: EditUserOperationInput,
}
