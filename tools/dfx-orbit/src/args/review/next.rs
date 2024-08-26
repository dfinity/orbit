//! CLI arguments for `dfx-orbit review next`.

use clap::Parser;
use station_api::GetNextApprovableRequestInput;

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
