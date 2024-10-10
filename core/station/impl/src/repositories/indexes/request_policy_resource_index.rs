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

impl RequestPolicyResourceIndexRepository {
    /// Clears the repository by removing all the entries.
    pub fn clear(&self) {
        DB.with(|m| m.borrow_mut().clear_new());
    }
}

#[cfg(test)]
impl RequestPolicyResourceIndexRepository {
    pub fn len(&self) -> usize {
        DB.with(|m| m.borrow().len() as usize)
    }

    pub fn is_empty(&self) -> bool {
        DB.with(|m| m.borrow().is_empty())
    }

    pub fn list(&self) -> Vec<RequestPolicyResourceIndex> {
        DB.with(|m| m.borrow().iter().map(|(key, _)| key).collect())
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

#[derive(Clone, Default)]
pub struct ExternalCanisterPoliciesList {
    pub change: Vec<UUID>,
    pub calls: Vec<UUID>,
}

impl ExternalCanisterPoliciesList {
    pub fn new() -> Self {
        Self {
            change: Vec::new(),
            calls: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.change.len() + self.calls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.change.is_empty() && self.calls.is_empty()
    }

    pub fn all(&self) -> Vec<UUID> {
        let mut all = Vec::new();
        all.extend(self.change.iter());
        all.extend(self.calls.iter());
        all
    }
}

impl RequestPolicyResourceIndexRepository {
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
        DB.with(|db| {
            let mut policies = ExternalCanisterPoliciesList::new();
            // Find all change related policies for the specified canister id.
            policies.change.extend(
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
            policies.calls.extend(
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

    // Find all external canister call policies related to the specified canister id and execution method.
    pub fn find_external_canister_call_policies_by_execution_method(
        &self,
        canister_id: &Principal,
        execution_method: &str,
    ) -> Vec<UUID> {
        DB.with(|db| {
            db.borrow()
                .range(
                    (RequestPolicyResourceIndex {
                        resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                            CallExternalCanisterResourceTarget {
                                execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                    CanisterMethod {
                                        canister_id: *canister_id,
                                        method_name: execution_method.to_string(),
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
                                    CanisterMethod { canister_id: id, method_name: method }
                                ),
                                ..
                            }
                        ))
                        if id == canister_id && execution_method == method
                    )
                })
                .map(|(index, _)| index.policy_id)
                .collect::<Vec<UUID>>()
        })
    }

    pub fn find_external_canister_call_policies_by_execution_and_validation_method(
        &self,
        canister_id: &Principal,
        execution_method: &str,
        validation_method: &ValidationMethodResourceTarget,
    ) -> Vec<UUID> {
        let find_with_resource = Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
            CallExternalCanisterResourceTarget {
                execution_method: ExecutionMethodResourceTarget::ExecutionMethod(CanisterMethod {
                    canister_id: *canister_id,
                    method_name: execution_method.to_string(),
                }),
                validation_method: validation_method.clone(),
            },
        ));

        DB.with(|db| {
            db.borrow()
                .range(
                    (RequestPolicyResourceIndex {
                        resource: find_with_resource.clone(),
                        policy_id: [u8::MIN; 16],
                    })..(RequestPolicyResourceIndex {
                        resource: find_with_resource,
                        policy_id: [u8::MAX; 16],
                    }),
                )
                .map(|(index, _)| index.policy_id)
                .collect::<Vec<UUID>>()
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

    #[test]
    fn test_find_external_canister_call_policies_by_methods() {
        let repository = RequestPolicyResourceIndexRepository::default();
        let mut expected_method_ids = Vec::new();
        for i in 0..20 {
            let index = RequestPolicyResourceIndex {
                resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                    CallExternalCanisterResourceTarget {
                        execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                            CanisterMethod {
                                canister_id: Principal::management_canister(),
                                method_name: format!("method_{}", i),
                            },
                        ),
                        validation_method: if i % 2 == 0 {
                            ValidationMethodResourceTarget::No
                        } else {
                            ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                                canister_id: Principal::management_canister(),
                                method_name: format!("validation_method_{}", i),
                            })
                        },
                    },
                )),
                policy_id: [i; 16],
            };

            repository.insert(index);
            expected_method_ids.push([i; 16]);
        }

        expected_method_ids.reverse();

        // first try to find each individual execution method and validation method combination
        for i in 0..20 {
            let policies = repository.find_external_canister_call_policies_by_execution_method(
                &Principal::management_canister(),
                &format!("method_{}", i),
            );

            let expected_method_id = expected_method_ids.pop().unwrap();

            assert_eq!(policies.len(), 1);
            assert_eq!(policies[0], expected_method_id);

            let validation_method = if i % 2 == 0 {
                ValidationMethodResourceTarget::No
            } else {
                ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                    canister_id: Principal::management_canister(),
                    method_name: format!("validation_method_{}", i),
                })
            };

            // and then find by validation method
            let policies = repository
                .find_external_canister_call_policies_by_execution_and_validation_method(
                    &Principal::management_canister(),
                    &format!("method_{}", i),
                    &validation_method,
                );

            assert_eq!(policies.len(), 1);
            assert_eq!(policies[0], expected_method_id);
        }

