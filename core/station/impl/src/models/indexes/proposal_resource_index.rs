use crate::models::{resource::Resource, Proposal};
use ic_canister_core::types::UUID;
use ic_canister_macros::storable;

/// Index of proposals by the resource derived from the proposal operation.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalResourceIndex {
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
    /// The resource derived from the proposal operation.
    pub resource: Resource,
}

#[derive(Clone, Debug)]
pub struct ProposalResourceIndexCriteria {
    pub proposal_id: UUID,
}

impl Proposal {
    pub fn to_index_for_resource(&self) -> Vec<ProposalResourceIndex> {
        self.operation
            .to_resources()
            .iter()
            .map(|resource| ProposalResourceIndex {
                proposal_id: self.id.to_owned(),
                resource: resource.to_owned(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        proposal_test_utils::mock_proposal, resource::UserResourceAction, AddUserOperation,
        AddUserOperationInput, ProposalOperation, UserStatus,
    };
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let proposal_id = [1; 16];
        let model = ProposalResourceIndex {
            proposal_id,
            resource: Resource::User(UserResourceAction::Create),
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = ProposalResourceIndex::from_bytes(serialized_model);

        assert_eq!(model.proposal_id, deserialized_model.proposal_id);
        assert!(matches!(
            deserialized_model.resource,
            Resource::User(UserResourceAction::Create)
        ));
    }

    #[test]
    fn valid_proposal_resource_indexes() {
        let mut proposal = mock_proposal();
        proposal.id = [1; 16];
        proposal.proposed_by = [u8::MAX; 16];
        proposal.operation = ProposalOperation::AddUser(AddUserOperation {
            input: AddUserOperationInput {
                groups: vec![],
                identities: vec![],
                name: None,
                status: UserStatus::Active,
            },
            user_id: None,
        });

        let index_entries = proposal.to_index_for_resource();

        assert_eq!(index_entries.len(), 1);
        assert_eq!(index_entries[0].proposal_id, proposal.id);
        assert!(matches!(
            index_entries[0].resource,
            Resource::User(UserResourceAction::Create)
        ));
    }
}
