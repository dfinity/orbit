use crate::models::access_policy::{AccessPolicy, AccessPolicyKey};
use ic_canister_core::model::ModelKey;
use ic_canister_macros::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AllowLevel {
    Any = 1,
    Authenticated = 2,
    Users = 3,
    UserGroups = 4,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessPolicyAllowLevelIndex {
    pub allow_level: AllowLevel,
    pub access_policy_key: AccessPolicyKey,
}

pub struct AccessPolicyAllowLevelIndexCriteria {
    pub allow_level: AllowLevel,
}

impl Ord for AccessPolicyAllowLevelIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.allow_level
            .cmp(&other.allow_level)
            .then_with(|| self.access_policy_key.cmp(&other.access_policy_key))
    }
}

impl PartialOrd for AccessPolicyAllowLevelIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl AccessPolicy {
    pub fn to_index_by_allow_levels(&self) -> Vec<AccessPolicyAllowLevelIndex> {
        let mut levels = Vec::new();
        if self.allowed_any() {
            levels.push(AllowLevel::Any)
        }

        if self.allowed_authenticated() {
            levels.push(AllowLevel::Authenticated)
        }

        if !self.allowed_users().is_empty() {
            levels.push(AllowLevel::Users)
        }

        if !self.allowed_user_groups().is_empty() {
            levels.push(AllowLevel::UserGroups)
        }

        levels
            .into_iter()
            .map(|level| AccessPolicyAllowLevelIndex {
                allow_level: level,
                access_policy_key: self.key().clone(),
            })
            .collect()
    }
}
