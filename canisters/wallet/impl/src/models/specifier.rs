use super::resource::{Resource, ResourceId, ResourceIds, UserResourceAction};
use super::{
    MetadataItem, Proposal, ProposalId, ProposalKey, ProposalOperation, ProposalOperationType,
};
use crate::errors::UserSpecifierError;
use crate::models::user::User;
use crate::repositories::{ADDRESS_BOOK_REPOSITORY, PROPOSAL_REPOSITORY};

use crate::core::validation::{
    EnsureAccount, EnsureAddressBookEntry, EnsureIdExists, EnsureProposalPolicy,
    EnsureResourceIdExists, EnsureUser, EnsureUserGroup, RecordNotFoundError,
};
use crate::services::ACCOUNT_SERVICE;
use crate::{errors::MatchError, repositories::USER_REPOSITORY};
use anyhow::anyhow;
use ic_canister_core::model::{ModelValidator, ModelValidatorResult};
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
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

impl ModelValidator<UserSpecifierError> for UserSpecifier {
    fn validate(&self) -> Result<(), UserSpecifierError> {
        match self {
            UserSpecifier::Any | UserSpecifier::Owner | UserSpecifier::Proposer => Ok(()),
            UserSpecifier::Group(group_ids) => {
                for group_id in group_ids {
                    EnsureUserGroup::id_exists(group_id)?;
                }
                Ok(())
            }
            UserSpecifier::Id(user_ids) => {
                for user_id in user_ids {
                    EnsureUser::id_exists(user_id)?;
                }
                Ok(())
            }
        }
    }
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
    EditAccount(ResourceIds),
    EditUser(ResourceIds),
    AddAddressBookEntry,
    EditAddressBookEntry(ResourceIds),
    RemoveAddressBookEntry(ResourceIds),
    Transfer(ResourceIds),
    ChangeCanister,
    EditAccessPolicy(ResourceSpecifier),
    AddProposalPolicy,
    EditProposalPolicy(ResourceIds),
    RemoveProposalPolicy(ResourceIds),
    AddUserGroup,
    EditUserGroup(ResourceIds),
    RemoveUserGroup(ResourceIds),
}

