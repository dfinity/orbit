use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    mappers::HelperMapper,
    models::{
        EditAccountOperation, EditAccountOperationInput, Proposal, ProposalExecutionPlan,
        ProposalOperation,
    },
    services::ACCOUNT_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct EditAccountProposalCreate {}

impl Create<wallet_api::EditAccountOperationInput> for EditAccountProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::EditAccountOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let from_account_id = HelperMapper::to_uuid(operation_input.account_id).map_err(|e| {
            ProposalError::ValidationError {
                info: format!("Invalid from_account_id: {}", e),
            }
        })?;

        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::EditAccount(EditAccountOperation {
                input: EditAccountOperationInput {
                    account_id: *from_account_id.as_bytes(),
                    owners: match operation_input.owners {
                        Some(owners) => Some(
                            owners
                                .into_iter()
                                .map(|owner| {
                                    HelperMapper::to_uuid(owner)
                                        .map_err(|e| ProposalError::ValidationError {
                                            info: format!("Invalid owner: {}", e),
                                        })
                                        .map(|uuid| *uuid.as_bytes())
                                })
                                .collect::<Result<Vec<UUID>, _>>()?,
                        ),
                        None => None,
                    },
                    policies: operation_input.policies.map(Into::into),
                    name: operation_input.name,
                },
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "Account edit".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct EditAccountProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditAccountOperation,
}

impl<'p, 'o> EditAccountProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditAccountOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for EditAccountProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        ACCOUNT_SERVICE
            .edit_account(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to update account: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.clone(),
        ))
    }
}
