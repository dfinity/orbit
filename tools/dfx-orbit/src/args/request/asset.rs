use crate::DfxOrbit;
use anyhow::bail;
use candid::Principal;
use clap::{Parser, Subcommand};
use ic_certified_assets::types::{GrantPermissionArguments, Permission};
use sha2::{Digest, Sha256};
use station_api::{GetRequestResponse, RequestOperationDTO, RequestOperationInput};

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetArgs {
    #[clap(subcommand)]
    pub(crate) action: RequestAssetActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestAssetActionArgs {
    /// Request to grant this user Prepare permission for the asset canister
    PreparePermission(RequestAssetPreparePermissionArgs),
    /// Upload assets to an asset canister
    Upload(RequestAssetUploadArgs),
}

impl RequestAssetArgs {
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        match self.action {
            RequestAssetActionArgs::PreparePermission(args) => args.into_request(dfx_orbit),
            RequestAssetActionArgs::Upload(args) => args.into_request(dfx_orbit).await,
        }
    }
}

// TODO: Verification call for this request
#[derive(Debug, Clone, Parser)]
pub struct RequestAssetPreparePermissionArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,
    /// The principal to grant the prepare permission to (defaults to self)
    pub id: Option<Principal>,
}

impl RequestAssetPreparePermissionArgs {
    pub(crate) fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let me = dfx_orbit.own_principal()?;
        let to_principal = self.id.unwrap_or(me);
        let asset_canister = dfx_orbit.canister_id(&self.canister)?;
        DfxOrbit::grant_permission_request(asset_canister, to_principal)
    }

    pub(crate) fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let RequestOperationDTO::CallExternalCanister(operation) = &request.request.operation
        else {
            bail!("The request is not a call external canister request");
        };

        let asset_canister = dfx_orbit.canister_id(&self.canister)?;
        if operation.execution_method.canister_id != asset_canister {
            bail!(
                "The request targets an unexpected canister. Expected: {}, actual: {}",
                asset_canister,
                operation.execution_method.canister_id
            );
        }
        if &operation.execution_method.method_name != "grant_permission" {
            bail!(
                "The method of this request is not \"grant_permission\" but \"{}\" instead",
                operation.execution_method.method_name
            );
        }

        let me = dfx_orbit.own_principal()?;
        let to_principal = self.id.unwrap_or(me);
        let args = GrantPermissionArguments {
            to_principal,
            permission: Permission::Prepare,
        };
        let arg = candid::encode_one(args)?;
        let computed_arg_checksum = hex::encode(Sha256::digest(arg));

        if operation.arg_checksum != Some(computed_arg_checksum) {
            bail!("Argument checksum does not match");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// Do not abort the upload, if the evidence does not match between local and remote calculation
    #[clap(long)]
    pub(crate) ignore_evidence: bool,

    /// The source directories to upload (multiple values possible)
    pub(crate) files: Vec<String>,
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
