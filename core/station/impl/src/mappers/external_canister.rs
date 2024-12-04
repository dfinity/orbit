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
        FundExternalCanisterSendCyclesInput, LogVisibility, MonitorExternalCanisterOperationInput,
        MonitorExternalCanisterOperationKind, MonitorExternalCanisterStartInput,
        MonitorExternalCanisterStrategy, MonitoringExternalCanisterCyclesThresholdInput,
        MonitoringExternalCanisterEstimatedRuntimeInput,
    },
    repositories::ExternalCanisterWhereClauseSort,
};
use candid::Principal;
use canfund::manager::options::{CyclesThreshold, EstimatedRuntime};
use ic_cdk::api::management_canister::main::{self as mgmt};
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
            name: input.name,
            description: input.description,
            labels: input.labels.unwrap_or_default(),
            metadata: input.metadata.unwrap_or_default(),
            state: ExternalCanisterState::Active,
            created_at: next_time(),
            modified_at: None,
            monitoring: None,
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
            metadata: self.metadata.into(),
            state: self.state.into(),
            permissions: permissions.into(),
            request_policies: policies.into(),
            created_at: timestamp_to_rfc3339(&self.created_at),
            modified_at: self.modified_at.map(|ts| timestamp_to_rfc3339(&ts)),
            monitoring: self.monitoring.map(Into::into),
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

impl From<LogVisibility> for mgmt::LogVisibility {
    fn from(input: LogVisibility) -> Self {
        match input {
            LogVisibility::Public => mgmt::LogVisibility::Public,
            LogVisibility::Controllers => mgmt::LogVisibility::Controllers,
        }
    }
}

