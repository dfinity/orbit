use crate::models::{Request, RequestId};
use orbit_essentials::storable;
use orbit_essentials::types::Timestamp;

/// Index of requests to use for sorting.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestSortIndex {
    /// The request id, which is a UUID.
    pub key: RequestSortIndexKey,
    /// The request's last modification timestamp.
    pub value: RequestSortIndexValue,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestSortIndexKey {
    pub request_id: RequestId,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestSortIndexValue {
    /// The request's last modification timestamp.
    pub modification_timestamp: Timestamp,
    /// The request's creation timestamp.
    pub creation_timestamp: Timestamp,
    /// The request's expiration_dt.
    pub expiration_timestamp: Timestamp,
}

#[derive(Clone, Debug)]
pub struct RequestSortIndexCriteria {
    pub request_id: RequestId,
}

impl Request {
    pub fn to_index_for_sorting(&self) -> RequestSortIndex {
        RequestSortIndex {
            key: RequestSortIndexKey {
                request_id: self.id,
            },
            value: RequestSortIndexValue {
                modification_timestamp: self.last_modification_timestamp,
                creation_timestamp: self.created_timestamp,
                expiration_timestamp: self.expiration_dt,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::request_test_utils::mock_request;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let request_id = [1; 16];
        let model = RequestSortIndex {
            key: RequestSortIndexKey { request_id },
            value: RequestSortIndexValue {
                creation_timestamp: 1,
                modification_timestamp: 2,
                expiration_timestamp: 3,
            },
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = RequestSortIndex::from_bytes(serialized_model);

        assert_eq!(model.key.request_id, deserialized_model.key.request_id);
        assert_eq!(
            model.value.creation_timestamp,
            deserialized_model.value.creation_timestamp
        );
        assert_eq!(
            model.value.modification_timestamp,
            deserialized_model.value.modification_timestamp
        );
        assert_eq!(
            model.value.expiration_timestamp,
            deserialized_model.value.expiration_timestamp
        );
    }

    #[test]
    fn valid_user_approver_indexes() {
        let mut request = mock_request();
        request.id = [1; 16];
        request.requested_by = [u8::MAX; 16];
        request.created_timestamp = 1;
        request.last_modification_timestamp = 2;
        request.expiration_dt = 3;

        let index = request.to_index_for_sorting();

        assert_eq!(index.key.request_id, request.id);
        assert_eq!(index.value.creation_timestamp, request.created_timestamp);
        assert_eq!(
            index.value.modification_timestamp,
            request.last_modification_timestamp
        );
        assert_eq!(index.value.expiration_timestamp, request.expiration_dt);
    }
}
