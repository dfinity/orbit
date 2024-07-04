//! CLI arguments for `dfx-orbit review list`.

use clap::Parser;

/// Reviews the next request.
#[derive(Debug, Parser)]
pub struct Args {
    /// Show only approvable requests.
    #[clap(short, long)]
    pub only_approvable: bool,
}

impl From<Args> for orbit_station_api::ListRequestsInput {
    fn from(args: Args) -> Self {
        let Args { only_approvable } = args;
        Self {
            requester_ids: None,
            approver_ids: None,
            statuses: None,
            operation_types: None,
            expiration_from_dt: None,
            expiration_to_dt: None,
            created_from_dt: None,
            created_to_dt: None,
            paginate: None,
            sort_by: None,
            only_approvable,
            with_evaluation_results: true,
        }
    }
}
