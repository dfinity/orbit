use crate::{
    core::{with_memory_manager, Memory, ACCOUNT_IDENTITY_INDEX_MEMORY_ID},
    models::{
        indexes::account_identity_index::{AccountIdentityIndex, AccountIdentityIndexCriteria},
        Account,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Transfer repository.
  static DB: RefCell<StableBTreeMap<AccountIdentityIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_IDENTITY_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables managing transfer in stable memory.
#[derive(Default, Debug)]
pub struct AccountIdentityIndexRepository {}

impl IndexRepository<AccountIdentityIndex, Account> for AccountIdentityIndexRepository {
    type FindByCriteria = AccountIdentityIndexCriteria;

    fn exists(&self, index: &AccountIdentityIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: AccountIdentityIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &AccountIdentityIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> Vec<Account> {
        DB.with(|db| {
            let start_key = AccountIdentityIndex {
                identity_id: criteria.identity_id,
                account_id: [u8::MIN; 16],
            };
            let end_key = AccountIdentityIndex {
                identity_id: criteria.identity_id,
                account_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .filter(|(index, _)| {
                    let account = index.to_account();
                    let mut criteria_matches_role = true;

                    if let Some(role) = &criteria.role {
                        criteria_matches_role = account.access_roles.contains(role);
                    }

                    criteria_matches_role
                })
                .map(|(index, _)| index.to_account())
                .collect::<Vec<Account>>()
        })
    }
}
