use crate::models::{ProposalOperation, ProposalOperationType};
use wallet_api::ProposalOperationTypeDTO;

impl From<ProposalOperationTypeDTO> for ProposalOperationType {
    fn from(dto: ProposalOperationTypeDTO) -> Self {
        match dto {
            ProposalOperationTypeDTO::Transfer => ProposalOperationType::Transfer,
            ProposalOperationTypeDTO::AddAccount => ProposalOperationType::AddAccount,
            ProposalOperationTypeDTO::EditAccount => ProposalOperationType::EditAccount,
            ProposalOperationTypeDTO::AddUser => ProposalOperationType::AddUser,
            ProposalOperationTypeDTO::EditUser => ProposalOperationType::EditUser,
        }
    }
}

impl From<ProposalOperation> for ProposalOperationType {
    fn from(operation: ProposalOperation) -> Self {
        match operation {
            ProposalOperation::Transfer(_) => ProposalOperationType::Transfer,
            ProposalOperation::AddAccount(_) => ProposalOperationType::AddAccount,
            ProposalOperation::EditAccount(_) => ProposalOperationType::EditAccount,
            ProposalOperation::AddUser(_) => ProposalOperationType::AddUser,
            ProposalOperation::EditUser(_) => ProposalOperationType::EditUser,
        }
    }
}
