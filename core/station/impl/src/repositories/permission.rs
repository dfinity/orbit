use crate::{
    core::{with_memory_manager, Memory, PERMISSION_MEMORY_ID},
    models::{
        permission::{Permission, PermissionKey},
        resource::{
            CallExternalCanisterResourceTarget, ExecutionMethodResourceTarget,
            ExternalCanisterResourceAction, Resource, ValidationMethodResourceTarget,
        },
        CanisterMethod,
    },
};
use candid::Principal;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::Repository;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<PermissionKey, Permission, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PERMISSION_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref PERMISSION_REPOSITORY: Arc<PermissionRepository> =
        Arc::new(PermissionRepository::default());
}

/// A repository that enables managing permissions in stable memory.
#[derive(Default, Debug)]
pub struct PermissionRepository {}

impl Repository<PermissionKey, Permission> for PermissionRepository {
    fn list(&self) -> Vec<Permission> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &PermissionKey) -> Option<Permission> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: PermissionKey, value: Permission) -> Option<Permission> {
        DB.with(|m| m.borrow_mut().insert(key, value.clone()))
    }

    fn remove(&self, key: &PermissionKey) -> Option<Permission> {
        DB.with(|m| m.borrow_mut().remove(key))
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl PermissionRepository {
    /// Finds all permissions that are available for a given external canister.
    pub fn find_external_canister_call_permissions(
        &self,
        canister_id: &Principal,
    ) -> Vec<Permission> {
        DB.with(|db| {
            db.borrow()
                .range(
                    Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                        CallExternalCanisterResourceTarget {
                            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                CanisterMethod {
                                    canister_id: *canister_id,
                                    method_name: String::new(),
                                },
                            ),
                            validation_method: ValidationMethodResourceTarget::No,
                        },
                    ))..,
                )
                .take_while(|(key, _)| {
                    matches!(
                        key,
                        Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                            CallExternalCanisterResourceTarget {
                                execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                    CanisterMethod {
                                        canister_id: id,
                                        ..
                                    }
                                ),
                                ..
                            }
                        ))

                        if id == canister_id
                    )
                })
                .map(|(_, permission)| permission)
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::permission::permission_test_utils::mock_permission;
    use crate::models::permission::Allow;
    use crate::models::resource::{
        CallExternalCanisterResourceTarget, ExecutionMethodResourceTarget,
        ExternalCanisterResourceAction, Resource, ResourceAction, ResourceId,
        ValidationMethodResourceTarget,
    };
    use crate::models::CanisterMethod;
    use candid::Principal;
    use orbit_essentials::model::ModelKey;

    #[test]
    fn test_crud() {
        let repository = &PERMISSION_REPOSITORY;
        let policy = mock_permission();

        assert!(repository.get(&policy.key()).is_none());

        repository.insert(policy.key(), policy.clone());

        assert!(repository.get(&policy.key()).is_some());
        assert!(repository.remove(&policy.key()).is_some());
        assert!(repository.get(&policy.key()).is_none());
    }

    #[test]
    fn test_find_by_resource_and_access() {
        for _ in 0..3 {
            let policy = mock_permission();
            PERMISSION_REPOSITORY.insert(policy.key(), policy.clone());
        }

        let mut policy = mock_permission();
        policy.resource = Resource::AddressBook(ResourceAction::Read(ResourceId::Any));

        PERMISSION_REPOSITORY.insert(policy.key(), policy.clone());

        let permission = PERMISSION_REPOSITORY
            .get(&Resource::AddressBook(ResourceAction::Read(
                ResourceId::Any,
            )))
            .unwrap();

        assert_eq!(permission, policy);
    }

    #[test]
    fn test_find_all_external_canister_call_permissions() {
        // adds permission to all canisters
        let permission = Permission {
            resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                CallExternalCanisterResourceTarget {
                    execution_method: ExecutionMethodResourceTarget::Any,
                    validation_method: ValidationMethodResourceTarget::No,
                },
            )),
            allow: Allow::authenticated(),
        };

        PERMISSION_REPOSITORY.insert(permission.key(), permission.clone());

        // adds permission to 3 different canisters and some of their methods
        for canister_nr in 0..3 {
            let canister_id = Principal::from_slice(&[canister_nr; 29]);

            for method_nr in 0..10 {
                let permission_with_no_validation = Permission {
                    resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                        CallExternalCanisterResourceTarget {
                            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                CanisterMethod {
                                    canister_id,
                                    method_name: format!("method_{}", method_nr),
                                },
                            ),
                            validation_method: ValidationMethodResourceTarget::No,
                        },
                    )),
                    allow: Allow::authenticated(),
                };

                let permission_with_validation = Permission {
                    resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                        CallExternalCanisterResourceTarget {
                            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                CanisterMethod {
                                    canister_id,
                                    method_name: format!("method_{}", method_nr),
                                },
                            ),
                            validation_method: ValidationMethodResourceTarget::ValidationMethod(
                                CanisterMethod {
                                    canister_id,
                                    method_name: format!("method_{}", method_nr),
                                },
                            ),
                        },
                    )),
                    allow: Allow::authenticated(),
                };

                PERMISSION_REPOSITORY.insert(
                    permission_with_no_validation.key(),
                    permission_with_no_validation.clone(),
                );
                PERMISSION_REPOSITORY.insert(
                    permission_with_validation.key(),
                    permission_with_validation.clone(),
                );
            }
        }

        let permissions = PERMISSION_REPOSITORY
            .find_external_canister_call_permissions(&Principal::from_slice(&[1; 29]));

        assert_eq!(permissions.len(), 20);

        for permission in permissions {
            assert!(matches!(
                permission.resource,
                Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                    CallExternalCanisterResourceTarget {
                        execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                            CanisterMethod {
                                canister_id,
                                ..
                            }
                        ),
                        ..
                    }
                ))

                if canister_id == Principal::from_slice(&[1; 29])
            ));
        }
    }
}
