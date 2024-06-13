//! Implements the dfx extension CLI commands for making requests about canisters.

use crate::args::request::canister::{Args, ChangeExternalCanister};
use anyhow::{anyhow, Context};
use candid::{CandidType, IDLArgs};
use orbit_station_api::{CreateRequestInput, RequestOperationInput};
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

/// The main entry point for the `dfx orbit` CLI.
pub fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Change(change_args) => change(change_args),
    }
}

/// Makes an API call to chnage an external canister.
fn change(args: ChangeExternalCanister) -> anyhow::Result<()> {
    // If we can be SURE that the orbit `station_api` types remain in sync with the .did files, we can use these types.
    let args = orbit_station_api::ChangeExternalCanisterOperationInput::from(args);
    let args = RequestOperationInput::ChangeExternalCanister(args);
    // TODO: Add title, summary and execution_plan to the CLI.
    let args = CreateRequestInput {
        operation: args,
        title: None,
        summary: None,
        execution_plan: None,
    };
    let idl_text = serialize_one_to_text(&args)?;
    // The idl text can be too large to pass on gthe command line.  We write it to a file and pass the file name instead.
    let dir = tempdir()?;
    let file_path = dir.path().join("args.idl");
    let file_name = file_path
        .to_str()
        .ok_or_else(|| anyhow!("Could not convert path to string"))?;
    let mut arg_file = File::create(&file_path)?;
    arg_file.write_all(idl_text.as_bytes())?;
    arg_file.flush()?;
    let orbit_canister_id = crate::local_config::default_station()?
        .ok_or_else(|| anyhow!("No default station specified"))?
        .canister_id;
    let command = vec![
        "canister",
        "call",
        &orbit_canister_id,
        "create_request",
        "--argument-file",
        &file_name,
    ];
    crate::dfx_extension_api::call_dfx_cli(command.clone())
    .with_context(|| {
        let saved_filename = "args.idl";
        fs::rename(file_name, saved_filename).ok();
        format!(
            "Failed to call the Orbit canister.  The argument file has been saved as {}.  The command was:\n dfx {}",
            saved_filename,
            command.join(" ")
        )
    })?; // TODO: Replace with actual API call
    Ok(())
}

/// Serializes a value to a Candid string.
fn serialize_one_to_text<T: CandidType>(value: &T) -> anyhow::Result<String> {
    // Afaik there still is no better way of doing this than serializing to binary candid, then convertingh the binary candid to text-type candid.  If true this is really unfortunate.
    let bytes = candid::encode_one(value)?;
    let decoded: IDLArgs = IDLArgs::from_bytes(&bytes)?;
    let text = decoded.to_string();
    Ok(text)
}