impl ModelValidator<RecordNotFoundError> for ProposalSpecifier {
    fn validate(&self) -> ModelValidatorResult<RecordNotFoundError> {
        match self {
            ProposalSpecifier::AddAccount
            | ProposalSpecifier::AddUser
            | ProposalSpecifier::AddAddressBookEntry
            | ProposalSpecifier::ChangeCanister
            | ProposalSpecifier::AddProposalPolicy
            | ProposalSpecifier::AddUserGroup => Ok(()),

            ProposalSpecifier::Transfer(resource_ids)
            | ProposalSpecifier::EditAccount(resource_ids) => {
                EnsureAccount::resource_ids_exist(resource_ids)
            }
            ProposalSpecifier::EditUser(resource_ids) => {
                EnsureUser::resource_ids_exist(resource_ids)
            }
            ProposalSpecifier::RemoveAddressBookEntry(resource_ids)
            | ProposalSpecifier::EditAddressBookEntry(resource_ids) => {
                EnsureAddressBookEntry::resource_ids_exist(resource_ids)
            }
            ProposalSpecifier::EditAccessPolicy(resource_specifier) => match resource_specifier {
                ResourceSpecifier::Any => Ok(()),
                ResourceSpecifier::Resource(resource) => resource.validate(),
            },

            ProposalSpecifier::EditProposalPolicy(resource_ids)
            | ProposalSpecifier::RemoveProposalPolicy(resource_ids) => {
                EnsureProposalPolicy::resource_ids_exist(resource_ids)
            }
            ProposalSpecifier::EditUserGroup(resource_ids)
            | ProposalSpecifier::RemoveUserGroup(resource_ids) => {
                EnsureUserGroup::resource_ids_exist(resource_ids)
            }
        }
    }
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

impl Match<(Proposal, UUID, ResourceIds)> for AccountMatcher {
    fn is_match(&self, v: (Proposal, UUID, ResourceIds)) -> Result<bool, MatchError> {
        let (_, account_id, specifier) = v;

        match specifier {
            ResourceIds::Any => Ok(true),
            ResourceIds::Ids(ids) => Ok(ids.contains(&account_id)),
        }
    }
}

#[derive(Clone)]
pub struct CommonIdMatcher;

impl Match<(Proposal, UUID, ResourceIds)> for CommonIdMatcher {
    fn is_match(&self, v: (Proposal, UUID, ResourceIds)) -> Result<bool, MatchError> {
        let (_, entity_id, specifier) = v;

        match specifier {
            ResourceIds::Any => Ok(true),
            ResourceIds::Ids(ids) => Ok(ids.contains(&entity_id)),
        }
    }
}

#[derive(Clone)]
pub struct UserMatcher;

pub struct UserInvolvedInCriteriaForProposalResource {
    pub proposal_operation_resources: Vec<Resource>,
    pub policy_criteria_user_specifier: UserSpecifier,
    pub user_id: UUID,
    pub proposal_id: ProposalId,
}

impl Match<UserInvolvedInCriteriaForProposalResource> for UserMatcher {
    fn is_match(
        &self,
        input: UserInvolvedInCriteriaForProposalResource,
    ) -> Result<bool, MatchError> {
        match input.policy_criteria_user_specifier {
            UserSpecifier::Any => Ok(true),
            UserSpecifier::Group(ids) => {
                if let Some(user) = USER_REPOSITORY.get(&User::key(input.user_id)) {
                    return Ok(user.groups.iter().any(|g| ids.contains(g)));
                }

                Ok(false)
            }
            UserSpecifier::Id(ids) => Ok(ids.contains(&input.user_id)),
            UserSpecifier::Owner => {
                for resource in input.proposal_operation_resources {
                    let is_match = match resource {
                        Resource::User(UserResourceAction::Update(user_resource)) => {
                            match user_resource {
                                ResourceId::Any => false, // not a real match
                                ResourceId::Id(edit_user_id) => edit_user_id == input.user_id,
                            }
                        }
                        _ => false,
                    };

                    if is_match {
                        return Ok(true);
                    }
                }

                Ok(false)
            }
            UserSpecifier::Proposer => {
                if let Some(proposal) = PROPOSAL_REPOSITORY.get(&ProposalKey {
                    id: input.proposal_id,
                }) {
                    Ok(proposal.proposed_by == input.user_id)
                } else {
                    Err(MatchError::UnexpectedError(anyhow!("Proposal not found")))
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct ProposalMatcher {
    pub account_matcher: Arc<dyn Match<(Proposal, UUID, ResourceIds)>>,
    pub user_matcher: Arc<dyn Match<UserInvolvedInCriteriaForProposalResource>>,
    pub common_id_matcher: Arc<dyn Match<(Proposal, UUID, ResourceIds)>>,
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
                .is_match(UserInvolvedInCriteriaForProposalResource {
                    proposal_operation_resources: p.operation.to_resources(),
                    policy_criteria_user_specifier: match user {
                        ResourceIds::Any => UserSpecifier::Any,
                        ResourceIds::Ids(ids) => UserSpecifier::Id(ids),
                    },
                    user_id: params.input.user_id,
                    proposal_id: p.id,
                })?,
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

    use crate::{
        core::validation::disable_mock_resource_validation,
        models::{
            access_policy::Allow,
            criteria::Criteria,
            proposal_test_utils::mock_proposal,
            resource::ResourceIds,
            specifier::{
                AccountMatcher, Match, ProposalMatcher, ProposalSpecifier,
                UserInvolvedInCriteriaForProposalResource, UserMatcher, UserSpecifier,
            },
            AddAccountOperation, AddAccountOperationInput, AddUserOperation, AddUserOperationInput,
            Blockchain, EditAccountOperation, EditAccountOperationInput, EditUserOperation,
            EditUserOperationInput, Metadata, ProposalKey, ProposalOperation, TransferOperation,
            TransferOperationInput, UserStatus,
        },
        repositories::PROPOSAL_REPOSITORY,
    };
    use anyhow::{anyhow, Error};
    use candid::Nat;
    use ic_canister_core::{model::ModelValidator, repository::Repository};
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
                ProposalSpecifier::EditAccount(ResourceIds::Any),
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
                ProposalSpecifier::EditUser(ResourceIds::Any),
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
                ProposalSpecifier::Transfer(ResourceIds::Any),
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
    async fn test_user_matcher() {
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
            PROPOSAL_REPOSITORY.insert(ProposalKey { id: proposal.id }, proposal.clone());

            let voter = tc.1;
            let specifier = tc.2;

            assert!(m
                .is_match(UserInvolvedInCriteriaForProposalResource {
                    proposal_operation_resources: proposal.operation.to_resources(),
                    policy_criteria_user_specifier: specifier,
                    user_id: voter,
                    proposal_id: proposal.id,
                })
                .expect("Could not test user matcher"));
        }
    }

    #[test]
    fn test_valid_user_specifier() {
        disable_mock_resource_validation();

        UserSpecifier::Any.validate().expect("Any should be valid");
        UserSpecifier::Owner
            .validate()
            .expect("Owner should be valid");
        UserSpecifier::Proposer
            .validate()
            .expect("Proposer should be valid");
    }

    #[test]
    fn fail_invalid_user_specifier() {
        disable_mock_resource_validation();

        UserSpecifier::Id(vec![[0; 16]])
            .validate()
            .expect_err("Non existent user ID should be invalid");
        UserSpecifier::Group(vec![[0; 16]])
            .validate()
            .expect_err("Non existent group ID should be invalid");
    }

    #[test]
    fn test_valid_proposal_specifier() {
        disable_mock_resource_validation();

        ProposalSpecifier::AddAccount
            .validate()
            .expect("AddAccount should be valid");
        ProposalSpecifier::AddUser
            .validate()
            .expect("AddUser should be valid");
        ProposalSpecifier::AddAddressBookEntry
            .validate()
            .expect("AddAddressBookEntry should be valid");
        ProposalSpecifier::ChangeCanister
            .validate()
            .expect("ChangeCanister should be valid");
        ProposalSpecifier::AddProposalPolicy
            .validate()
            .expect("AddProposalPolicy should be valid");
        ProposalSpecifier::AddUserGroup
            .validate()
            .expect("AddUserGroup should be valid");
    }

    #[test]
    fn fail_invalid_proposal_specifier() {
        disable_mock_resource_validation();

        ProposalSpecifier::Transfer(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent account ID should be invalid");
        ProposalSpecifier::EditAccount(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent account ID should be invalid");
        ProposalSpecifier::EditUser(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent user ID should be invalid");
        ProposalSpecifier::EditAddressBookEntry(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent address book entry ID should be invalid");
        ProposalSpecifier::RemoveAddressBookEntry(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent address book entry ID should be invalid");
        ProposalSpecifier::EditProposalPolicy(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent proposal policy ID should be invalid");
        ProposalSpecifier::RemoveProposalPolicy(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent proposal policy ID should be invalid");
        ProposalSpecifier::EditUserGroup(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent user group ID should be invalid");
        ProposalSpecifier::RemoveUserGroup(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent user group ID should be invalid");
    }
}
