//! Implements the dfx extension CLI commands for making requests.

pub mod canister;
pub mod permission;

use crate::args::request::Args;
use anyhow::anyhow;
use candid::{CandidType, IDLArgs, Principal};
use orbit_station_api::{ApiErrorDTO, CreateRequestInput, CreateRequestResponse};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

/// The main entry point for the `dfx orbit` CLI.
pub async fn main(args: Args) -> anyhow::Result<Result<CreateRequestResponse, ApiErrorDTO>> {
    // Converts the CLI arg type into the equivalent Orbit API type.
    let args = orbit_station_api::RequestOperationInput::from(args);
    // TODO: Add title, summary and execution_plan to the CLI.
    // TODO: Mopve this conversion to the cli types module.
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
    let mut arg_file = File::create(&file_path)?;
    arg_file.write_all(idl_text.as_bytes())?;
    arg_file.flush()?;
    let orbit_canister_id = crate::local_config::default_station()?
        .ok_or_else(|| anyhow!("No default station specified"))?
        .canister_id;

    let mut extension_agent = crate::dfx_extension_api::DfxExtensionAgent::new("orbit");
    let agent = extension_agent.agent().await?;
    let canister_id = Principal::from_text(&orbit_canister_id)?;
    let bytes = agent
        .update(&canister_id, "create_request")
        .with_arg(candid::encode_one(args)?)
        .call_and_wait()
        .await?;
    let ans: Result<CreateRequestResponse, ApiErrorDTO> = candid::decode_one(&bytes)?;
    println!("{ans:#?}");
    Ok(ans)
}

/// Serializes a value to a Candid string.
fn serialize_one_to_text<T: CandidType>(value: &T) -> anyhow::Result<String> {
    // Afaik there still is no better way of doing this than serializing to binary candid, then converting the binary candid to text-type candid.  If true this is really unfortunate.
    let bytes = candid::encode_one(value)?;
    let decoded: IDLArgs = IDLArgs::from_bytes(&bytes)?;
    let text = decoded.to_string();
    Ok(text)
}
