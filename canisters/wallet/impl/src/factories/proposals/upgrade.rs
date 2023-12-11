use async_trait::async_trait;
use ic_canister_core::types::UUID;
use wallet_api::{CreateProposalInput, UpgradeOperationInput};

use crate::{
    core::{canister_config, canister_config_mut, write_canister_config, CanisterConfig},
    errors::{ProposalError, ProposalEvaluateError, ProposalExecuteError},
    models::{
        EvaluationStatus, Proposal, ProposalExecutionPlan, ProposalOperation, UpgradeOperation,
        UpgradeTarget,
    },
    services::UPGRADE_SERVICE,
};

use super::{Create, CreateHook, Evaluate, Execute, ProposalExecuteStage, Validate};

pub struct UpgradeProposalCreate;

impl Create<UpgradeOperationInput> for UpgradeProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: CreateProposalInput,
        operation_input: UpgradeOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::Upgrade(UpgradeOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "Upgrade".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct UpgradeProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o UpgradeOperation,
}

impl<'p, 'o> UpgradeProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o UpgradeOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for UpgradeProposalCreateHook<'_, '_> {
    async fn on_created(&self) {}
}

pub struct UpgradeProposalValidate<'p, 'o> {
    proposal: &'p Proposal,
    _operation: &'o UpgradeOperation,
}

impl<'p, 'o> UpgradeProposalValidate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o UpgradeOperation) -> Self {
        Self {
            proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Validate for UpgradeProposalValidate<'_, '_> {
    fn can_vote(&self, _user_id: &UUID) -> bool {
        false
    }

    fn can_view(&self, user_id: &UUID) -> bool {
        self.can_vote(user_id)
            || self.proposal.voters().contains(user_id)
            || self.proposal.proposed_by == *user_id
    }
}

pub struct UpgradeProposalEvaluate<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o UpgradeOperation,
}

impl<'p, 'o> UpgradeProposalEvaluate<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o UpgradeOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl Evaluate for UpgradeProposalEvaluate<'_, '_> {
    async fn evaluate(&self) -> Result<EvaluationStatus, ProposalEvaluateError> {
        Ok(EvaluationStatus::Adopted)
    }
}

pub struct UpgradeProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o UpgradeOperation,
}

impl<'p, 'o> UpgradeProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o UpgradeOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for UpgradeProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        match self.operation.input.target {
            UpgradeTarget::Wallet => {
                write_canister_config(CanisterConfig {
                    upgrade_proposal: Some(self.proposal.id.to_owned()),
                    ..canister_config()
                });

                let out = UPGRADE_SERVICE
                    .upgrade_wallet(&self.operation.input.module, &self.operation.input.checksum)
                    .await
                    .map_err(|err| ProposalExecuteError::Failed {
                        reason: format!("failed to upgrade wallet: {}", err),
                    });

                if out.is_err() {
                    write_canister_config(CanisterConfig {
                        upgrade_proposal: None,
                        ..canister_config()
                    });
                }

                out?;

                Ok(ProposalExecuteStage::Processing(
                    self.proposal.operation.clone(),
                ))
            }

            UpgradeTarget::Upgrader => {
                UPGRADE_SERVICE
                    .upgrade_upgrader(&self.operation.input.module)
                    .await
                    .map_err(|err| ProposalExecuteError::Failed {
                        reason: format!("failed to upgrade upgrader: {}", err),
                    })?;

                Ok(ProposalExecuteStage::Completed(
                    self.proposal.operation.clone(),
                ))
            }
        }
    }
}
