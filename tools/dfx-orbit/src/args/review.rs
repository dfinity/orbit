//! Defines the command line arguments for `dfx-orbit review`.  These correspond to Orbit station `get_request`, `submit_request_approval` and related API calls.
pub mod id;
pub mod list;
pub mod next;

use clap::{Parser, Subcommand};
use id::ReviewIdArgs;
use list::ReviewListArgs;
use next::ReviewNextArgs;
use station_api::ListRequestsOperationTypeDTO;

/// Station management commands.
#[derive(Debug, Clone, Parser)]
pub struct ReviewArgs {
    /// Return output as JSON
    #[clap(short, long)]
    pub(crate) json: bool,

    #[clap(subcommand)]
    pub(crate) action: ReviewActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ReviewActionArgs {
    /// List requests
    List(ReviewListArgs),
    /// Review the next request.
    Next(ReviewNextArgs),
    /// Review a specific request.
    Id(ReviewIdArgs),
}

fn external_canister_operations() -> Vec<ListRequestsOperationTypeDTO> {
    vec![
        ListRequestsOperationTypeDTO::ChangeExternalCanister(None),
        ListRequestsOperationTypeDTO::CreateExternalCanister,
        ListRequestsOperationTypeDTO::CallExternalCanister(None),
        ListRequestsOperationTypeDTO::ConfigureExternalCanister(None),
    ]
}
