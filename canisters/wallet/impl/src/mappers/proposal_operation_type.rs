use crate::models::{ProposalOperation, ProposalOperationType};
use wallet_api::ProposalOperationTypeDTO;

impl From<ProposalOperationTypeDTO> for ProposalOperationType {
    fn from(dto: ProposalOperationTypeDTO) -> Self {
        match dto {
            ProposalOperationTypeDTO::Transfer => ProposalOperationType::Transfer,
            ProposalOperationTypeDTO::EditAccount => ProposalOperationType::EditAccount,
            ProposalOperationTypeDTO::AddAccount => ProposalOperationType::AddAccount,
        }
    }
}

impl From<ProposalOperation> for ProposalOperationType {
    fn from(operation: ProposalOperation) -> Self {
        match operation {
            ProposalOperation::Transfer(_) => ProposalOperationType::Transfer,
            ProposalOperation::EditAccount(_) => ProposalOperationType::EditAccount,
            ProposalOperation::AddAccount(_) => ProposalOperationType::AddAccount,
        }
    }
}
