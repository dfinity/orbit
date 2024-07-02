//! CLI arguments for `dfx-orbit review next`.

use clap::Parser;

/// Reviews the next request.
#[derive(Debug, Parser)]
pub struct Args {}

impl From<Args> for orbit_station_api::GetNextApprovableRequestInput {
    fn from(_: Args) -> Self {
        Self {
            excluded_request_ids: vec![],
            operation_types: None,
        }
    }
}
