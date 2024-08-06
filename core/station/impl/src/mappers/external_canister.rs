use crate::{
    core::ic_cdk::next_time,
    models::{
        ConfigureExternalCanisterOperationInput, ConfigureExternalCanisterOperationKind,
        ConfigureExternalCanisterSettingsInput, CreateExternalCanisterOperationInput,
        DefiniteCanisterSettingsInput, ExternalCanister, ExternalCanisterState,
    },
};
use candid::Principal;
use ic_cdk::api::management_canister::main::CanisterSettings;
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

impl From<DefiniteCanisterSettingsInput> for CanisterSettings {
    fn from(input: DefiniteCanisterSettingsInput) -> Self {
        CanisterSettings {
            controllers: input.controllers,
            compute_allocation: input.compute_allocation,
            freezing_threshold: input.freezing_threshold,
            memory_allocation: input.memory_allocation,
            reserved_cycles_limit: input.reserved_cycles_limit,
        }
    }
}

impl From<station_api::ConfigureExternalCanisterOperationInput>
    for ConfigureExternalCanisterOperationInput
{
    fn from(input: station_api::ConfigureExternalCanisterOperationInput) -> Self {
        ConfigureExternalCanisterOperationInput {
            canister_id: input.canister_id,
            operation: input.operation.into(),
        }
    }
}

impl From<station_api::ConfigureExternalCanisterOperationKindDTO>
    for ConfigureExternalCanisterOperationKind
{
    fn from(input: station_api::ConfigureExternalCanisterOperationKindDTO) -> Self {
        match input {
            station_api::ConfigureExternalCanisterOperationKindDTO::Delete => {
                ConfigureExternalCanisterOperationKind::Delete
            }
            station_api::ConfigureExternalCanisterOperationKindDTO::SoftDelete => {
                ConfigureExternalCanisterOperationKind::SoftDelete
            }
            station_api::ConfigureExternalCanisterOperationKindDTO::TopUp(cycles) => {
                ConfigureExternalCanisterOperationKind::TopUp(cycles)
            }
            station_api::ConfigureExternalCanisterOperationKindDTO::NativeSettings(settings) => {
                ConfigureExternalCanisterOperationKind::NativeSettings(settings.into())
            }
            station_api::ConfigureExternalCanisterOperationKindDTO::Settings(settings) => {
                ConfigureExternalCanisterOperationKind::Settings(settings.into())
            }
        }
    }
}

impl From<station_api::DefiniteCanisterSettingsInput> for DefiniteCanisterSettingsInput {
    fn from(input: station_api::DefiniteCanisterSettingsInput) -> Self {
        DefiniteCanisterSettingsInput {
            controllers: input.controllers,
            compute_allocation: input.compute_allocation,
            freezing_threshold: input.freezing_threshold,
            memory_allocation: input.memory_allocation,
            reserved_cycles_limit: input.reserved_cycles_limit,
        }
    }
}

impl From<station_api::ConfigureExternalCanisterSettingsInput>
    for ConfigureExternalCanisterSettingsInput
{
    fn from(input: station_api::ConfigureExternalCanisterSettingsInput) -> Self {
        ConfigureExternalCanisterSettingsInput {
            name: input.name,
            description: input.description,
            labels: input.labels,
            permissions: input.permissions.map(Into::into),
            request_policies: input.request_policies.map(Into::into),
        }
    }
}
