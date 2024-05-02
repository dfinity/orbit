use crate::models::{Request, RequestStatusCode};
use orbit_essentials::storable;
use orbit_essentials::types::UUID;
use std::hash::Hash;

/// Represents a request index by its status.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestStatusIndex {
    /// The status of the request.
    pub status: RequestStatusCode,
    /// The request id, which is a UUID.
    pub request_id: UUID,
}

#[derive(Clone, Debug)]
pub struct RequestStatusIndexCriteria {
    pub status: RequestStatusCode,
}

impl Request {
    pub fn to_index_by_status(&self) -> RequestStatusIndex {
        RequestStatusIndex {
            status: self.status.to_type(),
            request_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{request_test_utils::mock_request, RequestStatus, RequestStatusCode};

    #[test]
    fn test_request_to_index_by_status() {
        let mut request = mock_request();
        request.last_modification_timestamp = 5;
        request.status = RequestStatus::Created;

        let index = request.to_index_by_status();

        assert_eq!(index.request_id, request.id);
        assert_eq!(index.status, RequestStatusCode::Created);
    }
}
