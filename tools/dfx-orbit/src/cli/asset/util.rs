use crate::DfxOrbit;

use super::AssetAgent;
use candid::Principal;
use ic_certified_assets::types::{GrantPermissionArguments, Permission};
use orbit_station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, CreateRequestInput,
    CreateRequestResponse, RequestOperationInput,
};

impl DfxOrbit {
    /// Request from the station to grant the `Prepare` permission for the asset canister
    pub async fn request_prepare_permission(
        &self,
        canister_id: Principal,
        title: Option<String>,
        summary: Option<String>,
    ) -> anyhow::Result<CreateRequestResponse> {
        let me = self.own_principal()?;

        let args = GrantPermissionArguments {
            to_principal: me,
            permission: Permission::Prepare,
        };
        let arg = candid::encode_one(args)?;

        let response = self
            .station
            .request(CreateRequestInput {
                operation: RequestOperationInput::CallExternalCanister(
                    CallExternalCanisterOperationInput {
                        validation_method: None,
                        execution_method: CanisterMethodDTO {
                            canister_id,
                            method_name: String::from("grant_permission"),
                        },
                        arg: Some(arg),
                        execution_method_cycles: None,
                    },
                ),
                title,
                summary,
                execution_plan: None,
            })
            .await?;

        Ok(response)
    }
}

impl AssetAgent<'_> {
    // TODO: Turn into a functionality
    pub fn request_prepare_permission_payload(
        canister: Principal,
    ) -> Result<Vec<u8>, candid::Error> {
        let args = GrantPermissionArguments {
            to_principal: canister,
            permission: Permission::Prepare,
        };

        candid::encode_one(args)
    }
}
