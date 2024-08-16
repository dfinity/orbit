use candid::Nat;
use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetArgs {
    /// The operation to verify
    #[clap(subcommand)]
    pub(crate) action: VerifyAssetActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum VerifyAssetActionArgs {
    /// Upload assets to an asset canister
    Upload(VerifyAssetUploadArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// The batch ID to commit to
    pub(crate) batch_id: Nat,

    /// The source directories of the asset upload (multiple values possible)
    pub(crate) files: Vec<String>,
}
