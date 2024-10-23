use super::{
    display::{display_request_operation, display_request_status},
    util::external_canister_operations,
};
use crate::DfxOrbit;
use clap::Parser;
use slog::debug;
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
    #[clap(long, default_value = "20", num_args(0..))]
    pub chunk_size: u16,

    /// Start fetching values at the specified offset
    #[clap(long, default_value = "0")]
    pub offset: u64,

    /// Limit the amount of responses to the specified value
    #[clap(short, long)]
    pub limit: Option<u64>,
}

impl DfxOrbit {
    /// Fetch the list in chunks in parallel
    ///
    /// This function first fetches an empty subset, to find out how long the list is.
    /// It then generates a bunch of requests to fetch chunks of specified size, requests
    /// them in parallel and then reconstructs them back into one list.
    pub(super) async fn parallel_fetch_list(
        &self,
        args: &ReviewListArgs,
    ) -> anyhow::Result<ListRequestsResponse> {
        let initial_request = self.initial_request(args);
        let total = self
            .station
            .review_list(initial_request.clone())
            .await?
            .total;

        let requests = self.generate_requests(
            &initial_request,
            total,
            args.offset,
            args.chunk_size,
            args.limit,
        );
        debug!(
            self.logger,
            "There are {} entries, which will be fetched in {} parallel requests",
            total - args.offset,
            requests.len()
        );

        let calls = requests
            .into_iter()
            .map(|request| async move {
                debug!(self.logger, "Fetching {:?}", request.paginate);
                self.station.review_list(request).await
            })
            .collect::<Vec<_>>();
        let responses = futures::future::try_join_all(calls).await?;

        Ok(self.merge_all_responses(responses))
    }

    fn initial_request(&self, args: &ReviewListArgs) -> ListRequestsInput {
        ListRequestsInput {
            requester_ids: None,
            approver_ids: None,
            statuses: None,
            operation_types: (!args.all).then(external_canister_operations),
            expiration_from_dt: None,
            expiration_to_dt: None,
            created_from_dt: None,
            created_to_dt: None,
            paginate: Some(PaginationInput {
                offset: Some(0),
                limit: Some(0),
            }),
            sort_by: Some(ListRequestsSortBy::CreatedAt(SortDirection::Desc)),
            only_approvable: args.only_approvable,
            with_evaluation_results: true,
        }
    }

    fn generate_requests(
        &self,
        input: &ListRequestsInput,
        total: u64,
        offset: u64,
        chunk_size: u16,
        limit: Option<u64>,
    ) -> Vec<ListRequestsInput> {
        // Calculate the end (index) until which we are fetching
        let end = limit
            .map(|limit| limit + offset)
            .unwrap_or(total)
            .min(total);

        (offset..end)
            // Calculate the offset and size of every chunk
            .step_by(chunk_size as usize)
            .map(|offset| (offset, (offset + u64::from(chunk_size)).min(end)))
            .map(|(offset, end)| ListRequestsInput {
                paginate: Some(PaginationInput {
                    offset: Some(offset),
                    limit: Some((end - offset) as u16),
                }),
                ..input.clone()
            })
            .collect()
    }

    fn merge_all_responses(&self, responses: Vec<ListRequestsResponse>) -> ListRequestsResponse {
        ListRequestsResponse {
            requests: responses
                .iter()
                .flat_map(|response| response.requests.clone())
                .collect(),
            next_offset: responses.last().and_then(|response| response.next_offset),
            total: responses.last().map(|response| response.total).unwrap_or(0),
            privileges: responses
                .iter()
                .flat_map(|response| response.privileges.clone())
                .collect(),
            additional_info: responses
                .iter()
                .flat_map(|response| response.additional_info.clone())
                .collect(),
        }
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
