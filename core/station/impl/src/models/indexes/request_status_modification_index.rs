use crate::models::{Request, RequestId, RequestStatusCode};
use orbit_essentials::storable;
use orbit_essentials::types::Timestamp;
use std::hash::Hash;

/// Represents a request index by its status and the last modification timestamp.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestStatusModificationIndex {
    /// The status of the request.
    pub status: RequestStatusCode,
    /// The last modification timestamp of the request.
    pub modification_timestamp: Timestamp,
    /// The request id, which is a UUID.
    pub request_id: RequestId,
}

#[derive(Clone, Debug)]
pub struct RequestStatusModificationIndexCriteria {
    pub status: RequestStatusCode,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Request {
    pub fn to_index_by_status_and_modification(&self) -> RequestStatusModificationIndex {
        RequestStatusModificationIndex {
            status: self.status.to_type(),
            modification_timestamp: self.last_modification_timestamp,
            request_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{request_test_utils::mock_request, RequestStatus, RequestStatusCode};

    #[test]
    fn test_request_to_index_by_status_and_modification() {
        let mut request = mock_request();
        request.last_modification_timestamp = 5;
        request.status = RequestStatus::Created;

        let index = request.to_index_by_status_and_modification();

        assert_eq!(index.request_id, request.id);
        assert_eq!(index.status, RequestStatusCode::Created);
        assert_eq!(index.modification_timestamp, 5);
    }
}
