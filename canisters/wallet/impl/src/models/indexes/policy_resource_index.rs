use crate::models::{resource::Resource, ProposalPolicy};
use ic_canister_core::types::UUID;
use ic_canister_macros::storable;

/// Index of resources to their matching policy ids.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PolicyResourceIndex {
    pub resource: Resource,
    pub policy_id: UUID,
}

#[derive(Clone, Debug)]
pub struct PolicyResourceIndexCriteria {
    pub resource: Resource,
}

impl ProposalPolicy {
    pub fn to_index_for_resource(&self) -> Vec<PolicyResourceIndex> {
        self.specifier
            .to_resources()
            .iter()
            .map(|resource| PolicyResourceIndex {
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
        proposal_policy_test_utils::mock_proposal_policy,
        resource::{AccountResourceAction, UserResourceAction},
    };
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let policy_id = [1; 16];
        let model = PolicyResourceIndex {
            policy_id,
            resource: Resource::User(UserResourceAction::Create),
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = PolicyResourceIndex::from_bytes(serialized_model);

        assert_eq!(model.policy_id, deserialized_model.policy_id);
        assert!(matches!(
            deserialized_model.resource,
            Resource::User(UserResourceAction::Create)
        ));
    }

    #[test]
    fn valid_policy_resource_indexes() {
        let policy = mock_proposal_policy();

        let index_entries = policy.to_index_for_resource();

        assert_eq!(index_entries.len(), 1);
        assert_eq!(index_entries[0].policy_id, policy.id);
        assert!(matches!(
            index_entries[0].resource,
            Resource::Account(AccountResourceAction::Create)
        ));
    }
}
