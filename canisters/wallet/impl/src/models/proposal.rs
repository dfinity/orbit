use super::criteria::ApprovalCriteriaInput;
use super::{
    DisplayUser, EvaluationStatus, ProposalOperation, ProposalStatus, ProposalVote,
    ProposalVoteStatus, UserId, UserKey,
};
use crate::core::evaluation::{
    Evaluate, CRITERIA_EVALUATOR, PROPOSAL_MATCHER, PROPOSAL_POSSIBLE_VOTERS_CRITERIA_EVALUATOR,
    PROPOSAL_VOTE_RIGHTS_CRITERIA_EVALUATOR,
};
use crate::core::ic_cdk::api::{print, time};
use crate::core::proposal::{
    ProposalEvaluator, ProposalPossibleVotersFinder, ProposalVoteRightsEvaluator,
};
use crate::core::validation::{
    EnsureAccount, EnsureAddressBookEntry, EnsureIdExists, EnsureProposalPolicy, EnsureUser,
    EnsureUserGroup,
};
use crate::errors::{EvaluateError, ProposalError, RecordValidationError};
use crate::repositories::USER_REPOSITORY;
use candid::{CandidType, Deserialize};
use ic_canister_core::repository::Repository;
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::storable;
use std::collections::HashSet;

/// The proposal id, which is a UUID.
pub type ProposalId = UUID;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalExecutionPlan {
    Immediate,
    Scheduled { execution_time: Timestamp },
}

