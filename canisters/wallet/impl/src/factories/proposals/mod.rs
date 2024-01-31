use crate::{
    core::generate_uuid_v4,
    errors::{ProposalError, ProposalExecuteError},
    models::{Proposal, ProposalOperation},
    services::POLICY_SERVICE,
};
use async_trait::async_trait;
use ic_canister_core::types::UUID;
use std::sync::Arc;
use wallet_api::{CreateProposalInput, ProposalOperationInput};

mod add_access_policy;
mod add_account;
mod add_address_book_entry;
mod add_proposal_policy;
mod add_user;
mod add_user_group;
mod change_canister;
mod edit_access_policy;
mod edit_account;
mod edit_address_book_entry;
mod edit_proposal_policy;
mod edit_user;
mod edit_user_group;
mod remove_access_policy;
mod remove_address_book_entry;
mod remove_proposal_policy;
mod remove_user_group;
mod transfer;

use self::{
    add_access_policy::{AddAccessPolicyProposalCreate, AddAccessPolicyProposalExecute},
    add_account::{AddAccountProposalCreate, AddAccountProposalExecute},
    add_address_book_entry::{
        AddAddressBookEntryProposalCreate, AddAddressBookEntryProposalExecute,
    },
    add_proposal_policy::{AddProposalPolicyProposalCreate, AddProposalPolicyProposalExecute},
    add_user::{AddUserProposalCreate, AddUserProposalExecute},
    add_user_group::{AddUserGroupProposalCreate, AddUserGroupProposalExecute},
    change_canister::{ChangeCanisterProposalCreate, ChangeCanisterProposalExecute},
    edit_access_policy::{EditAccessPolicyProposalCreate, EditAccessPolicyProposalExecute},
    edit_account::{EditAccountProposalCreate, EditAccountProposalExecute},
    edit_address_book_entry::{
        EditAddressBookEntryProposalCreate, EditAddressBookEntryProposalExecute,
    },
    edit_proposal_policy::{EditProposalPolicyProposalCreate, EditProposalPolicyProposalExecute},
    edit_user::{EditUserProposalCreate, EditUserProposalExecute},
    edit_user_group::{EditUserGroupProposalCreate, EditUserGroupProposalExecute},
    remove_access_policy::{RemoveAccessPolicyProposalCreate, RemoveAccessPolicyProposalExecute},
    remove_address_book_entry::{
        RemoveAddressBookEntryProposalCreate, RemoveAddressBookEntryProposalExecute,
    },
    remove_proposal_policy::{
        RemoveProposalPolicyProposalCreate, RemoveProposalPolicyProposalExecute,
    },
    remove_user_group::{RemoveUserGroupProposalCreate, RemoveUserGroupProposalExecute},
    transfer::{TransferProposalCreate, TransferProposalExecute},
};

#[derive(Debug)]
pub enum ProposalExecuteStage {
    Completed(ProposalOperation),
    Processing(ProposalOperation),
}

#[async_trait]
pub trait Execute: Send + Sync {
    /// Executes the proposal and returns the operation that was executed with the stage that the execution is in.
    ///
    /// The stage is used to indicate if the operation was completed or if it is still processing.
    async fn execute(&self) -> Result<ProposalExecuteStage, ProposalExecuteError>;
}

pub trait Create<T>: Send + Sync {
    /// Creates a new proposal for the operation but does not save it.
    fn create(
        proposal_id: UUID,
        proposed_by_user: UUID,
        input: CreateProposalInput,
        operation_input: T,
    ) -> Result<Proposal, ProposalError>
    where
        Self: Sized;
}

fn create_proposal<OperationInput, Creator: Create<OperationInput>>(
    proposal_id: UUID,
    proposed_by_user: UUID,
    input: CreateProposalInput,
    operation_input: OperationInput,
) -> Result<Proposal, ProposalError> {
    Creator::create(proposal_id, proposed_by_user, input, operation_input)
}

#[derive(Debug)]
pub struct ProposalFactory {}

