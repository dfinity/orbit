use super::{
    access_control::{ResourceSpecifier, UserSpecifier},
    criteria::Criteria,
    specifier::ProposalSpecifier,
    AccountId, Blockchain, BlockchainStandard, UserId, UserStatus,
};
use candid::{CandidType, Deserialize, Principal};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;
use wallet_api::{AccountMetadataDTO, TransferMetadataDTO};

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalOperation {
    Transfer(TransferOperation),
    AddAccount(AddAccountOperation),
    EditAccount(EditAccountOperation),
    AddUser(AddUserOperation),
    EditUser(EditUserOperation),
    AddAccessPolicy(AddAccessPolicyOperation),
    EditAccessPolicy(EditAccessPolicyOperation),
    RemoveAccessPolicy(RemoveAccessPolicyOperation),
    AddUserGroup(AddUserGroupOperation),
    EditUserGroup(EditUserGroupOperation),
    RemoveUserGroup(RemoveUserGroupOperation),
    ChangeCanister(ChangeCanisterOperation),
    AddProposalPolicy(AddProposalPolicyOperation),
    EditProposalPolicy(EditProposalPolicyOperation),
    RemoveProposalPolicy(RemoveProposalPolicyOperation),
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperation {
    pub transfer_id: Option<UUID>,
    pub input: TransferOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperationInput {
    pub from_account_id: AccountId,
    pub to: String,
    pub amount: candid::Nat,
    pub metadata: Vec<TransferMetadataDTO>,
    pub network: String,
    pub fee: Option<candid::Nat>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountPoliciesInput {
    pub transfer: Option<Criteria>,
    pub edit: Option<Criteria>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperation {
    /// The account id is only available after the operation is executed.
    pub account_id: Option<AccountId>,
    pub input: AddAccountOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperationInput {
    pub name: String,
    pub owners: Vec<UserId>,
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub metadata: Vec<AccountMetadataDTO>,
    pub policies: AccountPoliciesInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccountOperation {
    pub input: EditAccountOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccountOperationInput {
    pub account_id: AccountId,
    pub owners: Option<Vec<UserId>>,
    pub policies: Option<AccountPoliciesInput>,
    pub name: Option<String>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserOperation {
    pub user_id: Option<UUID>,
    pub input: AddUserOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserOperationInput {
    pub name: Option<String>,
    pub identities: Vec<Principal>,
    pub unconfirmed_identities: Vec<Principal>,
    pub groups: Vec<UUID>,
    pub status: UserStatus,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperation {
    pub input: EditUserOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperationInput {
    pub user_id: UUID,
    pub name: Option<String>,
    pub identities: Option<Vec<Principal>>,
    pub unconfirmed_identities: Option<Vec<Principal>>,
    pub groups: Option<Vec<UUID>>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserGroupOperation {
    pub user_group_id: Option<UUID>,
    pub input: AddUserGroupOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserGroupOperationInput {
    pub name: String,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserGroupOperation {
    pub input: EditUserGroupOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserGroupOperationInput {
    pub user_group_id: UUID,
    pub name: String,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveUserGroupOperation {
    pub input: RemoveUserGroupOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveUserGroupOperationInput {
    pub user_group_id: UUID,
}

#[stable_object]
#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChangeCanisterTarget {
    UpgradeWallet,
    UpgradeUpgrader,
    UpgradeCanister(Principal),
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeCanisterOperationInput {
    pub target: ChangeCanisterTarget,
    pub module: Vec<u8>,
    pub arg: Option<Vec<u8>>,
    pub checksum: Vec<u8>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeCanisterOperation {
    pub input: ChangeCanisterOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccessPolicyOperationInput {
    pub user: UserSpecifier,
    pub resource: ResourceSpecifier,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccessPolicyOperation {
    pub policy_id: Option<UUID>,
    pub input: AddAccessPolicyOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccessPolicyOperationInput {
    pub policy_id: UUID,
    pub user: Option<UserSpecifier>,
    pub resource: Option<ResourceSpecifier>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccessPolicyOperation {
    pub input: EditAccessPolicyOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAccessPolicyOperationInput {
    pub policy_id: UUID,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAccessPolicyOperation {
    pub input: RemoveAccessPolicyOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddProposalPolicyOperationInput {
    pub specifier: ProposalSpecifier,
    pub criteria: Criteria,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddProposalPolicyOperation {
    pub policy_id: Option<UUID>,
    pub input: AddProposalPolicyOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditProposalPolicyOperationInput {
    pub policy_id: UUID,
    pub specifier: Option<ProposalSpecifier>,
    pub criteria: Option<Criteria>,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditProposalPolicyOperation {
    pub input: EditProposalPolicyOperationInput,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveProposalPolicyOperationInput {
    pub policy_id: UUID,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveProposalPolicyOperation {
    pub input: RemoveProposalPolicyOperationInput,
}
