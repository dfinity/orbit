use crate::DfxOrbit;
use anyhow::bail;
use candid::Principal;
use clap::{Parser, Subcommand, ValueEnum};
use ic_certified_assets::types::{GrantPermissionArguments, Permission, RevokePermissionArguments};
use sha2::{Digest, Sha256};
use station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, GetRequestResponse, RequestOperationDTO,
    RequestOperationInput,
};

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetArgs {
    #[clap(subcommand)]
    pub action: RequestAssetActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestAssetActionArgs {
    /// Request to grant a user permissions for an asset canister
    Permission(RequestAssetPermissionArgs),
    /// Upload assets to an asset canister
    Upload(RequestAssetUploadArgs),
}

impl RequestAssetArgs {
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        match self.action {
            RequestAssetActionArgs::Permission(args) => args.into_request(dfx_orbit),
            RequestAssetActionArgs::Upload(args) => args.into_request(dfx_orbit).await,
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetPermissionArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,
    /// The type of permission to grant / revoke
    pub permission: AssetPermissionTypeArgs,
    /// The principal to grant the prepare permission to (defaults to self)
    #[clap(short, long)]
    pub target: Option<Principal>,
    /// Request to revoke (rather than grant) the permission
    #[clap(short, long)]
    pub revoke: bool,
}

impl RequestAssetPermissionArgs {
    pub(crate) fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let me = dfx_orbit.own_principal()?;
        let target = self.target.unwrap_or(me);
        let asset_canister = dfx_orbit.canister_id(&self.canister)?;

        Ok(RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id: asset_canister,
                    method_name: self.method_name(),
                },
                arg: Some(self.encoded_args(target)?),
                execution_method_cycles: None,
            },
        ))
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

        let expected_method = self.method_name();
        if operation.execution_method.method_name != expected_method {
            bail!(
                "The method of this request is not \"{}\" but \"{}\" instead",
                expected_method,
                operation.execution_method.method_name
            );
        }

        let me = dfx_orbit.own_principal()?;
        let target = self.target.unwrap_or(me);
        let arg = self.encoded_args(target)?;
        let computed_arg_checksum = hex::encode(Sha256::digest(arg));

        if operation.arg_checksum != Some(computed_arg_checksum) {
            bail!("Argument checksum does not match");
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        match self.revoke {
            false => String::from("grant_permission"),
            true => String::from("revoke_permission"),
        }
    }

    fn encoded_args(&self, target: Principal) -> anyhow::Result<Vec<u8>> {
        match self.revoke {
            false => {
                let arg = GrantPermissionArguments {
                    to_principal: target,
                    permission: self.permission.into(),
                };
                Ok(candid::encode_one(arg)?)
            }
            true => {
                let arg = RevokePermissionArguments {
                    of_principal: target,
                    permission: self.permission.into(),
                };
                Ok(candid::encode_one(arg)?)
            }
        }
    }
}

/// Canister installation mode equivalent to `dfx canister install --mode XXX` and `orbit_station_api::CanisterInstallMode`.
#[derive(Copy, Clone, Eq, PartialEq, Debug, ValueEnum)]
pub enum AssetPermissionTypeArgs {
    /// Permission to prepare asset upload (is needed by the uploading developer)
    Prepare,
    /// Permission to commit a batch (should only be granted to the orbit station itself)
    Commit,
    /// Permission to grant and revoke the other permissions of the asset cansister
    /// (should not be needed if the orbit station is the controller)
    ManagePermissions,
}

impl From<AssetPermissionTypeArgs> for Permission {
    fn from(value: AssetPermissionTypeArgs) -> Self {
        match value {
            AssetPermissionTypeArgs::Prepare => Permission::Prepare,
            AssetPermissionTypeArgs::Commit => Permission::Commit,
            AssetPermissionTypeArgs::ManagePermissions => Permission::ManagePermissions,
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,

    /// Do not abort the upload, if the evidence does not match between local and remote calculation
    #[clap(long)]
    pub ignore_evidence: bool,

    /// The source directories to upload (multiple values possible)
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

// TODO: Request commit with --evidence and --cancel
