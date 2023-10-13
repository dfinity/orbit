use super::Account;
use crate::core::CanisterConfig;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BankSettings {
    /// The current configuration of the bank canister.
    pub config: CanisterConfig,
    /// The list of accounts that are considered as owners of the bank canister.
    pub owners: Vec<Account>,
}
