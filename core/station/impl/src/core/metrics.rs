use crate::{
    models::{Account, AddressBookEntry, Proposal, ProposalPolicy, Transfer, User, UserGroup},
    repositories::{
        policy::PROPOSAL_POLICY_REPOSITORY, ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY,
        USER_GROUP_REPOSITORY, USER_REPOSITORY,
    },
    SERVICE_NAME,
};
use orbit_essentials::{
    metrics::{labels, ApplicationGaugeMetric, ApplicationGaugeVecMetric, ApplicationMetric},
    utils::amount_to_f64,
};
use orbit_essentials::{
    metrics::{ApplicationCounterMetric, ApplicationCounterVecMetric},
    repository::Repository,
};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

thread_local! {
    /// A collection of user related metrics.
    ///
    /// This list should be updated with new user metrics as they are added.
    pub static USER_METRICS: Vec<Rc<RefCell<dyn ApplicationMetric<User>>>> = vec![
        Rc::new(RefCell::new(MetricTotalUsers)),
    ];

    /// A collection of user group related metrics.
    ///
    /// This list should be updated with new user group metrics as they are added.
    pub static USER_GROUP_METRICS: Vec<Rc<RefCell<dyn ApplicationMetric<UserGroup>>>> = vec![
        Rc::new(RefCell::new(MetricTotalUserGroups)),
    ];

    /// A collection of account related metrics.
    ///
    /// This list should be updated with new account metrics as they are added.
    pub static ACCOUNT_METRICS: Vec<Rc<RefCell<dyn ApplicationMetric<Account>>>> = vec![
        Rc::new(RefCell::new(MetricTotalAccounts)),
        Rc::new(RefCell::new(MetricAssetsTotalBalance)),
    ];

    /// A collection of transfer related metrics.
    ///
    /// This list should be updated with new transfer metrics as they are added.
    pub static TRANSFER_METRICS: Vec<Rc<RefCell<dyn ApplicationMetric<Transfer>>>> = vec![
        Rc::new(RefCell::new(MetricTotalTranfers)),
    ];

    /// A collection of proposal related metrics.
    ///
    /// This list should be updated with new proposal metrics as they are added.
    pub static PROPOSAL_METRICS: Vec<Rc<RefCell<dyn ApplicationMetric<Proposal>>>> = vec![
        Rc::new(RefCell::new(MetricTotalProposalsByType)),
    ];

    /// A collection of address book entry related metrics.
    ///
    /// This list should be updated with new address book entry metrics as they are added.
    pub static ADDRESS_BOOK_METRICS: Vec<Rc<RefCell<dyn ApplicationMetric<AddressBookEntry>>>> = vec![
        Rc::new(RefCell::new(MetricTotalAddressBookEntries)),
    ];

    /// A collection of proposal policy related metrics.
    ///
    /// This list should be updated with new proposal policy metrics as they are added.
    pub static PROPOSAL_POLICY_METRICS: Vec<Rc<RefCell<dyn ApplicationMetric<ProposalPolicy>>>>
        = vec![Rc::new(RefCell::new(MetricTotalPolicies))];

}

