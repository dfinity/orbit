use crate::DfxOrbit;
use anyhow::bail;
use candid::Principal;
use dfx_core::config::model::dfinity::CanisterTypeProperties;
use ic_certified_assets::types::{GrantPermissionArguments, Permission};
use orbit_station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, CreateRequestInput,
    CreateRequestResponse, RequestOperationInput,
};
use std::path::{Path, PathBuf};

impl DfxOrbit {
    /// Request from the station to grant the `Prepare` permission for the asset canister
    pub async fn request_prepare_permission(
        &self,
        canister_id: Principal,
        title: Option<String>,
        summary: Option<String>,
    ) -> anyhow::Result<CreateRequestResponse> {
        let me = self.own_principal()?;

        let response = self
            .station
            .request(CreateRequestInput {
                operation: Self::grant_permission_request(canister_id, me)?,
                title,
                summary,
                execution_plan: None,
            })
            .await?;

        Ok(response)
    }

    pub(crate) fn grant_permission_request(
        asset_canister: Principal,
        to_principal: Principal,
    ) -> anyhow::Result<RequestOperationInput> {
        let args = GrantPermissionArguments {
            to_principal,
            permission: Permission::Prepare,
        };
        let arg = candid::encode_one(args)?;

        Ok(RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id: asset_canister,
                    method_name: String::from("grant_permission"),
                },
                arg: Some(arg),
                execution_method_cycles: None,
            },
        ))
    }

    pub fn as_path_bufs(&self, canister: &str, paths: &[String]) -> anyhow::Result<Vec<PathBuf>> {
        if paths.is_empty() {
            let canister_config = self.get_canister_config(canister)?;
            let CanisterTypeProperties::Assets { source, .. } = &canister_config.type_specific
            else {
                bail!("Canister {canister} is not an asset canister");
            };
            Ok(source.clone())
        } else {
            Ok(paths.iter().map(|source| PathBuf::from(&source)).collect())
        }
    }

    pub(crate) fn as_paths(paths: &[PathBuf]) -> Vec<&Path> {
        paths.iter().map(|pathbuf| pathbuf.as_path()).collect()
    }
}
