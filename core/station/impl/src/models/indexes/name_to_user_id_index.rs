use crate::{
    core::utils::format_unique_string,
    models::{User, UserId},
};
use orbit_essentials::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NameToUserIdIndex {
    /// The name of the account.
    pub name: String,
    /// The account id, which is a UUID.
    pub user_id: UserId,
}

#[derive(Clone, Debug)]
pub struct NameToUserIdIndexCriteria {
    pub name: String,
}

impl User {
    pub fn to_index_by_name(&self) -> NameToUserIdIndex {
        NameToUserIdIndex {
            name: format_unique_string(&self.name.clone()),
            user_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::user_test_utils::mock_user;

    #[test]
    fn test_name_to_user_id_association() {
        let mut user = mock_user();
        user.name = "testuser".to_string();

        let index = user.to_index_by_name();

        assert_eq!(index.name, user.name);
        assert_eq!(index.user_id, user.id);
    }

    #[test]
    fn test_name_is_formatted() {
        let mut user = mock_user();
        user.name = "Test User".to_string();

        let index = user.to_index_by_name();

        assert_eq!(index.name, "testuser");
    }
}
