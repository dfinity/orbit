#![allow(dead_code)]

use crate::DfxOrbit;
use anyhow::bail;
use candid::Principal;
use clap::Parser;
use station_api::{
    ConfigureExternalCanisterOperationInput, ConfigureExternalCanisterOperationKindDTO,
    DefiniteCanisterSettingsInput, GetRequestResponse, RequestOperationDTO, RequestOperationInput,
};
use std::collections::BTreeSet;

// ^ Utility function to get the latests response directly printed, to get UX similar to dfx canister call

#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterUpdateSettingsArgs {
    /// The canister name or ID.
    canister: String,

    /// Add a principal to the list of controllers of the canister
    #[clap(long)]
    pub(crate) add_controller: Vec<Principal>,

    /// Removes a principal from the list of controllers of the canister
    #[clap(long)]
    pub(crate) remove_controller: Vec<Principal>,
}

impl RequestCanisterUpdateSettingsArgs {
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let controllers = get_new_controller_set(
            dfx_orbit,
            canister_id,
            self.add_controller,
            self.remove_controller,
        )
        .await?;

        let operations = ConfigureExternalCanisterOperationInput {
            canister_id,
            kind: ConfigureExternalCanisterOperationKindDTO::NativeSettings(
                DefiniteCanisterSettingsInput {
                    controllers: Some(controllers),
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                    reserved_cycles_limit: None,
                },
            ),
        };

        Ok(RequestOperationInput::ConfigureExternalCanister(operations))
    }

    pub(crate) async fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let controllers = get_new_controller_set(
            dfx_orbit,
            canister_id,
            self.add_controller.clone(),
            self.remove_controller.clone(),
        )
        .await?;

        let RequestOperationDTO::ConfigureExternalCanister(op) = &request.request.operation else {
            bail!("This request is not a configure external canister request");
        };
        if op.canister_id != canister_id {
            bail!(
                "Mismatch of canister ids: request: {}, local: {}",
                op.canister_id,
                canister_id
            );
        }
        let ConfigureExternalCanisterOperationKindDTO::NativeSettings(op) = &op.kind else {
            bail!("This request is not a native setting request");
        };
        if op.controllers.as_ref() != Some(&controllers) {
            bail!(
                "Mismatch in the controller sets: request: {:?}, local {:?}",
                op.controllers,
                controllers
            );
        }

        Ok(())
    }
}

async fn get_new_controller_set(
    dfx_orbit: &DfxOrbit,
    canister_id: Principal,
    add: Vec<Principal>,
    remove: Vec<Principal>,
) -> anyhow::Result<Vec<Principal>> {
    // Transform into maps to deduplicates
    let old_controllers = dfx_orbit.get_controllers(canister_id).await?;
    let controllers = old_controllers
        .iter()
        .chain(add.iter())
        .collect::<BTreeSet<_>>();
    let remove = remove.iter().collect::<BTreeSet<_>>();

    let new_controllers = controllers
        .difference(&remove)
        .map(|&&v| v)
        .collect::<Vec<_>>();

    Ok(new_controllers)
}