        assert!(expected_method_ids.is_empty());

        // then insert another policy with the same execution method but different validation method
        let execution_method_policy_first_policy_id = [1; 16];
        let execution_method_policy_second_policy_id = [20; 16];
        let execution_method_policy_third_policy_id = [21; 16];
        repository.insert(RequestPolicyResourceIndex {
            resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                CallExternalCanisterResourceTarget {
                    execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                        CanisterMethod {
                            canister_id: Principal::management_canister(),
                            method_name: "method_1".to_string(),
                        },
                    ),
                    validation_method: ValidationMethodResourceTarget::ValidationMethod(
                        CanisterMethod {
                            canister_id: Principal::management_canister(),
                            method_name: "another_validation_method".to_string(),
                        },
                    ),
                },
            )),
            policy_id: execution_method_policy_second_policy_id,
        });

        repository.insert(RequestPolicyResourceIndex {
            resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                CallExternalCanisterResourceTarget {
                    execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                        CanisterMethod {
                            canister_id: Principal::management_canister(),
                            method_name: "method_1".to_string(),
                        },
                    ),
                    validation_method: ValidationMethodResourceTarget::No,
                },
            )),
            policy_id: execution_method_policy_third_policy_id,
        });

        // and find it by the new validation method
        let policies = repository
            .find_external_canister_call_policies_by_execution_and_validation_method(
                &Principal::management_canister(),
                "method_1",
                &ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                    canister_id: Principal::management_canister(),
                    method_name: "another_validation_method".to_string(),
                }),
            );

        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0], execution_method_policy_second_policy_id);

        // and find it by the other validation method
        let policies = repository
            .find_external_canister_call_policies_by_execution_and_validation_method(
                &Principal::management_canister(),
                "method_1",
                &ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                    canister_id: Principal::management_canister(),
                    method_name: "validation_method_1".to_string(),
                }),
            );

        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0], execution_method_policy_first_policy_id);

        // and find it by the no validation method
        let policies = repository
            .find_external_canister_call_policies_by_execution_and_validation_method(
                &Principal::management_canister(),
                "method_1",
                &ValidationMethodResourceTarget::No,
            );

        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0], execution_method_policy_third_policy_id);

        // and find both policies by the execution method
        let policies = repository.find_external_canister_call_policies_by_execution_method(
            &Principal::management_canister(),
            "method_1",
        );

        assert_eq!(policies.len(), 3);
        assert!(policies.contains(&execution_method_policy_first_policy_id));
        assert!(policies.contains(&execution_method_policy_second_policy_id));

        // and insert another policy for the same execution method and validation method combination
        let execution_method_policy_repeated_policy_id = [22; 16];
        repository.insert(RequestPolicyResourceIndex {
            resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                CallExternalCanisterResourceTarget {
                    execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                        CanisterMethod {
                            canister_id: Principal::management_canister(),
                            method_name: "method_1".to_string(),
                        },
                    ),
                    validation_method: ValidationMethodResourceTarget::ValidationMethod(
                        CanisterMethod {
                            canister_id: Principal::management_canister(),
                            method_name: "validation_method_1".to_string(),
                        },
                    ),
                },
            )),
            policy_id: execution_method_policy_repeated_policy_id,
        });

        // and find the two policies with the same execution method and validation method combination
        let policies = repository
            .find_external_canister_call_policies_by_execution_and_validation_method(
                &Principal::management_canister(),
                "method_1",
                &ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                    canister_id: Principal::management_canister(),
                    method_name: "validation_method_1".to_string(),
                }),
            );

        assert_eq!(policies.len(), 2);
        assert!(policies.contains(&execution_method_policy_first_policy_id));
        assert!(policies.contains(&execution_method_policy_repeated_policy_id));
    }
}
