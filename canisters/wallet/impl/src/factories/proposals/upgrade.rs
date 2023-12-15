use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    core::{canister_config, write_canister_config, CanisterConfig},
    errors::{ProposalError, ProposalExecuteError},
    models::{Proposal, ProposalExecutionPlan, ProposalOperation, UpgradeOperation, UpgradeTarget},
    services::UPGRADE_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use wallet_api::{CreateProposalInput, UpgradeOperationInput};

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
