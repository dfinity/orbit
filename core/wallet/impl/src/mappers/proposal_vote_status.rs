use crate::models::ProposalVoteStatus;
use wallet_api::ProposalVoteStatusDTO;

impl From<ProposalVoteStatus> for ProposalVoteStatusDTO {
    fn from(status: ProposalVoteStatus) -> Self {
        match status {
            ProposalVoteStatus::Accepted => ProposalVoteStatusDTO::Accepted,
            ProposalVoteStatus::Rejected => ProposalVoteStatusDTO::Rejected,
        }
    }
}

impl From<ProposalVoteStatusDTO> for ProposalVoteStatus {
    fn from(status: ProposalVoteStatusDTO) -> Self {
        match status {
            ProposalVoteStatusDTO::Accepted => ProposalVoteStatus::Accepted,
            ProposalVoteStatusDTO::Rejected => ProposalVoteStatus::Rejected,
        }
    }
}
