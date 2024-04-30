use crate::models::{resource::Resource, RequestPolicy};
use orbit_essentials::storable;
use orbit_essentials::types::UUID;

/// Index of resources to their matching policy ids.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestPolicyResourceIndex {
    pub resource: Resource,
    pub policy_id: UUID,
}

#[derive(Clone, Debug)]
pub struct RequestPolicyResourceIndexCriteria {
    pub resource: Resource,
}

impl RequestPolicy {
    pub fn to_index_for_resource(&self) -> Vec<RequestPolicyResourceIndex> {
        self.specifier
            .to_resources()
            .iter()
            .map(|resource| RequestPolicyResourceIndex {
                resource: resource.to_owned(),
                policy_id: self.id.to_owned(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        request_policy_test_utils::mock_request_policy,
        resource::{AccountResourceAction, UserResourceAction},
    };
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let policy_id = [1; 16];
        let model = RequestPolicyResourceIndex {
            policy_id,
            resource: Resource::User(UserResourceAction::Create),
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = RequestPolicyResourceIndex::from_bytes(serialized_model);

        assert_eq!(model.policy_id, deserialized_model.policy_id);
        assert!(matches!(
            deserialized_model.resource,
            Resource::User(UserResourceAction::Create)
        ));
    }

    #[test]
    fn valid_policy_resource_indexes() {
        let policy = mock_request_policy();

        let index_entries = policy.to_index_for_resource();

        assert_eq!(index_entries.len(), 1);
        assert_eq!(index_entries[0].policy_id, policy.id);
        assert!(matches!(
            index_entries[0].resource,
            Resource::Account(AccountResourceAction::Create)
        ));
    }
}
