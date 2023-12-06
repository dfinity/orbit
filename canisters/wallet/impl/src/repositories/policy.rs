use crate::{
    core::{with_memory_manager, Memory, PROPOSAL_POLICIES_MEMORY_ID},
    models::ProposalPolicy,
};
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the proposal policies repository.
  static DB: RefCell<StableBTreeMap<UUID, ProposalPolicy, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_POLICIES_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref PROPOSAL_POLICY_REPOSITORY: ProposalPolicyRepository = ProposalPolicyRepository;
}

/// A repository that enables managing proposal policies in stable memory.
#[derive(Default, Debug)]
pub struct ProposalPolicyRepository;

impl Repository<UUID, ProposalPolicy> for ProposalPolicyRepository {
    fn list(&self) -> Vec<ProposalPolicy> {
        DB.with(|m| m.borrow().iter().map(|(k, v)| v).collect())
    }

    fn get(&self, key: &UUID) -> Option<ProposalPolicy> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: UUID, value: ProposalPolicy) -> Option<ProposalPolicy> {
        DB.with(|m| m.borrow_mut().insert(key, value.clone()))
    }

    fn remove(&self, key: &UUID) -> Option<ProposalPolicy> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}
