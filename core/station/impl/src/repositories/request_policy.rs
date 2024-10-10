use super::indexes::request_policy_resource_index::{
    ExternalCanisterPoliciesList, RequestPolicyResourceIndexRepository,
};
use crate::{
    core::{
        metrics::REQUEST_POLICY_METRICS, with_memory_manager, Memory, REQUEST_POLICIES_MEMORY_ID,
    },
    models::{
        indexes::request_policy_resource_index::RequestPolicyResourceIndexCriteria,
        resource::{Resource, ValidationMethodResourceTarget},
        RequestPolicy,
    },
};
use candid::Principal;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::{IndexRepository, IndexedRepository, Repository, StableDb};
use orbit_essentials::types::UUID;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  /// The memory reference to the request policies repository.
  static DB: RefCell<StableBTreeMap<UUID, RequestPolicy, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_POLICIES_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref REQUEST_POLICY_REPOSITORY: Arc<RequestPolicyRepository> =
        Arc::new(RequestPolicyRepository::default());
}

/// A repository that enables managing request policies in stable memory.
#[derive(Default, Debug)]
pub struct RequestPolicyRepository {
    resource_index: RequestPolicyResourceIndexRepository,
}

impl StableDb<UUID, RequestPolicy, VirtualMemory<Memory>> for RequestPolicyRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<UUID, RequestPolicy, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<UUID, RequestPolicy, VirtualMemory<Memory>> for RequestPolicyRepository {
    fn remove_entry_indexes(&self, entry: &RequestPolicy) {
        entry.to_index_for_resource().iter().for_each(|index| {
            self.resource_index.remove(index);
        });
    }

    fn add_entry_indexes(&self, entry: &RequestPolicy) {
        entry.to_index_for_resource().into_iter().for_each(|index| {
            self.resource_index.insert(index);
        });
    }

    /// Clears the indexes of the repository.
    fn clear_indexes(&self) {
        self.resource_index.clear();
    }
}

impl Repository<UUID, RequestPolicy, VirtualMemory<Memory>> for RequestPolicyRepository {
    fn insert(&self, key: UUID, value: RequestPolicy) -> Option<RequestPolicy> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            // Update metrics when a policy is upserted.
            REQUEST_POLICY_METRICS.with(|metrics| {
                metrics
                    .iter()
                    .for_each(|metric| metric.borrow_mut().sum(&value, prev.as_ref()))
            });

            self.save_entry_indexes(&value, prev.as_ref());

            prev
        })
    }

    fn remove(&self, key: &UUID) -> Option<RequestPolicy> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            // Update metrics when a policy is removed.
            if let Some(prev) = &prev {
                REQUEST_POLICY_METRICS.with(|metrics| {
                    metrics
                        .iter()
                        .for_each(|metric| metric.borrow_mut().sub(prev))
                });

                self.remove_entry_indexes(prev);
            }

            prev
        })
    }
}

impl RequestPolicyRepository {
    pub fn find_by_resource(&self, resource: Resource) -> Vec<RequestPolicy> {
        let ids = self
            .resource_index
            .find_by_criteria(RequestPolicyResourceIndexCriteria { resource });

        ids.iter().filter_map(|id| self.get(id)).collect()
    }

    /// Finds all external canister policies related to the specified canister id.
    ///
    /// Includes:
    ///
    /// - `Change` related policies.
    /// - `Call` related policies.
    pub fn find_external_canister_policies(
        &self,
        canister_id: &Principal,
    ) -> ExternalCanisterPoliciesList {
        self.resource_index
            .find_external_canister_policies(canister_id)
    }

    /// Finds all external canister call policies related to the specified canister id and execution method.
    pub fn find_external_canister_call_policies_by_execution_method(
        &self,
        canister_id: &Principal,
        execution_method: &str,
    ) -> Vec<UUID> {
        self.resource_index
            .find_external_canister_call_policies_by_execution_method(canister_id, execution_method)
    }

