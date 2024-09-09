use orbit_essentials::types::UUID;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct RequestRateLimiterKey {
    // user: None => any principal without a user
    pub user_id: Option<UUID>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct RequestRateLimiterSize(pub u64);
