use crate::models::RequestApproval;
use orbit_essentials::utils::timestamp_to_rfc3339;
use station_api::RequestApprovalDTO;
use uuid::Uuid;

impl From<RequestApproval> for RequestApprovalDTO {
    fn from(approval: RequestApproval) -> Self {
        Self {
            approver_id: Uuid::from_bytes(approval.approver_id)
                .hyphenated()
                .to_string(),
            decided_at: timestamp_to_rfc3339(&approval.decided_dt),
            status: approval.status.into(),
            status_reason: approval.status_reason,
        }
    }
}
