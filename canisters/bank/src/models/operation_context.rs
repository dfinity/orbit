use super::{Transfer, Wallet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationContext {
    pub transfer: Option<Transfer>,
    pub wallet: Option<Wallet>,
}
