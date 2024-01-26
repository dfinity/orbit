use crate::mappers::HelperMapper;
use crate::models::{ProposalOperation, ProposalOperationType};
use wallet_api::{ListProposalsOperationTypeDTO, ProposalOperationTypeDTO};

impl From<ProposalOperationTypeDTO> for ProposalOperationType {
    fn from(dto: ProposalOperationTypeDTO) -> Self {
        match dto {
            ProposalOperationTypeDTO::Transfer => ProposalOperationType::Transfer,
            ProposalOperationTypeDTO::AddAccount => ProposalOperationType::AddAccount,
            ProposalOperationTypeDTO::EditAccount => ProposalOperationType::EditAccount,
            ProposalOperationTypeDTO::AddAddressBookEntry => {
                ProposalOperationType::AddAddressBookEntry
            }
            ProposalOperationTypeDTO::EditAddressBookEntry => {
                ProposalOperationType::EditAddressBookEntry
            }
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
            ProposalOperationType::AddAddressBookEntry => {
                ProposalOperationTypeDTO::AddAddressBookEntry
            }
            ProposalOperationType::EditAddressBookEntry => {
                ProposalOperationTypeDTO::EditAddressBookEntry
            }
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
            ProposalOperation::AddAddressBookEntry(_) => ProposalOperationType::AddAddressBookEntry,
            ProposalOperation::EditAddressBookEntry(_) => {
                ProposalOperationType::EditAddressBookEntry
            }
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

impl ProposalOperation {
    pub fn is_of_type(&self, operation: &ListProposalsOperationTypeDTO) -> bool {
        match (self, operation) {
            (
                ProposalOperation::Transfer(transfer_operation),
                ListProposalsOperationTypeDTO::Transfer(from_account_id),
            ) => {
                if let Some(account_id) = from_account_id {
                    HelperMapper::to_uuid(account_id.clone()).map(|uuid| *uuid.as_bytes())
                        == Ok(transfer_operation.input.from_account_id)
                } else {
                    true
                }
            }
            (ProposalOperation::AddAccount(_), ListProposalsOperationTypeDTO::AddAccount) => true,
            (ProposalOperation::EditAccount(_), ListProposalsOperationTypeDTO::EditAccount) => true,
            (
                ProposalOperation::AddAddressBookEntry(_),
                ListProposalsOperationTypeDTO::AddAddressBookEntry,
            ) => true,
            (
                ProposalOperation::EditAddressBookEntry(_),
                ListProposalsOperationTypeDTO::EditAddressBookEntry,
            ) => true,
            (ProposalOperation::AddUser(_), ListProposalsOperationTypeDTO::AddUser) => true,
            (ProposalOperation::EditUser(_), ListProposalsOperationTypeDTO::EditUser) => true,
            (ProposalOperation::AddUserGroup(_), ListProposalsOperationTypeDTO::AddUserGroup) => {
                true
            }
            (ProposalOperation::EditUserGroup(_), ListProposalsOperationTypeDTO::EditUserGroup) => {
                true
            }
            (
                ProposalOperation::RemoveUserGroup(_),
                ListProposalsOperationTypeDTO::RemoveUserGroup,
            ) => true,
            (
                ProposalOperation::ChangeCanister(_),
                ListProposalsOperationTypeDTO::ChangeCanister,
            ) => true,
            (
                ProposalOperation::AddAccessPolicy(_),
                ListProposalsOperationTypeDTO::AddAccessPolicy,
            ) => true,
            (
                ProposalOperation::EditAccessPolicy(_),
                ListProposalsOperationTypeDTO::EditAccessPolicy,
            ) => true,
            (
                ProposalOperation::RemoveAccessPolicy(_),
                ListProposalsOperationTypeDTO::RemoveAccessPolicy,
            ) => true,
            (
                ProposalOperation::AddProposalPolicy(_),
                ListProposalsOperationTypeDTO::AddProposalPolicy,
            ) => true,
            (
                ProposalOperation::EditProposalPolicy(_),
                ListProposalsOperationTypeDTO::EditProposalPolicy,
            ) => true,
            (
                ProposalOperation::RemoveProposalPolicy(_),
                ListProposalsOperationTypeDTO::RemoveProposalPolicy,
            ) => true,
            _ => false,
        }
    }
}
