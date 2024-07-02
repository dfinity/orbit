//! CLI arguments for `dfx-orbit review next`.

use clap::Parser;
use orbit_station_api::GetRequestInput;

/// Reviews the next request.
#[derive(Debug, Parser)]
pub struct Args {
    /// The ID of the request to review.
    request_id: String,
}

impl From<Args> for GetRequestInput {
    fn from(args: Args) -> Self {
        let Args { request_id } = args;
        GetRequestInput { request_id }
    }
}
