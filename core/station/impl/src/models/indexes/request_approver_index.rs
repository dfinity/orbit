use crate::models::Request;
use orbit_essentials::storable;
use orbit_essentials::types::UUID;
use std::collections::HashSet;

/// Index of requests by the approvers' user id.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestApproverIndex {
    /// The user that has added it's approval decision to the request.
    pub approver_id: UUID,
    /// The request id, which is a UUID.
    pub request_id: UUID,
}

#[derive(Clone, Debug)]
pub struct RequestApproverIndexCriteria {
    pub approver_id: UUID,
}

impl Request {
    pub fn to_index_for_approvers(&self) -> Vec<RequestApproverIndex> {
        let mut approvers = HashSet::<UUID>::new();
        self.approvals.iter().for_each(|d| {
            approvers.insert(d.approver_id);
        });

        approvers
            .iter()
            .map(|user_id| RequestApproverIndex {
                approver_id: user_id.to_owned(),
                request_id: self.id.to_owned(),
            })
            .collect()
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
        let model = RequestApproverIndex {
            approver_id: user_id,
            request_id,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = RequestApproverIndex::from_bytes(serialized_model);

        assert_eq!(model.request_id, deserialized_model.request_id);
        assert_eq!(model.approver_id, deserialized_model.approver_id);
    }

    #[test]
    fn valid_user_approver_indexes() {
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

        let indexes = request.to_index_for_approvers();

        assert_eq!(indexes.len(), 2);
        assert!(indexes.iter().any(|i| i.approver_id == [1; 16]));
        assert!(indexes.iter().any(|i| i.approver_id == [2; 16]));
    }
}
