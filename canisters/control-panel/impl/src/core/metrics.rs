use crate::{models::User, repositories::USER_REPOSITORY, SERVICE_NAME};
use ic_canister_core::metrics::{
    labels, ApplicationCounterVecMetric, ApplicationGaugeMetric, ApplicationGaugeVecMetric,
    ApplicationMetric,
};
use ic_canister_core::repository::Repository;
use std::{
    collections::BTreeMap,
    ops::{Add, Sub},
};

/// A collection of user related metrics.
///
/// This list should be updated with new metrics as they are added to the system.
pub const USER_METRICS: &[&dyn ApplicationMetric<User>] = &[
    &MetricRegisteredUsers,
    &MetricDeployedWallets,
    &MetricUserWallets,
    &MetricActiveUsers,
];

/// Recompute all metrics for the canister, updating the values in the metrics registry.
///
/// This function is expensive and should be used only when necessary (e.g. canister upgrade)
pub fn recompute_all_metrics() {
    let users = USER_REPOSITORY.list();

    USER_METRICS
        .iter()
        .for_each(|metric| metric.recalculate(&users));
}

/// Metric for the number of users that have been registered, labeled by subscription status.
pub struct MetricRegisteredUsers;

impl ApplicationGaugeVecMetric<User> for MetricRegisteredUsers {
    const LABELS: &'static [&'static str] = &["status"];
}

impl ApplicationMetric<User> for MetricRegisteredUsers {
    fn name(&self) -> &'static str {
        "total_users"
    }

    fn help(&self) -> &'static str {
        "Total number of users registered on the platform, labeled by subscription status."
    }

    fn recalculate(&self, models: &[User]) {
        let mut labeled_totals = BTreeMap::new();

        for user in models {
            let label = user.subscription_status.to_string();
            let current_total = labeled_totals.get(&label).unwrap_or(&0.0);

            labeled_totals.insert(label, current_total + 1.0);
        }

        for (label, total) in labeled_totals.into_iter() {
            self.set(SERVICE_NAME, &labels! { "status" => label.as_str() }, total);
        }
    }

    fn sum(&self, current: &User, previous: Option<&User>) {
        let previous_label = previous.map(|user| user.subscription_status.to_string());
        let label = current.subscription_status.to_string();

        match (label, previous_label) {
            (label, Some(previous_label)) => {
                // Only update the metric if the label has changed.
                if label != previous_label {
                    self.dec(
                        SERVICE_NAME,
                        &labels! { "status" => previous_label.as_str() },
                    );
                    self.inc(SERVICE_NAME, &labels! { "status" => label.as_str() });
                }
            }
            (label, None) => {
                self.inc(SERVICE_NAME, &labels! { "status" => label.as_str() });
            }
        }
    }

    fn sub(&self, model: &User) {
        let label_value = model.subscription_status.to_string();

        self.dec(SERVICE_NAME, &labels! { "status" => label_value.as_str() });
    }
}

/// Metric for the number of deployed wallets that have been created by the control panel.
pub struct MetricDeployedWallets;

impl ApplicationGaugeMetric<User> for MetricDeployedWallets {}

impl ApplicationMetric<User> for MetricDeployedWallets {
    fn name(&self) -> &'static str {
        "deployed_wallets"
    }

    fn help(&self) -> &'static str {
        "Total number of deployed wallets that have been created by the control panel."
    }

    fn recalculate(&self, models: &[User]) {
        let mut deployed_wallets = 0.0;
        for user in models {
            deployed_wallets += user.deployed_wallets.len() as f64;
        }

        self.set(SERVICE_NAME, deployed_wallets);
    }

    fn sum(&self, current: &User, previous: Option<&User>) {
        let diff_deployed_wallets = current.deployed_wallets.len() as f64
            - previous.map_or(0.0, |user| user.deployed_wallets.len() as f64);

        let current_total = self.get(SERVICE_NAME);

        self.set(SERVICE_NAME, current_total.add(diff_deployed_wallets));
    }

    fn sub(&self, model: &User) {
        let current_total = self.get(SERVICE_NAME);

        self.set(
            SERVICE_NAME,
            current_total.sub(model.deployed_wallets.len() as f64),
        );
    }
}

/// Metric for the number of wallets users have associated with their user account.
pub struct MetricUserWallets;

impl ApplicationGaugeMetric<User> for MetricUserWallets {}

