use super::{
    display::{display_request_operation, display_request_status},
    util::external_canister_operations,
};
use crate::DfxOrbit;
use clap::Parser;
use slog::{debug, warn};
use station_api::{
    ListRequestsInput, ListRequestsResponse, ListRequestsSortBy, PaginationInput, SortDirection,
};
use std::collections::HashMap;
use tabled::{
    settings::{Settings, Style},
    Table,
};

// TODO: Filter by open only
/// Reviews the next request.
#[derive(Debug, Clone, Parser)]
pub struct ReviewListArgs {
    /// Show all request types, not only the ones related to canister management
    #[clap(short, long)]
    pub all: bool,

    /// Show only approvable requests.
    #[clap(short, long)]
    pub only_approvable: bool,

    /// Fetch the values in chunks of this size
    #[clap(long, default_value = "20")]
    pub chunk_size: u16,

    /// Start fetching values at the specified offset
    #[clap(long)]
    pub offset: Option<u64>,

    /// Limit the amount of responses to the specified value
    #[clap(short, long)]
    pub limit: Option<u64>,
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
            paginate: Some(PaginationInput {
                offset: args.offset,
                limit: Some(args.chunk_size),
            }),
            sort_by: Some(ListRequestsSortBy::CreatedAt(SortDirection::Desc)),
            only_approvable: args.only_approvable,
            with_evaluation_results: true,
        }
    }
}

impl DfxOrbit {
    pub(super) async fn fetch_list(
        &self,
        request: ListRequestsInput,
        _limit: Option<u64>,
    ) -> anyhow::Result<ListRequestsResponse> {
        let mut response = self.station.review_list(request.clone()).await?;

        while let Some(request) = self.next_request(request.clone(), &response) {
            debug!(
                self.logger,
                "Fetching request list page {:?}", request.paginate
            );

            let new_response = self.station.review_list(request.clone()).await?;
            response = self.merge_responses(response, new_response);

            debug!(
                self.logger,
                "Got response total: {}, next_offset: {:?}", response.total, response.next_offset
            );
        }

        Ok(response)
    }

    fn merge_responses(
        &self,
        mut left: ListRequestsResponse,
        right: ListRequestsResponse,
    ) -> ListRequestsResponse {
        if left.total != right.total {
            warn!(self.logger, "The length of the list does not match");
        }

        left.requests.extend(right.requests);
        left.next_offset = right.next_offset;
        left.privileges.extend(right.privileges);
        left.additional_info.extend(right.additional_info);

        left
    }

    fn next_request(
        &self,
        mut last_request: ListRequestsInput,
        response: &ListRequestsResponse,
    ) -> Option<ListRequestsInput> {
        response.next_offset.map(|offset| {
            last_request
                .paginate
                .as_mut()
                .map(|paginate| paginate.offset = Some(offset));
            last_request
        })
    }

    pub(super) fn display_list(&self, data: ListRequestsResponse) -> String {
        let add_info = data
            .additional_info
            .into_iter()
            .map(|info| (info.id.clone(), info))
            .collect::<HashMap<String, _>>();

        let data_iter = data.requests.iter().map(|request| {
            let add_info = add_info.get(&request.id);

            [
                request.id.clone(),
                add_info
                    .map(|add_info| add_info.requester_name.clone())
                    .unwrap_or(String::from("-")),
                request.title.clone(),
                display_request_operation(&request.operation).to_string(),
                display_request_status(&request.status).to_string(),
            ]
        });
        let titled_iter = std::iter::once([
            String::from("ID"),
            String::from("Requested by"),
            String::from("Title"),
            String::from("Operation"),
            String::from("Execution Status"),
        ])
        .chain(data_iter);

        let table_config = Settings::default().with(Style::psql());
        let table = Table::from_iter(titled_iter).with(table_config).to_string();

        table
    }
}
