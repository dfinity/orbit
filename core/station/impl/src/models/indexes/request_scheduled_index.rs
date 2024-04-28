use crate::models::{Request, RequestStatus};
use orbit_essentials::storable;
use orbit_essentials::types::{Timestamp, UUID};
use std::hash::Hash;

/// Represents a request index by its status.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestScheduledIndex {
    /// The time the request is scheduled to be executed.
    pub schedule_dt: Timestamp,
    /// The request id, which is a UUID.
    pub request_id: UUID,
}

#[derive(Clone, Debug)]
pub struct RequestScheduledIndexCriteria {
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Request {
    pub fn to_index_by_scheduled(&self) -> Option<RequestScheduledIndex> {
        if let RequestStatus::Scheduled { scheduled_at } = &self.status {
            return Some(RequestScheduledIndex {
                schedule_dt: *scheduled_at,
                request_id: self.id,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{request_test_utils::mock_request, RequestStatus};

    #[test]
    fn test_request_to_index_by_scheduled() {
        let mut request = mock_request();
        request.last_modification_timestamp = 5;
        request.status = RequestStatus::Scheduled { scheduled_at: 0 };

        let index = request.to_index_by_scheduled().unwrap();

        assert_eq!(index.request_id, request.id);
        assert_eq!(index.schedule_dt, 0);
    }
}
