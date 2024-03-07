use crate::models::{AccountId, Proposal, ProposalId, ProposalOperation};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

/// Index of proposals by account id.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalAccountIndex {
    /// The account id that is associated with this proposal.
    pub account_id: AccountId,
    /// The proposal id, which is a UUID.
    pub proposal_id: ProposalId,
}

#[derive(Clone, Debug)]
pub struct ProposalAccountIndexCriteria {
    pub account_id: AccountId,
}

impl Proposal {
    pub fn to_index_for_account(&self) -> Option<ProposalAccountIndex> {
        if let ProposalOperation::Transfer(ctx) = &self.operation {
            return Some(ProposalAccountIndex {
                proposal_id: self.id.to_owned(),
                account_id: ctx.input.from_account_id.to_owned(),
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        proposal_test_utils::mock_proposal, Metadata, TransferOperation, TransferOperationInput,
    };
    use ic_stable_structures::Storable;
    use num_bigint::BigUint;

    #[test]
    fn valid_model_serialization() {
        let account_id = [0; 16];
        let proposal_id = [1; 16];
        let model = ProposalAccountIndex {
            account_id,
            proposal_id,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = ProposalAccountIndex::from_bytes(serialized_model);

        assert_eq!(model.proposal_id, deserialized_model.proposal_id);
        assert_eq!(model.account_id, deserialized_model.account_id);
    }

    #[test]
    fn correct_proposal_account_index_mapping() {
        let mut proposal = mock_proposal();
        proposal.operation = ProposalOperation::Transfer(TransferOperation {
            transfer_id: None,
            input: TransferOperationInput {
                amount: candid::Nat(BigUint::from(100u32)),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
                from_account_id: [0; 16],
            },
        });

        let index = proposal.to_index_for_account();

        assert!(index.is_some());
    }
}
