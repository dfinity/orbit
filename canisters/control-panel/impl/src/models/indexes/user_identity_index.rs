use crate::models::{User, UserId};
use candid::Principal;
use ic_canister_macros::storable;

/// Represents an user identity index within the system.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserIdentityIndex {
    /// The identity associated with the user.
    pub identity_id: Principal,
    /// The user id, which is a UUID.
    pub user_id: UserId,
}

#[derive(Clone, Debug)]
pub struct UserIdentityIndexCriteria {
    pub identity_id: Principal,
}

impl User {
    pub fn to_index_for_identity(&self) -> UserIdentityIndex {
        UserIdentityIndex {
            identity_id: self.identity,
            user_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user_model_utils::mock_user;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = UserIdentityIndex {
            identity_id: Principal::from_slice(&[u8::MAX; 29]),
            user_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = UserIdentityIndex::from_bytes(serialized_model);

        assert_eq!(model.identity_id, deserialized_model.identity_id);
        assert_eq!(model.user_id, deserialized_model.user_id);
    }

    #[test]
    fn valid_user_identities_to_indexes() {
        let mut user = mock_user();
        user.identity = Principal::from_slice(&[u8::MAX; 29]);
        let index = user.to_index_for_identity();

        assert_eq!(index.identity_id, user.identity);
        assert_eq!(index.user_id, user.id);
    }
}
