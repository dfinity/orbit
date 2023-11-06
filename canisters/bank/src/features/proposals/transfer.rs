use crate::core::ic_cdk::api::time;
use crate::core::PostProcessor;
use crate::models::{Account, AccountId, ProposalOperation, ProposalVoteStatus};
use crate::repositories::AccountRepository;
use crate::{
    core::ic_cdk::api::trap,
    models::{Proposal, ProposalStatus, Transfer, TransferStatus},
    repositories::{ProposalRepository, TransferRepository},
};
use ic_canister_core::api::ApiError;
use ic_canister_core::repository::Repository;
use uuid::Uuid;

#[derive(Debug)]
pub struct TransferProposalProcessor<'proposal> {
    transfer_repository: TransferRepository,
    proposal_repository: ProposalRepository,
    account_repository: AccountRepository,
    proposal: &'proposal Proposal,
}

impl<'proposal> PostProcessor for TransferProposalProcessor<'proposal> {
    fn post_process(&mut self) -> Result<(), ApiError> {
        self.reevaluate_transfer(self.proposal)?;

        Ok(())
    }
}

impl<'proposal> TransferProposalProcessor<'proposal> {
    pub fn new(proposal: &'proposal Proposal) -> Self {
        Self {
            proposal,
            transfer_repository: TransferRepository::default(),
            proposal_repository: ProposalRepository::default(),
            account_repository: AccountRepository::default(),
        }
    }

    fn get_transfer(&self, proposal: &Proposal) -> Transfer {
        let ProposalOperation::Transfer(ctx) = &proposal.operation;

        self.transfer_repository
            .get(&Transfer::key(ctx.transfer_id))
            .unwrap_or_else(|| {
                trap(&format!(
                    "Transfer not found: {}",
                    Uuid::from_bytes(ctx.transfer_id).hyphenated()
                ))
            })
    }

    fn get_account(&self, account_id: &AccountId) -> Account {
        self.account_repository
            .get(&Account::key(*account_id))
            .unwrap_or_else(|| {
                trap(&format!(
                    "Account not found: {}",
                    Uuid::from_bytes(*account_id).hyphenated()
                ))
            })
    }

    fn reevaluate_transfer(&self, proposal: &Proposal) -> Result<(), ApiError> {
        let mut transfer = self.get_transfer(proposal);
        let account = &self.get_account(&transfer.from_account);

        let total_approvals = proposal
            .votes
            .iter()
            .filter(|vote| vote.status == ProposalVoteStatus::Adopted)
            .count();
        let missing_feedback = proposal
            .votes
            .iter()
            .filter(|vote| vote.status == ProposalVoteStatus::Pending)
            .count();

        let policy_requirements = transfer.policy_requirements(account);
        let is_approved = total_approvals >= policy_requirements.min_approvals as usize;
        let can_still_be_approved =
            total_approvals + missing_feedback >= policy_requirements.min_approvals as usize;

        if !can_still_be_approved || is_approved {
            transfer.status = match is_approved {
                true => TransferStatus::Approved,
                _ => TransferStatus::Rejected {
                    reason: "Not enough approvals".to_string(),
                },
            };

            transfer.last_modification_timestamp = time();
            self.transfer_repository
                .insert(transfer.to_key(), transfer.to_owned());

            let mut updated_proposal = proposal.to_owned();
            updated_proposal.status = match is_approved {
                true => ProposalStatus::Adopted,
                _ => ProposalStatus::Rejected,
            };

            updated_proposal.votes.iter_mut().for_each(|vote| {
                if vote.status == ProposalVoteStatus::Pending {
                    vote.status = ProposalVoteStatus::NotRequired;
                    vote.last_modification_timestamp = time();
                    vote.decided_dt = Some(time());
                }
            });
            self.proposal_repository
                .insert(updated_proposal.to_key(), updated_proposal.to_owned());
        }

        Ok(())
    }
}
