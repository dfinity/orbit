//! Implementation of the `dfx-orbit me` command.

use anyhow::anyhow;

/// A command line tool for interacting with Orbit on the Internet Computer.
pub async fn exec() -> anyhow::Result<()> {
    let station_principal = &crate::local_config::default_station()?
        .ok_or_else(|| anyhow!("No default station specified"))?
        .station_id;
    let ans =
        crate::dfx_extension_api::call_dfx_cli(vec!["canister", "call", station_principal, "me"])?;
    print!("{ans}");
    Ok(())
}
