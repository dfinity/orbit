use crate::models::{ProposalOperation, ProposalOperationType};
use wallet_api::ProposalOperationTypeDTO;

impl From<ProposalOperationTypeDTO> for ProposalOperationType {
    fn from(dto: ProposalOperationTypeDTO) -> Self {
        match dto {
            ProposalOperationTypeDTO::Transfer => ProposalOperationType::Transfer,
            ProposalOperationTypeDTO::AccountEdit => ProposalOperationType::AccountEdit,
        }
    }
}

impl From<ProposalOperation> for ProposalOperationType {
    fn from(operation: ProposalOperation) -> Self {
        match operation {
            ProposalOperation::Transfer(_) => ProposalOperationType::Transfer,
            ProposalOperation::AccountEdit(_) => ProposalOperationType::AccountEdit,
        }
    }
}
