use crate::{
    core::ic_cdk::api::trap,
    core::middlewares::{authorize, call_context},
    models::resource::{ExternalCanisterId, ExternalCanisterResourceAction, Resource},
    services::{ExternalCanisterService, EXTERNAL_CANISTER_SERVICE},
};
use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterStatusResponse};
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{
    ExternalCanisterCallerPrivilegesDTO, GetExternalCanisterFiltersInput,
    GetExternalCanisterFiltersResponse, GetExternalCanisterInput, GetExternalCanisterResponse,
    ListExternalCanistersInput, ListExternalCanistersResponse, Snapshot,
};
use std::sync::Arc;

// Canister entrypoints for the controller.
#[update(name = "canister_status")]
async fn canister_status(input: CanisterIdRecord) -> CanisterStatusResponse {
    CONTROLLER
        .canister_status(input)
        .await
        .unwrap_or_else(|e| trap(&format!("{:?}", e)))
}

#[update(name = "canister_snapshots")]
async fn canister_snapshots(input: CanisterIdRecord) -> ApiResult<Vec<Snapshot>> {
    CONTROLLER.canister_snapshots(input).await
}

#[query(name = "get_external_canister")]
async fn get_external_canister(
    input: GetExternalCanisterInput,
) -> ApiResult<GetExternalCanisterResponse> {
    CONTROLLER.get_external_canister(input).await
}

#[query(name = "list_external_canisters")]
async fn list_external_canisters(
    input: ListExternalCanistersInput,
) -> ApiResult<ListExternalCanistersResponse> {
    CONTROLLER.list_external_canisters(input).await
}

#[query(name = "get_external_canister_filters")]
async fn get_external_canister_filters(
    input: GetExternalCanisterFiltersInput,
) -> ApiResult<GetExternalCanisterFiltersResponse> {
    CONTROLLER.get_external_canister_filters(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: ExternalCanisterController =
        ExternalCanisterController::new(Arc::clone(&EXTERNAL_CANISTER_SERVICE));
}

#[derive(Debug, Default)]
pub struct ExternalCanisterController {
    canister_service: Arc<ExternalCanisterService>,
}

impl ExternalCanisterController {
    fn new(canister_service: Arc<ExternalCanisterService>) -> Self {
        Self { canister_service }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::Read(ExternalCanisterId::Canister(input.canister_id)))]))]
    async fn canister_status(&self, input: CanisterIdRecord) -> ApiResult<CanisterStatusResponse> {
        self.canister_service.canister_status(input).await
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::Read(ExternalCanisterId::Canister(input.canister_id)))]))]
    async fn canister_snapshots(&self, input: CanisterIdRecord) -> ApiResult<Vec<Snapshot>> {
        self.canister_service
            .canister_snapshots(input)
            .await
            .map(|snapshots| snapshots.into_iter().map(|s| s.into()).collect())
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::Read(ExternalCanisterId::Canister(input.canister_id)))]))]
    async fn get_external_canister(
        &self,
        input: GetExternalCanisterInput,
    ) -> ApiResult<GetExternalCanisterResponse> {
        let ctx = call_context();
        let external_canister = self
            .canister_service
            .get_external_canister_by_canister_id(&input.canister_id)?;
        let external_canister_policies = self
            .canister_service
            .get_external_canister_request_policies(&external_canister.canister_id);
        let external_canister_permissions = self
            .canister_service
            .get_external_canister_permissions(&external_canister.canister_id);
        let caller_privileges = self
            .canister_service
            .get_caller_privileges_for_external_canister(
                &external_canister.id,
                &external_canister.canister_id,
                &ctx,
            );

        Ok(GetExternalCanisterResponse {
            canister: external_canister
                .into_dto(external_canister_permissions, external_canister_policies),
            privileges: caller_privileges.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::List)]))]
    async fn list_external_canisters(
        &self,
        input: ListExternalCanistersInput,
    ) -> ApiResult<ListExternalCanistersResponse> {
        let ctx = call_context();
        let result = self.canister_service.list_external_canisters(input, &ctx)?;

        let mut privileges = Vec::new();
        for external_canister in &result.items {
            let caller_privileges = self
                .canister_service
                .get_caller_privileges_for_external_canister(
                    &external_canister.id,
                    &external_canister.canister_id,
                    &ctx,
                );

            privileges.push(ExternalCanisterCallerPrivilegesDTO::from(caller_privileges));
        }

        Ok(ListExternalCanistersResponse {
            canisters: result
                .items
                .into_iter()
                .map(|external_canister| {
                    let policies = self
                        .canister_service
                        .get_external_canister_permissions(&external_canister.canister_id);
                    let permissions = self
                        .canister_service
                        .get_external_canister_request_policies(&external_canister.canister_id);

                    external_canister.into_dto(policies, permissions)
                })
                .collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ExternalCanister(ExternalCanisterResourceAction::List)]))]
    async fn get_external_canister_filters(
        &self,
        input: GetExternalCanisterFiltersInput,
    ) -> ApiResult<GetExternalCanisterFiltersResponse> {
        let ctx = call_context();
        let filters = self
            .canister_service
            .available_external_canisters_filters(input, &ctx);

        Ok(GetExternalCanisterFiltersResponse {
            names: filters.names,
            labels: filters.labels,
        })
    }
}
