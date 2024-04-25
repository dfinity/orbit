use crate::models::{User, UserStatus};
use ic_canister_core::types::UUID;
use ic_canister_macros::storable;

/// Represents an index to facilitate the search of a user by the group and the user status.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserStatusGroupIndex {
    pub group_id: UUID,
    pub user_status: UserStatus,
    pub user_id: UUID,
}

#[derive(Clone, Debug)]
pub struct UserStatusGroupIndexCriteria {
    pub group_id: UUID,
    pub user_status: UserStatus,
}

impl User {
    pub fn to_index_for_groups(&self) -> Vec<UserStatusGroupIndex> {
        self.groups
            .iter()
            .map(|group_id| UserStatusGroupIndex {
                group_id: group_id.to_owned(),
                user_status: self.status.to_owned(),
                user_id: self.id,
            })
            .collect::<Vec<UserStatusGroupIndex>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user_test_utils::mock_user;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = UserStatusGroupIndex {
            group_id: [0; 16],
            user_status: UserStatus::Active,
            user_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = UserStatusGroupIndex::from_bytes(serialized_model);

        assert_eq!(model.group_id, deserialized_model.group_id);
        assert_eq!(model.user_id, deserialized_model.user_id);
        assert_eq!(model.user_status, deserialized_model.user_status);
    }

    #[test]
    fn valid_user_groups_to_indexes() {
        let user_id = [u8::MAX; 16];
        let mut user = mock_user();
        user.id = user_id;
        user.groups = vec![[0; 16], [1; 16]];

        let indexes = user.to_index_for_groups();

        assert_eq!(indexes.len(), 2);
        assert_eq!(indexes[0].group_id, [0; 16]);
        assert_eq!(indexes[1].group_id, [1; 16]);
        assert_eq!(indexes[0].user_id, user_id);
        assert_eq!(indexes[1].user_id, user_id);
        assert_eq!(indexes[0].user_status, user.status);
        assert_eq!(indexes[1].user_status, user.status);
    }
}
