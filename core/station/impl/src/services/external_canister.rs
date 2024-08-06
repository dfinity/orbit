use crate::core::validation::EnsureExternalCanister;
use crate::errors::ExternalCanisterError;
use crate::mappers::ExternalCanisterMapper;
use crate::models::{
    ConfigureExternalCanisterSettingsInput, CreateExternalCanisterOperationInput,
    CreateExternalCanisterOperationKind, DefiniteCanisterSettingsInput, ExternalCanister,
    ExternalCanisterId,
};
use crate::repositories::{ExternalCanisterRepository, EXTERNAL_CANISTER_REPOSITORY};
use candid::{Encode, Principal};
use ic_cdk::api::call::call_raw;
use ic_cdk::api::management_canister::main::{
    self as mgmt, delete_canister, deposit_cycles, stop_canister, update_settings,
    CanisterIdRecord, CanisterStatusResponse, CreateCanisterArgument, UpdateSettingsArgument,
};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::model::ModelValidator;
use orbit_essentials::repository::Repository;
use std::sync::Arc;
use uuid::Uuid;

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

    // Returns the external canister if found, otherwise an error.
    pub fn get_external_canister(
        &self,
        id: &ExternalCanisterId,
    ) -> ServiceResult<ExternalCanister> {
        let resource = self
            .external_canister_repository
            .get(&ExternalCanister::key(*id))
            .ok_or(ExternalCanisterError::NotFound {
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            })?;

        Ok(resource)
    }

    // Returns the external canister by its canister id if found, otherwise an error.
    pub fn get_external_canister_by_canister_id(
        &self,
        canister_id: &Principal,
    ) -> ServiceResult<ExternalCanister> {
        let recource_id = self
            .external_canister_repository
            .find_by_canister_id(canister_id)
            .ok_or(ExternalCanisterError::InvalidExternalCanister {
                principal: *canister_id,
            })?;

        self.get_external_canister(&recource_id)
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
        self.check_unique_name(input.name.clone().as_str(), None)?;

        let canister_id = match &input.kind {
            CreateExternalCanisterOperationKind::CreateNew(opts) => self
                .create_canister(opts.initial_cycles.map(|cycles| cycles as u128))
                .await
                .map_err(|err| ExternalCanisterError::Failed {
                    reason: format!("failed to create external canister: {}", err),
                })?,
            CreateExternalCanisterOperationKind::AddExisting(opts) => {
                EnsureExternalCanister::is_external_canister(opts.canister_id)?;

                self.check_unique_canister_id(&opts.canister_id, None)?;

                opts.canister_id
            }
        };

        let external_canister = ExternalCanisterMapper::from_create_input(canister_id, input);

        external_canister.validate()?;

        // todo: add permissions and request policies handling

        self.external_canister_repository
            .insert(external_canister.to_key(), external_canister.clone());

        Ok(external_canister)
    }

    /// Edits an external canister's settings.
    pub fn edit_external_canister(
        &self,
        id: &ExternalCanisterId,
        input: ConfigureExternalCanisterSettingsInput,
    ) -> ServiceResult<ExternalCanister> {
        let mut external_canister = self.get_external_canister(id)?;

        external_canister.update_with(input);
        external_canister.validate()?;

        // todo: add permissions and request policies handling

        self.external_canister_repository
            .insert(external_canister.to_key(), external_canister.clone());

        Ok(external_canister)
    }

    /// Adds cycles to the external canister, the cycles are taken from the station's balance.
    pub async fn top_up_canister(&self, canister_id: Principal, cycles: u128) -> ServiceResult<()> {
        if let Err((err_code, err_msg)) =
            deposit_cycles(CanisterIdRecord { canister_id }, cycles).await
        {
            Err(ExternalCanisterError::Failed {
                reason: format!(
                    "Failed to top up canister {} with {} cycles, code: {:?} and reason: {:?}",
                    canister_id.to_text(),
                    cycles,
                    err_code,
                    err_msg
                ),
            })?;
        }

        Ok(())
    }

    /// Only deletes the external canister from the system.
    pub fn soft_delete_external_canister(
        &self,
        id: &ExternalCanisterId,
    ) -> ServiceResult<ExternalCanister> {
        let external_canister = self.get_external_canister(id)?;
        self.external_canister_repository
            .remove(&external_canister.to_key());

        // todo: remove permissions and request policies

        Ok(external_canister)
    }

    /// Deletes an external canister from the system, as well as from the subnet.
    pub async fn hard_delete_external_canister(
        &self,
        id: &ExternalCanisterId,
    ) -> ServiceResult<ExternalCanister> {
        let external_canister = self.get_external_canister(id)?;

        // Deleting a canister requires the canister to be stopped first.
        //
        // See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-delete_canister
        if let Err((err_code, err_msg)) = stop_canister(CanisterIdRecord {
            canister_id: external_canister.canister_id,
        })
        .await
        {
            Err(ExternalCanisterError::Failed {
                reason: format!(
                    "Failed to stop canister {}, code: {:?} and reason: {:?}",
                    external_canister.canister_id.to_text(),
                    err_code,
                    err_msg
                ),
            })?;
        }

        if let Err((err_code, err_msg)) = delete_canister(CanisterIdRecord {
            canister_id: external_canister.canister_id,
        })
        .await
        {
            Err(ExternalCanisterError::Failed {
                reason: format!(
                    "Failed to delete canister {} from the subnet, code: {:?} and reason: {:?}",
                    external_canister.canister_id.to_text(),
                    err_code,
                    err_msg
                ),
            })?;
        }

        // The soft delete is done after the hard delete to ensure that the external canister
        // is removed from the subnet before it is removed from the system.
        //
        // The intercanister call is more likely to fail than the local operation.
        self.soft_delete_external_canister(id)?;

        Ok(external_canister)
    }

    /// Changes the IC settings of the external canister.
    pub async fn change_canister_ic_settings(
        &self,
        canister_id: Principal,
        settings: DefiniteCanisterSettingsInput,
    ) -> ServiceResult<()> {
        if let Err((err_code, err_msg)) = update_settings(UpdateSettingsArgument {
            canister_id,
            settings: settings.into(),
        })
        .await
        {
            Err(ExternalCanisterError::Failed {
                reason: format!(
                    "Failed to update canister {} settings, code: {:?} and reason: {:?}",
                    canister_id.to_text(),
                    err_code,
                    err_msg
                ),
            })?;
        }

        Ok(())
    }

    /// Verifies that the name is unique among external canisters.
    fn check_unique_name(
        &self,
        name: &str,
        skip_id: Option<ExternalCanisterId>,
    ) -> ServiceResult<()> {
        if !self
            .external_canister_repository
            .is_unique_name(name, skip_id)
        {
            Err(ExternalCanisterError::ValidationError {
                info: format!("The name '{}' is already in use.", name),
            })?;
        }

        Ok(())
    }

    /// Verifies that the canister id is unique among external canisters.
    fn check_unique_canister_id(
        &self,
        canister_id: &Principal,
        skip_id: Option<ExternalCanisterId>,
    ) -> ServiceResult<()> {
        if !self
            .external_canister_repository
            .is_unique_canister_id(canister_id, skip_id)
        {
            Err(ExternalCanisterError::ValidationError {
                info: format!("The canister id '{}' is already in use.", canister_id),
            })?;
        }

        Ok(())
    }
}
