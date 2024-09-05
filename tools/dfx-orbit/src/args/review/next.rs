//! CLI arguments for `dfx-orbit review next`.

use super::external_canister_operations;
use clap::Parser;
use station_api::GetNextApprovableRequestInput;

/// Reviews the next request.
#[derive(Debug, Parser)]
pub struct ReviewNextArgs {
    /// Show any request type, not only the ones related to canister management
    #[clap(short, long)]
    any: bool,
}

impl From<ReviewNextArgs> for GetNextApprovableRequestInput {
    fn from(args: ReviewNextArgs) -> Self {
        Self {
            excluded_request_ids: vec![],
            operation_types: (!args.any).then(external_canister_operations),
        }
    }
}
