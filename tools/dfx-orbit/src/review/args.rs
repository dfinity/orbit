use super::util::external_canister_operations;
use clap::{Parser, Subcommand};
use station_api::{
    GetNextApprovableRequestInput, GetRequestInput, ListRequestsInput, ListRequestsSortBy,
    RequestApprovalStatusDTO, SortDirection, SubmitRequestApprovalInput,
};

/// Station management commands.
#[derive(Debug, Clone, Parser)]
pub struct ReviewArgs {
    /// Return output as JSON
    #[clap(short, long)]
    pub(crate) json: bool,

    #[clap(subcommand)]
    pub(crate) action: ReviewActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ReviewActionArgs {
    /// List requests
    List(ReviewListArgs),
    /// Review the next request.
    Next(ReviewNextArgs),
    /// Review a specific request.
    Id(ReviewIdArgs),
}

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
            sort_by: Some(ListRequestsSortBy::CreatedAt(SortDirection::Asc)),
            only_approvable: args.only_approvable,
            with_evaluation_results: true,
        }
    }
}

/// Reviews the next request.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct ReviewIdArgs {
    /// The ID of the request to review.
    pub(crate) request_id: String,
    /// Prompt the user to approve the request
    #[clap(short, long, action, value_name = "REASON", conflicts_with = "reject")]
    pub(crate) approve: Option<Option<String>>,
    /// Prompt the user to reject the request
    #[clap(short, long, action, value_name = "REASON", conflicts_with = "approve")]
    pub(crate) reject: Option<Option<String>>,
}

impl From<ReviewIdArgs> for GetRequestInput {
    fn from(args: ReviewIdArgs) -> Self {
        GetRequestInput {
            request_id: args.request_id,
        }
    }
}

impl TryFrom<ReviewIdArgs> for SubmitRequestApprovalInput {
    type Error = ();

    fn try_from(value: ReviewIdArgs) -> Result<Self, Self::Error> {
        let (decision, reason) = match (value.approve, value.reject) {
            (None, None) => return Err(()),
            (None, Some(reason)) => (RequestApprovalStatusDTO::Rejected, reason),
            (Some(reason), None) => (RequestApprovalStatusDTO::Approved, reason),
            (Some(_), Some(_)) => unreachable!(),
        };

        Ok(Self {
            request_id: value.request_id,
            decision,
            reason,
        })
    }
}

/// Reviews the next request.
#[derive(Debug, Clone, Parser)]
pub struct ReviewNextArgs {
    /// Show any request type, not only the ones related to canister management
    #[clap(short, long)]
    any: bool,
}

impl From<ReviewNextArgs> for GetNextApprovableRequestInput {
    fn from(args: ReviewNextArgs) -> Self {
        Self {
            excluded_request_ids: vec![],
            operation_types: (!args.any).then(external_canister_operations),
        }
    }
}
