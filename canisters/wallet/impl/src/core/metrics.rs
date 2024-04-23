use crate::{
    core::ic_cdk::api::print,
    mappers::HelperMapper,
    models::{Account, Proposal, Transfer, User, UserGroup},
    repositories::{ACCOUNT_REPOSITORY, USER_GROUP_REPOSITORY, USER_REPOSITORY},
    SERVICE_NAME,
};
use ic_canister_core::{metrics::ApplicationCounterMetric, repository::Repository};
use ic_canister_core::{
    metrics::{labels, ApplicationGaugeVecMetric, ApplicationMetric},
    utils::format_amount,
};
use std::collections::BTreeMap;

/// A collection of user related metrics.
///
/// This list should be updated with new user metrics as they are added.
pub const USER_METRICS: [&dyn ApplicationMetric<User>; 1] = [&MetricTotalUsers];

/// A collection of user group related metrics.
///
/// This list should be updated with new user group metrics as they are added.
pub const USER_GROUP_METRICS: [&dyn ApplicationMetric<UserGroup>; 1] = [&MetricTotalUserGroups];

/// A collection of account related metrics.
///
/// This list should be updated with new account metrics as they are added.
pub const ACCOUNT_METRICS: [&dyn ApplicationMetric<Account>; 2] =
    [&MetricTotalAccounts, &MetricAssetsTotalBalance];

/// A collection of transfer related metrics.
///
/// This list should be updated with new transfer metrics as they are added.
pub const TRANSFER_METRICS: [&dyn ApplicationMetric<Transfer>; 1] = [&MetricTotalTranfers];

/// A collection of proposal related metrics.
///
/// This list should be updated with new proposal metrics as they are added.
pub const PROPOSAL_METRICS: [&dyn ApplicationMetric<Proposal>; 1] = [&MetricTotalProposals];

/// Recompute all metrics for the canister, updating the values in the metrics registry.
///
/// This function is expensive and should be used only when necessary (e.g. canister upgrade)
pub fn recompute_metrics() {
    let users = USER_REPOSITORY.list();
    let user_groups = USER_GROUP_REPOSITORY.list();
    let accounts = ACCOUNT_REPOSITORY.list();

    USER_METRICS
        .iter()
        .for_each(|metric| metric.recalculate(&users));

    USER_GROUP_METRICS
        .iter()
        .for_each(|metric| metric.recalculate(&user_groups));

    ACCOUNT_METRICS
        .iter()
        .for_each(|metric| metric.recalculate(&accounts));
}

/// Metric for the number of users that have been registered, labeled by their status.
pub struct MetricTotalUsers;

impl ApplicationGaugeVecMetric<User> for MetricTotalUsers {
    const LABELS: &'static [&'static str] = &["status"];
}

impl ApplicationMetric<User> for MetricTotalUsers {
    fn name(&self) -> &'static str {
        "total_users"
    }

    fn help(&self) -> &'static str {
        "The total number of users that are registered, labeled by their status."
    }

    fn recalculate(&self, models: &[User]) {
        let mut labeled_totals = BTreeMap::new();

        for user in models {
            let label = user.status.to_string();
            let current_total = labeled_totals.get(&label).unwrap_or(&0.0);

            labeled_totals.insert(label, current_total + 1.0);
        }

        for (label, total) in labeled_totals.into_iter() {
            self.set(SERVICE_NAME, &labels! { "status" => label.as_str() }, total);
        }
    }

    fn sum(&self, current: &User, previous: Option<&User>) {
        let label = current.status.to_string();

        if let Some(previous) = previous {
            let previous_label = previous.status.to_string();
            if label != previous_label {
                self.dec(
                    SERVICE_NAME,
                    &labels! { "status" => previous_label.as_str() },
                );
                self.inc(SERVICE_NAME, &labels! { "status" => label.as_str() });
            }
        } else {
            self.inc(SERVICE_NAME, &labels! { "status" => label.as_str() });
        }
    }

    fn sub(&self, current: &User) {
        let label = current.status.to_string();
        self.dec(SERVICE_NAME, &labels! { "status" => label.as_str() });
    }
}

/// Metric for the number of user groups that are available, labeled by their status.
///
/// User groups are only active, so the metric only has one label, once more user group statuses are added
/// this metric will need to be updated.
pub struct MetricTotalUserGroups;

impl ApplicationGaugeVecMetric<UserGroup> for MetricTotalUserGroups {
    const LABELS: &'static [&'static str] = &["status"];
}

impl ApplicationMetric<UserGroup> for MetricTotalUserGroups {
    fn name(&self) -> &'static str {
        "total_user_groups"
    }

    fn help(&self) -> &'static str {
        "The total number of user groups that are available, labeled by their status."
    }

    fn recalculate(&self, models: &[UserGroup]) {
        self.set(
            SERVICE_NAME,
            &labels! { "status" => "active" },
            models.len() as f64,
        );
    }

    fn sum(&self, _: &UserGroup, previous: Option<&UserGroup>) {
        if previous.is_none() {
            self.inc(SERVICE_NAME, &labels! { "status" => "active" });
        }
    }

    fn sub(&self, _: &UserGroup) {
        self.dec(SERVICE_NAME, &labels! { "status" => "active" });
    }
}

/// Metric for the number of transfers that have been created.
pub struct MetricTotalTranfers;

impl ApplicationCounterMetric<Transfer> for MetricTotalTranfers {}

