//! CLI arguments for `dfx-orbit review list`.

use clap::Parser;
use orbit_station_api::{ListRequestsInput, SortDirection};

use super::external_canister_operations;

// TODO: Ideas what we could filter by:
// - Filter by status: -> Only the ones which are in Created
// - Filter by times -> There are four times that could be set
// - Filter by request ids

/// Reviews the next request.
#[derive(Debug, Clone, Parser)]
pub struct ReviewListArgs {
    /// Show all request types, not only the ones related to canister management
    #[clap(short, long)]
    pub all: bool,

    /// Show only approvable requests.
    #[clap(short, long)]
    pub only_approvable: bool,
}

impl From<ReviewListArgs> for ListRequestsInput {
    fn from(args: ReviewListArgs) -> Self {
        Self {
            requester_ids: None,
            approver_ids: None,
            statuses: None,
            operation_types: (!args.all).then(external_canister_operations),
            expiration_from_dt: None,
            expiration_to_dt: None,
            created_from_dt: None,
            created_to_dt: None,
            paginate: None,
            sort_by: Some(orbit_station_api::ListRequestsSortBy::CreatedAt(
                SortDirection::Asc,
            )),
            only_approvable: args.only_approvable,
            with_evaluation_results: true,
        }
    }
}
