use crate::models::{access_control::Resource, ProposalOperationType};
use wallet_api::ProposalOperationInput;

impl From<ProposalOperationType> for Resource {
    fn from(proposal_operation_type: ProposalOperationType) -> Self {
        match proposal_operation_type {
            ProposalOperationType::Transfer => Resource::TransferProposal,
            ProposalOperationType::AddAccount => Resource::AddAccountProposal,
            ProposalOperationType::AddUser => Resource::AddUserProposal,
            ProposalOperationType::AddUserGroup => Resource::AddUserGroupProposal,
            ProposalOperationType::EditAccount => Resource::EditAccountProposal,
            ProposalOperationType::EditUser => Resource::EditUserProposal,
            ProposalOperationType::EditUserGroup => Resource::EditUserGroupProposal,
            ProposalOperationType::EditUserStatus => Resource::EditUserStatusProposal,
            ProposalOperationType::RemoveUserGroup => Resource::RemoveUserGroupProposal,
        }
    }
}

impl From<ProposalOperationInput> for Resource {
    fn from(proposal_operation_input: ProposalOperationInput) -> Self {
        match proposal_operation_input {
            ProposalOperationInput::Transfer(_) => Resource::TransferProposal,
            ProposalOperationInput::AddAccount(_) => Resource::AddAccountProposal,
            ProposalOperationInput::AddUser(_) => Resource::AddUserProposal,
            ProposalOperationInput::AddUserGroup(_) => Resource::AddUserGroupProposal,
            ProposalOperationInput::EditAccount(_) => Resource::EditAccountProposal,
            ProposalOperationInput::EditUser(_) => Resource::EditUserProposal,
            ProposalOperationInput::EditUserGroup(_) => Resource::EditUserGroupProposal,
            ProposalOperationInput::EditUserStatus(_) => Resource::EditUserStatusProposal,
            ProposalOperationInput::RemoveUserGroup(_) => Resource::RemoveUserGroupProposal,
        }
    }
}
