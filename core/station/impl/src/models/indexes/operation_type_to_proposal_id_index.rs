use crate::models::{
    proposal_operation_filter_type::ProposalOperationFilterType, Proposal, ProposalOperation,
};
use orbit_essentials::storable;
use orbit_essentials::types::UUID;

/// Index of proposals by the voters' user id.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationTypeToProposalIdIndex {
    /// The user that has voted on this proposal.
    pub operation_type: ProposalOperationFilterType,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct OperationTypeToProposalIdIndexCriteria {
    pub operation_type: ProposalOperationFilterType,
}

impl Proposal {
    pub fn to_index_by_operation_types(&self) -> Vec<OperationTypeToProposalIdIndex> {
        match &self.operation {
            ProposalOperation::Transfer(operation) => {
                vec![
                    OperationTypeToProposalIdIndex {
                        operation_type: ProposalOperationFilterType::Transfer(None),
                        proposal_id: self.id,
                    },
                    OperationTypeToProposalIdIndex {
                        operation_type: ProposalOperationFilterType::Transfer(Some(
                            operation.input.from_account_id,
                        )),
                        proposal_id: self.id,
                    },
                ]
            }
            ProposalOperation::AddAccount(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::AddAccount,
                proposal_id: self.id,
            }],
            ProposalOperation::EditAccount(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::EditAccount,
                proposal_id: self.id,
            }],
            ProposalOperation::AddUser(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::AddUser,
                proposal_id: self.id,
            }],
            ProposalOperation::EditUser(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::EditUser,
                proposal_id: self.id,
            }],
            ProposalOperation::AddUserGroup(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::AddUserGroup,
                proposal_id: self.id,
            }],
            ProposalOperation::EditUserGroup(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::EditUserGroup,
                proposal_id: self.id,
            }],
            ProposalOperation::RemoveUserGroup(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::RemoveUserGroup,
                proposal_id: self.id,
            }],
            ProposalOperation::ChangeCanister(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::ChangeCanister,
                proposal_id: self.id,
            }],
            ProposalOperation::EditAccessPolicy(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::EditAccessPolicy,
                proposal_id: self.id,
            }],
            ProposalOperation::AddProposalPolicy(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::AddProposalPolicy,
                proposal_id: self.id,
            }],
            ProposalOperation::EditProposalPolicy(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::EditProposalPolicy,
                proposal_id: self.id,
            }],
            ProposalOperation::RemoveProposalPolicy(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::RemoveProposalPolicy,
                proposal_id: self.id,
            }],
            ProposalOperation::AddAddressBookEntry(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::AddAddressBookEntry,
                proposal_id: self.id,
            }],
            ProposalOperation::EditAddressBookEntry(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::EditAddressBookEntry,
                proposal_id: self.id,
            }],
            ProposalOperation::RemoveAddressBookEntry(_) => vec![OperationTypeToProposalIdIndex {
                operation_type: ProposalOperationFilterType::RemoveAddressBookEntry,
                proposal_id: self.id,
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        indexes::operation_type_to_proposal_id_index::OperationTypeToProposalIdIndex,
        proposal_operation_filter_type::ProposalOperationFilterType,
        proposal_test_utils::mock_proposal, Metadata, ProposalOperation, TransferOperation,
        TransferOperationInput,
    };
    use ic_stable_structures::Storable;
    use num_bigint::BigUint;

    #[test]
    fn valid_model_serialization() {
        let proposal_id = [1; 16];
        let model = OperationTypeToProposalIdIndex {
            operation_type: ProposalOperationFilterType::RemoveAddressBookEntry,
            proposal_id,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = OperationTypeToProposalIdIndex::from_bytes(serialized_model);

        assert_eq!(model.proposal_id, deserialized_model.proposal_id);
        assert_eq!(model.operation_type, deserialized_model.operation_type);
    }

    #[test]
    fn valid_user_voter_indexes() {
        let mut proposal = mock_proposal();
        let account_id = [0; 16];
        proposal.operation = ProposalOperation::Transfer(TransferOperation {
            transfer_id: None,
            input: TransferOperationInput {
                amount: candid::Nat(BigUint::from(100u32)),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
                from_account_id: account_id,
            },
        });

        let indexes = proposal.to_index_by_operation_types();

        assert_eq!(indexes.len(), 2);
        assert!(indexes
            .iter()
            .any(|i| i.operation_type == ProposalOperationFilterType::Transfer(None)));
        assert!(indexes
            .iter()
            .any(|i| i.operation_type == ProposalOperationFilterType::Transfer(Some(account_id))));
    }
}
