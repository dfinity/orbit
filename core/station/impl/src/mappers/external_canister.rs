use crate::{
    core::ic_cdk::next_time,
    models::{CreateExternalCanisterOperationInput, ExternalCanister, ExternalCanisterState},
};
use candid::Principal;
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct ExternalCanisterMapper {}

impl ExternalCanisterMapper {
    pub fn from_create_input(
        canister_id: Principal,
        input: CreateExternalCanisterOperationInput,
    ) -> ExternalCanister {
        ExternalCanister {
            id: *Uuid::new_v4().as_bytes(),
            canister_id,
            name: input.name.clone(),
            description: input.description.clone(),
            labels: input.labels.clone().unwrap_or_default(),
            state: ExternalCanisterState::Active,
            created_at: next_time(),
            modified_at: None,
        }
    }
}
