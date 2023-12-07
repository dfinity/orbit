use super::evaluation::Evaluate;
use crate::{
    errors::EvaluateError,
    models::{
        criteria::EvaluateCriteria,
        specifier::{Match, ProposalSpecifier},
        EvaluationStatus, Proposal,
    },
    repositories::policy::PROPOSAL_POLICY_REPOSITORY,
};
use anyhow::Context;
use async_trait::async_trait;
use ic_canister_core::repository::Repository;
use std::sync::Arc;

pub struct ProposalEvaluator {
    pub proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
    pub criteria_evaluator: Arc<dyn EvaluateCriteria>,
    pub proposal: Proposal,
}

impl ProposalEvaluator {
    pub fn new(
        proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
        criteria_evaluator: Arc<dyn EvaluateCriteria>,
        proposal: Proposal,
    ) -> Self {
        Self {
            proposal_matcher,
            criteria_evaluator,
            proposal,
        }
    }
}

#[async_trait]
impl Evaluate<EvaluationStatus> for ProposalEvaluator {
    async fn evaluate(&self) -> Result<EvaluationStatus, EvaluateError> {
        for plc in PROPOSAL_POLICY_REPOSITORY.list() {
            // Check if the proposal matches the policy
            if !self
                .proposal_matcher
                .is_match((self.proposal.to_owned(), plc.specifier.to_owned()))
                .await
                .context("failed to match proposal")?
            {
                continue;
            }

            // Evaluate the criteria
            let s = self
                .criteria_evaluator
                .evaluate(&self.proposal, &plc.criteria)
                .await
                .context("failed to evaluate criteria")?;

            match s {
                EvaluationStatus::Adopted | EvaluationStatus::Rejected => return Ok(s),
                _ => {}
            }
        }

        Ok(EvaluationStatus::Pending)
    }
}