/// Represents a proposal within the system.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Proposal {
    /// The proposal id, which is a UUID.
    pub id: ProposalId,
    /// The title of the proposal.
    pub title: String,
    /// The summary of the proposal, this is a longer description of the proposal.
    pub summary: Option<String>,
    /// The user id that resulted in the proposal creation.
    pub proposed_by: UserId,
    /// The status that the proposal is in.
    pub status: ProposalStatus,
    /// An operation that the proposal should execute, e.g. "transfer".
    pub operation: ProposalOperation,
    /// The expiration date of the proposal.
    pub expiration_dt: Timestamp,
    /// The execution plan of the proposal.
    pub execution_plan: ProposalExecutionPlan,
    /// The votes that the proposal has received.
    pub votes: Vec<ProposalVote>,
    /// The timestamp of the proposal creation.
    pub created_timestamp: Timestamp,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalKey {
    /// The proposal id, which is a UUID.
    pub id: ProposalId,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalCallerPrivileges {
    pub id: UUID,
    pub can_vote: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalAdditionalInfo {
    pub id: UUID,
    pub proposer_name: Option<String>,
    pub voters: Vec<DisplayUser>,
}

fn validate_title(title: &str) -> ModelValidatorResult<ProposalError> {
    if title.len() > Proposal::MAX_TITLE_LEN as usize {
        return Err(ProposalError::ValidationError {
            info: format!(
                "Proposal title length exceeds the maximum allowed: {}",
                Proposal::MAX_TITLE_LEN
            ),
        });
    }

    Ok(())
}

fn validate_summary(summary: &Option<String>) -> ModelValidatorResult<ProposalError> {
    if let Some(summary) = summary {
        if summary.len() > Proposal::MAX_SUMMARY_LEN as usize {
            return Err(ProposalError::ValidationError {
                info: format!(
                    "Proposal summary length exceeds the maximum allowed: {}",
                    Proposal::MAX_SUMMARY_LEN
                ),
            });
        }
    }

    Ok(())
}

fn validate_proposed_by(proposed_by: &UserId) -> ModelValidatorResult<ProposalError> {
    USER_REPOSITORY
        .get(&UserKey { id: *proposed_by })
        .ok_or(ProposalError::ValidationError {
            info: "The proposed_by user does not exist".to_owned(),
        })?;
    Ok(())
}

fn validate_proposal_operation_foreign_keys(
    operation: &ProposalOperation,
) -> ModelValidatorResult<RecordValidationError> {
    match operation {
        ProposalOperation::Transfer(op) => EnsureAccount::id_exists(&op.input.from_account_id),
        ProposalOperation::AddAccount(op) => {
            op.input.read_access_policy.validate()?;
            op.input.update_access_policy.validate()?;
            op.input.transfer_access_policy.validate()?;

            if let Some(criteria) = &op.input.transfer_approval_policy {
                criteria.validate()?;
            }

            if let Some(criteria) = &op.input.update_approval_policy {
                criteria.validate()?;
            }

            Ok(())
        }
        ProposalOperation::EditAccount(op) => {
            EnsureAccount::id_exists(&op.input.account_id)?;

            if let Some(allow) = &op.input.read_access_policy {
                allow.validate()?;
            }

            if let Some(allow) = &op.input.update_access_policy {
                allow.validate()?;
            }

            if let Some(allow) = &op.input.transfer_access_policy {
                allow.validate()?;
            }

            if let Some(ApprovalCriteriaInput::Set(criteria)) = &op.input.update_approval_policy {
                criteria.validate()?;
            }

            if let Some(ApprovalCriteriaInput::Set(criteria)) = &op.input.transfer_approval_policy {
                criteria.validate()?;
            }

            Ok(())
        }
        ProposalOperation::AddAddressBookEntry(_) => Ok(()),
        ProposalOperation::EditAddressBookEntry(op) => {
            EnsureAddressBookEntry::id_exists(&op.input.address_book_entry_id)
        }
        ProposalOperation::RemoveAddressBookEntry(op) => {
            EnsureAddressBookEntry::id_exists(&op.input.address_book_entry_id)
        }
        ProposalOperation::AddUser(op) => EnsureUserGroup::id_list_exists(&op.input.groups),
        ProposalOperation::EditUser(op) => {
            EnsureUser::id_exists(&op.input.user_id)?;

            if let Some(group_ids) = &op.input.groups {
                EnsureUserGroup::id_list_exists(group_ids)?;
            }

            Ok(())
        }
        ProposalOperation::EditAccessPolicy(op) => {
            op.input.resource.validate()?;

            if let Some(user_ids) = &op.input.users {
                EnsureUser::id_list_exists(user_ids)?;
            }

            if let Some(group_ids) = &op.input.user_groups {
                EnsureUserGroup::id_list_exists(group_ids)?;
            }

            Ok(())
        }
        ProposalOperation::AddUserGroup(_) => Ok(()),
        ProposalOperation::EditUserGroup(op) => EnsureUserGroup::id_exists(&op.input.user_group_id),
        ProposalOperation::RemoveUserGroup(ok) => {
            EnsureUserGroup::id_exists(&ok.input.user_group_id)
        }
        ProposalOperation::ChangeCanister(_) => Ok(()),
        ProposalOperation::AddProposalPolicy(op) => {
            op.input.specifier.validate()?;
            op.input.criteria.validate()?;

            Ok(())
        }
        ProposalOperation::EditProposalPolicy(op) => {
            EnsureProposalPolicy::id_exists(&op.input.policy_id)?;

            if let Some(specifier) = &op.input.specifier {
                specifier.validate()?;
            }

            if let Some(criteria) = &op.input.criteria {
                criteria.validate()?;
            }

            Ok(())
        }
        ProposalOperation::RemoveProposalPolicy(op) => {
            EnsureProposalPolicy::id_exists(&op.input.policy_id)
        }
    }
}

impl ModelValidator<ProposalError> for Proposal {
    fn validate(&self) -> ModelValidatorResult<ProposalError> {
        validate_title(&self.title)?;
        validate_summary(&self.summary)?;
        validate_proposed_by(&self.proposed_by)?;

        validate_proposal_operation_foreign_keys(&self.operation).map_err(|err| match err {
            RecordValidationError::NotFound { model_name, id } => ProposalError::ValidationError {
                info: format!(
                    "Invalid proposal operation: {} {} does not exist",
                    model_name, id
                ),
            },
        })?;

        Ok(())
    }
}

impl Proposal {
    pub const MAX_TITLE_LEN: u8 = 255;
    pub const MAX_SUMMARY_LEN: u16 = 1000;

    /// Creates a new proposal key from the given key components.
    pub fn key(proposal_id: ProposalId) -> ProposalKey {
        ProposalKey { id: proposal_id }
    }

    pub fn to_key(&self) -> ProposalKey {
        Proposal::key(self.id.to_owned())
    }

    pub fn voters(&self) -> HashSet<UserId> {
        let mut users = HashSet::new();

        self.votes
            .iter()
            .map(|decision| decision.user_id.to_owned())
            .for_each(|user_id| {
                users.insert(user_id);
            });

        users
    }

    /// Gives the default expiration date for a proposal which is 7 days from the current time.
    pub fn default_expiration_dt_ns() -> Timestamp {
        let time_in_ns: u64 = 7 * 24 * 60 * 60 * 1_000_000_000;

        time() + time_in_ns
    }

    pub async fn can_vote(&self, user_id: &UUID) -> bool {
        let validator = ProposalVoteRightsEvaluator {
            proposal_id: self.id,
            voter_id: *user_id,
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            vote_rights_evaluator: PROPOSAL_VOTE_RIGHTS_CRITERIA_EVALUATOR.clone(),
        };

        match validator.evaluate() {
            Ok(can_vote) => can_vote,
            Err(_) => {
                print(format!(
                    "Failed to evaluate voting rights for proposal: {:?}",
                    self
                ));

                false
            }
        }
    }

    pub fn add_vote(
        &mut self,
        user_id: UUID,
        vote: ProposalVoteStatus,
        reason: Option<String>,
    ) -> ModelValidatorResult<ProposalError> {
        if self.votes.iter().any(|vote| vote.user_id == user_id) {
            // users can only vote once per proposal
            return Err(ProposalError::VoteNotAllowed);
        }

        let vote = ProposalVote {
            user_id,
            status: vote,
            status_reason: reason,
            decided_dt: time(),
            last_modification_timestamp: time(),
        };

        vote.validate()?;

        self.votes.push(vote);

        Ok(())
    }

    pub async fn reevaluate(&mut self) -> Result<(), EvaluateError> {
        if let ProposalStatus::Created = self.status {
            let evaluator = ProposalEvaluator {
                proposal: self.to_owned(),
                proposal_matcher: PROPOSAL_MATCHER.to_owned(),
                criteria_evaluator: CRITERIA_EVALUATOR.to_owned(),
            };

            let evaluation_status = evaluator.evaluate()?;

            if evaluation_status == EvaluationStatus::Adopted {
                self.status = ProposalStatus::Adopted;
            } else if evaluation_status == EvaluationStatus::Rejected {
                self.status = ProposalStatus::Rejected;
            }
        }

        Ok(())
    }

    pub async fn find_all_possible_voters(&self) -> Result<HashSet<UUID>, EvaluateError> {
        let evaluator = ProposalPossibleVotersFinder {
            proposal: self,
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            possible_voters_criteria_evaluator: PROPOSAL_POSSIBLE_VOTERS_CRITERIA_EVALUATOR
                .to_owned(),
        };

        evaluator.evaluate()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::validation::disable_mock_resource_validation;
    use crate::models::access_policy::Allow;
    use crate::models::{
        AddAccountOperationInput, AddUserOperation, AddUserOperationInput, Metadata,
        TransferOperation, TransferOperationInput,
    };
    use crate::services::AccountService;

    use super::proposal_test_utils::mock_proposal;
    use super::*;

    #[test]
    fn fail_proposal_title_too_big() {
        let mut proposal = mock_proposal();
        proposal.title = "a".repeat(Proposal::MAX_TITLE_LEN as usize + 1);

        let result = validate_title(&proposal.title);

        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_title_is_valid() {
        let mut proposal = mock_proposal();
        proposal.title = "a".repeat(Proposal::MAX_TITLE_LEN as usize);

        let result = validate_title(&proposal.title);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_proposal_summary_too_big() {
        let mut proposal = mock_proposal();
        proposal.summary = Some("a".repeat(Proposal::MAX_SUMMARY_LEN as usize + 1));

        let result = validate_summary(&proposal.summary);

        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_summary_is_valid() {
        let mut proposal = mock_proposal();
        proposal.summary = Some("a".repeat(Proposal::MAX_SUMMARY_LEN as usize));

        let result = validate_summary(&proposal.summary);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_proposal_operation_is_valid() {
        disable_mock_resource_validation();

        let account_service = AccountService::default();
        let account = account_service
            .create_account(AddAccountOperationInput {
                name: "a".to_owned(),
                blockchain: crate::models::Blockchain::InternetComputer,
                standard: crate::models::BlockchainStandard::Native,
                metadata: Metadata::default(),
                read_access_policy: Allow::default(),
                update_access_policy: Allow::default(),
                transfer_access_policy: Allow::default(),
                update_approval_policy: None,
                transfer_approval_policy: None,
            })
            .await
            .expect("Failed to create account");

        let operation = ProposalOperation::Transfer(TransferOperation {
            transfer_id: None,
            input: TransferOperationInput {
                network: "mainnet".to_string(),
                amount: 1u64.into(),
                fee: None,
                metadata: Metadata::default(),
                to: "0x1234".to_string(),
                from_account_id: account.id,
            },
        });

        let result = validate_proposal_operation_foreign_keys(&operation);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn fail_proposal_operation_with_invalid_id() {
        disable_mock_resource_validation();

        validate_proposal_operation_foreign_keys(&ProposalOperation::Transfer(TransferOperation {
            transfer_id: None,
            input: TransferOperationInput {
                network: "mainnet".to_string(),
                amount: 1u64.into(),
                fee: None,
                metadata: Metadata::default(),
                to: "0x1234".to_string(),
                from_account_id: [0; 16],
            },
        }))
        .expect_err("Invalid account id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::AddUser(AddUserOperation {
            user_id: None,
            input: AddUserOperationInput {
                name: None,
                identities: vec![],
                groups: vec![[1; 16]],
                status: crate::models::UserStatus::Active,
            },
        }))
        .expect_err("Invalid user group id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::EditUserGroup(
            crate::models::EditUserGroupOperation {
                input: crate::models::EditUserGroupOperationInput {
                    user_group_id: [0; 16],
                    name: "a".to_owned(),
                },
            },
        ))
        .expect_err("Invalid user group id should fail");
        validate_proposal_operation_foreign_keys(&ProposalOperation::RemoveUserGroup(
            crate::models::RemoveUserGroupOperation {
                input: crate::models::RemoveUserGroupOperationInput {
                    user_group_id: [0; 16],
                },
            },
        ))
        .expect_err("Invalid user group id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::AddProposalPolicy(
            crate::models::AddProposalPolicyOperation {
                policy_id: None,
                input: crate::models::AddProposalPolicyOperationInput {
                    specifier: crate::models::specifier::ProposalSpecifier::EditUser(
                        crate::models::resource::ResourceIds::Ids(vec![[1; 16]]),
                    ),
                    criteria: crate::models::criteria::Criteria::AutoAdopted,
                },
            },
        ))
        .expect_err("Invalid proposal specifier should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::EditProposalPolicy(
            crate::models::EditProposalPolicyOperation {
                input: crate::models::EditProposalPolicyOperationInput {
                    policy_id: [0; 16],
                    specifier: None,
                    criteria: None,
                },
            },
        ))
        .expect_err("Invalid proposal policy id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::RemoveProposalPolicy(
            crate::models::RemoveProposalPolicyOperation {
                input: crate::models::RemoveProposalPolicyOperationInput { policy_id: [0; 16] },
            },
        ))
        .expect_err("Invalid proposal policy id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::AddAccount(
            crate::models::AddAccountOperation {
                account_id: None,
                input: crate::models::AddAccountOperationInput {
                    name: "a".to_owned(),
                    blockchain: crate::models::Blockchain::InternetComputer,
                    standard: crate::models::BlockchainStandard::Native,
                    metadata: Metadata::default(),
                    read_access_policy: Allow {
                        auth_scope: crate::models::access_policy::AuthScope::Restricted,
                        users: vec![[1; 16]],
                        user_groups: vec![],
                    },
                    update_access_policy: Allow::default(),
                    transfer_access_policy: Allow::default(),
                    update_approval_policy: None,
                    transfer_approval_policy: None,
                },
            },
        ))
        .expect_err("Invalid user id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::EditAccount(
            crate::models::EditAccountOperation {
                input: crate::models::EditAccountOperationInput {
                    account_id: [0; 16],
                    read_access_policy: None,
                    update_access_policy: None,
                    transfer_access_policy: None,
                    update_approval_policy: None,
                    transfer_approval_policy: None,
                    name: None,
                },
            },
        ))
        .expect_err("Invalid account id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::EditAddressBookEntry(
            crate::models::EditAddressBookEntryOperation {
                input: crate::models::EditAddressBookEntryOperationInput {
                    address_book_entry_id: [0; 16],
                    address_owner: None,
                    change_metadata: None,
                },
            },
        ))
        .expect_err("Invalid address book entry id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::RemoveAddressBookEntry(
            crate::models::RemoveAddressBookEntryOperation {
                input: crate::models::RemoveAddressBookEntryOperationInput {
                    address_book_entry_id: [0; 16],
                },
            },
        ))
        .expect_err("Invalid address book entry id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::EditUser(
            crate::models::EditUserOperation {
                input: crate::models::EditUserOperationInput {
                    user_id: [0; 16],
                    groups: None,
                    name: None,
                    identities: None,
                    status: None,
                },
            },
        ))
        .expect_err("Invalid user id should fail");

        validate_proposal_operation_foreign_keys(&ProposalOperation::EditAccessPolicy(
            crate::models::EditAccessPolicyOperation {
                input: crate::models::EditAccessPolicyOperationInput {
                    resource: crate::models::resource::Resource::Account(
                        crate::models::resource::AccountResourceAction::Read(
                            crate::models::resource::ResourceId::Id([0; 16]),
                        ),
                    ),
                    users: None,
                    user_groups: None,
                    auth_scope: None,
                },
            },
        ))
        .expect_err("Invalid resource id should fail");
    }
}

#[cfg(any(test, feature = "canbench"))]
pub mod proposal_test_utils {
    use super::*;
    use crate::models::{Metadata, ProposalVoteStatus, TransferOperation, TransferOperationInput};
    use num_bigint::BigUint;
    use uuid::Uuid;

    pub fn mock_proposal() -> Proposal {
        Proposal {
            id: *Uuid::new_v4().as_bytes(),
            title: "foo".to_string(),
            summary: Some("bar".to_string()),
            proposed_by: [1; 16],
            status: ProposalStatus::Adopted,
            expiration_dt: 100,
            execution_plan: ProposalExecutionPlan::Immediate,
            operation: ProposalOperation::Transfer(TransferOperation {
                transfer_id: None,
                input: TransferOperationInput {
                    network: "mainnet".to_string(),
                    amount: candid::Nat(BigUint::from(100u32)),
                    fee: None,
                    metadata: Metadata::default(),
                    to: "0x1234".to_string(),
                    from_account_id: [1; 16],
                },
            }),
            votes: vec![ProposalVote {
                user_id: [1; 16],
                status: ProposalVoteStatus::Accepted,
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
            }],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        }
    }
}
