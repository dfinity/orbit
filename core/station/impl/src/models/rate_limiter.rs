use orbit_essentials::types::UUID;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct RequestRateLimiterKey {
    // user: None => any principal without a user
    pub user_id: Option<UUID>,
    pub request_rate_limiter_type: RequestRateLimiterType,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RequestRateLimiterType {
    Cheap,
    Expensive,
}
