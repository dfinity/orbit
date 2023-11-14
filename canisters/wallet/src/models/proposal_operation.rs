use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use super::{AccountId, TransferId};

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalOperation {
    Transfer(TransferOperationContext),
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperationContext {
    pub transfer_id: TransferId,
    pub account_id: AccountId,
}

impl Display for ProposalOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalOperation::Transfer(ctx) => {
                write!(
                    f,
                    "transfer id {} from account {}",
                    Uuid::from_bytes(ctx.transfer_id).hyphenated(),
                    Uuid::from_bytes(ctx.account_id).hyphenated()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_string_representation() {
        assert_eq!(
            ProposalOperation::Transfer(TransferOperationContext {
                account_id: [0; 16],
                transfer_id: [1; 16]
            })
            .to_string(),
            "transfer id 01010101-0101-0101-0101-010101010101 from account 00000000-0000-0000-0000-000000000000"
        );
    }
}
