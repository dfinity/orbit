use crate::{
    core::{with_memory_manager, Memory, Repository, ACCOUNT_IDENTITY_MEMORY_ID},
    entities::{AccountIdentity, AccountIdentityKey},
    errors::AccountIdentityRepositoryError,
};
use candid::Principal;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

/// The storage schema for the AccountIdentity repository.
pub type AccountIdentityStorageSchema =
    StableBTreeMap<AccountIdentityKey, AccountIdentity, VirtualMemory<Memory>>;

thread_local! {
  /// The memory reference to the AccountIdentity repository.
  static DB: RefCell<AccountIdentityStorageSchema> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_IDENTITY_MEMORY_ID))
    )
  })
}

/// A repository that enables managing account identities in stable memory.
#[derive(Default)]
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

/// Enables the initialization of the AccountIdentityRepository repository.
impl AccountIdentityRepository {
    pub fn find_by_identity_id(
        &self,
        identity: &Principal,
    ) -> Result<Option<AccountIdentity>, AccountIdentityRepositoryError> {
        DB.with(|m| {
            let start_key = AccountIdentity::key(identity, &[std::u8::MIN; 16]);
            let end_key = AccountIdentity::key(identity, &[std::u8::MAX; 16]);

            let results = m
                .borrow()
                .range(start_key..=end_key)
                .map(|(_, account_identity)| account_identity)
                .collect::<Vec<AccountIdentity>>();

            match results.len() {
                0 => Ok(None),
                1 => Ok(Some(results.first().unwrap().clone())),
                _ => Err(AccountIdentityRepositoryError::NotAllowedMultipleAccountsWithIdentity),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_account_identity_repository() {
        let repository = AccountIdentityRepository::default();
        let key = AccountIdentityKey::default();
        let identity = repository.get(&key);
        assert!(identity.is_none());
    }

    #[test]
    fn insert_account_identity() {
        let repository = AccountIdentityRepository::default();
        let key = AccountIdentityKey::default();
        let identity = AccountIdentity::default();
        let result = repository.insert(key.clone(), identity.clone());
        let inserted_record = repository.get(&key);

        assert!(result.is_none());
        assert!(inserted_record.is_some());
        assert_eq!(identity, inserted_record.unwrap());
    }

    #[test]
    fn insert_identity_with_same_key_overrides_and_returns_previous() {
        let repository = AccountIdentityRepository::default();
        let key = AccountIdentityKey::default();
        let previous_identity = AccountIdentity::default();

        repository.insert(key.clone(), previous_identity.clone());

        let new_identity = AccountIdentity {
            name: Some(String::from("test")),
            ..Default::default()
        };
        let result = repository.insert(key.clone(), new_identity.clone());

        assert!(result.is_some());
        assert_eq!(previous_identity, result.unwrap());
    }

    #[test]
    fn removes_inserted_identity() {
        let repository = AccountIdentityRepository::default();
        let key = AccountIdentityKey::default();
        let identity = AccountIdentity::default();

        repository.insert(key.clone(), identity.clone());

        let before_delete_result = repository.get(&key);
        let removed_value = repository.remove(&key);
        let after_delete_result = repository.get(&key);

        assert!(before_delete_result.is_some());
        assert!(after_delete_result.is_none());
        assert_eq!(removed_value.unwrap(), identity);
    }

    #[test]
    fn not_allowed_multiple_accounts_with_same_identity() {
        let repository = AccountIdentityRepository::default();
        let identity = Principal::anonymous();
        let account_id: [u8; 16] = [0; 16];
        let second_account_id: [u8; 16] = [1; 16];
        let key = AccountIdentity::key(&identity, &account_id);
        let second_key = AccountIdentity::key(&identity, &second_account_id);
        let mut record = AccountIdentity::default();
        let mut second_record = AccountIdentity::default();
        record.identity = identity;
        second_record.identity = identity;
        second_record.account_id = second_account_id;

        repository.insert(key.clone(), record);
        repository.insert(second_key.clone(), second_record);

        let account_identity = repository.find_by_identity_id(&identity);

        assert!(account_identity.is_err());
        assert_eq!(
            account_identity.err(),
            Some(AccountIdentityRepositoryError::NotAllowedMultipleAccountsWithIdentity)
        );
    }

    #[test]
    fn finds_an_account_identity_by_identity_principal_id() {
        let repository = AccountIdentityRepository::default();
        let identity = Principal::from_slice(&[0; 16]);
        let account_id: [u8; 16] = [0; 16];
        let second_identity = Principal::from_slice(&[1; 16]);
        let second_account_id: [u8; 16] = [1; 16];
        let key = AccountIdentity::key(&identity, &account_id);
        let second_key = AccountIdentity::key(&second_identity, &second_account_id);
        let mut record = AccountIdentity::default();
        let mut second_record = AccountIdentity::default();
        record.identity = identity;
        second_record.identity = second_identity;
        second_record.account_id = second_account_id;

        repository.insert(key.clone(), record.clone());
        repository.insert(second_key.clone(), second_record.clone());

        let account_identity = repository.find_by_identity_id(&identity);
        assert!(account_identity.is_ok());
        assert_eq!(account_identity.unwrap().unwrap(), record);
    }
}
