//! CLI arguments for `dfx-orbit review next`.
use clap::Parser;
use orbit_station_api::GetRequestInput;

/// Reviews the next request.
#[derive(Debug, Parser)]
pub struct ReviewIdArgs {
    /// The ID of the request to review.
    pub(crate) request_id: String,
    /// Prompt the user to approve the request
    #[clap(
        long,
        action,
        value_name = "REASON",
        conflicts_with = "reject",
        default_missing_value = "None"
    )]
    pub(crate) approve: Option<Option<String>>,
    /// Prompt the user to reject the request
    #[clap(
        long,
        action,
        value_name = "REASON",
        conflicts_with = "approve",
        default_missing_value = "None"
    )]
    pub(crate) reject: Option<Option<String>>,
}

impl From<ReviewIdArgs> for GetRequestInput {
    fn from(args: ReviewIdArgs) -> Self {
        GetRequestInput {
            request_id: args.request_id,
        }
    }
}