impl From<DefiniteCanisterSettingsInput> for mgmt::CanisterSettings {
    fn from(input: DefiniteCanisterSettingsInput) -> Self {
        mgmt::CanisterSettings {
            controllers: input.controllers,
            compute_allocation: input.compute_allocation,
            freezing_threshold: input.freezing_threshold,
            memory_allocation: input.memory_allocation,
            reserved_cycles_limit: input.reserved_cycles_limit,
            log_visibility: input
                .log_visibility
                .map(|log_visibility| log_visibility.into()),
            wasm_memory_limit: input.wasm_memory_limit,
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

impl From<station_api::LogVisibility> for LogVisibility {
    fn from(input: station_api::LogVisibility) -> Self {
        match input {
            station_api::LogVisibility::Public => LogVisibility::Public,
            station_api::LogVisibility::Controllers => LogVisibility::Controllers,
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
            log_visibility: input
                .log_visibility
                .map(|log_visibility| log_visibility.into()),
            wasm_memory_limit: input.wasm_memory_limit,
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
            change_metadata: input.change_metadata.map(Into::into),
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

impl From<station_api::MonitorExternalCanisterOperationInput>
    for MonitorExternalCanisterOperationInput
{
    fn from(input: station_api::MonitorExternalCanisterOperationInput) -> Self {
        MonitorExternalCanisterOperationInput {
            canister_id: input.canister_id,
            kind: input.kind.into(),
        }
    }
}

impl From<MonitorExternalCanisterOperationInput>
    for station_api::MonitorExternalCanisterOperationInput
{
    fn from(input: MonitorExternalCanisterOperationInput) -> Self {
        station_api::MonitorExternalCanisterOperationInput {
            canister_id: input.canister_id,
            kind: input.kind.into(),
        }
    }
}

impl From<station_api::MonitorExternalCanisterOperationKindDTO>
    for MonitorExternalCanisterOperationKind
{
    fn from(kind: station_api::MonitorExternalCanisterOperationKindDTO) -> Self {
        match kind {
            station_api::MonitorExternalCanisterOperationKindDTO::Start(input) => {
                MonitorExternalCanisterOperationKind::Start(input.into())
            }
            station_api::MonitorExternalCanisterOperationKindDTO::Stop => {
                MonitorExternalCanisterOperationKind::Stop
            }
        }
    }
}

impl From<MonitorExternalCanisterOperationKind>
    for station_api::MonitorExternalCanisterOperationKindDTO
{
    fn from(kind: MonitorExternalCanisterOperationKind) -> Self {
        match kind {
            MonitorExternalCanisterOperationKind::Start(input) => {
                station_api::MonitorExternalCanisterOperationKindDTO::Start(input.into())
            }
            MonitorExternalCanisterOperationKind::Stop => {
                station_api::MonitorExternalCanisterOperationKindDTO::Stop
            }
        }
    }
}

impl From<station_api::MonitorExternalCanisterStartInput> for MonitorExternalCanisterStartInput {
    fn from(input: station_api::MonitorExternalCanisterStartInput) -> Self {
        MonitorExternalCanisterStartInput {
            funding_strategy: input.funding_strategy.into(),
            cycle_obtain_strategy: input.cycle_obtain_strategy.map(Into::into),
        }
    }
}

impl From<MonitorExternalCanisterStartInput> for station_api::MonitorExternalCanisterStartInput {
    fn from(input: MonitorExternalCanisterStartInput) -> Self {
        station_api::MonitorExternalCanisterStartInput {
            funding_strategy: input.funding_strategy.into(),
            cycle_obtain_strategy: input.cycle_obtain_strategy.map(Into::into),
        }
    }
}

impl From<station_api::MonitorExternalCanisterStrategyDTO> for MonitorExternalCanisterStrategy {
    fn from(strategy: station_api::MonitorExternalCanisterStrategyDTO) -> Self {
        match strategy {
            station_api::MonitorExternalCanisterStrategyDTO::Always(cycles) => {
                MonitorExternalCanisterStrategy::Always(cycles)
            }
            station_api::MonitorExternalCanisterStrategyDTO::BelowThreshold(threshold) => {
                MonitorExternalCanisterStrategy::BelowThreshold(threshold.into())
            }
            station_api::MonitorExternalCanisterStrategyDTO::BelowEstimatedRuntime(runtime) => {
                MonitorExternalCanisterStrategy::BelowEstimatedRuntime(runtime.into())
            }
        }
    }
}

impl From<MonitorExternalCanisterStrategy> for station_api::MonitorExternalCanisterStrategyDTO {
    fn from(strategy: MonitorExternalCanisterStrategy) -> Self {
        match strategy {
            MonitorExternalCanisterStrategy::Always(cycles) => {
                station_api::MonitorExternalCanisterStrategyDTO::Always(cycles)
            }
            MonitorExternalCanisterStrategy::BelowThreshold(threshold) => {
                station_api::MonitorExternalCanisterStrategyDTO::BelowThreshold(threshold.into())
            }
            MonitorExternalCanisterStrategy::BelowEstimatedRuntime(runtime) => {
                station_api::MonitorExternalCanisterStrategyDTO::BelowEstimatedRuntime(
                    runtime.into(),
                )
            }
        }
    }
}

impl From<station_api::MonitoringExternalCanisterCyclesThresholdInput>
    for MonitoringExternalCanisterCyclesThresholdInput
{
    fn from(input: station_api::MonitoringExternalCanisterCyclesThresholdInput) -> Self {
        MonitoringExternalCanisterCyclesThresholdInput {
            min_cycles: input.min_cycles,
            fund_cycles: input.fund_cycles,
        }
    }
}

impl From<MonitoringExternalCanisterCyclesThresholdInput>
    for station_api::MonitoringExternalCanisterCyclesThresholdInput
{
    fn from(input: MonitoringExternalCanisterCyclesThresholdInput) -> Self {
        station_api::MonitoringExternalCanisterCyclesThresholdInput {
            min_cycles: input.min_cycles,
            fund_cycles: input.fund_cycles,
        }
    }
}

impl From<station_api::MonitoringExternalCanisterEstimatedRuntimeInput>
    for MonitoringExternalCanisterEstimatedRuntimeInput
{
    fn from(input: station_api::MonitoringExternalCanisterEstimatedRuntimeInput) -> Self {
        MonitoringExternalCanisterEstimatedRuntimeInput {
            fund_runtime_secs: input.fund_runtime_secs,
            min_runtime_secs: input.min_runtime_secs,
            max_runtime_cycles_fund: input.max_runtime_cycles_fund,
            fallback_fund_cycles: input.fallback_fund_cycles,
            fallback_min_cycles: input.fallback_min_cycles,
        }
    }
}

impl From<MonitoringExternalCanisterEstimatedRuntimeInput>
    for station_api::MonitoringExternalCanisterEstimatedRuntimeInput
{
    fn from(input: MonitoringExternalCanisterEstimatedRuntimeInput) -> Self {
        station_api::MonitoringExternalCanisterEstimatedRuntimeInput {
            fund_runtime_secs: input.fund_runtime_secs,
            min_runtime_secs: input.min_runtime_secs,
            max_runtime_cycles_fund: input.max_runtime_cycles_fund,
            fallback_fund_cycles: input.fallback_fund_cycles,
            fallback_min_cycles: input.fallback_min_cycles,
        }
    }
}

impl From<MonitorExternalCanisterStrategy> for canfund::manager::options::FundStrategy {
    fn from(strategy: MonitorExternalCanisterStrategy) -> Self {
        match strategy {
            MonitorExternalCanisterStrategy::Always(cycles) => {
                canfund::manager::options::FundStrategy::Always(cycles)
            }
            MonitorExternalCanisterStrategy::BelowThreshold(threshold) => {
                canfund::manager::options::FundStrategy::BelowThreshold(threshold.into())
            }
            MonitorExternalCanisterStrategy::BelowEstimatedRuntime(runtime) => {
                canfund::manager::options::FundStrategy::BelowEstimatedRuntime(runtime.into())
            }
        }
    }
}

impl From<MonitoringExternalCanisterCyclesThresholdInput> for CyclesThreshold {
    fn from(input: MonitoringExternalCanisterCyclesThresholdInput) -> Self {
        CyclesThreshold::new()
            .with_min_cycles(input.min_cycles)
            .with_fund_cycles(input.fund_cycles)
    }
}

impl From<MonitoringExternalCanisterEstimatedRuntimeInput> for EstimatedRuntime {
    fn from(input: MonitoringExternalCanisterEstimatedRuntimeInput) -> Self {
        EstimatedRuntime::new()
            .with_fund_runtime_secs(input.fund_runtime_secs)
            .with_min_runtime_secs(input.min_runtime_secs)
            .with_max_runtime_cycles_fund(input.max_runtime_cycles_fund)
            .with_fallback_fund_cycles(input.fallback_fund_cycles)
            .with_fallback_min_cycles(input.fallback_min_cycles)
    }
}
