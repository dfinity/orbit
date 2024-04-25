use super::{Create, Execute, ProposalExecuteStage};
use crate::{
    errors::{ProposalError, ProposalExecuteError},
    models::{AddAddressBookEntryOperation, Proposal, ProposalExecutionPlan, ProposalOperation},
    services::ADDRESS_BOOK_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;

pub struct AddAddressBookEntryProposalCreate {}

impl Create<station_api::AddAddressBookEntryOperationInput> for AddAddressBookEntryProposalCreate {
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: station_api::CreateProposalInput,
        operation_input: station_api::AddAddressBookEntryOperationInput,
    ) -> Result<Proposal, ProposalError> {
        let proposal = Proposal::new(
            proposal_id,
            proposed_by_user,
            Proposal::default_expiration_dt_ns(),
            ProposalOperation::AddAddressBookEntry(AddAddressBookEntryOperation {
                address_book_entry_id: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(ProposalExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Address book entry creation".to_string()),
            input.summary,
        );

        Ok(proposal)
    }
}

pub struct AddAddressBookEntryProposalExecute<'p, 'o> {
    proposal: &'p Proposal,
    operation: &'o AddAddressBookEntryOperation,
}

impl<'p, 'o> AddAddressBookEntryProposalExecute<'p, 'o> {
    pub fn new(proposal: &'p Proposal, operation: &'o AddAddressBookEntryOperation) -> Self {
        Self {
            proposal,
            operation,
        }
    }
}

#[async_trait]
impl Execute for AddAddressBookEntryProposalExecute<'_, '_> {
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError> {
        let address_book_entry = ADDRESS_BOOK_SERVICE
            .create_entry(self.operation.input.to_owned())
            .await
            .map_err(|e| ProposalExecuteError::Failed {
                reason: format!("Failed to create address book entry: {}", e),
            })?;

        let mut operation = self.proposal.operation.clone();

        if let ProposalOperation::AddAddressBookEntry(ref mut operation) = operation {
            operation.address_book_entry_id = Some(address_book_entry.id);
        }

        Ok(ProposalExecuteStage::Completed(operation))
    }
}
