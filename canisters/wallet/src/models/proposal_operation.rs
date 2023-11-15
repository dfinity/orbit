use super::AccountId;
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalOperation {
    Transfer(TransferOperation),
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperation {
    pub from_account_id: AccountId,
    pub to: String,
    pub amount: candid::Nat,
    pub metadata: Vec<(String, String)>,
    pub network: String,
    pub fee: Option<candid::Nat>,
}

impl Display for ProposalOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalOperation::Transfer(ctx) => {
                write!(
                    f,
                    "transfer from account {}",
                    Uuid::from_bytes(ctx.from_account_id).hyphenated(),
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
            ProposalOperation::Transfer(TransferOperation {
                to: "0x1234".to_string(),
                fee: None,
                metadata: vec![],
                network: "mainnet".to_string(),
                amount: candid::Nat::from(0),
                from_account_id: [0; 16],
            })
            .to_string(),
            "transfer from account 00000000-0000-0000-0000-000000000000"
        );
    }
}
