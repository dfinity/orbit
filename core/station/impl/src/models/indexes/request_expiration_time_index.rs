use crate::models::Request;
use orbit_essentials::storable;
use orbit_essentials::types::{Timestamp, UUID};
use std::hash::Hash;

/// Represents a request index by expiration time.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestExpirationTimeIndex {
    /// The time the request is scheduled to be set as expired if still pending.
    pub expiration_dt: Timestamp,
    /// The request id, which is a UUID.
    pub request_id: UUID,
}

#[derive(Clone, Debug)]
pub struct RequestExpirationTimeIndexCriteria {
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Request {
    pub fn to_index_by_expiration_dt(&self) -> RequestExpirationTimeIndex {
        RequestExpirationTimeIndex {
            expiration_dt: self.expiration_dt,
            request_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::request_test_utils::mock_request;

    #[test]
    fn test_request_to_index_by_expiration_dt() {
        let mut request = mock_request();
        request.expiration_dt = 5;

        let index = request.to_index_by_expiration_dt();

        assert_eq!(index.request_id, request.id);
        assert_eq!(index.expiration_dt, 5);
    }
}
