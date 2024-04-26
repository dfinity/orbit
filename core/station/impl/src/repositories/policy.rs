use crate::{
    core::{
        metrics::PROPOSAL_POLICY_METRICS, with_memory_manager, Memory, PROPOSAL_POLICIES_MEMORY_ID,
    },
    models::{
        indexes::policy_resource_index::PolicyResourceIndexCriteria, resource::Resource,
        ProposalPolicy,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::{IndexRepository, RefreshIndexMode, Repository};
use orbit_essentials::types::UUID;
use std::{cell::RefCell, sync::Arc};

use super::indexes::policy_resource_index::PolicyResourceIndexRepository;

thread_local! {
  /// The memory reference to the proposal policies repository.
  static DB: RefCell<StableBTreeMap<UUID, ProposalPolicy, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_POLICIES_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref PROPOSAL_POLICY_REPOSITORY: Arc<ProposalPolicyRepository> =
        Arc::new(ProposalPolicyRepository::default());
}

/// A repository that enables managing proposal policies in stable memory.
#[derive(Default, Debug)]
pub struct ProposalPolicyRepository {
    resource_index: PolicyResourceIndexRepository,
}

impl Repository<UUID, ProposalPolicy> for ProposalPolicyRepository {
    fn list(&self) -> Vec<ProposalPolicy> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &UUID) -> Option<ProposalPolicy> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: UUID, value: ProposalPolicy) -> Option<ProposalPolicy> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            // Update metrics when a policy is upserted.
            PROPOSAL_POLICY_METRICS.with(|metrics| {
                metrics
                    .iter()
                    .for_each(|metric| metric.borrow_mut().sum(&value, prev.as_ref()))
            });

            self.resource_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map(|prev| prev.to_index_for_resource())
                        .unwrap_or_default(),
                    current: value.to_index_for_resource(),
                });

            prev
        })
    }

    fn remove(&self, key: &UUID) -> Option<ProposalPolicy> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            // Update metrics when a policy is removed.
            if let Some(prev) = &prev {
                PROPOSAL_POLICY_METRICS.with(|metrics| {
                    metrics
                        .iter()
                        .for_each(|metric| metric.borrow_mut().sub(prev))
                });
            }

            self.resource_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map(|prev| prev.to_index_for_resource())
                        .unwrap_or_default(),
                });

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl ProposalPolicyRepository {
    pub fn find_by_resource(&self, resource: Resource) -> Vec<ProposalPolicy> {
        let ids = self
            .resource_index
            .find_by_criteria(PolicyResourceIndexCriteria { resource });

        ids.iter().filter_map(|id| self.get(id)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        criteria::Criteria,
        indexes::policy_resource_index::PolicyResourceIndex,
        proposal_policy_test_utils::mock_proposal_policy,
        resource::{AccountResourceAction, Resource, ResourceId, ResourceIds},
        specifier::ProposalSpecifier,
    };

    #[test]
    fn perform_crud() {
        let repository = ProposalPolicyRepository::default();
        let policy = mock_proposal_policy();

        assert!(repository.get(&policy.id).is_none());

        repository.insert(policy.id, policy.clone());

        assert!(repository.get(&policy.id).is_some());
        assert!(repository.remove(&policy.id).is_some());
        assert!(repository.get(&policy.id).is_none());
    }

    #[test]
    fn update_policy_resource_index_on_policy_mutation() {
        let repository = ProposalPolicyRepository::default();

        let policy = mock_proposal_policy();

        repository.insert(policy.id, policy.clone());

        assert!(repository.resource_index.len() == 1);

        assert!(repository.resource_index.exists(&PolicyResourceIndex {
            policy_id: policy.id,
            resource: Resource::Account(AccountResourceAction::Create),
        }));

        let mut other_policy = ProposalPolicy {
            criteria: Criteria::AutoAdopted,
            id: [1; 16],
            specifier: ProposalSpecifier::Transfer(ResourceIds::Ids(vec![
                [10; 16], [11; 16], [12; 16],
            ])),
        };

        repository.insert(other_policy.id, other_policy.clone());

        assert!(repository.resource_index.len() == 4);

        assert!(repository.resource_index.exists(&PolicyResourceIndex {
            policy_id: policy.id,
            resource: Resource::Account(AccountResourceAction::Create),
        }));
        assert!(repository.resource_index.exists(&PolicyResourceIndex {
            policy_id: other_policy.id,
            resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id([10; 16]))),
        }));
        assert!(repository.resource_index.exists(&PolicyResourceIndex {
            policy_id: other_policy.id,
            resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id([11; 16]))),
        }));
        assert!(repository.resource_index.exists(&PolicyResourceIndex {
            policy_id: other_policy.id,
            resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id([12; 16]))),
        }));

        other_policy.specifier =
            ProposalSpecifier::Transfer(ResourceIds::Ids(vec![[13; 16], [14; 16]]));

        repository.insert(other_policy.id, other_policy.clone());

        assert!(repository.resource_index.len() == 3);

        assert!(repository.resource_index.exists(&PolicyResourceIndex {
            policy_id: policy.id,
            resource: Resource::Account(AccountResourceAction::Create),
        }));
        assert!(repository.resource_index.exists(&PolicyResourceIndex {
            policy_id: other_policy.id,
            resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id([13; 16]))),
        }));
        assert!(repository.resource_index.exists(&PolicyResourceIndex {
            policy_id: other_policy.id,
            resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id([14; 16]))),
        }));

        repository.remove(&other_policy.id);

        assert!(repository.resource_index.len() == 1);

        assert!(repository.resource_index.exists(&PolicyResourceIndex {
            policy_id: policy.id,
            resource: Resource::Account(AccountResourceAction::Create),
        }));
    }
}
