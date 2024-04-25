use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    mappers::HelperMapper,
    models::{
        Proposal, ProposalExecutionPlan, ProposalOperation, RemoveAddressBookEntryOperation,
        RemoveAddressBookEntryOperationInput,
    },
    services::ADDRESS_BOOK_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct RemoveAddressBookEntryProposalCreate {}

impl Create<wallet_api::RemoveAddressBookEntryOperationInput>
    for RemoveAddressBookEntryProposalCreate
{
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::RemoveAddressBookEntryOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let address_book_entry_id = HelperMapper::to_uuid(operation_input.address_book_entry_id)
            .map_err(|e| ProposalError::ValidationError {
                info: format!("Invalid address book entry id: {}", e),
            })?;

        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::RemoveAddressBookEntry(RemoveAddressBookEntryOperation {
                input: RemoveAddressBookEntryOperationInput {
                    address_book_entry_id: *address_book_entry_id.as_bytes(),
                },
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Address book entry removal".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct RemoveAddressBookEntryProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o RemoveAddressBookEntryOperation,
}

impl<'p, 'o> RemoveAddressBookEntryProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o RemoveAddressBookEntryOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for RemoveAddressBookEntryProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        ADDRESS_BOOK_SERVICE
            .remove_entry(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to remove address book entry: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.clone(),
        ))
    }
}
