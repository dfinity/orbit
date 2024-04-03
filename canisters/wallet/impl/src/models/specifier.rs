use super::resource::Resource;
use super::{MetadataItem, Proposal, ProposalOperation, ProposalOperationType};
use crate::models::user::User;
use crate::repositories::ADDRESS_BOOK_REPOSITORY;
use crate::services::ACCOUNT_SERVICE;
use crate::{errors::MatchError, repositories::USER_REPOSITORY};
use ic_canister_core::{repository::Repository, types::UUID};
use ic_canister_macros::storable;
use std::sync::Arc;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommonSpecifier {
    Any,
    Group(Vec<UUID>),
    Id(Vec<UUID>),
}

pub type AccountSpecifier = CommonSpecifier;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserSpecifier {
    Any,
    Group(Vec<UUID>),
    Id(Vec<UUID>),
    Owner,
    Proposer,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceSpecifier {
    Any,
    Resource(Resource),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalSpecifier {
    AddAccount,
    AddUser,
    EditAccount(AccountSpecifier),
    EditUser(UserSpecifier),
    AddAddressBookEntry,
    EditAddressBookEntry(CommonSpecifier),
    RemoveAddressBookEntry(CommonSpecifier),
    Transfer(AccountSpecifier),
    ChangeCanister,
    EditAccessPolicy(ResourceSpecifier),
    AddProposalPolicy,
    EditProposalPolicy(CommonSpecifier),
    RemoveProposalPolicy(CommonSpecifier),
    AddUserGroup,
    EditUserGroup(CommonSpecifier),
    RemoveUserGroup(CommonSpecifier),
}

impl From<&ProposalSpecifier> for ProposalOperationType {
    fn from(value: &ProposalSpecifier) -> Self {
        match value {
            ProposalSpecifier::AddAccount => ProposalOperationType::AddAccount,
            ProposalSpecifier::AddUser => ProposalOperationType::AddUser,
            ProposalSpecifier::EditAccount(_) => ProposalOperationType::EditAccount,
            ProposalSpecifier::EditUser(_) => ProposalOperationType::EditUser,
            ProposalSpecifier::AddAddressBookEntry => ProposalOperationType::AddAddressBookEntry,
            ProposalSpecifier::EditAddressBookEntry(_) => {
                ProposalOperationType::EditAddressBookEntry
            }
            ProposalSpecifier::RemoveAddressBookEntry(_) => {
                ProposalOperationType::RemoveAddressBookEntry
            }
            ProposalSpecifier::Transfer(_) => ProposalOperationType::Transfer,
            ProposalSpecifier::EditAccessPolicy(_) => ProposalOperationType::EditAccessPolicy,
            ProposalSpecifier::ChangeCanister => ProposalOperationType::ChangeCanister,
            ProposalSpecifier::AddProposalPolicy => ProposalOperationType::AddProposalPolicy,
            ProposalSpecifier::EditProposalPolicy(_) => ProposalOperationType::EditProposalPolicy,
            ProposalSpecifier::RemoveProposalPolicy(_) => {
                ProposalOperationType::RemoveProposalPolicy
            }
            ProposalSpecifier::AddUserGroup => ProposalOperationType::AddUserGroup,
            ProposalSpecifier::EditUserGroup(_) => ProposalOperationType::EditUserGroup,
            ProposalSpecifier::RemoveUserGroup(_) => ProposalOperationType::RemoveUserGroup,
        }
    }
}

pub trait Match<T>: Sync + Send {
    fn is_match(&self, v: T) -> Result<bool, MatchError>;
}

#[derive(Clone)]
pub struct AccountMatcher;

impl Match<(Proposal, UUID, AccountSpecifier)> for AccountMatcher {
    fn is_match(&self, v: (Proposal, UUID, AccountSpecifier)) -> Result<bool, MatchError> {
        let (_, account_id, specifier) = v;

        match specifier {
            AccountSpecifier::Any => Ok(true),
            // TODO: Add once account groups are implemented
            AccountSpecifier::Group(_ids) => todo!(),
            AccountSpecifier::Id(ids) => Ok(ids.contains(&account_id)),
        }
    }
}

#[derive(Clone)]
pub struct CommonIdMatcher;

impl Match<(Proposal, UUID, CommonSpecifier)> for CommonIdMatcher {
    fn is_match(&self, v: (Proposal, UUID, CommonSpecifier)) -> Result<bool, MatchError> {
        let (_, entity_id, specifier) = v;

        match specifier {
            CommonSpecifier::Any => Ok(true),
            CommonSpecifier::Id(ids) => Ok(ids.contains(&entity_id)),
            CommonSpecifier::Group(_) => {
                // Common id matcher does not support groups
                Ok(false)
            }
        }
    }
}

#[derive(Clone)]
pub struct UserMatcher;

pub type VoterId = UUID;
pub type ProposalHasVoterInUserSpecifier = (Proposal, VoterId, UserSpecifier);

impl Match<ProposalHasVoterInUserSpecifier> for UserMatcher {
    fn is_match(&self, v: (Proposal, VoterId, UserSpecifier)) -> Result<bool, MatchError> {
        let (proposal, voter_id, specifier) = v;

        match specifier {
            UserSpecifier::Any => Ok(true),
            UserSpecifier::Group(ids) => {
                if let Some(user) = USER_REPOSITORY.get(&User::key(voter_id)) {
                    return Ok(user.groups.iter().any(|g| ids.contains(g)));
                }

                Ok(false)
            }
            UserSpecifier::Id(ids) => Ok(ids.contains(&voter_id)),
            UserSpecifier::Owner => {
                if let ProposalOperation::EditUser(operation) = proposal.operation {
                    return Ok(operation.input.user_id == voter_id);
                }

                Ok(false)
            }
            UserSpecifier::Proposer => Ok(proposal.proposed_by == voter_id),
        }
    }
}

#[derive(Clone)]
pub struct ProposalMatcher {
    pub account_matcher: Arc<dyn Match<(Proposal, UUID, AccountSpecifier)>>,
    pub user_matcher: Arc<dyn Match<ProposalHasVoterInUserSpecifier>>,
    pub common_id_matcher: Arc<dyn Match<(Proposal, UUID, CommonSpecifier)>>,
}

impl Match<(Proposal, ProposalSpecifier)> for ProposalMatcher {
    fn is_match(&self, v: (Proposal, ProposalSpecifier)) -> Result<bool, MatchError> {
        let (p, s) = v;

        Ok(match (p.operation.to_owned(), s.to_owned()) {
            (ProposalOperation::AddAccount(_), ProposalSpecifier::AddAccount) => true,
            (ProposalOperation::AddUser(_), ProposalSpecifier::AddUser) => true,
            (ProposalOperation::EditAccount(params), ProposalSpecifier::EditAccount(account)) => {
                self.account_matcher
                    .is_match((p, params.input.account_id, account))?
            }
            (ProposalOperation::EditUser(params), ProposalSpecifier::EditUser(user)) => self
                .user_matcher
                .is_match((p, params.input.user_id, user))?,
            (ProposalOperation::AddAddressBookEntry(_), ProposalSpecifier::AddAddressBookEntry) => {
                true
            }
            (
                ProposalOperation::EditAddressBookEntry(params),
                ProposalSpecifier::EditAddressBookEntry(address_book_entry),
            ) => self.common_id_matcher.is_match((
                p,
                params.input.address_book_entry_id,
                address_book_entry,
            ))?,
            (
                ProposalOperation::RemoveAddressBookEntry(params),
                ProposalSpecifier::RemoveAddressBookEntry(address_book_entry),
            ) => self.common_id_matcher.is_match((
                p,
                params.input.address_book_entry_id,
                address_book_entry,
            ))?,
            (ProposalOperation::Transfer(params), ProposalSpecifier::Transfer(account)) => self
                .account_matcher
                .is_match((p.clone(), params.input.from_account_id, account))?,
            (ProposalOperation::ChangeCanister(_), ProposalSpecifier::ChangeCanister) => true,
            (ProposalOperation::AddUserGroup(_), ProposalSpecifier::AddUserGroup) => true,
            (
                ProposalOperation::EditAccessPolicy(operation),
                ProposalSpecifier::EditAccessPolicy(specifier),
            ) => match specifier {
                ResourceSpecifier::Any => true,
                ResourceSpecifier::Resource(resource) => resource == operation.input.resource,
            },
            (ProposalOperation::AddProposalPolicy(_), ProposalSpecifier::AddProposalPolicy) => true,
            (
                ProposalOperation::EditProposalPolicy(operation),
                ProposalSpecifier::EditProposalPolicy(specifier),
            ) => self
                .common_id_matcher
                .is_match((p, operation.input.policy_id, specifier))?,
            (
                ProposalOperation::RemoveProposalPolicy(operation),
                ProposalSpecifier::RemoveProposalPolicy(specifier),
            ) => self
                .common_id_matcher
                .is_match((p, operation.input.policy_id, specifier))?,
            (
                ProposalOperation::EditUserGroup(operation),
                ProposalSpecifier::EditUserGroup(specifier),
            ) => self
                .common_id_matcher
                .is_match((p, operation.input.user_group_id, specifier))?,
            (
                ProposalOperation::RemoveUserGroup(operation),
                ProposalSpecifier::RemoveUserGroup(specifier),
            ) => self
                .common_id_matcher
                .is_match((p, operation.input.user_group_id, specifier))?,
            // this is here to make sure that new operations are not added without updating this
            (ProposalOperation::AddAccount(_), _)
            | (ProposalOperation::AddUser(_), _)
            | (ProposalOperation::EditAccount(_), _)
            | (ProposalOperation::EditUser(_), _)
            | (ProposalOperation::AddAddressBookEntry(_), _)
            | (ProposalOperation::EditAddressBookEntry(_), _)
            | (ProposalOperation::RemoveAddressBookEntry(_), _)
            | (ProposalOperation::ChangeCanister(_), _)
            | (ProposalOperation::AddProposalPolicy(_), _)
            | (ProposalOperation::EditProposalPolicy(_), _)
            | (ProposalOperation::EditAccessPolicy(_), _)
            | (ProposalOperation::EditUserGroup(_), _)
            | (ProposalOperation::RemoveUserGroup(_), _)
            | (ProposalOperation::RemoveProposalPolicy(_), _)
            | (ProposalOperation::AddUserGroup(_), _)
            | (ProposalOperation::Transfer(_), _) => false,
        })
    }
}

#[derive(Clone)]
pub struct AddressBookMetadataMatcher;

pub type ProposalHasMetadata = (Proposal, MetadataItem);

impl Match<ProposalHasMetadata> for AddressBookMetadataMatcher {
    fn is_match(&self, v: ProposalHasMetadata) -> Result<bool, MatchError> {
        let (proposal, metadata) = v;

        Ok(match proposal.operation.to_owned() {
            ProposalOperation::Transfer(transfer) => {
                if let Ok(account) = ACCOUNT_SERVICE.get_account(&transfer.input.from_account_id) {
                    if let Some(address_book_entry) = ADDRESS_BOOK_REPOSITORY.find_by_address(
                        account.blockchain,
                        account.standard,
                        transfer.input.to,
                    ) {
                        address_book_entry.metadata.contains(&metadata)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        access_policy::Allow,
        criteria::Criteria,
        proposal_test_utils::mock_proposal,
        specifier::{
            AccountMatcher, AccountSpecifier, Match, ProposalMatcher, ProposalSpecifier,
            UserMatcher, UserSpecifier,
        },
        AddAccountOperation, AddAccountOperationInput, AddUserOperation, AddUserOperationInput,
        Blockchain, EditAccountOperation, EditAccountOperationInput, EditUserOperation,
        EditUserOperationInput, Metadata, ProposalOperation, TransferOperation,
        TransferOperationInput, UserStatus,
    };
    use anyhow::{anyhow, Error};
    use candid::Nat;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_proposal_matcher_empty_proposal() -> Result<(), Error> {
        let m = ProposalMatcher {
            account_matcher: Arc::new(AccountMatcher),
            user_matcher: Arc::new(UserMatcher),
            common_id_matcher: Arc::new(AccountMatcher),
        };

        let tcs = vec![
            (
                ProposalOperation::AddAccount(AddAccountOperation {
                    account_id: None,
                    input: AddAccountOperationInput {
                        name: "account-1".into(),
                        blockchain: Blockchain::InternetComputer,
                        standard: crate::models::BlockchainStandard::Native,
                        metadata: Metadata::default(),
                        transfer_approval_policy: Some(Criteria::AutoAdopted),
                        update_approval_policy: Some(Criteria::AutoAdopted),
                        read_access_policy: Allow::authenticated(),
                        update_access_policy: Allow::authenticated(),
                        transfer_access_policy: Allow::authenticated(),
                    },
                }),
                ProposalSpecifier::AddAccount,
            ),
            (
                ProposalOperation::AddUser(AddUserOperation {
                    user_id: None,
                    input: AddUserOperationInput {
                        name: None,
                        identities: vec![],
                        groups: vec![],
                        status: UserStatus::Active,
                    },
                }),
                ProposalSpecifier::AddUser,
            ),
            (
                ProposalOperation::EditAccount(EditAccountOperation {
                    input: EditAccountOperationInput {
                        account_id: [0; 16],
                        name: None,
                        read_access_policy: None,
                        update_access_policy: None,
                        transfer_access_policy: None,
                        transfer_approval_policy: None,
                        update_approval_policy: None,
                    },
                }),
                ProposalSpecifier::EditAccount(AccountSpecifier::Any),
            ),
            (
                ProposalOperation::EditUser(EditUserOperation {
                    input: EditUserOperationInput {
                        user_id: [0; 16],
                        name: None,
                        identities: None,
                        groups: None,
                        status: None,
                    },
                }),
                ProposalSpecifier::EditUser(UserSpecifier::Any),
            ),
            (
                ProposalOperation::Transfer(TransferOperation {
                    transfer_id: None,
                    input: TransferOperationInput {
                        from_account_id: [0; 16],
                        to: "address-1".into(),
                        amount: Nat::from(1_u64),
                        metadata: Metadata::default(),
                        network: "network-1".into(),
                        fee: None,
                    },
                }),
                ProposalSpecifier::Transfer(AccountSpecifier::Any),
            ),
        ];

        for tc in tcs {
            let mut proposal = mock_proposal();
            proposal.operation = tc.0;

            let specifier = tc.1;

            if !m.is_match((proposal, specifier))? {
                return Err(anyhow!("expected true but got false"));
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_user_matcher() -> Result<(), Error> {
        let m = UserMatcher;

        let tcs = vec![
            (
                [0; 16],            // proposer
                [1; 16],            // voter
                UserSpecifier::Any, // specifier
            ),
            (
                [0; 16],                          // proposer
                [1; 16],                          // voter
                UserSpecifier::Id(vec![[1; 16]]), // specifier
            ),
            (
                [0; 16],                 // proposer
                [0; 16],                 // voter
                UserSpecifier::Proposer, // specifier
            ),
        ];

        for tc in tcs {
            let mut proposal = mock_proposal();
            proposal.proposed_by = tc.0;

            let voter = tc.1;
            let specifier = tc.2;

            if !m.is_match((proposal, voter, specifier))? {
                return Err(anyhow!("expected true but got false"));
            };
        }

        Ok(())
    }
}
