use crate::core::validation::EnsureExternalCanister;
use crate::errors::ExternalCanisterError;
use crate::mappers::ExternalCanisterMapper;
use crate::models::{
    CreateExternalCanisterOperationInput, CreateExternalCanisterOperationKind, ExternalCanister,
    ExternalCanisterId,
};
use crate::repositories::{ExternalCanisterRepository, EXTERNAL_CANISTER_REPOSITORY};
use candid::{Encode, Principal};
use ic_cdk::api::call::call_raw;
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterIdRecord, CanisterStatusResponse, CreateCanisterArgument,
};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::model::ModelValidator;
use orbit_essentials::repository::Repository;
use std::sync::Arc;

lazy_static! {
    pub static ref EXTERNAL_CANISTER_SERVICE: Arc<ExternalCanisterService> = Arc::new(
        ExternalCanisterService::new(Arc::clone(&EXTERNAL_CANISTER_REPOSITORY))
    );
}

const CREATE_CANISTER_CYCLES: u128 = 100_000_000_000; // the default fee of 100 B cycles

#[derive(Default, Debug)]
pub struct ExternalCanisterService {
    external_canister_repository: Arc<ExternalCanisterRepository>,
}

impl ExternalCanisterService {
    pub fn new(external_canister_repository: Arc<ExternalCanisterRepository>) -> Self {
        Self {
            external_canister_repository,
        }
    }

    pub async fn create_canister(
        &self,
        cycles: Option<u128>,
    ) -> ServiceResult<Principal, ExternalCanisterError> {
        let create_canister_arg = CreateCanisterArgument { settings: None };

        let canister_id = mgmt::create_canister(
            create_canister_arg,
            cycles.unwrap_or(CREATE_CANISTER_CYCLES),
        )
        .await
        .map_err(|(_, err)| ExternalCanisterError::Failed {
            reason: err.to_string(),
        })?
        .0
        .canister_id;

        Ok(canister_id)
    }

    pub async fn canister_status(
        &self,
        input: CanisterIdRecord,
    ) -> ServiceResult<CanisterStatusResponse> {
        let canister_status_arg = CanisterIdRecord {
            canister_id: input.canister_id,
        };

        let canister_status_response = mgmt::canister_status(canister_status_arg)
            .await
            .map_err(|(_, err)| ExternalCanisterError::Failed {
                reason: err.to_string(),
            })?
            .0;

        Ok(canister_status_response)
    }

    pub async fn call_external_canister(
        &self,
        canister_id: Principal,
        method_name: String,
        arg: Option<Vec<u8>>,
        cycles: Option<u64>,
    ) -> ServiceResult<Vec<u8>, ExternalCanisterError> {
        EnsureExternalCanister::is_external_canister(canister_id)?;

        call_raw(
            canister_id,
            &method_name,
            arg.unwrap_or(Encode!(&()).unwrap()),
            cycles.unwrap_or_default(),
        )
        .await
        .map_err(|(_, err)| ExternalCanisterError::Failed {
            reason: err.to_string(),
        })
    }

    /// Adds a new external canister to the system.
    ///
    /// Can be used to create another canister to a subnet or add an existing canister.
    pub async fn add_external_canister(
        &self,
        input: CreateExternalCanisterOperationInput,
    ) -> ServiceResult<ExternalCanister> {
        self.assert_name_is_unique(input.name.clone().as_str(), None)?;

        let canister_id = match &input.kind {
            CreateExternalCanisterOperationKind::CreateNew(opts) => self
                .create_canister(opts.initial_cycles.map(|cycles| cycles as u128))
                .await
                .map_err(|err| ExternalCanisterError::Failed {
                    reason: format!("failed to create external canister: {}", err),
                })?,
            CreateExternalCanisterOperationKind::AddExisting(opts) => {
                EnsureExternalCanister::is_external_canister(opts.canister_id)?;

                opts.canister_id
            }
        };

        let external_canister = ExternalCanisterMapper::from_create_input(canister_id, input);

        external_canister.validate()?;

        self.external_canister_repository
            .insert(external_canister.to_key(), external_canister.clone());

        Ok(external_canister)
    }

    /// Verifies that the name is unique among external canisters.
    fn assert_name_is_unique(
        &self,
        name: &str,
        skip_id: Option<ExternalCanisterId>,
    ) -> ServiceResult<()> {
        if let Some(existing_id) = self.external_canister_repository.find_by_name(name) {
            if skip_id == Some(existing_id) {
                // The name is the same as the one being updated, so it's valid.
                return Ok(());
            }

            return Err(ExternalCanisterError::ValidationError {
                info: format!("The name '{}' is already in use.", name),
            })?;
        }

        Ok(())
    }
}
