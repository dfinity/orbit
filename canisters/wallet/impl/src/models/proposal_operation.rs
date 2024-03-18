use super::{
    access_policy::{AuthScope, Resource},
    criteria::Criteria,
    specifier::ProposalSpecifier,
    AccountId, AddressBookEntryId, Blockchain, BlockchainStandard, ChangeMetadata, MetadataItem,
    UserGroupId, UserId, UserStatus,
};
use crate::models::Metadata;
use candid::Principal;
use ic_canister_core::types::UUID;
use ic_canister_macros::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalOperation {
    Transfer(TransferOperation),
    AddAccount(AddAccountOperation),
    EditAccount(EditAccountOperation),
    AddAddressBookEntry(AddAddressBookEntryOperation),
    EditAddressBookEntry(EditAddressBookEntryOperation),
    RemoveAddressBookEntry(RemoveAddressBookEntryOperation),
    AddUser(AddUserOperation),
    EditUser(EditUserOperation),
    EditAccessPolicy(EditAccessPolicyOperation),
    AddUserGroup(AddUserGroupOperation),
    EditUserGroup(EditUserGroupOperation),
    RemoveUserGroup(RemoveUserGroupOperation),
    ChangeCanister(ChangeCanisterOperation),
    AddProposalPolicy(AddProposalPolicyOperation),
    EditProposalPolicy(EditProposalPolicyOperation),
    RemoveProposalPolicy(RemoveProposalPolicyOperation),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperation {
    pub transfer_id: Option<UUID>,
    pub input: TransferOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperationInput {
    pub from_account_id: AccountId,
    pub to: String,
    pub amount: candid::Nat,
    pub metadata: Metadata,
    pub network: String,
    pub fee: Option<candid::Nat>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountPoliciesInput {
    pub transfer: Option<Criteria>,
    pub edit: Option<Criteria>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperation {
    /// The account id is only available after the operation is executed.
    pub account_id: Option<AccountId>,
    pub input: AddAccountOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperationInput {
    pub name: String,
    pub owners: Vec<UserId>,
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub metadata: Metadata,
    pub policies: AccountPoliciesInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccountOperation {
    pub input: EditAccountOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccountOperationInput {
    pub account_id: AccountId,
    pub owners: Option<Vec<UserId>>,
    pub policies: Option<AccountPoliciesInput>,
    pub name: Option<String>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAddressBookEntryOperation {
    /// The address book entry id is only available after the operation is executed.
    pub address_book_entry_id: Option<AddressBookEntryId>,
    pub input: AddAddressBookEntryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAddressBookEntryOperationInput {
    pub address_owner: String,
    pub address: String,
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub metadata: Vec<MetadataItem>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAddressBookEntryOperation {
    pub input: EditAddressBookEntryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAddressBookEntryOperationInput {
    pub address_book_entry_id: AddressBookEntryId,
    pub address_owner: Option<String>,
    pub change_metadata: Option<ChangeMetadata>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAddressBookEntryOperation {
    pub input: RemoveAddressBookEntryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAddressBookEntryOperationInput {
    pub address_book_entry_id: AddressBookEntryId,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserOperation {
    pub user_id: Option<UUID>,
    pub input: AddUserOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserOperationInput {
    pub name: Option<String>,
    pub identities: Vec<Principal>,
    pub groups: Vec<UUID>,
    pub status: UserStatus,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperation {
    pub input: EditUserOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperationInput {
    pub user_id: UUID,
    pub name: Option<String>,
    pub identities: Option<Vec<Principal>>,
    pub groups: Option<Vec<UUID>>,
    pub status: Option<UserStatus>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserGroupOperation {
    pub user_group_id: Option<UUID>,
    pub input: AddUserGroupOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserGroupOperationInput {
    pub name: String,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserGroupOperation {
    pub input: EditUserGroupOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserGroupOperationInput {
    pub user_group_id: UUID,
    pub name: String,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveUserGroupOperation {
    pub input: RemoveUserGroupOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveUserGroupOperationInput {
    pub user_group_id: UUID,
}

#[storable]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChangeCanisterTarget {
    UpgradeWallet,
    UpgradeUpgrader,
    UpgradeCanister(Principal),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeCanisterOperationInput {
    pub target: ChangeCanisterTarget,
    pub module: Vec<u8>,
    pub arg: Option<Vec<u8>>,
    pub checksum: Vec<u8>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeCanisterOperation {
    pub arg_checksum: Option<Vec<u8>>,
    pub input: ChangeCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccessPolicyOperationInput {
    pub resource: Resource,
    pub auth_scope: Option<AuthScope>,
    pub users: Option<Vec<UserId>>,
    pub user_groups: Option<Vec<UserGroupId>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccessPolicyOperation {
    pub input: EditAccessPolicyOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddProposalPolicyOperationInput {
    pub specifier: ProposalSpecifier,
    pub criteria: Criteria,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddProposalPolicyOperation {
    pub policy_id: Option<UUID>,
    pub input: AddProposalPolicyOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditProposalPolicyOperationInput {
    pub policy_id: UUID,
    pub specifier: Option<ProposalSpecifier>,
    pub criteria: Option<Criteria>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditProposalPolicyOperation {
    pub input: EditProposalPolicyOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveProposalPolicyOperationInput {
    pub policy_id: UUID,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveProposalPolicyOperation {
    pub input: RemoveProposalPolicyOperationInput,
}
