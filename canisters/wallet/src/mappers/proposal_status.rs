use crate::{models::ProposalStatus, transport::ProposalStatusDTO};

impl From<ProposalStatus> for ProposalStatusDTO {
    fn from(status: ProposalStatus) -> Self {
        match status {
            ProposalStatus::Pending => ProposalStatusDTO::Pending,
            ProposalStatus::Adopted => ProposalStatusDTO::Adopted,
            ProposalStatus::Rejected => ProposalStatusDTO::Rejected,
        }
    }
}

impl From<ProposalStatusDTO> for ProposalStatus {
    fn from(status: ProposalStatusDTO) -> Self {
        match status {
            ProposalStatusDTO::Pending => ProposalStatus::Pending,
            ProposalStatusDTO::Adopted => ProposalStatus::Adopted,
            ProposalStatusDTO::Rejected => ProposalStatus::Rejected,
        }
    }
}
