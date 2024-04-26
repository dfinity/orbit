use super::{
    criteria::{ApprovalCriteriaInput, Criteria},
    permission::{Allow, AuthScope},
    resource::Resource,
    specifier::ProposalSpecifier,
    AccountId, AddressBookEntryId, Blockchain, BlockchainStandard, ChangeMetadata, MetadataItem,
    UserGroupId, UserId, UserStatus,
};
use crate::models::Metadata;
use candid::Principal;
use orbit_essentials::{storable, types::UUID};
use std::fmt::Display;

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
    EditPermission(EditPermissionOperation),
    AddUserGroup(AddUserGroupOperation),
    EditUserGroup(EditUserGroupOperation),
    RemoveUserGroup(RemoveUserGroupOperation),
    ChangeCanister(ChangeCanisterOperation),
    AddProposalPolicy(AddProposalPolicyOperation),
    EditProposalPolicy(EditProposalPolicyOperation),
    RemoveProposalPolicy(RemoveProposalPolicyOperation),
}

impl Display for ProposalOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalOperation::Transfer(_) => write!(f, "transfer"),
            ProposalOperation::AddAccount(_) => write!(f, "add_account"),
            ProposalOperation::EditAccount(_) => write!(f, "edit_account"),
            ProposalOperation::AddAddressBookEntry(_) => write!(f, "add_address_book_entry"),
            ProposalOperation::EditAddressBookEntry(_) => write!(f, "edit_address_book_entry"),
            ProposalOperation::RemoveAddressBookEntry(_) => write!(f, "remove_address_book_entry"),
            ProposalOperation::AddUser(_) => write!(f, "add_user"),
            ProposalOperation::EditUser(_) => write!(f, "edit_user"),
            ProposalOperation::EditPermission(_) => write!(f, "edit_permission"),
            ProposalOperation::AddUserGroup(_) => write!(f, "add_user_group"),
            ProposalOperation::EditUserGroup(_) => write!(f, "adit_user_group"),
            ProposalOperation::RemoveUserGroup(_) => write!(f, "remove_user_group"),
            ProposalOperation::ChangeCanister(_) => write!(f, "change_canister"),
            ProposalOperation::AddProposalPolicy(_) => write!(f, "add_proposal_policy"),
            ProposalOperation::EditProposalPolicy(_) => write!(f, "edit_proposal_policy"),
            ProposalOperation::RemoveProposalPolicy(_) => write!(f, "remove_proposal_policy"),
        }
    }
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
pub struct AddAccountOperation {
    /// The account id is only available after the operation is executed.
    pub account_id: Option<AccountId>,
    pub input: AddAccountOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperationInput {
    pub name: String,
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub metadata: Metadata,
    pub read_permission: Allow,
    pub update_permission: Allow,
    pub transfer_permission: Allow,
    pub update_approval_policy: Option<Criteria>,
    pub transfer_approval_policy: Option<Criteria>,
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
    pub name: Option<String>,
    pub read_permission: Option<Allow>,
    pub update_permission: Option<Allow>,
    pub transfer_permission: Option<Allow>,
    pub update_approval_policy: Option<ApprovalCriteriaInput>,
    pub transfer_approval_policy: Option<ApprovalCriteriaInput>,
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
    UpgradeStation,
    UpgradeUpgrader,
    UpgradeCanister(Principal),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeCanisterOperationInput {
    pub target: ChangeCanisterTarget,
    pub module: Vec<u8>,
    pub arg: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeCanisterOperation {
    pub module_checksum: Vec<u8>,
    pub arg_checksum: Option<Vec<u8>>,
    pub input: ChangeCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditPermissionOperationInput {
    pub resource: Resource,
    pub auth_scope: Option<AuthScope>,
    pub users: Option<Vec<UserId>>,
    pub user_groups: Option<Vec<UserGroupId>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditPermissionOperation {
    pub input: EditPermissionOperationInput,
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
