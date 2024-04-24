use crate::{models::User, repositories::USER_REPOSITORY, SERVICE_NAME};
use ic_canister_core::metrics::{
    labels, ApplicationGaugeMetric, ApplicationGaugeVecMetric, ApplicationMetric,
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
}
