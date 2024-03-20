use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    core::{canister_config, write_canister_config, CanisterConfig},
    errors::{ProposalError, ProposalExecuteError},
    models::{
        ChangeCanisterOperation, ChangeCanisterTarget, Proposal, ProposalExecutionPlan,
        ProposalOperation,
    },
    services::CHANGE_CANISTER_SERVICE,
};
use async_trait::async_trait;
use candid::Encode;
use ic_canister_core::types::UUID;
use ic_cdk::api::management_canister::main::CanisterInstallMode;
use sha2::{Digest, Sha256};
use wallet_api::{ChangeCanisterOperationInput, CreateProposalInput};

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
                arg_checksum: operation_input.arg.as_ref().map(|arg| {
                    let mut hasher = Sha256::new();
                    hasher.update(arg);
                    hasher.finalize().to_vec()
                }),
                module_checksum: {
                    let mut hasher = Sha256::new();
                    hasher.update(&operation_input.module);
                    hasher.finalize().to_vec()
                },
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

                let default_arg = Encode!(&()).unwrap();
                let arg = self.operation.input.arg.as_ref().unwrap_or(&default_arg);
                let out = CHANGE_CANISTER_SERVICE
                    .upgrade_wallet(&self.operation.input.module, arg)
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
                    .upgrade_upgrader(
                        &self.operation.input.module,
                        self.operation.input.arg.clone(),
                    )
                    .await
                    .map_err(|err| ProposalExecuteError::Failed {
                        reason: format!("failed to upgrade upgrader: {}", err),
                    })?;

                Ok(ProposalExecuteStage::Completed(
                    self.proposal.operation.clone(),
                ))
            }

            ChangeCanisterTarget::UpgradeCanister(canister_id) => {
                CHANGE_CANISTER_SERVICE
                    .install_canister(
                        canister_id,
                        CanisterInstallMode::Upgrade,
                        &self.operation.input.module,
                        self.operation.input.arg.clone(),
                    )
                    .await
                    .map_err(|err| ProposalExecuteError::Failed {
                        reason: format!("failed to upgrade canister {}: {}", canister_id, err),
                    })?;

                Ok(ProposalExecuteStage::Completed(
                    self.proposal.operation.clone(),
                ))
            }
        }
    }
}
