use crate::models::RequestApprovalStatus;
use station_api::RequestApprovalStatusDTO;

impl From<RequestApprovalStatus> for RequestApprovalStatusDTO {
    fn from(status: RequestApprovalStatus) -> Self {
        match status {
            RequestApprovalStatus::Approved => RequestApprovalStatusDTO::Approved,
            RequestApprovalStatus::Rejected => RequestApprovalStatusDTO::Rejected,
        }
    }
}

impl From<RequestApprovalStatusDTO> for RequestApprovalStatus {
    fn from(status: RequestApprovalStatusDTO) -> Self {
        match status {
            RequestApprovalStatusDTO::Approved => RequestApprovalStatus::Approved,
            RequestApprovalStatusDTO::Rejected => RequestApprovalStatus::Rejected,
        }
    }
}
