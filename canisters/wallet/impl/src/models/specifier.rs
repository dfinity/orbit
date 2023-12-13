use super::{Account, Proposal, ProposalOperation, ProposalOperationType};
use crate::models::user::User;
use crate::repositories::ACCOUNT_REPOSITORY;
use crate::{errors::MatchError, repositories::USER_REPOSITORY};
use async_trait::async_trait;
use candid::{CandidType, Deserialize};
use ic_canister_core::{repository::Repository, types::UUID};
use ic_canister_macros::stable_object;
use std::sync::Arc;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommonSpecifier {
    Any,
    Group(Vec<UUID>),
    Id(Vec<UUID>),
}

pub type AccountSpecifier = CommonSpecifier;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AddressSpecifier {
    Any,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserSpecifier {
    Any,
    Group(Vec<UUID>),
    Id(Vec<UUID>),
    Owner,
    Proposer,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalSpecifier {
    AddAccount,
    AddUser,
    EditAccount(AccountSpecifier),
    EditUser(UserSpecifier),
    Transfer(AccountSpecifier, AddressSpecifier),
}

impl From<&ProposalSpecifier> for ProposalOperationType {
    fn from(value: &ProposalSpecifier) -> Self {
        match value {
            ProposalSpecifier::AddAccount => ProposalOperationType::AddAccount,
            ProposalSpecifier::AddUser => ProposalOperationType::AddUser,
            ProposalSpecifier::EditAccount(_) => ProposalOperationType::EditAccount,
            ProposalSpecifier::EditUser(_) => ProposalOperationType::EditUser,
            ProposalSpecifier::Transfer(_, _) => ProposalOperationType::Transfer,
        }
    }
}

#[async_trait]
pub trait Match<T>: Sync + Send {
    async fn is_match(&self, v: T) -> Result<bool, MatchError>;
}

#[derive(Clone)]
pub struct AccountMatcher;

#[async_trait]
impl Match<(Proposal, UUID, AccountSpecifier)> for AccountMatcher {
    async fn is_match(&self, v: (Proposal, UUID, AccountSpecifier)) -> Result<bool, MatchError> {
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
pub struct AddressMatcher;

#[async_trait]
impl Match<(Proposal, String, AddressSpecifier)> for AddressMatcher {
    async fn is_match(&self, v: (Proposal, String, AddressSpecifier)) -> Result<bool, MatchError> {
        let (_, _, s) = v;

        match s {
            AddressSpecifier::Any => Ok(true),
        }
    }
}

#[derive(Clone)]
pub struct UserMatcher;

pub type VoterId = UUID;

#[async_trait]
impl Match<(Proposal, VoterId, UserSpecifier)> for UserMatcher {
    async fn is_match(&self, v: (Proposal, VoterId, UserSpecifier)) -> Result<bool, MatchError> {
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
                match proposal.operation {
                    ProposalOperation::Transfer(operation) => {
                        if let Some(account) =
                            ACCOUNT_REPOSITORY.get(&Account::key(operation.input.from_account_id))
                        {
                            return Ok(account.owners.contains(&voter_id));
                        }
                    }
                    ProposalOperation::EditUser(operation) => {
                        return Ok(operation.input.user_id == voter_id);
                    }
                    ProposalOperation::EditAccount(operation) => {
                        if let Some(account) =
                            ACCOUNT_REPOSITORY.get(&Account::key(operation.input.account_id))
                        {
                            return Ok(account.owners.contains(&voter_id));
                        }
                    }
                    _ => {}
                };

                Ok(false)
            }
            UserSpecifier::Proposer => Ok(proposal.proposed_by == voter_id),
        }
    }
}

#[derive(Clone)]
pub struct ProposalMatcher {
    pub account_matcher: Arc<dyn Match<(Proposal, UUID, AccountSpecifier)>>,
    pub address_matcher: Arc<dyn Match<(Proposal, String, AddressSpecifier)>>,
    pub user_matcher: Arc<dyn Match<(Proposal, UUID, UserSpecifier)>>,
}

#[async_trait]
impl Match<(Proposal, ProposalSpecifier)> for ProposalMatcher {
    async fn is_match(&self, v: (Proposal, ProposalSpecifier)) -> Result<bool, MatchError> {
        let (p, s) = v;

        Ok(match (p.operation.to_owned(), s) {
            // AddAccount
            (ProposalOperation::AddAccount(_), ProposalSpecifier::AddAccount) => true,

            // AddUser
            (ProposalOperation::AddUser(_), ProposalSpecifier::AddUser) => true,

            // EditAccount
            (ProposalOperation::EditAccount(params), ProposalSpecifier::EditAccount(account)) => {
                self.account_matcher
                    .is_match((p, params.input.account_id, account))
                    .await?
            }

            // EditUser
            (ProposalOperation::EditUser(params), ProposalSpecifier::EditUser(user)) => {
                self.user_matcher
                    .is_match((p, params.input.user_id, user))
                    .await?
            }

            // Transfer
            (
                ProposalOperation::Transfer(params),
                ProposalSpecifier::Transfer(account, address),
            ) => vec![
                // Account
                self.account_matcher
                    .is_match((p.clone(), params.input.from_account_id, account))
                    .await?,
                // Address
                self.address_matcher
                    .is_match((p.clone(), params.input.to, address))
                    .await?,
            ]
            .into_iter()
            .all(|v| v),

            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        criteria::Criteria,
        proposal_test_utils::mock_proposal,
        specifier::{
            AccountMatcher, AccountSpecifier, AddressMatcher, AddressSpecifier, Match,
            ProposalMatcher, ProposalSpecifier, UserMatcher, UserSpecifier,
        },
        AccountPoliciesInput, AddAccountOperation, AddAccountOperationInput, AddUserOperation,
        AddUserOperationInput, Blockchain, EditAccountOperation, EditAccountOperationInput,
        EditUserOperation, EditUserOperationInput, ProposalOperation, TransferOperation,
        TransferOperationInput, UserStatus,
    };
    use anyhow::{anyhow, Error};
    use candid::Nat;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_proposal_matcher_empty_proposal() -> Result<(), Error> {
        let m = ProposalMatcher {
            account_matcher: Arc::new(AccountMatcher),
            address_matcher: Arc::new(AddressMatcher),
            user_matcher: Arc::new(UserMatcher),
        };

        let tcs = vec![
            (
                ProposalOperation::AddAccount(AddAccountOperation {
                    account_id: None,
                    input: AddAccountOperationInput {
                        name: "account-1".into(),
                        owners: vec![],
                        blockchain: Blockchain::InternetComputer,
                        standard: crate::models::BlockchainStandard::Native,
                        metadata: vec![],
                        policies: AccountPoliciesInput {
                            transfer: Some(Criteria::AutoAdopted),
                            edit: Some(Criteria::AutoAdopted),
                        },
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
                        unconfirmed_identities: vec![],
                    },
                }),
                ProposalSpecifier::AddUser,
            ),
            (
                ProposalOperation::EditAccount(EditAccountOperation {
                    input: EditAccountOperationInput {
                        account_id: [0; 16],
                        owners: None,
                        policies: None,
                        name: None,
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
                        unconfirmed_identities: None,
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
                        amount: Nat::from(1),
                        metadata: vec![],
                        network: "network-1".into(),
                        fee: None,
                    },
                }),
                ProposalSpecifier::Transfer(AccountSpecifier::Any, AddressSpecifier::Any),
            ),
        ];

        for tc in tcs {
            let mut proposal = mock_proposal();
            proposal.operation = tc.0;

            let specifier = tc.1;

            if !m.is_match((proposal, specifier)).await? {
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

            if !m.is_match((proposal, voter, specifier)).await? {
                return Err(anyhow!("expected true but got false"));
            };
        }

        Ok(())
    }
}
