use crate::{models::OperationStatus, transport::OperationStatusDTO};

impl From<OperationStatus> for OperationStatusDTO {
    fn from(status: OperationStatus) -> Self {
        match status {
            OperationStatus::Pending => OperationStatusDTO::Pending,
            OperationStatus::Adopted => OperationStatusDTO::Adopted,
            OperationStatus::Rejected => OperationStatusDTO::Rejected,
            OperationStatus::NotRequired => OperationStatusDTO::NotRequired,
        }
    }
}

impl From<OperationStatusDTO> for OperationStatus {
    fn from(status: OperationStatusDTO) -> Self {
        match status {
            OperationStatusDTO::Pending => OperationStatus::Pending,
            OperationStatusDTO::Adopted => OperationStatus::Adopted,
            OperationStatusDTO::Rejected => OperationStatus::Rejected,
            OperationStatusDTO::NotRequired => OperationStatus::NotRequired,
        }
    }
}
