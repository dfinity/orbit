use crate::models::access_policy::{AccessPolicy, AccessPolicyId, AllowKey, Resource};
use ic_canister_macros::storable;

/// Represents an index to facilitate the search of access policies by the resource.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd)]
pub struct AccessPolicyIndex {
    pub resource: Resource,
    pub allow: AllowKey,
    pub policy_id: AccessPolicyId,
}

impl Ord for AccessPolicyIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resource.cmp(&other.resource).then(
            self.allow
                .cmp(&other.allow)
                .then(self.policy_id.cmp(&other.policy_id)),
        )
    }
}

#[derive(Clone, Debug)]
pub struct AccessPolicyIndexCriteria {
    pub resource: Resource,
    pub allow: Option<AllowKey>,
}

impl AccessPolicy {
    pub fn to_index_by_resource(&self) -> AccessPolicyIndex {
        AccessPolicyIndex {
            allow: self.allow.to_owned().into(),
            resource: self.resource.to_owned(),
            policy_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::access_policy::{Resource, UserResourceAction};
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = AccessPolicyIndex {
            allow: AllowKey::Any,
            resource: Resource::User(UserResourceAction::Create),
            policy_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = AccessPolicyIndex::from_bytes(serialized_model);

        assert_eq!(model.resource, deserialized_model.resource);
        assert_eq!(model.policy_id, deserialized_model.policy_id);
        assert_eq!(model.allow, deserialized_model.allow);
    }
}
