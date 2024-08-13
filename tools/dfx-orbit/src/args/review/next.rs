//! CLI arguments for `dfx-orbit review next`.

use clap::Parser;
use orbit_station_api::GetNextApprovableRequestInput;

// TODO: Only show review types that are relevant to dfx-orbbit -> can deactivate with --all

/// Reviews the next request.
#[derive(Debug, Parser)]
pub struct ReviewNextArgs {}

impl From<ReviewNextArgs> for GetNextApprovableRequestInput {
    fn from(_: ReviewNextArgs) -> Self {
        Self {
            excluded_request_ids: vec![],
            operation_types: None,
        }
    }
}
