use crate::models::UserGroup;
use orbit_essentials::storable;
use orbit_essentials::types::UUID;

/// Represents the user group name index within the system.
///
/// This index is used to find the user group by name.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserGroupNameIndex {
    /// The name of the user group (e.g. "Finance").
    pub name: String,
    /// The user group id, which is a UUID.
    pub user_group_id: UUID,
}

#[derive(Clone, Debug)]
pub struct UserGroupNameIndexCriteria {
    pub name: String,
}

impl UserGroup {
    pub fn to_index_by_name(&self) -> UserGroupNameIndex {
        UserGroupNameIndex {
            name: self.name.to_owned(),
            user_group_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user_group_test_utils::mock_user_group;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = UserGroupNameIndex {
            name: "Finance".to_string(),
            user_group_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = UserGroupNameIndex::from_bytes(serialized_model);

        assert_eq!(model.name, deserialized_model.name);
        assert_eq!(model.user_group_id, deserialized_model.user_group_id);
    }

    #[test]
    fn valid_user_group_name_index_mapping() {
        let mut user_group = mock_user_group();
        user_group.id = [u8::MAX; 16];
        user_group.name = "Finance".to_string();

        let index = user_group.to_index_by_name();

        assert_eq!(index.name, "Finance".to_string());
        assert_eq!(index.user_group_id, [u8::MAX; 16]);
    }
}
