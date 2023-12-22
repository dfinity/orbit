use super::{Create, CreateHook, Execute, ProposalExecuteStage};
use crate::{
    core::{canister_config, write_canister_config, CanisterConfig},
    errors::{ProposalError, ProposalExecuteError},
    models::{Proposal, ProposalExecutionPlan, ProposalOperation, ChangeCanisterOperation, ChangeCanisterTarget},
    services::CHANGE_CANISTER_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use wallet_api::{CreateProposalInput, ChangeCanisterOperationInput};

pub struct ChangeCanisterProposalCreate;

impl Create<ChangeCanisterOperationInput> for ChangeCanisterProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: CreateProposalInput,
        operation_input: ChangeCanisterOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::ChangeCanister(ChangeCanisterOperation {
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "ChangeCanister".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct ChangeCanisterProposalCreateHook<'p, 'o> {
    _proposal: &'p Proposal,
    _operation: &'o ChangeCanisterOperation,
}

impl<'p, 'o> ChangeCanisterProposalCreateHook<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o ChangeCanisterOperation) -> Self {
        Self {
            _proposal: proposal,
            _operation: operation,
        }
    }
}

#[async_trait]
impl CreateHook for ChangeCanisterProposalCreateHook<'_, '_> {
    async fn on_created(&self) {}
}

pub struct ChangeCanisterProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o ChangeCanisterOperation,
}

impl<'p, 'o> ChangeCanisterProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o ChangeCanisterOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for ChangeCanisterProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        match self.operation.input.target {
            ChangeCanisterTarget::UpgradeWallet => {
                write_canister_config(CanisterConfig {
                    change_canister_proposal: Some(self.proposal.id.to_owned()),
                    ..canister_config()
                });

                let out = CHANGE_CANISTER_SERVICE
                    .upgrade_wallet(&self.operation.input.module, &self.operation.input.checksum)
                    .await
                    .map_err(|err| ProposalExecuteError::Failed {
                        reason: format!("failed to upgrade wallet: {}", err),
                    });

                if out.is_err() {
                    write_canister_config(CanisterConfig {
                        change_canister_proposal: None,
                        ..canister_config()
                    });
                }

                out?;

                Ok(ProposalExecuteStage::Processing(
                    self.proposal.operation.clone(),
                ))
            }

            ChangeCanisterTarget::UpgradeUpgrader => {
                CHANGE_CANISTER_SERVICE
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
