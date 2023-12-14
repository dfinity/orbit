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
mod add_user;
mod add_user_group;
mod edit_access_policy;
mod edit_account;
mod edit_user;
mod edit_user_group;
mod edit_user_status;
mod remove_access_policy;
mod remove_user_group;
mod transfer;
mod upgrade;

use self::{
    add_access_policy::{
        AddAccessPolicyProposalCreate, AddAccessPolicyProposalCreateHook,
        AddAccessPolicyProposalExecute,
    },
    add_account::{
        AddAccountProposalCreate, AddAccountProposalCreateHook, AddAccountProposalExecute,
    },
    add_user::{AddUserProposalCreate, AddUserProposalCreateHook, AddUserProposalExecute},
    add_user_group::{
        AddUserGroupProposalCreate, AddUserGroupProposalCreateHook, AddUserGroupProposalExecute,
    },
    edit_access_policy::{
        EditAccessPolicyProposalCreate, EditAccessPolicyProposalCreateHook,
        EditAccessPolicyProposalExecute,
    },
    edit_account::{
        EditAccountProposalCreate, EditAccountProposalCreateHook, EditAccountProposalExecute,
    },
    edit_user::{EditUserProposalCreate, EditUserProposalCreateHook, EditUserProposalExecute},
    edit_user_group::{
        EditUserGroupProposalCreate, EditUserGroupProposalCreateHook, EditUserGroupProposalExecute,
    },
    edit_user_status::{
        EditUserStatusProposalCreate, EditUserStatusProposalCreateHook,
        EditUserStatusProposalExecute,
    },
    remove_access_policy::{
        RemoveAccessPolicyProposalCreate, RemoveAccessPolicyProposalCreateHook,
        RemoveAccessPolicyProposalExecute,
    },
    remove_user_group::{
        RemoveUserGroupProposalCreate, RemoveUserGroupProposalCreateHook,
        RemoveUserGroupProposalExecute,
    },
    transfer::{TransferProposalCreate, TransferProposalCreateHook, TransferProposalExecute},
    upgrade::{UpgradeProposalCreate, UpgradeProposalCreateHook, UpgradeProposalExecute},
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

#[async_trait]
pub trait CreateHook: Send + Sync {
    /// The post create hook is called after the proposal is created and can be used
    /// for additional processing (e.g. sending notifications).
    async fn on_created(&self) {
        // noop by default
    }
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
            ProposalOperationInput::EditUserStatus(operation) => {
                create_proposal::<
                    wallet_api::EditUserStatusOperationInput,
                    EditUserStatusProposalCreate,
                >(id, proposed_by_user, input.clone(), operation.clone())
            }
            ProposalOperationInput::Upgrade(operation) => {
                create_proposal::<wallet_api::UpgradeOperationInput, UpgradeProposalCreate>(
                    id,
                    proposed_by_user,
                    input.clone(),
                    operation.clone(),
                )
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
        }
    }

    pub fn create_hook<'p>(proposal: &'p Proposal) -> Box<dyn CreateHook + 'p> {
        match &proposal.operation {
            ProposalOperation::Transfer(operation) => {
                Box::new(TransferProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::AddAccount(operation) => {
                Box::new(AddAccountProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::EditAccount(operation) => {
                Box::new(EditAccountProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::AddUserGroup(operation) => {
                Box::new(AddUserGroupProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::EditUserGroup(operation) => {
                Box::new(EditUserGroupProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::RemoveUserGroup(operation) => {
                Box::new(RemoveUserGroupProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::AddUser(operation) => {
                Box::new(AddUserProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::EditUser(operation) => {
                Box::new(EditUserProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::EditUserStatus(operation) => {
                Box::new(EditUserStatusProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::Upgrade(operation) => {
                Box::new(UpgradeProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::AddAccessPolicy(operation) => {
                Box::new(AddAccessPolicyProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::EditAccessPolicy(operation) => {
                Box::new(EditAccessPolicyProposalCreateHook::new(proposal, operation))
            }
            ProposalOperation::RemoveAccessPolicy(operation) => Box::new(
                RemoveAccessPolicyProposalCreateHook::new(proposal, operation),
            ),
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
            ProposalOperation::EditUserStatus(operation) => {
                Box::new(EditUserStatusProposalExecute::new(proposal, operation))
            }
            ProposalOperation::Upgrade(operation) => {
                Box::new(UpgradeProposalExecute::new(proposal, operation))
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
        }
    }
}
