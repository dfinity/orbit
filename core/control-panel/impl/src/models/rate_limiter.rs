use crate::core::ic_cdk::api::time;
use crate::models::CanDeployStation;
use orbit_essentials::storable;

pub const MAX_DEPLOYED_STATIONS_PER_DAY: usize = 100;
pub const MAX_DEPLOYED_STATIONS_PER_USER_PER_DAY: usize = 2;

#[storable]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RateLimiter {
    unix_date: u64,
    num_deployed_stations: usize,
    max_deployed_stations: usize,
}

impl RateLimiter {
    pub fn new_global() -> Self {
        Self::new(MAX_DEPLOYED_STATIONS_PER_DAY)
    }

    pub fn new_user() -> Self {
        Self::new(MAX_DEPLOYED_STATIONS_PER_USER_PER_DAY)
    }

    fn new(max_deployed_stations: usize) -> Self {
        Self {
            unix_date: 0,
            num_deployed_stations: 0,
            max_deployed_stations,
        }
    }

    pub fn can_deploy_station(&self) -> CanDeployStation {
        match self.remaining_quota() {
            Some(remaining) => CanDeployStation::Allowed(remaining),
            None => CanDeployStation::QuotaExceeded,
        }
    }

    pub fn remaining_quota(&self) -> Option<usize> {
        let current_unix_date = time() / 86_400_000_000_000;
        if self.unix_date == current_unix_date {
            if self.num_deployed_stations >= self.max_deployed_stations {
                None
            } else {
                Some(self.max_deployed_stations - self.num_deployed_stations)
            }
        } else {
            Some(self.max_deployed_stations)
        }
    }

    pub fn add_deployed_station(&mut self) {
        let current_unix_date = time() / 86_400_000_000_000;
        if self.unix_date == current_unix_date {
            self.num_deployed_stations += 1;
        } else {
            self.unix_date = current_unix_date;
            self.num_deployed_stations = 1;
        }
    }
}
