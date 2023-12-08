use crate::models::access_control::{AccessControlPolicy, AccessModifier};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;

/// Represents an index to facilitate the search of access control policies by the resource and access modifier.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccessControlPolicyResourceIndex {
    pub resource: String,
    pub access: AccessModifier,
    pub policy_id: UUID,
}

#[derive(Clone, Debug)]
pub struct AccessControlPolicyResourceIndexCriteria {
    pub resource: String,
    pub access: AccessModifier,
}

impl AccessControlPolicy {
    pub fn to_index_by_resource(&self) -> AccessControlPolicyResourceIndex {
        AccessControlPolicyResourceIndex {
            resource: self.resource.to_string(),
            access: self.access.to_owned(),
            policy_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::access_control::ResourceSpecifier;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = AccessControlPolicyResourceIndex {
            resource: ResourceSpecifier::AddressBook.to_string(),
            access: AccessModifier::Read,
            policy_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = AccessControlPolicyResourceIndex::from_bytes(serialized_model);

        assert_eq!(model.resource, deserialized_model.resource);
        assert_eq!(model.access, deserialized_model.access);
        assert_eq!(model.policy_id, deserialized_model.policy_id);
    }
}