impl ProposalFactory {
    pub async fn create_proposal(
        proposed_by_user: UUID,
        input: CreateProposalInput,
    ) -> Result<Proposal, ProposalError> {
        let id = *generate_uuid_v4().await.as_bytes();
        match &input.operation {
            ProposalOperationInput::Transfer(operation) => {
                create_proposal::<wallet_api::TransferOperationInput, TransferProposalCreate>(
                    id,
                    proposed_by_user,
                    input.clone(),
                    operation.clone(),
                )
            }
            ProposalOperationInput::AddAccount(operation) => {
                create_proposal::<wallet_api::AddAccountOperationInput, AddAccountProposalCreate>(
                    id,
                    proposed_by_user,
                    input.clone(),
                    operation.clone(),
                )
            }
            ProposalOperationInput::EditAccount(operation) => {
                create_proposal::<wallet_api::EditAccountOperationInput, EditAccountProposalCreate>(
                    id,
                    proposed_by_user,
                    input.clone(),
                    operation.clone(),
                )
            }
            ProposalOperationInput::AddAddressBookEntry(operation) => {
                create_proposal::<
                    wallet_api::AddAddressBookEntryOperationInput,
                    AddAddressBookEntryProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::EditAddressBookEntry(operation) => {
                create_proposal::<
                    wallet_api::EditAddressBookEntryOperationInput,
                    EditAddressBookEntryProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::RemoveAddressBookEntry(operation) => {
                create_proposal::<
                    wallet_api::RemoveAddressBookEntryOperationInput,
                    RemoveAddressBookEntryProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::AddUserGroup(operation) => {
                create_proposal::<wallet_api::AddUserGroupOperationInput, AddUserGroupProposalCreate>(
                    id,
                    proposed_by_user,
                    input.clone(),
                    operation.clone(),
                )
            }
            ProposalOperationInput::EditUserGroup(operation) => {
                create_proposal::<
                    wallet_api::EditUserGroupOperationInput,
                    EditUserGroupProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::RemoveUserGroup(operation) => {
                create_proposal::<
                    wallet_api::RemoveUserGroupOperationInput,
                    RemoveUserGroupProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::AddUser(operation) => {
                create_proposal::<wallet_api::AddUserOperationInput, AddUserProposalCreate>(
                    id,
                    proposed_by_user,
                    input.clone(),
                    operation.clone(),
                )
            }
            ProposalOperationInput::EditUser(operation) => {
                create_proposal::<wallet_api::EditUserOperationInput, EditUserProposalCreate>(
                    id,
                    proposed_by_user,
                    input.clone(),
                    operation.clone(),
                )
            }
            ProposalOperationInput::ChangeCanister(operation) => {
                create_proposal::<
                    wallet_api::ChangeCanisterOperationInput,
                    ChangeCanisterProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::AddAccessPolicy(operation) => {
                create_proposal::<
                    wallet_api::AddAccessPolicyOperationInput,
                    AddAccessPolicyProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::EditAccessPolicy(operation) => {
                create_proposal::<
                    wallet_api::EditAccessPolicyOperationInput,
                    EditAccessPolicyProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::RemoveAccessPolicy(operation) => {
                create_proposal::<
                    wallet_api::RemoveAccessPolicyOperationInput,
                    RemoveAccessPolicyProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::AddProposalPolicy(operation) => {
                create_proposal::<
                    wallet_api::AddProposalPolicyOperationInput,
                    AddProposalPolicyProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::EditProposalPolicy(operation) => {
                create_proposal::<
                    wallet_api::EditProposalPolicyOperationInput,
                    EditProposalPolicyProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::RemoveProposalPolicy(operation) => {
                create_proposal::<
                    wallet_api::RemoveProposalPolicyOperationInput,
                    RemoveProposalPolicyProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
        }
    }

    pub fn executor<'p>(proposal: &'p Proposal) -> Box<dyn Execute + 'p> {
        match &proposal.operation {
            ProposalOperation::Transfer(operation) => {
                Box::new(TransferProposalExecute::new(proposal, operation))
            }
            ProposalOperation::AddAccount(operation) => {
                Box::new(AddAccountProposalExecute::new(proposal, operation))
            }
            ProposalOperation::EditAccount(operation) => {
                Box::new(EditAccountProposalExecute::new(proposal, operation))
            }
            ProposalOperation::AddAddressBookEntry(operation) => {
                Box::new(AddAddressBookEntryProposalExecute::new(proposal, operation))
            }
            ProposalOperation::EditAddressBookEntry(operation) => Box::new(
                EditAddressBookEntryProposalExecute::new(proposal, operation),
            ),
            ProposalOperation::RemoveAddressBookEntry(operation) => Box::new(
                RemoveAddressBookEntryProposalExecute::new(proposal, operation),
            ),
            ProposalOperation::AddUserGroup(operation) => {
                Box::new(AddUserGroupProposalExecute::new(proposal, operation))
            }
            ProposalOperation::EditUserGroup(operation) => {
                Box::new(EditUserGroupProposalExecute::new(proposal, operation))
            }
            ProposalOperation::RemoveUserGroup(operation) => {
                Box::new(RemoveUserGroupProposalExecute::new(proposal, operation))
            }
            ProposalOperation::AddUser(operation) => {
                Box::new(AddUserProposalExecute::new(proposal, operation))
            }
            ProposalOperation::EditUser(operation) => {
                Box::new(EditUserProposalExecute::new(proposal, operation))
            }
            ProposalOperation::ChangeCanister(operation) => {
                Box::new(ChangeCanisterProposalExecute::new(proposal, operation))
            }
            ProposalOperation::AddAccessPolicy(operation) => {
                Box::new(AddAccessPolicyProposalExecute::new(
                    proposal,
                    operation,
                    Arc::clone(&POLICY_SERVICE),
                ))
            }
            ProposalOperation::EditAccessPolicy(operation) => {
                Box::new(EditAccessPolicyProposalExecute::new(
                    proposal,
                    operation,
                    Arc::clone(&POLICY_SERVICE),
                ))
            }
            ProposalOperation::RemoveAccessPolicy(operation) => {
                Box::new(RemoveAccessPolicyProposalExecute::new(
                    proposal,
                    operation,
                    Arc::clone(&POLICY_SERVICE),
                ))
            }
            ProposalOperation::AddProposalPolicy(operation) => {
                Box::new(AddProposalPolicyProposalExecute::new(
                    proposal,
                    operation,
                    Arc::clone(&POLICY_SERVICE),
                ))
            }
            ProposalOperation::EditProposalPolicy(operation) => {
                Box::new(EditProposalPolicyProposalExecute::new(
                    proposal,
                    operation,
                    Arc::clone(&POLICY_SERVICE),
                ))
            }
            ProposalOperation::RemoveProposalPolicy(operation) => {
                Box::new(RemoveProposalPolicyProposalExecute::new(
                    proposal,
                    operation,
                    Arc::clone(&POLICY_SERVICE),
                ))
            }
        }
    }
}
