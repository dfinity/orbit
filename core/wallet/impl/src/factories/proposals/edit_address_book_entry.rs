use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    mappers::HelperMapper,
    models::{
        EditAddressBookEntryOperation, EditAddressBookEntryOperationInput, Proposal,
        ProposalExecutionPlan, ProposalOperation,
    },
    services::ADDRESS_BOOK_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct EditAddressBookEntryProposalCreate {}

impl Create<wallet_api::EditAddressBookEntryOperationInput> for EditAddressBookEntryProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: wallet_api::CreateProposalInput,
        operation_input: wallet_api::EditAddressBookEntryOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let address_book_entry_id = HelperMapper::to_uuid(operation_input.address_book_entry_id)
            .map_err(|e| ProposalError::ValidationError {
                info: format!("Invalid address book entry id: {}", e),
            })?;

        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::EditAddressBookEntry(EditAddressBookEntryOperation {
                input: EditAddressBookEntryOperationInput {
                    address_book_entry_id: *address_book_entry_id.as_bytes(),
                    address_owner: operation_input.address_owner,
                    change_metadata: operation_input.change_metadata.map(|m| m.into()),
                },
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Address book entry update".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct EditAddressBookEntryProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o EditAddressBookEntryOperation,
}

impl<'p, 'o> EditAddressBookEntryProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o EditAddressBookEntryOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for EditAddressBookEntryProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        ADDRESS_BOOK_SERVICE
            .edit_entry(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to update address book entry: {}", e),
            })?;

        Ok(ProposalExecuteStage::Completed(
            self.proposal.operation.clone(),
        ))
    }
}
