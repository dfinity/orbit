//! Command to put a canister under Orbit control.
use crate::{args::canister::CanisterClaimArgs, dfx_extension_api, local_config};
use anyhow::anyhow;

/// Puts a canister controlled by the user under Orbit control.
// TODO: Need to be able to specify which Orbit to use, e.g. as a global flag.
// TODO: Implement this without calling the `dfx` executable.
pub fn exec(args: CanisterClaimArgs) -> anyhow::Result<()> {
    let CanisterClaimArgs {
        canister,
        exclusive,
    } = args;
    let orbit_principal = &local_config::default_station()?
        .ok_or_else(|| anyhow!("No default station specified"))?
        .station_id;
    let claim_type = if exclusive {
        "--set-controller"
    } else {
        "--add-controller"
    };
    let network = local_config::default_station()?
        .ok_or_else(|| anyhow!("No default station specified"))?
        .network;
    dfx_extension_api::call_dfx_cli(vec![
        "canister",
        "update-settings",
        "--network",
        &network,
        claim_type,
        orbit_principal,
        &canister,
    ])?;
    Ok(())
}
