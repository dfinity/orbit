use crate::models::Request;
use orbit_essentials::storable;
use orbit_essentials::types::UUID;

/// Index of requests by the requester user id.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestRequesterIndex {
    /// The user who requested this request.
    pub requester_id: UUID,
    /// The request id, which is a UUID.
    pub request_id: UUID,
}

#[derive(Clone, Debug)]
pub struct RequestRequesterIndexCriteria {
    pub requester_id: UUID,
}

impl Request {
    pub fn to_index_for_requester(&self) -> RequestRequesterIndex {
        RequestRequesterIndex {
            requester_id: self.requested_by.to_owned(),
            request_id: self.id.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{request_test_utils::mock_request, RequestApproval, RequestApprovalStatus};
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let request_id = [1; 16];
        let user_id = [u8::MAX; 16];
        let model = RequestRequesterIndex {
            requester_id: user_id,
            request_id,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = RequestRequesterIndex::from_bytes(serialized_model);

        assert_eq!(model.request_id, deserialized_model.request_id);
        assert_eq!(model.requester_id, deserialized_model.requester_id);
    }

    #[test]
    fn valid_user_request_indexes() {
        let mut request = mock_request();
        request.id = [1; 16];
        request.requested_by = [u8::MAX; 16];
        request.approvals = vec![
            RequestApproval {
                approver_id: [1; 16],
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
                status: RequestApprovalStatus::Approved,
            },
            RequestApproval {
                approver_id: [2; 16],
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
                status: RequestApprovalStatus::Approved,
            },
        ];

        let index = request.to_index_for_requester();

        assert_eq!(index.request_id, request.id);
        assert_eq!(index.requester_id, request.requested_by);
    }
}
