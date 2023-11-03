use crate::{models::OperationDecision, transport::OperationDecisionDTO};
use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;

impl From<OperationDecision> for OperationDecisionDTO {
    fn from(decision: OperationDecision) -> Self {
        Self {
            user_id: Uuid::from_bytes(decision.user_id).hyphenated().to_string(),
            decided_at: decision.decided_dt.map(|dt| timestamp_to_rfc3339(&dt)),
            read: decision.read,
            status: decision.status.into(),
            status_reason: decision.status_reason,
        }
    }
}
