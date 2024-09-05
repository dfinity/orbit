//! CLI arguments for `dfx-orbit review next`.
use clap::Parser;
use station_api::{GetRequestInput, RequestApprovalStatusDTO, SubmitRequestApprovalInput};

/// Reviews the next request.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct ReviewIdArgs {
    /// The ID of the request to review.
    pub(crate) request_id: String,
    /// Prompt the user to approve the request
    #[clap(
        long,
        action,
        value_name = "REASON",
        conflicts_with = "reject",
        default_missing_value = "None"
    )]
    pub(crate) approve: Option<Option<String>>,
    /// Prompt the user to reject the request
    #[clap(
        long,
        action,
        value_name = "REASON",
        conflicts_with = "approve",
        default_missing_value = "None"
    )]
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