impl ApplicationMetric<User> for MetricUserWallets {
    fn name(&self) -> &'static str {
        "user_wallets"
    }

    fn help(&self) -> &'static str {
        "Total number of wallets users have associated with their user account."
    }

    fn recalculate(&self, models: &[User]) {
        let mut user_wallets = 0.0;
        for user in models {
            user_wallets += user.wallets.len() as f64;
        }

        self.set(SERVICE_NAME, user_wallets);
    }

    fn sum(&self, current: &User, previous: Option<&User>) {
        let diff_user_wallets =
            current.wallets.len() as f64 - previous.map_or(0.0, |user| user.wallets.len() as f64);

        let current_total = self.get(SERVICE_NAME);

        self.set(SERVICE_NAME, current_total.add(diff_user_wallets));
    }

    fn sub(&self, model: &User) {
        let current_total = self.get(SERVICE_NAME);

        self.set(SERVICE_NAME, current_total.sub(model.wallets.len() as f64));
    }
}

/// Metric that tracks the total number of active users on the platform.
pub struct MetricActiveUsers;

impl ApplicationCounterVecMetric<User> for MetricActiveUsers {
    const LABELS: &'static [&'static str] = &["time"];
}

impl ApplicationMetric<User> for MetricActiveUsers {
    fn name(&self) -> &'static str {
        "active_users"
    }

    fn help(&self) -> &'static str {
        "Total number of active users in the system, labeled by the time interval."
    }

    fn sum(&self, current: &User, previous: Option<&User>) {
        match previous {
            Some(previous) => {
                if previous.last_active == current.last_active {
                    return;
                }

                if current.last_active_intervals.hourly > previous.last_active_intervals.hourly {
                    self.inc(SERVICE_NAME, &labels! { "time" => "1h" });
                }

                if current.last_active_intervals.daily > previous.last_active_intervals.daily {
                    self.inc(SERVICE_NAME, &labels! { "time" => "1d" });
                }

                if current.last_active_intervals.weekly > previous.last_active_intervals.weekly {
                    self.inc(SERVICE_NAME, &labels! { "time" => "1w" });
                }

                if current.last_active_intervals.monthly > previous.last_active_intervals.monthly {
                    self.inc(SERVICE_NAME, &labels! { "time" => "1m" });
                }
            }
            None => {
                self.inc(SERVICE_NAME, &labels! { "time" => "1h" });
                self.inc(SERVICE_NAME, &labels! { "time" => "1d" });
                self.inc(SERVICE_NAME, &labels! { "time" => "1w" });
                self.inc(SERVICE_NAME, &labels! { "time" => "1m" });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{user_model_utils::mock_user, UserSubscriptionStatus, UserWallet};
    use candid::Principal;

    #[test]
    fn test_user_metrics() {
        let mut user = mock_user();
        user.wallets = vec![UserWallet {
            canister_id: Principal::from_slice(&[1; 29]),
            name: None,
        }];
        user.deployed_wallets = vec![
            Principal::from_slice(&[1; 29]),
            Principal::from_slice(&[2; 29]),
        ];
        user.subscription_status = UserSubscriptionStatus::Approved;
        let status = user.subscription_status.to_string();

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(MetricUserWallets.get(SERVICE_NAME), 1.0);
        assert_eq!(MetricDeployedWallets.get(SERVICE_NAME), 2.0);
        assert_eq!(
            MetricRegisteredUsers.get(SERVICE_NAME, &labels! { "status" => status.as_str() }),
            1.0
        );

        // update on user removal
        USER_REPOSITORY.remove(&user.to_key());

        assert_eq!(MetricUserWallets.get(SERVICE_NAME), 0.0);
        assert_eq!(MetricDeployedWallets.get(SERVICE_NAME), 0.0);
        assert_eq!(
            MetricRegisteredUsers.get(SERVICE_NAME, &labels! { "status" => status.as_str() }),
            0.0
        );
    }

    #[test]
    fn test_aggregated_user_metrics() {
        let mut user = mock_user();
        user.wallets = vec![UserWallet {
            canister_id: Principal::from_slice(&[1; 29]),
            name: None,
        }];
        user.deployed_wallets = vec![
            Principal::from_slice(&[1; 29]),
            Principal::from_slice(&[2; 29]),
        ];
        user.subscription_status = UserSubscriptionStatus::Approved;
        let status = user.subscription_status.to_string();

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let mut user2 = mock_user();
        user2.wallets = vec![UserWallet {
            canister_id: Principal::from_slice(&[1; 29]),
            name: None,
        }];
        user2.deployed_wallets = vec![Principal::from_slice(&[1; 29])];
        user2.subscription_status = UserSubscriptionStatus::Pending("email".to_string());
        let status2 = user2.subscription_status.to_string();

        USER_REPOSITORY.insert(user2.to_key(), user2.clone());

        assert_eq!(MetricUserWallets.get(SERVICE_NAME), 2.0);
        assert_eq!(MetricDeployedWallets.get(SERVICE_NAME), 3.0);
        assert_eq!(
            MetricRegisteredUsers.get(SERVICE_NAME, &labels! { "status" => status.as_str() }),
            1.0
        );
        assert_eq!(
            MetricRegisteredUsers.get(SERVICE_NAME, &labels! { "status" => status2.as_str() }),
            1.0
        );

        // update on user removal
        USER_REPOSITORY.remove(&user.to_key());

        assert_eq!(MetricUserWallets.get(SERVICE_NAME), 1.0);
        assert_eq!(MetricDeployedWallets.get(SERVICE_NAME), 1.0);
        assert_eq!(
            MetricRegisteredUsers.get(SERVICE_NAME, &labels! { "status" => status.as_str() }),
            0.0
        );
        assert_eq!(
            MetricRegisteredUsers.get(SERVICE_NAME, &labels! { "status" => status2.as_str() }),
            1.0
        );
    }

    #[test]
    fn test_user_metrics_diff() {
        let mut user = mock_user();
        user.wallets = vec![UserWallet {
            canister_id: Principal::from_slice(&[1; 29]),
            name: None,
        }];
        user.deployed_wallets = vec![
            Principal::from_slice(&[1; 29]),
            Principal::from_slice(&[2; 29]),
        ];
        user.subscription_status = UserSubscriptionStatus::Approved;
        let status = user.subscription_status.to_string();

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(MetricUserWallets.get(SERVICE_NAME), 1.0);
        assert_eq!(MetricDeployedWallets.get(SERVICE_NAME), 2.0);
        assert_eq!(
            MetricRegisteredUsers.get(SERVICE_NAME, &labels! { "status" => status.as_str() }),
            1.0
        );

        user.wallets = vec![
            UserWallet {
                canister_id: Principal::from_slice(&[1; 29]),
                name: None,
            },
            UserWallet {
                canister_id: Principal::from_slice(&[2; 29]),
                name: None,
            },
        ];
        user.deployed_wallets = vec![
            Principal::from_slice(&[1; 29]),
            Principal::from_slice(&[2; 29]),
            Principal::from_slice(&[3; 29]),
        ];
        user.subscription_status = UserSubscriptionStatus::Pending("test@email.com".to_string());
        let new_status = user.subscription_status.to_string();

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(MetricUserWallets.get(SERVICE_NAME), 2.0);
        assert_eq!(MetricDeployedWallets.get(SERVICE_NAME), 3.0);
        assert_eq!(
            MetricRegisteredUsers.get(SERVICE_NAME, &labels! { "status" => status.as_str() }),
            0.0
        );
        assert_eq!(
            MetricRegisteredUsers.get(SERVICE_NAME, &labels! { "status" => new_status.as_str() }),
            1.0
        );
    }

    #[test]
    fn test_active_users_metric_starts_with_none() {
        let hourly = labels! { "time" => "1h" };
        let daily = labels! { "time" => "1d" };
        let weekly = labels! { "time" => "1w" };
        let monthly = labels! { "time" => "1m" };

        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &hourly), 0.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &daily), 0.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &weekly), 0.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &monthly), 0.0);
    }

    #[test]
    fn test_active_users_metric_increments_only_if_interval_passes() {
        let hourly = labels! { "time" => "1h" };
        let daily = labels! { "time" => "1d" };
        let weekly = labels! { "time" => "1w" };
        let monthly = labels! { "time" => "1m" };

        let mut user = mock_user();
        user.set_last_active(0);

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &hourly), 1.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &daily), 1.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &weekly), 1.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &monthly), 1.0);

        // Update the user with the same last active time should not increment the metrics
        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &hourly), 1.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &daily), 1.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &weekly), 1.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &monthly), 1.0);

        // Advance the time by 1 hour should only increment the hourly metric
        user.set_last_active(60 * 60 * 1_000_000_000);

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &hourly), 2.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &daily), 1.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &weekly), 1.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &monthly), 1.0);

        // Advance the time by 1 day should only increment the hourly and daily metrics
        user.set_last_active(24 * 60 * 60 * 1_000_000_000);

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &hourly), 3.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &daily), 2.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &weekly), 1.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &monthly), 1.0);

        // Advance the time by 1 week should only increment the hourly, daily and weekly metrics
        user.set_last_active(7 * 24 * 60 * 60 * 1_000_000_000);

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &hourly), 4.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &daily), 3.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &weekly), 2.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &monthly), 1.0);

        // Advance the time by 1 month should only increment the hourly, daily, weekly and monthly metrics
        user.set_last_active(30 * 24 * 60 * 60 * 1_000_000_000);

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &hourly), 5.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &daily), 4.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &weekly), 3.0);
        assert_eq!(MetricActiveUsers.get(SERVICE_NAME, &monthly), 2.0);
    }
}
