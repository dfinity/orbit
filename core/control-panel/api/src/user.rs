use crate::TimestampRfc3339;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserDTO {
    pub identity: Principal,
    pub subscription_status: UserSubscriptionStatusDTO,
    pub last_active: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetUserResponse {
    pub user: UserDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SubscribedUserDTO {
    pub user_principal: Principal,
    pub email: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetWaitingListResponse {
    pub subscribed_users: Vec<SubscribedUserDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum UserSubscriptionStatusDTO {
    Unsubscribed,
    Pending,
    Approved,
    Denylisted,
}

impl std::fmt::Display for UserSubscriptionStatusDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserSubscriptionStatusDTO::Unsubscribed => write!(f, "unsubscribed"),
            UserSubscriptionStatusDTO::Pending => write!(f, "pending"),
            UserSubscriptionStatusDTO::Approved => write!(f, "approved"),
            UserSubscriptionStatusDTO::Denylisted => write!(f, "denylisted"),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UpdateWaitingListInput {
    pub users: Vec<Principal>,
    pub new_status: UserSubscriptionStatusDTO,
}
