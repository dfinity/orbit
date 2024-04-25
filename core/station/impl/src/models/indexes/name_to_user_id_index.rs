use crate::{
    core::utils::format_unique_string,
    models::{User, UserId},
};
use ic_canister_macros::storable;

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
    pub fn to_index_by_name(&self) -> Option<NameToUserIdIndex> {
        if let Some(name) = self.name.as_ref() {
            return Some(NameToUserIdIndex {
                name: format_unique_string(name),
                user_id: self.id,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::models::user_test_utils::mock_user;

    #[test]
    fn test_name_to_user_id_association() {
        let mut user = mock_user();
        user.name = Some("testuser".to_string());

        let index = user.to_index_by_name();
        let index = index.unwrap();

        assert_eq!(index.name, user.name.unwrap());
        assert_eq!(index.user_id, user.id);
    }

    #[test]
    fn test_name_is_formatted() {
        let mut user = mock_user();
        user.name = Some("Test User".to_string());

        let index = user.to_index_by_name();
        let index = index.unwrap();

        assert_eq!(index.name, "testuser");
    }
}
