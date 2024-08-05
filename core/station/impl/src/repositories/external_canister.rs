use super::indexes::external_canister_index::ExternalCanisterIndexRepository;
use crate::{
    core::{utils::format_unique_string, with_memory_manager, Memory, EXTERNAL_CANISTER_MEMORY_ID},
    models::{
        indexes::external_canister_index::{
            ExternalCanisterIndexCriteria, ExternalCanisterIndexKind,
        },
        ExternalCanister, ExternalCanisterId, ExternalCanisterKey,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::IndexRepository;
use orbit_essentials::repository::{RefreshIndexMode, Repository};
use std::{cell::RefCell, sync::Arc};

thread_local! {
  /// The memory reference to the external canister repository.
  static DB: RefCell<StableBTreeMap<ExternalCanisterKey, ExternalCanister, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(EXTERNAL_CANISTER_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref EXTERNAL_CANISTER_REPOSITORY: Arc<ExternalCanisterRepository> =
        Arc::new(ExternalCanisterRepository::default());
}

/// A repository that enables managing external canisters in stable memory.
#[derive(Debug, Default)]
pub struct ExternalCanisterRepository {
    indexes: ExternalCanisterIndexRepository,
}

impl Repository<ExternalCanisterKey, ExternalCanister> for ExternalCanisterRepository {
    fn list(&self) -> Vec<ExternalCanister> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &ExternalCanisterKey) -> Option<ExternalCanister> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(
        &self,
        key: ExternalCanisterKey,
        value: ExternalCanister,
    ) -> Option<ExternalCanister> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.indexes
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev: ExternalCanister| prev.indexes()),
                    current: value.indexes(),
                });

            prev
        })
    }

    fn remove(&self, key: &ExternalCanisterKey) -> Option<ExternalCanister> {
        DB.with(|m| m.borrow_mut().remove(key))
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl ExternalCanisterRepository {
    /// Returns an external canister by its name if it exists.
    pub fn find_by_name(&self, name: &str) -> Option<ExternalCanisterId> {
        let name = format_unique_string(name);
        let found = self
            .indexes
            .find_by_criteria(ExternalCanisterIndexCriteria {
                from: ExternalCanisterIndexKind::Name(name.to_string()),
                to: ExternalCanisterIndexKind::Name(name.to_string()),
            });

        found.into_iter().next()
    }
}