impl ApplicationMetric<Transfer> for MetricTotalTranfers {
    fn name(&self) -> &'static str {
        "total_transfers"
    }

    fn help(&self) -> &'static str {
        "The total number of transfers that have been creted."
    }

    fn sum(&self, _: &Transfer, previous: Option<&Transfer>) {
        if previous.is_none() {
            self.inc(SERVICE_NAME);
        }
    }
}

/// Metric for the number of accounts that have been created, labeled by their status.
///
/// Accounts are only active, so the metric only has one label, once more account statuses are added this
/// metric will need to be updated.
pub struct MetricTotalAccounts;

impl ApplicationGaugeVecMetric<Account> for MetricTotalAccounts {
    const LABELS: &'static [&'static str] = &["status"];
}

impl ApplicationMetric<Account> for MetricTotalAccounts {
    fn name(&self) -> &'static str {
        "total_accounts"
    }

    fn help(&self) -> &'static str {
        "The total number of accounts that have been created, labeled by their status."
    }

    fn recalculate(&self, models: &[Account]) {
        self.set(
            SERVICE_NAME,
            &labels! { "status" => "active" },
            models.len() as f64,
        );
    }

    fn sum(&self, _: &Account, previous: Option<&Account>) {
        if previous.is_none() {
            self.inc(SERVICE_NAME, &labels! { "status" => "active" });
        }
    }

    fn sub(&self, _: &Account) {
        self.dec(SERVICE_NAME, &labels! { "status" => "active" });
    }
}

/// Metric for total balance of all accounts, labeled by the blockchain and token symbol.
pub struct MetricAssetsTotalBalance;

impl ApplicationGaugeVecMetric<Account> for MetricAssetsTotalBalance {
    const LABELS: &'static [&'static str] = &["blockchain", "symbol"];
}

impl ApplicationMetric<Account> for MetricAssetsTotalBalance {
    fn name(&self) -> &'static str {
        "assets_total_balance"
    }

    fn help(&self) -> &'static str {
        "The total balance of all accounts, labeled by the blockchain and token symbol."
    }

    fn recalculate(&self, accounts: &[Account]) {
        let mut labeled_totals = BTreeMap::new();

        for account in accounts {
            let label_key = (
                account.blockchain.to_string().clone(),
                account.symbol.clone(),
            );

            let current_total = labeled_totals.get(&label_key).unwrap_or(&0.0);
            let balance = account
                .balance
                .clone()
                .map(|b| {
                    HelperMapper::nat_to_u64(b.balance.clone()).unwrap_or_else(|_| {
                        print(format!("Failed to convert balance to u64: {:?}", b.balance));

                        0u64
                    })
                })
                .unwrap_or(0u64);

            let formatted_balance = format_amount(balance as i128, account.decimals);

            labeled_totals.insert(label_key, current_total + formatted_balance);
        }

        for ((blockchain, symbol), total) in labeled_totals.into_iter() {
            self.set(
                SERVICE_NAME,
                &labels! { "blockchain" => blockchain.as_str(), "symbol" => symbol.as_str() },
                total,
            );
        }
    }

    fn sum(&self, current: &Account, previous: Option<&Account>) {
        let blockchain = current.blockchain.to_string();
        let account_labels =
            labels! { "blockchain" => blockchain.as_str(), "symbol" => current.symbol.as_str() };

        let balance = current
            .balance
            .clone()
            .map(|b| {
                HelperMapper::nat_to_u64(b.balance.clone()).unwrap_or_else(|_| {
                    print(format!("Failed to convert balance to u64: {:?}", b.balance));

                    0u64
                })
            })
            .unwrap_or(0u64);

        let previous_balance = previous
            .and_then(|p| {
                p.balance.clone().map(|b| {
                    HelperMapper::nat_to_u64(b.balance.clone()).unwrap_or_else(|_| {
                        print(format!("Failed to convert balance to u64: {:?}", b.balance));

                        0u64
                    })
                })
            })
            .unwrap_or(0u64);

        let diff_balance = balance as i128 - previous_balance as i128;
        let current_total = self.get(SERVICE_NAME, &account_labels);

        let formatted_balance = format_amount(diff_balance, current.decimals);
        let new_total = current_total + formatted_balance;

        self.set(SERVICE_NAME, &account_labels, new_total);
    }

    fn sub(&self, current: &Account) {
        let blockchain = current.blockchain.to_string();
        let account_labels =
            labels! { "blockchain" => blockchain.as_str(), "symbol" => current.symbol.as_str() };

        let balance = current
            .balance
            .clone()
            .map(|b| {
                HelperMapper::nat_to_u64(b.balance.clone()).unwrap_or_else(|_| {
                    print(format!("Failed to convert balance to u64: {:?}", b.balance));

                    0u64
                })
            })
            .unwrap_or(0u64);

        let formatted_balance = format_amount(balance as i128, current.decimals);
        let current_total = self.get(SERVICE_NAME, &account_labels);

        let new_total = current_total - formatted_balance;
        self.set(SERVICE_NAME, &account_labels, new_total);
    }
}

/// Metric for the total number of proposals.
pub struct MetricTotalProposals;

impl ApplicationCounterMetric<Proposal> for MetricTotalProposals {}

impl ApplicationMetric<Proposal> for MetricTotalProposals {
    fn name(&self) -> &'static str {
        "total_proposals"
    }

    fn help(&self) -> &'static str {
        "The total number of proposals."
    }

    fn sum(&self, _: &Proposal, previous: Option<&Proposal>) {
        if previous.is_none() {
            self.inc(SERVICE_NAME);
        }
    }
}
