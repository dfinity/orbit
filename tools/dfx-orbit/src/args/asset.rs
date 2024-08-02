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
    /// Upload assets to an asset canister
    Upload(AssetUploadArgs),
    /// Compute local evidence
    ComputeEvidence(AssetComputeEvidenceArgs),
    /// Commit assets previously uploaded to an assed canister
    Commit(AssetCommitArgs),
    /// Check an asset upload request
    Check(AssetCheckArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct AssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// Do not abort the upload, if the evidence does not match between local and remote calculation
    #[clap(long)]
    pub(crate) ignore_evidence: bool,

    /// Do not submit a request to commit the batch ID to the orbit station
    #[clap(long)]
    pub(crate) skip_commit: bool,

    /// The source directories to upload (multiple values possible)
    #[clap(num_args = 1..)]
    pub(crate) files: Vec<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct AssetComputeEvidenceArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,
    /// The source directories to compute evidence from (multiple values possible)
    #[clap(num_args = 1..)]
    pub(crate) files: Vec<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct AssetCommitArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// The batch ID to commit to
    pub(crate) batch_id: Nat,

    /// The evidence (as hex string) to commit to
    pub(crate) evidence: String,
}
#[derive(Debug, Clone, Parser)]
pub struct AssetCheckArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// The ID of the request to commit the assets
    pub(crate) request_id: String,

    /// The source directories of the asset upload (multiple values possible)
    #[clap(num_args = 1..)]
    pub(crate) files: Vec<String>,

    /// Automatically approve the request, if the request evidence matches the local evidence
    #[clap(long)]
    try_approve: bool,
}
