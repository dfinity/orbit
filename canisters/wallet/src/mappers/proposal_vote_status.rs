use crate::{models::ProposalVoteStatus, transport::ProposalVoteStatusDTO};

impl From<ProposalVoteStatus> for ProposalVoteStatusDTO {
    fn from(status: ProposalVoteStatus) -> Self {
        match status {
            ProposalVoteStatus::Pending => ProposalVoteStatusDTO::Pending,
            ProposalVoteStatus::Adopted => ProposalVoteStatusDTO::Adopted,
            ProposalVoteStatus::Rejected => ProposalVoteStatusDTO::Rejected,
            ProposalVoteStatus::NotRequired => ProposalVoteStatusDTO::NotRequired,
        }
    }
}

impl From<ProposalVoteStatusDTO> for ProposalVoteStatus {
    fn from(status: ProposalVoteStatusDTO) -> Self {
        match status {
            ProposalVoteStatusDTO::Pending => ProposalVoteStatus::Pending,
            ProposalVoteStatusDTO::Adopted => ProposalVoteStatus::Adopted,
            ProposalVoteStatusDTO::Rejected => ProposalVoteStatus::Rejected,
            ProposalVoteStatusDTO::NotRequired => ProposalVoteStatus::NotRequired,
        }
    }
}
