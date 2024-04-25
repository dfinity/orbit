use crate::mappers::HelperMapper;
use crate::models::proposal_operation_filter_type::ProposalOperationFilterType;
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
            ProposalOperationTypeDTO::RemoveAddressBookEntry => {
                ProposalOperationType::RemoveAddressBookEntry
            }
            ProposalOperationTypeDTO::AddUser => ProposalOperationType::AddUser,
            ProposalOperationTypeDTO::EditUser => ProposalOperationType::EditUser,
            ProposalOperationTypeDTO::AddUserGroup => ProposalOperationType::AddUserGroup,
            ProposalOperationTypeDTO::EditUserGroup => ProposalOperationType::EditUserGroup,
            ProposalOperationTypeDTO::RemoveUserGroup => ProposalOperationType::RemoveUserGroup,
            ProposalOperationTypeDTO::ChangeCanister => ProposalOperationType::ChangeCanister,
            ProposalOperationTypeDTO::EditAccessPolicy => ProposalOperationType::EditAccessPolicy,
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
            ProposalOperationType::RemoveAddressBookEntry => {
                ProposalOperationTypeDTO::RemoveAddressBookEntry
            }
            ProposalOperationType::AddUser => ProposalOperationTypeDTO::AddUser,
            ProposalOperationType::EditUser => ProposalOperationTypeDTO::EditUser,
            ProposalOperationType::AddUserGroup => ProposalOperationTypeDTO::AddUserGroup,
            ProposalOperationType::EditUserGroup => ProposalOperationTypeDTO::EditUserGroup,
            ProposalOperationType::RemoveUserGroup => ProposalOperationTypeDTO::RemoveUserGroup,
            ProposalOperationType::ChangeCanister => ProposalOperationTypeDTO::ChangeCanister,
            ProposalOperationType::EditAccessPolicy => ProposalOperationTypeDTO::EditAccessPolicy,
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
            ProposalOperation::RemoveAddressBookEntry(_) => {
                ProposalOperationType::RemoveAddressBookEntry
            }
            ProposalOperation::AddUser(_) => ProposalOperationType::AddUser,
            ProposalOperation::EditUser(_) => ProposalOperationType::EditUser,
            ProposalOperation::AddUserGroup(_) => ProposalOperationType::AddUserGroup,
            ProposalOperation::EditUserGroup(_) => ProposalOperationType::EditUserGroup,
            ProposalOperation::RemoveUserGroup(_) => ProposalOperationType::RemoveUserGroup,
            ProposalOperation::ChangeCanister(_) => ProposalOperationType::ChangeCanister,
            ProposalOperation::EditAccessPolicy(_) => ProposalOperationType::EditAccessPolicy,
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
            (
                ProposalOperation::RemoveAddressBookEntry(_),
                ListProposalsOperationTypeDTO::RemoveAddressBookEntry,
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
                ProposalOperation::EditAccessPolicy(_),
                ListProposalsOperationTypeDTO::EditAccessPolicy,
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

impl From<wallet_api::ListProposalsOperationTypeDTO> for ProposalOperationFilterType {
    fn from(dto: wallet_api::ListProposalsOperationTypeDTO) -> Self {
        match dto {
            wallet_api::ListProposalsOperationTypeDTO::Transfer(from_account_id) => {
                ProposalOperationFilterType::Transfer(from_account_id.map(|id| {
                    *HelperMapper::to_uuid(id)
                        .expect("Invalid account id")
                        .as_bytes()
                }))
            }
            wallet_api::ListProposalsOperationTypeDTO::AddAccount => {
                ProposalOperationFilterType::AddAccount
            }
            wallet_api::ListProposalsOperationTypeDTO::EditAccount => {
                ProposalOperationFilterType::EditAccount
            }
            wallet_api::ListProposalsOperationTypeDTO::AddAddressBookEntry => {
                ProposalOperationFilterType::AddAddressBookEntry
            }
            wallet_api::ListProposalsOperationTypeDTO::EditAddressBookEntry => {
                ProposalOperationFilterType::EditAddressBookEntry
            }
            wallet_api::ListProposalsOperationTypeDTO::RemoveAddressBookEntry => {
                ProposalOperationFilterType::RemoveAddressBookEntry
            }
            wallet_api::ListProposalsOperationTypeDTO::AddUser => {
                ProposalOperationFilterType::AddUser
            }
            wallet_api::ListProposalsOperationTypeDTO::EditUser => {
                ProposalOperationFilterType::EditUser
            }
            wallet_api::ListProposalsOperationTypeDTO::AddUserGroup => {
                ProposalOperationFilterType::AddUserGroup
            }
            wallet_api::ListProposalsOperationTypeDTO::EditUserGroup => {
                ProposalOperationFilterType::EditUserGroup
            }
            wallet_api::ListProposalsOperationTypeDTO::RemoveUserGroup => {
                ProposalOperationFilterType::RemoveUserGroup
            }
            wallet_api::ListProposalsOperationTypeDTO::ChangeCanister => {
                ProposalOperationFilterType::ChangeCanister
            }
            wallet_api::ListProposalsOperationTypeDTO::EditAccessPolicy => {
                ProposalOperationFilterType::EditAccessPolicy
            }
            wallet_api::ListProposalsOperationTypeDTO::AddProposalPolicy => {
                ProposalOperationFilterType::AddProposalPolicy
            }
            wallet_api::ListProposalsOperationTypeDTO::EditProposalPolicy => {
                ProposalOperationFilterType::EditProposalPolicy
            }
            wallet_api::ListProposalsOperationTypeDTO::RemoveProposalPolicy => {
                ProposalOperationFilterType::RemoveProposalPolicy
            }
        }
    }
}
