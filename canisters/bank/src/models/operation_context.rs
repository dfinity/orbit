use super::{Transfer, Account};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationContext {
    pub transfer: Option<Transfer>,
    pub account: Option<Account>,
}
