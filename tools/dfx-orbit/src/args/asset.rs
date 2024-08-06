use candid::Nat;
use clap::{Parser, Subcommand};

/// Station management commands.
#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct AssetArgs {
    #[command(subcommand)]
    pub(crate) action: AssetArgsAction,
}

#[derive(Debug, Clone, Subcommand)]
pub enum AssetArgsAction {
    /// Request to grant this user Prepare permission for the asset canister
    RequestPreparePermission(AssetReqeustPreparePermissionArgs),
    /// Upload assets to an asset canister
    Upload(AssetUploadArgs),
    /// Compute local evidence
    ComputeEvidence(AssetComputeEvidenceArgs),
    /// Check an asset upload request
    Check(AssetCheckArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct AssetReqeustPreparePermissionArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// The title of the request to grant Prepare permission
    #[clap(long)]
    pub(crate) title: Option<String>,

    /// The summary of the request to grant Prepare permission
    #[clap(long)]
    pub(crate) summary: Option<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct AssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// Do not abort the upload, if the evidence does not match between local and remote calculation
    #[clap(long)]
    pub(crate) ignore_evidence: bool,

    /// The title of the request to commit the batch
    #[clap(long)]
    pub(crate) title: Option<String>,

    /// The summary of the request to commit the batch
    #[clap(long)]
    pub(crate) summary: Option<String>,

    /// The source directories to upload (multiple values possible)
    pub(crate) files: Vec<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct AssetComputeEvidenceArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,
    /// The source directories to compute evidence from (multiple values possible)
    pub(crate) files: Vec<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct AssetCheckArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// The ID of the request to commit the assets
    pub(crate) request_id: String,

    /// The batch ID to commit to
    pub(crate) batch_id: Nat,

    /// The source directories of the asset upload (multiple values possible)
    pub(crate) files: Vec<String>,

    /// Automatically approve the request, if the request's evidence matches the local evidence
    #[clap(long)]
    pub(crate) then_approve: bool,
}
