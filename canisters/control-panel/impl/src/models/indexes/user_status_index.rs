use crate::models::{User, UserId, UserSubscriptionStatus};
use candid::Principal;
use ic_canister_macros::storable;

/// The subscription status of an user to be stored in the user index.
/// Unlike `UserSubscriptionStatus`, the `Pending` variant does not
/// contain an e-mail address to facilitate searching *all* users
/// with the `Pending` status irrespective of their e-mail address.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserIndexSubscriptionStatus {
    Unsubscribed,
    Pending,
    Approved,
    Denylisted,
}

impl From<&UserSubscriptionStatus> for UserIndexSubscriptionStatus {
    fn from(status: &UserSubscriptionStatus) -> Self {
        match status {
            UserSubscriptionStatus::Unsubscribed => UserIndexSubscriptionStatus::Unsubscribed,
            UserSubscriptionStatus::Pending(_) => UserIndexSubscriptionStatus::Pending,
            UserSubscriptionStatus::Approved => UserIndexSubscriptionStatus::Approved,
            UserSubscriptionStatus::Denylisted => UserIndexSubscriptionStatus::Denylisted,
        }
    }
}

/// Represents an user identity index within the system.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserStatusIndex {
    /// The identity associated with the user.
    pub status: UserIndexSubscriptionStatus,
    /// The user id, which is a UUID.
    pub user_id: UserId,
}

#[derive(Clone, Debug)]
pub struct UserStatusIndexCriteria {
    pub status: UserIndexSubscriptionStatus,
}

impl User {
    pub fn to_index_for_status(&self) -> UserStatusIndex {
        UserStatusIndex {
            status: (&self.subscription_status).into(),
            user_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UserSubscriptionStatus;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = UserStatusIndex {
            status: UserIndexSubscriptionStatus::Unsubscribed,
            user_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = UserStatusIndex::from_bytes(serialized_model);

        assert_eq!(model.status, deserialized_model.status);
        assert_eq!(model.user_id, deserialized_model.user_id);
    }

    #[test]
    fn valid_user_status_to_index() {
        let user = User {
            id: [u8::MAX; 16],
            identity: Principal::from_slice(&[u8::MAX; 29]),
            subscription_status: UserSubscriptionStatus::Pending("john@example.com".to_string()),
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 10,
        };
        let index = user.to_index_for_status();

        assert_eq!(index.status, (&user.subscription_status).into());
        assert_eq!(index.user_id, user.id);
    }
}
