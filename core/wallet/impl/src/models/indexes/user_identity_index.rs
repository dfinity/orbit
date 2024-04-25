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
    pub fn to_index_for_identities(&self) -> Vec<UserIdentityIndex> {
        self.identities
            .iter()
            .map(|identity| UserIdentityIndex {
                identity_id: identity.to_owned(),
                user_id: self.id,
            })
            .collect::<Vec<UserIdentityIndex>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user_test_utils::mock_user;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = UserIdentityIndex {
            identity_id: Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap(),
            user_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = UserIdentityIndex::from_bytes(serialized_model);

        assert_eq!(model.identity_id, deserialized_model.identity_id);
        assert_eq!(model.user_id, deserialized_model.user_id);
    }

    #[test]
    fn valid_user_identities_to_indexes() {
        let user_id = [u8::MAX; 16];
        let mut user = mock_user();
        user.id = user_id;
        user.identities = vec![
            Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap(),
            Principal::from_text("werw6-ayaaa-aaaaa-774aa-cai").unwrap(),
        ];

        let indexes = user.to_index_for_identities();

        assert_eq!(indexes.len(), 2);
        assert_eq!(
            indexes[0].identity_id,
            Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap()
        );
        assert_eq!(
            indexes[1].identity_id,
            Principal::from_text("werw6-ayaaa-aaaaa-774aa-cai").unwrap()
        );
        assert_eq!(indexes[0].user_id, user_id);
        assert_eq!(indexes[1].user_id, user_id);
    }
}
