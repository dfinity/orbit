use super::User;
use crate::core::CanisterConfig;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalletSettings {
    /// The current configuration of the wallet canister.
    pub config: CanisterConfig,
    /// The list of users that are considered as owners of the wallet canister.
    pub owners: Vec<User>,
}
