use crate::{models::ProposalStatus, transport::ProposalStatusDTO};
use ic_canister_core::utils::{rfc3339_to_timestamp, timestamp_to_rfc3339};

impl From<ProposalStatus> for ProposalStatusDTO {
    fn from(status: ProposalStatus) -> Self {
        match status {
            ProposalStatus::Created => ProposalStatusDTO::Created,
            ProposalStatus::Adopted => ProposalStatusDTO::Adopted,
            ProposalStatus::Rejected => ProposalStatusDTO::Rejected,
            ProposalStatus::Completed { completed_at } => ProposalStatusDTO::Completed {
                completed_at: timestamp_to_rfc3339(&completed_at),
            },
            ProposalStatus::Failed { reason } => ProposalStatusDTO::Failed { reason },
            ProposalStatus::Processing { started_at } => ProposalStatusDTO::Processing {
                started_at: timestamp_to_rfc3339(&started_at),
            },
            ProposalStatus::Scheduled { scheduled_at } => ProposalStatusDTO::Scheduled {
                scheduled_at: timestamp_to_rfc3339(&scheduled_at),
            },
            ProposalStatus::Cancelled { reason } => ProposalStatusDTO::Cancelled { reason },
        }
    }
}

impl From<ProposalStatusDTO> for ProposalStatus {
    fn from(status: ProposalStatusDTO) -> Self {
        match status {
            ProposalStatusDTO::Created => ProposalStatus::Created,
            ProposalStatusDTO::Adopted => ProposalStatus::Adopted,
            ProposalStatusDTO::Rejected => ProposalStatus::Rejected,
            ProposalStatusDTO::Completed { completed_at } => ProposalStatus::Completed {
                completed_at: rfc3339_to_timestamp(completed_at.as_str()),
            },
            ProposalStatusDTO::Failed { reason } => ProposalStatus::Failed { reason },
            ProposalStatusDTO::Processing { started_at } => ProposalStatus::Processing {
                started_at: rfc3339_to_timestamp(started_at.as_str()),
            },
            ProposalStatusDTO::Scheduled { scheduled_at } => ProposalStatus::Scheduled {
                scheduled_at: rfc3339_to_timestamp(&scheduled_at),
            },
            ProposalStatusDTO::Cancelled { reason } => ProposalStatus::Cancelled { reason },
        }
    }
}
