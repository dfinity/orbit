use crate::DfxOrbit;
use anyhow::bail;
use candid::{Nat, Principal};
use clap::Parser;
use ic_certified_assets::types::{CommitProposedBatchArguments, DeleteBatchArguments};
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};
use slog::{info, warn};
use station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, GetRequestResponse, RequestOperationDTO,
    RequestOperationInput,
};
use std::path::Path;

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,

    /// Do not abort the upload, if the evidence does not match between local and remote calculation
    #[clap(long)]
    pub ignore_evidence: bool,

    /// The source directories to upload
    /// (multiple values possible, picks up sources from dfx.json by default)
    #[clap(short, long)]
    pub files: Vec<String>,
}

impl RequestAssetUploadArgs {
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let pathbufs = dfx_orbit.as_path_bufs(&self.canister, &self.files)?;
        let paths = DfxOrbit::as_paths(&pathbufs);
        let canister_id = dfx_orbit.canister_id(&self.canister)?;

        let (batch_id, evidence) = dfx_orbit
            .upload(canister_id, &paths, self.ignore_evidence)
            .await?;
        println!("Batch id: {batch_id}");
        println!("Evidence: {}", hex::encode(&evidence));

        DfxOrbit::commit_batch_input(canister_id, batch_id, evidence)
    }
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,

    /// The batch ID to commit to
    #[clap(short, long)]
    pub batch_id: Nat,

    /// The source directories to upload
    /// (multiple values possible, picks up sources from dfx.json by default)
    #[clap(short, long)]
    pub files: Vec<String>,
}

impl VerifyAssetUploadArgs {
    pub(crate) async fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let pathbufs = dfx_orbit.as_path_bufs(&self.canister, &self.files)?;
        let paths = DfxOrbit::as_paths(&pathbufs);

        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let asset_agent = dfx_orbit.asset_agent(canister_id)?;

        let evidence = asset_agent.compute_evidence(&paths).await?;

        println!("Computed evidence: 0x{evidence}");
        DfxOrbit::check_evidence(request, canister_id, self.batch_id.clone(), evidence)?;

        Ok(())
    }
}

impl DfxOrbit {
    pub async fn upload(
        &self,
        canister_id: Principal,
        sources: &[&Path],
        ignore_evidence: bool,
    ) -> anyhow::Result<(Nat, ByteBuf)> {
        let asset_agent = self.asset_agent(canister_id)?;
        let (batch_id, evidence) = asset_agent.upload_assets(sources).await?;

        let remote_evidence = hex::encode(&evidence);
        let local_evidence = asset_agent.compute_evidence(sources).await?;

        if !ignore_evidence {
            if local_evidence != remote_evidence {
                warn!(
                    self.logger,
                    "Local evidence does not match remotely calculated evidence"
                );
                warn!(self.logger, "Local:  {local_evidence}");
                warn!(self.logger, "Remote: {remote_evidence}");
                bail!("Evidence did not match!");
            } else {
                info!(self.logger, "Local and remote evidence match!");
            }
        }

        Ok((batch_id, evidence))
    }

    pub fn commit_batch_input(
        canister_id: Principal,
        batch_id: Nat,
        evidence: ByteBuf,
    ) -> anyhow::Result<RequestOperationInput> {
        let args = CommitProposedBatchArguments { batch_id, evidence };
        let arg = candid::encode_one(args)?;

        Ok(RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id,
                    method_name: String::from("commit_proposed_batch"),
                },
                arg: Some(arg),
                execution_method_cycles: None,
            },
        ))
    }

    pub fn check_evidence(
        request: &GetRequestResponse,
        canister_id: Principal,
        batch_id: Nat,
        evidence: String,
    ) -> anyhow::Result<()> {
        // Check:
        // - Request is actually a CallExternalCanister
        // - Target is the canister we are expecting
        // - Method is `propose_commit_batch`
        // - `arg_checksum` exists
        let RequestOperationDTO::CallExternalCanister(request) = &request.request.operation else {
            bail!("{} is not an external canister request. Are you sure you have the correct request id?", {&request.request.id});
        };
        let CanisterMethodDTO {
            canister_id: request_canister_id,
            method_name,
        } = &request.execution_method;
        if *request_canister_id != canister_id {
            bail!(
                "Canister id of the request {} does not match canister id of asset canister {}",
                request_canister_id,
                canister_id
            );
        }
        if method_name != "commit_proposed_batch" {
            bail!(
                "Method name if the request is not \"commit_proposed_batch\", but instead \"{}\"",
                method_name
            );
        }
        let Some(remote_checksum) = &request.arg_checksum else {
            bail!("The request has no arguments. This likely means that is is malformed.");
        };

        // Now we check that the argument that we construct locally matches the hash of the argument
        let evidence = hex::decode(evidence)?;
        let args = CommitProposedBatchArguments {
            batch_id,
            evidence: ByteBuf::from(evidence),
        };
        let arg = candid::encode_one(args)?;
        let local_checksum = hex::encode(Sha256::digest(arg));

        if &local_checksum != remote_checksum {
            bail!("Local evidence does not match expected arguments");
        }

        Ok(())
    }

    pub fn cancel_batch_input(
        canister_id: Principal,
        batch_id: Nat,
    ) -> anyhow::Result<RequestOperationInput> {
        let args = DeleteBatchArguments { batch_id };
        let arg = candid::encode_one(args)?;

        Ok(RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id,
                    method_name: String::from("delete_batch"),
                },
                arg: Some(arg),
                execution_method_cycles: None,
            },
        ))
    }
}
