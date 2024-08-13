use crate::{
    core::ic_cdk::next_time,
    models::{
        ConfigureExternalCanisterOperationInput, ConfigureExternalCanisterOperationKind,
        ConfigureExternalCanisterSettingsInput, CreateExternalCanisterOperationInput,
        DefiniteCanisterSettingsInput, ExternalCanister, ExternalCanisterCallRequestPolicyRule,
        ExternalCanisterCallerMethodsPrivileges, ExternalCanisterCallerPrivileges,
        ExternalCanisterChangeRequestPolicyRule, ExternalCanisterPermissions,
        ExternalCanisterRequestPolicies, ExternalCanisterState, FundExternalCanisterOperation,
        FundExternalCanisterOperationInput, FundExternalCanisterOperationKind,
        FundExternalCanisterSendCyclesInput,
    },
    repositories::ExternalCanisterWhereClauseSort,
};
use candid::Principal;
use ic_cdk::api::management_canister::main::CanisterSettings;
use orbit_essentials::{repository::SortDirection, utils::timestamp_to_rfc3339};
use station_api::ExternalCanisterDTO;
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

impl ExternalCanister {
    pub fn into_dto(
        self,
        permissions: ExternalCanisterPermissions,
        policies: ExternalCanisterRequestPolicies,
    ) -> ExternalCanisterDTO {
        ExternalCanisterDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            canister_id: self.canister_id,
            name: self.name,
            description: self.description,
            labels: self.labels,
            state: self.state.into(),
            permissions: permissions.into(),
            request_policies: policies.into(),
            created_at: timestamp_to_rfc3339(&self.created_at),
            modified_at: self.modified_at.map(|ts| timestamp_to_rfc3339(&ts)),
        }
    }
}

impl From<ExternalCanisterPermissions> for station_api::ExternalCanisterPermissionsDTO {
    fn from(permissions: ExternalCanisterPermissions) -> Self {
        station_api::ExternalCanisterPermissionsDTO {
            read: permissions.read.into(),
            change: permissions.change.into(),
            calls: permissions.calls.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ExternalCanisterRequestPolicies> for station_api::ExternalCanisterRequestPoliciesDTO {
    fn from(policies: ExternalCanisterRequestPolicies) -> Self {
        station_api::ExternalCanisterRequestPoliciesDTO {
            change: policies.change.into_iter().map(Into::into).collect(),
            calls: policies.calls.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ExternalCanisterChangeRequestPolicyRule>
    for station_api::ExternalCanisterChangeRequestPolicyRuleDTO
{
    fn from(rule: ExternalCanisterChangeRequestPolicyRule) -> Self {
        station_api::ExternalCanisterChangeRequestPolicyRuleDTO {
            policy_id: Uuid::from_bytes(rule.policy_id).hyphenated().to_string(),
            rule: rule.rule.into(),
        }
    }
}

impl From<ExternalCanisterCallRequestPolicyRule>
    for station_api::ExternalCanisterCallRequestPolicyRuleDTO
{
    fn from(privileges: ExternalCanisterCallRequestPolicyRule) -> Self {
        station_api::ExternalCanisterCallRequestPolicyRuleDTO {
            policy_id: Uuid::from_bytes(privileges.policy_id)
                .hyphenated()
                .to_string(),
            rule: privileges.rule.into(),
            validation_method: privileges.validation_method.into(),
            execution_method: privileges.execution_method,
        }
    }
}

impl From<ExternalCanisterCallerPrivileges> for station_api::ExternalCanisterCallerPrivilegesDTO {
    fn from(privileges: ExternalCanisterCallerPrivileges) -> Self {
        station_api::ExternalCanisterCallerPrivilegesDTO {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            canister_id: privileges.canister_id,
            can_change: privileges.can_change,
            can_fund: privileges.can_fund,
            can_call: privileges.can_call.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<station_api::ListExternalCanistersSortInput> for ExternalCanisterWhereClauseSort {
    fn from(input: station_api::ListExternalCanistersSortInput) -> Self {
        match input {
            station_api::ListExternalCanistersSortInput::Name(direction) => {
                ExternalCanisterWhereClauseSort::Name(match direction {
                    station_api::SortDirection::Asc => SortDirection::Ascending,
                    station_api::SortDirection::Desc => SortDirection::Descending,
                })
            }
        }
    }
}

impl From<ExternalCanisterCallerMethodsPrivileges>
    for station_api::ExternalCanisterCallerMethodPrivilegesDTO
{
    fn from(privileges: ExternalCanisterCallerMethodsPrivileges) -> Self {
        station_api::ExternalCanisterCallerMethodPrivilegesDTO {
            validation_method: privileges.validation_method.into(),
            execution_method: privileges.execution_method,
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
            kind: input.kind.into(),
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
            state: input.state.map(Into::into),
            permissions: input.permissions.map(Into::into),
            request_policies: input.request_policies.map(Into::into),
        }
    }
}

impl From<ExternalCanisterState> for station_api::ExternalCanisterStateDTO {
    fn from(state: ExternalCanisterState) -> Self {
        match state {
            ExternalCanisterState::Active => station_api::ExternalCanisterStateDTO::Active,
            ExternalCanisterState::Archived => station_api::ExternalCanisterStateDTO::Archived,
        }
    }
}

impl From<station_api::ExternalCanisterStateDTO> for ExternalCanisterState {
    fn from(state: station_api::ExternalCanisterStateDTO) -> Self {
        match state {
            station_api::ExternalCanisterStateDTO::Active => ExternalCanisterState::Active,
            station_api::ExternalCanisterStateDTO::Archived => ExternalCanisterState::Archived,
        }
    }
}

impl From<FundExternalCanisterOperation> for station_api::FundExternalCanisterOperationDTO {
    fn from(operation: FundExternalCanisterOperation) -> Self {
        station_api::FundExternalCanisterOperationDTO {
            canister_id: operation.canister_id,
            kind: operation.kind.into(),
        }
    }
}

impl From<station_api::FundExternalCanisterOperationDTO> for FundExternalCanisterOperationInput {
    fn from(operation: station_api::FundExternalCanisterOperationDTO) -> Self {
        FundExternalCanisterOperationInput {
            canister_id: operation.canister_id,
            kind: operation.kind.into(),
        }
    }
}

impl From<FundExternalCanisterOperationKind> for station_api::FundExternalCanisterOperationKindDTO {
    fn from(kind: FundExternalCanisterOperationKind) -> Self {
        match kind {
            FundExternalCanisterOperationKind::Send(input) => {
                station_api::FundExternalCanisterOperationKindDTO::Send(input.into())
            }
        }
    }
}

impl From<station_api::FundExternalCanisterOperationKindDTO> for FundExternalCanisterOperationKind {
    fn from(kind: station_api::FundExternalCanisterOperationKindDTO) -> Self {
        match kind {
            station_api::FundExternalCanisterOperationKindDTO::Send(input) => {
                FundExternalCanisterOperationKind::Send(input.into())
            }
        }
    }
}

impl From<FundExternalCanisterSendCyclesInput>
    for station_api::FundExternalCanisterSendCyclesInput
{
    fn from(input: FundExternalCanisterSendCyclesInput) -> Self {
        station_api::FundExternalCanisterSendCyclesInput {
            cycles: input.cycles,
        }
    }
}

impl From<station_api::FundExternalCanisterSendCyclesInput>
    for FundExternalCanisterSendCyclesInput
{
    fn from(input: station_api::FundExternalCanisterSendCyclesInput) -> Self {
        FundExternalCanisterSendCyclesInput {
            cycles: input.cycles,
        }
    }
}
