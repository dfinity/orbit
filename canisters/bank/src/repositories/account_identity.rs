use crate::{
    core::{with_memory_manager, Memory, ACCOUNT_IDENTITY_MEMORY_ID},
    errors::RepositoryError,
    models::{AccountIdentity, AccountIdentityKey},
};
use candid::Principal;
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  /// The memory reference to the Account identity repository.
  static DB: RefCell<StableBTreeMap<AccountIdentityKey, AccountIdentity, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_IDENTITY_MEMORY_ID))
    )
  })
}

/// A repository that enables managing accounts and identities in stable memory.
#[derive(Default, Debug)]
pub struct AccountIdentityRepository {}

impl Repository<AccountIdentityKey, AccountIdentity> for AccountIdentityRepository {
    fn get(&self, key: &AccountIdentityKey) -> Option<AccountIdentity> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccountIdentityKey, value: AccountIdentity) -> Option<AccountIdentity> {
        DB.with(|m| m.borrow_mut().insert(key, value))
    }

    fn remove(&self, key: &AccountIdentityKey) -> Option<AccountIdentity> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

impl AccountIdentityRepository {
    pub fn find_by_identity_id(
        &self,
        identity: &Principal,
    ) -> Result<Option<AccountIdentity>, RepositoryError> {
        DB.with(|db| {
            let start_key = AccountIdentity::key(identity, &[std::u8::MIN; 16]);
            let end_key = AccountIdentity::key(identity, &[std::u8::MAX; 16]);

            let results = db
                .borrow()
                .range(start_key..=end_key)
                .map(|(_, account_identity)| account_identity)
                .collect::<Vec<AccountIdentity>>();

            match results.len() {
                0 => Ok(None),
                1 => Ok(Some(results.first().unwrap().clone())),
                _ => Err(RepositoryError::NotAllowedMultipleAssociation {
                    entity: "account_identity".to_string(),
                    entity_id: identity.to_text(),
                }),
            }
        })
    }
}
