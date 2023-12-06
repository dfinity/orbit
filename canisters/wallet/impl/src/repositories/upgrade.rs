use crate::{
    core::{with_memory_manager, Memory, UPGRADES_MEMORY_ID},
    models::upgrade::Upgrade,
};
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::cell::RefCell;

thread_local! {
  static DB: RefCell<StableBTreeMap<UUID, Upgrade, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(UPGRADES_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref UPGRADE_REPOSITORY: UpgradeRepository = UpgradeRepository;
}

/// A repository that enables managing upgrades in stable memory.
#[derive(Debug)]
pub struct UpgradeRepository;

impl Repository<UUID, Upgrade> for UpgradeRepository {
    fn list(&self) -> Vec<Upgrade> {
        DB.with(|m| m.borrow().iter().map(|(k, v)| v).collect())
    }

    fn get(&self, key: &UUID) -> Option<Upgrade> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: UUID, value: Upgrade) -> Option<Upgrade> {
        DB.with(|m| m.borrow_mut().insert(key, value.clone()))
    }

    fn remove(&self, key: &UUID) -> Option<Upgrade> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}
