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
        let current_unix_date = time() / 86_400_000_000_000;
        if self.unix_date == current_unix_date {
            if self.num_deployed_stations >= self.max_deployed_stations {
                CanDeployStation::QuotaExceeded
            } else {
                CanDeployStation::Allowed
            }
        } else {
            CanDeployStation::Allowed
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
