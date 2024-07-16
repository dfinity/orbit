//! CLI arguments for `dfx-orbit review next`.
use clap::Parser;
use orbit_station_api::GetRequestInput;

/// Reviews the next request.
#[derive(Debug, Parser)]
pub struct ReviewIdArgs {
    /// The ID of the request to review.
    pub(crate) request_id: String,
}

impl From<ReviewIdArgs> for GetRequestInput {
    fn from(args: ReviewIdArgs) -> Self {
        GetRequestInput {
            request_id: args.request_id,
        }
    }
}
