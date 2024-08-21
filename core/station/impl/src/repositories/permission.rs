use crate::{
    core::{cache::Cache, ic_cdk::api::print, with_memory_manager, Memory, PERMISSION_MEMORY_ID},
    models::{
        permission::{Allow, Permission, PermissionKey},
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
use orbit_essentials::repository::{IndexedRepository, Repository, StableDb};
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<PermissionKey, Permission, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PERMISSION_MEMORY_ID))
    )
  });

  static CACHE: RefCell<Cache<Resource, Allow>> = RefCell::new(Cache::new(PermissionRepository::MAX_CACHE_SIZE));
}

lazy_static! {
    pub static ref PERMISSION_REPOSITORY: Arc<PermissionRepository> =
        Arc::new(PermissionRepository::default());
}

/// A repository that enables managing permissions in stable memory.
#[derive(Default, Debug)]
pub struct PermissionRepository {}

impl StableDb<PermissionKey, Permission, VirtualMemory<Memory>> for PermissionRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<PermissionKey, Permission, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|db| f(&mut db.borrow_mut()))
    }
}

impl IndexedRepository<PermissionKey, Permission, VirtualMemory<Memory>> for PermissionRepository {
    fn remove_entry_indexes(&self, _: &Permission) {
        // no indexes to remove
    }

    fn add_entry_indexes(&self, _: &Permission) {
        // no indexes to add
    }
}

impl Repository<PermissionKey, Permission, VirtualMemory<Memory>> for PermissionRepository {
    fn list(&self) -> Vec<Permission> {
        if self.use_only_cache() {
            return CACHE.with(|cache| {
                cache
                    .borrow()
                    .iter()
                    .map(|(resource, allow)| Permission {
                        resource: resource.clone(),
                        allow: allow.clone(),
                    })
                    .collect()
            });
        }

        Self::with_db(|db| db.iter().map(|(_, v)| v.clone()).collect())
    }

    fn get(&self, key: &PermissionKey) -> Option<Permission> {
        let maybe_cache_hit = CACHE.with(|cache| {
            cache.borrow().get(key).map(|allow| Permission {
                resource: key.clone(),
                allow: allow.clone(),
            })
        });

        if self.use_only_cache() {
            return maybe_cache_hit;
        }

        maybe_cache_hit.or_else(|| Self::with_db(|db| db.get(key).clone()))
    }

    fn insert(&self, key: PermissionKey, value: Permission) -> Option<Permission> {
        // Update the cache with the new value.
        CACHE.with(|cache| {
            cache
                .borrow_mut()
                .insert(value.resource.clone(), value.allow.clone())
        });

        Self::with_db(|db| db.insert(key.clone(), value.clone()))
    }

    fn remove(&self, key: &PermissionKey) -> Option<Permission> {
        // Remove the value from the cache.
        CACHE.with(|cache| cache.borrow_mut().remove(key));

        Self::with_db(|db| db.remove(key))
    }
}

impl PermissionRepository {
    /// Currently the cache uses around 0.35KiB per entry (Resource, Allow),
    /// so the max cache size is around 108MiB.
    ///
    /// Moreover, it takes approximately 70million instructions to load each entry
    /// to the cache, which means that rebuilding the cache from the repository
    /// would take around 20B instructions.
    ///
    /// Since init/upgrade hooks can use up to 200B instructions, rebuilding
    /// a cache in the worst case would take up to 10% of the available instructions.
    pub const MAX_CACHE_SIZE: usize = 300_000;

    /// Checks if every permission in the repository is in the cache.
    fn use_only_cache(&self) -> bool {
        self.len() <= Self::MAX_CACHE_SIZE
    }

    /// Builds the cache from the stable memory repository.
    ///
    /// This method should only be called during init or upgrade hooks to ensure that the cache is
    /// up-to-date with the repository and that we have enough instructions to rebuild the cache.
    pub fn build_cache(&self) {
        if self.len() > Self::MAX_CACHE_SIZE {
            print(format!(
                "Only the first {} permissions will be added to the cache, the reposity has {} permissions.",
                Self::MAX_CACHE_SIZE,
                PERMISSION_REPOSITORY.len(),
            ));
        }

        CACHE.with(|cache| {
            cache.borrow_mut().clear();

            DB.with(|db| {
                for (_, permission) in db.borrow().iter().take(Self::MAX_CACHE_SIZE) {
                    cache
                        .borrow_mut()
                        .insert(permission.resource.clone(), permission.allow.clone());
                }
            });
        });
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

    #[test]
    fn insert_updates_cache() {
        let repository = &PERMISSION_REPOSITORY;
        let policy = mock_permission();

        assert!(repository.get(&policy.key()).is_none());

        repository.insert(policy.key(), policy.clone());

        assert!(repository.get(&policy.key()).is_some());
        assert!(CACHE.with(|cache| cache.borrow().contains_key(&policy.resource)));
    }

    #[test]
    fn remove_updates_cache() {
        let repository = &PERMISSION_REPOSITORY;
        let policy = mock_permission();

        assert!(repository.get(&policy.key()).is_none());

        repository.insert(policy.key(), policy.clone());

        assert!(repository.get(&policy.key()).is_some());
        assert!(CACHE.with(|cache| cache.borrow().contains_key(&policy.resource)));

        repository.remove(&policy.key());

        assert!(repository.get(&policy.key()).is_none());
        assert!(!CACHE.with(|cache| cache.borrow().contains_key(&policy.resource)));
    }

    #[test]
    fn get_uses_cache() {
        let repository = &PERMISSION_REPOSITORY;
        let policy = mock_permission();

        assert!(repository.get(&policy.key()).is_none());

        repository.insert(policy.key(), policy.clone());

        // Clear the stable repository to ensure that the cache is used.
        DB.with(|db| db.borrow_mut().remove(&policy.key()));

        assert!(repository.get(&policy.key()).is_some());
    }

    #[test]
    fn list_uses_cache_when_all_entries_are_loaded() {
        let repository = &PERMISSION_REPOSITORY;
        let permissions = (0..10).map(|_| mock_permission()).collect::<Vec<_>>();

        for permission in permissions.iter() {
            repository.insert(permission.key(), permission.clone());
        }

        // Clear the stable repository to ensure that the cache is used.
        DB.with(|db| {
            let mut db = db.borrow_mut();

            for permission in permissions.iter() {
                db.remove(&permission.key());
            }

            assert_eq!(db.len(), 0);
        });

        // even though the stable repository is empty, the cache should still be in-place which
        // would allow the list method to return the permissions.
        assert_eq!(repository.list().len(), permissions.len());
    }
}
