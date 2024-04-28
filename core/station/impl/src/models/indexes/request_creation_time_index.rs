use crate::models::Request;
use orbit_essentials::storable;
use orbit_essentials::types::{Timestamp, UUID};
use std::hash::Hash;

/// Represents a request index by creation time.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestCreationTimeIndex {
    /// The time the request was created.
    pub created_at: Timestamp,
    /// The request id, which is a UUID.
    pub request_id: UUID,
}

#[derive(Clone, Debug)]
pub struct RequestCreationTimeIndexCriteria {
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Request {
    pub fn to_index_by_creation_dt(&self) -> RequestCreationTimeIndex {
        RequestCreationTimeIndex {
            created_at: self.created_timestamp,
            request_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::request_test_utils::mock_request;

    #[test]
    fn test_request_to_index_by_creation_dt() {
        let mut request = mock_request();
        request.created_timestamp = 5;

        let index = request.to_index_by_creation_dt();

        assert_eq!(index.request_id, request.id);
        assert_eq!(index.created_at, 5);
    }
}
