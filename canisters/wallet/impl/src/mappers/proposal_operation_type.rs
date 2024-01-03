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
            ProposalOperationTypeDTO::AddUserGroup => ProposalOperationType::AddUserGroup,
            ProposalOperationTypeDTO::EditUserGroup => ProposalOperationType::EditUserGroup,
            ProposalOperationTypeDTO::RemoveUserGroup => ProposalOperationType::RemoveUserGroup,
            ProposalOperationTypeDTO::ChangeCanister => ProposalOperationType::ChangeCanister,
            ProposalOperationTypeDTO::AddAccessPolicy => ProposalOperationType::AddAccessPolicy,
            ProposalOperationTypeDTO::EditAccessPolicy => ProposalOperationType::EditAccessPolicy,
            ProposalOperationTypeDTO::RemoveAccessPolicy => {
                ProposalOperationType::RemoveAccessPolicy
            }
            ProposalOperationTypeDTO::AddProposalPolicy => ProposalOperationType::AddProposalPolicy,
            ProposalOperationTypeDTO::EditProposalPolicy => {
                ProposalOperationType::EditProposalPolicy
            }
            ProposalOperationTypeDTO::RemoveProposalPolicy => {
                ProposalOperationType::RemoveProposalPolicy
            }
        }
    }
}

impl From<ProposalOperationType> for ProposalOperationTypeDTO {
    fn from(operation_type: ProposalOperationType) -> Self {
        match operation_type {
            ProposalOperationType::Transfer => ProposalOperationTypeDTO::Transfer,
            ProposalOperationType::AddAccount => ProposalOperationTypeDTO::AddAccount,
            ProposalOperationType::EditAccount => ProposalOperationTypeDTO::EditAccount,
            ProposalOperationType::AddUser => ProposalOperationTypeDTO::AddUser,
            ProposalOperationType::EditUser => ProposalOperationTypeDTO::EditUser,
            ProposalOperationType::AddUserGroup => ProposalOperationTypeDTO::AddUserGroup,
            ProposalOperationType::EditUserGroup => ProposalOperationTypeDTO::EditUserGroup,
            ProposalOperationType::RemoveUserGroup => ProposalOperationTypeDTO::RemoveUserGroup,
            ProposalOperationType::ChangeCanister => ProposalOperationTypeDTO::ChangeCanister,
            ProposalOperationType::AddAccessPolicy => ProposalOperationTypeDTO::AddAccessPolicy,
            ProposalOperationType::EditAccessPolicy => ProposalOperationTypeDTO::EditAccessPolicy,
            ProposalOperationType::RemoveAccessPolicy => {
                ProposalOperationTypeDTO::RemoveAccessPolicy
            }
            ProposalOperationType::AddProposalPolicy => ProposalOperationTypeDTO::AddProposalPolicy,
            ProposalOperationType::EditProposalPolicy => {
                ProposalOperationTypeDTO::EditProposalPolicy
            }
            ProposalOperationType::RemoveProposalPolicy => {
                ProposalOperationTypeDTO::RemoveProposalPolicy
            }
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
            ProposalOperation::AddUserGroup(_) => ProposalOperationType::AddUserGroup,
            ProposalOperation::EditUserGroup(_) => ProposalOperationType::EditUserGroup,
            ProposalOperation::RemoveUserGroup(_) => ProposalOperationType::RemoveUserGroup,
            ProposalOperation::ChangeCanister(_) => ProposalOperationType::ChangeCanister,
            ProposalOperation::AddAccessPolicy(_) => ProposalOperationType::AddAccessPolicy,
            ProposalOperation::EditAccessPolicy(_) => ProposalOperationType::EditAccessPolicy,
            ProposalOperation::RemoveAccessPolicy(_) => ProposalOperationType::RemoveAccessPolicy,
            ProposalOperation::AddProposalPolicy(_) => ProposalOperationType::AddProposalPolicy,
            ProposalOperation::EditProposalPolicy(_) => ProposalOperationType::EditProposalPolicy,
            ProposalOperation::RemoveProposalPolicy(_) => {
                ProposalOperationType::RemoveProposalPolicy
            }
        }
    }
}
