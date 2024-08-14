use crate::{
    core::{with_memory_manager, Memory, POLICY_RESOURCE_INDEX_MEMORY_ID},
    models::{
        indexes::request_policy_resource_index::{
            RequestPolicyResourceIndex, RequestPolicyResourceIndexCriteria,
        },
        resource::{
            CallExternalCanisterResourceTarget, ExecutionMethodResourceTarget, ExternalCanisterId,
            ExternalCanisterResourceAction, Resource, ValidationMethodResourceTarget,
        },
        CanisterMethod,
    },
};
use candid::Principal;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestPolicyResourceIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(POLICY_RESOURCE_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding requests based on the requester in stable memory.
#[derive(Default, Debug)]
pub struct RequestPolicyResourceIndexRepository {}

#[cfg(test)]
impl RequestPolicyResourceIndexRepository {
    pub fn len(&self) -> usize {
        DB.with(|m| m.borrow().len() as usize)
    }

    pub fn is_empty(&self) -> bool {
        DB.with(|m| m.borrow().is_empty())
    }

    pub fn list(&self) -> Vec<RequestPolicyResourceIndex> {
        DB.with(|m| m.borrow().iter().map(|(k, _)| k.clone()).collect())
    }
}

impl IndexRepository<RequestPolicyResourceIndex, UUID> for RequestPolicyResourceIndexRepository {
    type FindByCriteria = RequestPolicyResourceIndexCriteria;

    fn exists(&self, index: &RequestPolicyResourceIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: RequestPolicyResourceIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestPolicyResourceIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = RequestPolicyResourceIndex {
                resource: criteria.resource.clone(),
                policy_id: [u8::MIN; 16],
            };
            let end_key = RequestPolicyResourceIndex {
                resource: criteria.resource,
                policy_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.policy_id)
                .collect::<HashSet<UUID>>()
        })
    }
}

impl RequestPolicyResourceIndexRepository {
    /// Finds all external canister policies related to the specified canister id.
    ///
    /// Includes:
    ///
    /// - `Change` related policies.
    /// - `Call` related policies.
    pub fn find_external_canister_policies(&self, canister_id: &Principal) -> Vec<UUID> {
        DB.with(|db| {
            let mut policies = Vec::new();
            // Find all change related policies for the specified canister id.
            policies.extend(
                db.borrow()
                    .range(
                        (RequestPolicyResourceIndex {
                            resource: Resource::ExternalCanister(
                                ExternalCanisterResourceAction::Change(
                                    ExternalCanisterId::Canister(*canister_id),
                                ),
                            ),
                            policy_id: [u8::MIN; 16],
                        })..(RequestPolicyResourceIndex {
                            resource: Resource::ExternalCanister(
                                ExternalCanisterResourceAction::Change(
                                    ExternalCanisterId::Canister(*canister_id),
                                ),
                            ),
                            policy_id: [u8::MAX; 16],
                        }),
                    )
                    .map(|(index, _)| index.policy_id)
                    .collect::<Vec<UUID>>(),
            );

            // Find all call related policies for the specified canister id.
            policies.extend(
                db.borrow()
                .range(
                    (RequestPolicyResourceIndex {
                        resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                            CallExternalCanisterResourceTarget {
                                execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                    CanisterMethod {
                                        canister_id: *canister_id,
                                        method_name: String::new(),
                                    },
                                ),
                                validation_method: ValidationMethodResourceTarget::No,
                            },
                        )),
                        policy_id: [u8::MIN; 16],
                    })..,
                )
                .take_while(|(index, _)| {
                    matches!(
                        &index.resource,
                        Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                            CallExternalCanisterResourceTarget {
                                execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                    CanisterMethod { canister_id: id, .. }
                                ),
                                ..
                            }
                        ))
                        if id == canister_id
                    )
                })
                .map(|(index, _)| index.policy_id)
                .collect::<Vec<UUID>>()
            );

            policies
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::resource::{Resource, UserResourceAction};

    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = RequestPolicyResourceIndexRepository::default();
        let index = RequestPolicyResourceIndex {
            policy_id: [0; 16],
            resource: Resource::User(UserResourceAction::Create),
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = RequestPolicyResourceIndexRepository::default();
        let index = RequestPolicyResourceIndex {
            resource: Resource::User(UserResourceAction::Create),
            policy_id: [0; 16],
        };

        repository.insert(index.clone());

        let criteria = RequestPolicyResourceIndexCriteria {
            resource: Resource::User(UserResourceAction::Create),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.policy_id));
    }

    #[test]
    fn test_find_external_canister_policies() {
        let repository = RequestPolicyResourceIndexRepository::default();
        for i in 0..10 {
            let index = RequestPolicyResourceIndex {
                resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                    ExternalCanisterId::Canister(Principal::from_slice(&[i % 2; 29])),
                )),
                policy_id: [i; 16],
            };

            repository.insert(index);
        }

        let policies = repository.find_external_canister_policies(&Principal::from_slice(&[1; 29]));

        assert_eq!(policies.len(), 5);
    }

    #[test]
    fn test_find_external_canister_policies_with_call() {
        let repository = RequestPolicyResourceIndexRepository::default();
        for i in 0..10 {
            let index = RequestPolicyResourceIndex {
                resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                    CallExternalCanisterResourceTarget {
                        execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                            CanisterMethod {
                                canister_id: Principal::from_slice(&[i % 2; 29]),
                                method_name: format!("method_{}", i),
                            },
                        ),
                        validation_method: ValidationMethodResourceTarget::No,
                    },
                )),
                policy_id: [i; 16],
            };

            repository.insert(index);
        }

        let policies = repository.find_external_canister_policies(&Principal::from_slice(&[1; 29]));

        assert_eq!(policies.len(), 5);
    }
}
