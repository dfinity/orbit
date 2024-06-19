//! Command to put a canister under Orbit control.
use anyhow::anyhow;

use crate::{args::canister::Claim, dfx_extension_api, local_config};

/// Puts a canister controlled by the user under Orbit control.
pub fn exec(args: Claim) -> anyhow::Result<()> {
    let Claim {
        canister,
        exclusive,
    } = args;
    // TODO: Need to be able to specify which Orbit to use, e.g. as a global flag.
    let orbit_principal = &local_config::default_station()?
        .ok_or_else(|| anyhow!("No default station specified"))?
        .canister_id;
    let claim_type = if exclusive {
        "--set-controller"
    } else {
        "--add-controller"
    };
    dfx_extension_api::call_dfx_cli(vec![
        "canister",
        "update-settings",
        claim_type,
        orbit_principal,
        &canister,
    ])?;
    Ok(())
}
