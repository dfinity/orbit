use crate::models::{resource::Resource, Request};
use orbit_essentials::storable;
use orbit_essentials::types::UUID;

/// Index of requests by the resource derived from the request operation.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestResourceIndex {
    /// The request id, which is a UUID.
    pub request_id: UUID,
    /// The resource derived from the request operation.
    pub resource: Resource,
}

#[derive(Clone, Debug)]
pub struct RequestResourceIndexCriteria {
    pub request_id: UUID,
}

impl Request {
    pub fn to_index_for_resource(&self) -> Vec<RequestResourceIndex> {
        self.operation
            .to_resources()
            .iter()
            .map(|resource| RequestResourceIndex {
                request_id: self.id.to_owned(),
                resource: resource.to_owned(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        request_test_utils::mock_request, resource::UserResourceAction, AddUserOperation,
        AddUserOperationInput, RequestOperation, UserStatus,
    };
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let request_id = [1; 16];
        let model = RequestResourceIndex {
            request_id,
            resource: Resource::User(UserResourceAction::Create),
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = RequestResourceIndex::from_bytes(serialized_model);

        assert_eq!(model.request_id, deserialized_model.request_id);
        assert!(matches!(
            deserialized_model.resource,
            Resource::User(UserResourceAction::Create)
        ));
    }

    #[test]
    fn valid_request_resource_indexes() {
        let mut request = mock_request();
        request.id = [1; 16];
        request.requested_by = [u8::MAX; 16];
        request.operation = RequestOperation::AddUser(AddUserOperation {
            input: AddUserOperationInput {
                groups: vec![],
                identities: vec![],
                name: None,
                status: UserStatus::Active,
            },
            user_id: None,
        });

        let index_entries = request.to_index_for_resource();

        assert_eq!(index_entries.len(), 1);
        assert_eq!(index_entries[0].request_id, request.id);
        assert!(matches!(
            index_entries[0].resource,
            Resource::User(UserResourceAction::Create)
        ));
    }
}
