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
            ProposalOperationTypeDTO::EditUserStatus => ProposalOperationType::EditUserStatus,
            ProposalOperationTypeDTO::AddUserGroup => ProposalOperationType::AddUserGroup,
            ProposalOperationTypeDTO::EditUserGroup => ProposalOperationType::EditUserGroup,
            ProposalOperationTypeDTO::RemoveUserGroup => ProposalOperationType::RemoveUserGroup,
            ProposalOperationTypeDTO::Upgrade => ProposalOperationType::Upgrade,
            ProposalOperationTypeDTO::AddAccessPolicy => ProposalOperationType::AddAccessPolicy,
            ProposalOperationTypeDTO::EditAccessPolicy => ProposalOperationType::EditAccessPolicy,
            ProposalOperationTypeDTO::RemoveAccessPolicy => {
                ProposalOperationType::RemoveAccessPolicy
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
            ProposalOperation::EditUserStatus(_) => ProposalOperationType::EditUserStatus,
            ProposalOperation::AddUserGroup(_) => ProposalOperationType::AddUserGroup,
            ProposalOperation::EditUserGroup(_) => ProposalOperationType::EditUserGroup,
            ProposalOperation::RemoveUserGroup(_) => ProposalOperationType::RemoveUserGroup,
            ProposalOperation::Upgrade(_) => ProposalOperationType::Upgrade,
            ProposalOperation::AddAccessPolicy(_) => ProposalOperationType::AddAccessPolicy,
            ProposalOperation::EditAccessPolicy(_) => ProposalOperationType::EditAccessPolicy,
            ProposalOperation::RemoveAccessPolicy(_) => ProposalOperationType::RemoveAccessPolicy,
        }
    }
}