/// Recompute all metrics for the canister, updating the values in the metrics registry.
///
/// This function is expensive and should be used only when necessary (e.g. canister upgrade)
pub fn recompute_metrics() {
    let users = USER_REPOSITORY.list();
    let user_groups = USER_GROUP_REPOSITORY.list();
    let accounts = ACCOUNT_REPOSITORY.list();

    // To avoid deserialize all the data, we can use the repository length to get the total number of entries of
    // simple gauge metrics.
    MetricTotalAddressBookEntries.set(SERVICE_NAME, ADDRESS_BOOK_REPOSITORY.len() as f64);
    MetricTotalPolicies.set(SERVICE_NAME, PROPOSAL_POLICY_REPOSITORY.len() as f64);

    USER_METRICS.with(|metrics| {
        metrics
            .iter()
            .for_each(|metric| metric.borrow_mut().recalculate(&users))
    });

    USER_GROUP_METRICS.with(|metrics| {
        metrics
            .iter()
            .for_each(|metric| metric.borrow_mut().recalculate(&user_groups))
    });

    ACCOUNT_METRICS.with(|metrics| {
        metrics
            .iter()
            .for_each(|metric| metric.borrow_mut().recalculate(&accounts))
    });
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

    fn recalculate(&mut self, models: &[User]) {
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

    fn sum(&mut self, current: &User, previous: Option<&User>) {
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

    fn sub(&mut self, current: &User) {
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

    fn recalculate(&mut self, models: &[UserGroup]) {
        self.set(
            SERVICE_NAME,
            &labels! { "status" => "active" },
            models.len() as f64,
        );
    }

    fn sum(&mut self, _: &UserGroup, previous: Option<&UserGroup>) {
        if previous.is_none() {
            self.inc(SERVICE_NAME, &labels! { "status" => "active" });
        }
    }

    fn sub(&mut self, _: &UserGroup) {
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
        "The total number of transfers that have been created."
    }

    fn sum(&mut self, _: &Transfer, previous: Option<&Transfer>) {
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

    fn recalculate(&mut self, models: &[Account]) {
        self.set(
            SERVICE_NAME,
            &labels! { "status" => "active" },
            models.len() as f64,
        );
    }

    fn sum(&mut self, _: &Account, previous: Option<&Account>) {
        if previous.is_none() {
            self.inc(SERVICE_NAME, &labels! { "status" => "active" });
        }
    }

    fn sub(&mut self, _: &Account) {
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

    fn recalculate(&mut self, accounts: &[Account]) {
        let mut labeled_totals = BTreeMap::new();

        for account in accounts {
            let label_key = (
                account.blockchain.to_string().clone(),
                account.symbol.clone().to_lowercase(),
            );

            let current_total = labeled_totals.get(&label_key).unwrap_or(&0.0);
            let balance = account.balance.clone().map(|b| b.to_u64()).unwrap_or(0u64);

            let formatted_balance = amount_to_f64(balance as i128, account.decimals);

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

    fn sum(&mut self, current: &Account, previous: Option<&Account>) {
        let blockchain = current.blockchain.to_string();
        let symbol = current.symbol.clone().to_lowercase();
        let account_labels =
            labels! { "blockchain" => blockchain.as_str(), "symbol" => symbol.as_str() };

        let balance = current.balance.clone().map(|b| b.to_u64()).unwrap_or(0u64);

        let previous_balance = previous
            .and_then(|p| p.balance.clone().map(|b| b.to_u64()))
            .unwrap_or(0u64);

        let diff_balance = balance as i128 - previous_balance as i128;
        let current_total = self.get(SERVICE_NAME, &account_labels);

        let formatted_balance = amount_to_f64(diff_balance, current.decimals);
        let new_total = current_total + formatted_balance;

        self.set(SERVICE_NAME, &account_labels, new_total.max(0.0));
    }

    fn sub(&mut self, current: &Account) {
        let blockchain = current.blockchain.to_string();
        let symbol = current.symbol.clone().to_lowercase();
        let account_labels =
            labels! { "blockchain" => blockchain.as_str(), "symbol" => symbol.as_str() };

        let balance = current.balance.clone().map(|b| b.to_u64()).unwrap_or(0u64);

        let formatted_balance = amount_to_f64(balance as i128, current.decimals);
        let current_total = self.get(SERVICE_NAME, &account_labels);

        let new_total = current_total - formatted_balance;
        self.set(SERVICE_NAME, &account_labels, new_total.max(0.0));
    }
}

/// Metric for the total number of proposals.
pub struct MetricTotalProposalsByType;

impl ApplicationCounterVecMetric<Proposal> for MetricTotalProposalsByType {
    const LABELS: &'static [&'static str] = &["type", "status"];
}

impl ApplicationMetric<Proposal> for MetricTotalProposalsByType {
    fn name(&self) -> &'static str {
        "total_proposals_by_type"
    }

    fn help(&self) -> &'static str {
        "The total number of proposals, labeled by their type."
    }

    fn sum(&mut self, current: &Proposal, previous: Option<&Proposal>) {
        let operation = current.operation.to_string();
        let status = current.status.to_type().to_string();
        let labels = labels! { "type" => operation.as_str(), "status" => status.as_str() };

        match previous {
            Some(previous) => {
                if previous.status != current.status {
                    self.inc(SERVICE_NAME, &labels);
                }
            }
            None => {
                self.inc(SERVICE_NAME, &labels);
            }
        }
    }
}

/// Metric for the total number of address book entries.
pub struct MetricTotalAddressBookEntries;

impl ApplicationGaugeMetric<AddressBookEntry> for MetricTotalAddressBookEntries {}

impl ApplicationMetric<AddressBookEntry> for MetricTotalAddressBookEntries {
    fn name(&self) -> &'static str {
        "total_address_book_entries"
    }

    fn help(&self) -> &'static str {
        "The total number of address book entries."
    }

    fn sum(&mut self, _: &AddressBookEntry, previous: Option<&AddressBookEntry>) {
        if previous.is_none() {
            self.inc(SERVICE_NAME);
        }
    }

    fn sub(&mut self, _: &AddressBookEntry) {
        self.dec(SERVICE_NAME);
    }
}

/// Metric for the total number of policies that are available.
pub struct MetricTotalPolicies;

impl ApplicationGaugeMetric<ProposalPolicy> for MetricTotalPolicies {}

impl ApplicationMetric<ProposalPolicy> for MetricTotalPolicies {
    fn name(&self) -> &'static str {
        "total_policies"
    }

    fn help(&self) -> &'static str {
        "The total number of policies that are available."
    }

    fn sum(&mut self, _: &ProposalPolicy, previous: Option<&ProposalPolicy>) {
        if previous.is_none() {
            self.inc(SERVICE_NAME);
        }
    }

    fn sub(&mut self, _: &ProposalPolicy) {
        self.dec(SERVICE_NAME);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{
            account_test_utils::mock_account,
            address_book_entry_test_utils::mock_address_book_entry,
            proposal_policy_test_utils::mock_proposal_policy, proposal_test_utils::mock_proposal,
            transfer_test_utils::mock_transfer, user_group_test_utils, user_test_utils::mock_user,
            AccountBalance, Blockchain, ProposalStatus, TransferStatus, UserStatus,
        },
        repositories::{PROPOSAL_REPOSITORY, TRANSFER_REPOSITORY},
    };
    use candid::Nat;

    #[test]
    fn test_total_users_metric() {
        let mut user = mock_user();
        user.status = UserStatus::Active;

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(
            MetricTotalUsers.get(SERVICE_NAME, &labels! { "status" => "active" }),
            1.0
        );

        user.status = UserStatus::Inactive;

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        assert_eq!(
            MetricTotalUsers.get(SERVICE_NAME, &labels! { "status" => "active" }),
            0.0
        );
        assert_eq!(
            MetricTotalUsers.get(SERVICE_NAME, &labels! { "status" => "inactive" }),
            1.0
        );
    }

    #[test]
    fn test_total_user_groups_metric() {
        user_group_test_utils::add_group("finance");
        user_group_test_utils::add_group("hr");

        assert_eq!(
            MetricTotalUserGroups.get(SERVICE_NAME, &labels! { "status" => "active" }),
            2.0
        );
    }

    #[test]
    fn test_total_accounts_metric() {
        let mut account = mock_account();
        account.blockchain = Blockchain::InternetComputer;
        account.symbol = "ICP".to_string();

        ACCOUNT_REPOSITORY.insert(account.to_key(), account);

        assert_eq!(
            MetricTotalAccounts.get(SERVICE_NAME, &labels! { "status" => "active" }),
            1.0
        );

        let mut account = mock_account();
        account.name = "Test2".to_string();
        account.blockchain = Blockchain::InternetComputer;
        account.symbol = "ICP".to_string();

        ACCOUNT_REPOSITORY.insert(account.to_key(), account);

        assert_eq!(
            MetricTotalAccounts.get(SERVICE_NAME, &labels! { "status" => "active" }),
            2.0
        );
    }

    #[test]
    fn test_total_transfers_metric() {
        let mut transfer = mock_transfer();
        transfer.status = TransferStatus::Created;

        TRANSFER_REPOSITORY.insert(transfer.to_key(), transfer.clone());

        assert_eq!(MetricTotalTranfers.get(SERVICE_NAME), 1.0);

        transfer.status = TransferStatus::Processing { started_at: 0 };

        TRANSFER_REPOSITORY.insert(transfer.to_key(), transfer.clone());

        assert_eq!(MetricTotalTranfers.get(SERVICE_NAME), 1.0);

        let transfer = mock_transfer();

        TRANSFER_REPOSITORY.insert(transfer.to_key(), transfer.clone());

        assert_eq!(MetricTotalTranfers.get(SERVICE_NAME), 2.0);
    }

    #[test]
    fn test_total_proposals_by_type_metric() {
        let mut proposal = mock_proposal();
        proposal.status = ProposalStatus::Created;

        let operation = proposal.operation.to_string();
        let status = proposal.status.to_type().to_string();
        let label = labels! { "type" => operation.as_str(), "status" => status.as_str() };

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        assert_eq!(MetricTotalProposalsByType.get(SERVICE_NAME, &label), 1.0);

        let mut proposal = mock_proposal();
        proposal.status = ProposalStatus::Created;
        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        assert_eq!(MetricTotalProposalsByType.get(SERVICE_NAME, &label), 2.0);

        proposal.status = ProposalStatus::Processing { started_at: 0 };
        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        let status = proposal.status.to_type().to_string();
        let new_label = labels! { "type" => operation.as_str(), "status" => status.as_str() };

        assert_eq!(
            MetricTotalProposalsByType.get(SERVICE_NAME, &new_label),
            1.0
        );
    }

    #[test]
    fn test_assets_balance_metric() {
        let blockchain_name = Blockchain::InternetComputer.to_string();

        let mut account = mock_account();
        account.blockchain = Blockchain::InternetComputer;
        account.symbol = "icp".to_string();
        account.balance = Some(AccountBalance {
            balance: Nat::from(1_000_000_000u64),
            last_modification_timestamp: 0,
        });
        account.decimals = 8;

        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());

        assert_eq!(
            MetricAssetsTotalBalance.get(
                SERVICE_NAME,
                &labels! { "blockchain" => blockchain_name.as_str(), "symbol" => "icp" }
            ),
            10.00000000
        );

        let mut account = mock_account();
        account.blockchain = Blockchain::InternetComputer;
        account.symbol = "icp".to_string();
        account.balance = Some(AccountBalance {
            balance: Nat::from(10_000_000_000u64),
            last_modification_timestamp: 0,
        });
        account.decimals = 8;

        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());

        assert_eq!(
            MetricAssetsTotalBalance.get(
                SERVICE_NAME,
                &labels! { "blockchain" => blockchain_name.as_str(), "symbol" => "icp" }
            ),
            110.00000000
        );

        account.balance = Some(AccountBalance {
            balance: Nat::from(100_000_000u64),
            last_modification_timestamp: 0,
        });

        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());

        assert_eq!(
            MetricAssetsTotalBalance.get(
                SERVICE_NAME,
                &labels! { "blockchain" => blockchain_name.as_str(), "symbol" => "icp" }
            ),
            11.00000000
        );
    }

    #[test]
    fn test_total_address_book_entries_metric() {
        let address_book_entry = mock_address_book_entry();

        ADDRESS_BOOK_REPOSITORY.insert(address_book_entry.to_key(), address_book_entry.clone());

        assert_eq!(MetricTotalAddressBookEntries.get(SERVICE_NAME), 1.0);

        let address_book_entry = mock_address_book_entry();

        ADDRESS_BOOK_REPOSITORY.insert(address_book_entry.to_key(), address_book_entry.clone());

        assert_eq!(MetricTotalAddressBookEntries.get(SERVICE_NAME), 2.0);
    }

    #[test]
    fn test_total_policies_metric() {
        let policy = mock_proposal_policy();

        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        assert_eq!(MetricTotalPolicies.get(SERVICE_NAME), 1.0);

        let policy = mock_proposal_policy();

        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        assert_eq!(MetricTotalPolicies.get(SERVICE_NAME), 2.0);
    }
}