    /// Finds all external canister call policies related to the specified canister id, execution method and
    /// validation method.
    pub fn find_external_canister_call_policies_by_execution_and_validation_method(
        &self,
        canister_id: &Principal,
        execution_method: &str,
        validation_method: &ValidationMethodResourceTarget,
    ) -> Vec<UUID> {
        self.resource_index
            .find_external_canister_call_policies_by_execution_and_validation_method(
                canister_id,
                execution_method,
                validation_method,
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        indexes::request_policy_resource_index::RequestPolicyResourceIndex,
        request_policy_rule::RequestPolicyRule,
        request_policy_test_utils::mock_request_policy,
        request_specifier::RequestSpecifier,
        resource::{AccountResourceAction, Resource, ResourceId, ResourceIds},
    };

    #[test]
    fn perform_crud() {
        let repository = RequestPolicyRepository::default();
        let policy = mock_request_policy();

        assert!(repository.get(&policy.id).is_none());

        repository.insert(policy.id, policy.clone());

        assert!(repository.get(&policy.id).is_some());
        assert!(repository.remove(&policy.id).is_some());
        assert!(repository.get(&policy.id).is_none());
    }

    #[test]
    fn update_policy_resource_index_on_policy_mutation() {
        let repository = RequestPolicyRepository::default();

        let policy = mock_request_policy();

        repository.insert(policy.id, policy.clone());

        assert!(repository.resource_index.len() == 1);

        assert!(repository
            .resource_index
            .exists(&RequestPolicyResourceIndex {
                policy_id: policy.id,
                resource: Resource::Account(AccountResourceAction::Create),
            }));

        let mut other_policy = RequestPolicy {
            rule: RequestPolicyRule::AutoApproved,
            id: [1; 16],
            specifier: RequestSpecifier::Transfer(ResourceIds::Ids(vec![
                [10; 16], [11; 16], [12; 16],
            ])),
        };

        repository.insert(other_policy.id, other_policy.clone());

        assert!(repository.resource_index.len() == 4);

        assert!(repository
            .resource_index
            .exists(&RequestPolicyResourceIndex {
                policy_id: policy.id,
                resource: Resource::Account(AccountResourceAction::Create),
            }));
        assert!(repository
            .resource_index
            .exists(&RequestPolicyResourceIndex {
                policy_id: other_policy.id,
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    [10; 16]
                ))),
            }));
        assert!(repository
            .resource_index
            .exists(&RequestPolicyResourceIndex {
                policy_id: other_policy.id,
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    [11; 16]
                ))),
            }));
        assert!(repository
            .resource_index
            .exists(&RequestPolicyResourceIndex {
                policy_id: other_policy.id,
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    [12; 16]
                ))),
            }));

        other_policy.specifier =
            RequestSpecifier::Transfer(ResourceIds::Ids(vec![[13; 16], [14; 16]]));

        repository.insert(other_policy.id, other_policy.clone());

        assert!(repository.resource_index.len() == 3);

        assert!(repository
            .resource_index
            .exists(&RequestPolicyResourceIndex {
                policy_id: policy.id,
                resource: Resource::Account(AccountResourceAction::Create),
            }));
        assert!(repository
            .resource_index
            .exists(&RequestPolicyResourceIndex {
                policy_id: other_policy.id,
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    [13; 16]
                ))),
            }));
        assert!(repository
            .resource_index
            .exists(&RequestPolicyResourceIndex {
                policy_id: other_policy.id,
                resource: Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    [14; 16]
                ))),
            }));

        repository.remove(&other_policy.id);

        assert!(repository.resource_index.len() == 1);

        assert!(repository
            .resource_index
            .exists(&RequestPolicyResourceIndex {
                policy_id: policy.id,
                resource: Resource::Account(AccountResourceAction::Create),
            }));
    }
}

#[cfg(feature = "canbench")]
mod benchs {
    use super::*;
    use crate::models::{
        request_specifier::RequestSpecifier,
        resource::{
            CallExternalCanisterResourceTarget, ExecutionMethodResourceTarget, ExternalCanisterId,
            ValidationMethodResourceTarget,
        },
        CanisterMethod, RequestPolicyRule,
    };
    use canbench_rs::{bench, BenchResult};
    use uuid::Uuid;

    #[bench(raw)]
    fn find_500_external_canister_policies_from_50k_dataset() -> BenchResult {
        // adds 50k policies: 100 different canisters with 10 change policies and 490 call policies each
        for i in 0..100 {
            let canister_id = Principal::from_slice(&[i; 29]);
            let mut policies = Vec::new();
            for _ in 0..10 {
                policies.push(RequestPolicy {
                    id: *Uuid::new_v4().as_bytes(),
                    rule: RequestPolicyRule::AutoApproved,
                    specifier: RequestSpecifier::ChangeExternalCanister(
                        ExternalCanisterId::Canister(canister_id),
                    ),
                });
            }

            for j in 0..90 {
                policies.push(RequestPolicy {
                    id: *Uuid::new_v4().as_bytes(),
                    rule: RequestPolicyRule::AutoApproved,
                    specifier: RequestSpecifier::CallExternalCanister(
                        CallExternalCanisterResourceTarget {
                            validation_method: ValidationMethodResourceTarget::No,
                            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                CanisterMethod {
                                    canister_id,
                                    method_name: format!("method_{}", j),
                                },
                            ),
                        },
                    ),
                });
            }

            for j in 0..400 {
                policies.push(RequestPolicy {
                    id: *Uuid::new_v4().as_bytes(),
                    rule: RequestPolicyRule::AutoApproved,
                    specifier: RequestSpecifier::CallExternalCanister(
                        CallExternalCanisterResourceTarget {
                            validation_method: ValidationMethodResourceTarget::ValidationMethod(
                                CanisterMethod {
                                    canister_id,
                                    method_name: format!("validate_method_{}", j),
                                },
                            ),
                            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                CanisterMethod {
                                    canister_id,
                                    method_name: format!("method_{}", j),
                                },
                            ),
                        },
                    ),
                });
            }

            for policy in policies {
                REQUEST_POLICY_REPOSITORY.insert(policy.id, policy);
            }
        }

        canbench_rs::bench_fn(|| {
            let lookup_canister_id = Principal::from_slice(&[30; 29]);
            let policies =
                REQUEST_POLICY_REPOSITORY.find_external_canister_policies(&lookup_canister_id);

            if policies.len() != 500 {
                panic!("Expected 500 policies, got {}", policies.len());
            }
        })
    }
}
