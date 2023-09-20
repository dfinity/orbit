use crate::{
    core::{
        max_principal_id, min_principal_id, with_memory_manager, Memory, Repository,
        ACCOUNT_BANK_MEMORY_ID, UUID,
    },
    entities::{AccountBank, AccountBankKey},
    errors::AccountBankRepositoryError,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

/// The storage schema for the AccountBank repository.
pub type AccountBankStorageSchema =
    StableBTreeMap<AccountBankKey, AccountBank, VirtualMemory<Memory>>;

thread_local! {
  /// The memory reference to the AccountBank repository.
  static DB: RefCell<AccountBankStorageSchema> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_BANK_MEMORY_ID))
    )
  })
}

/// A repository that enables managing account banks in stable memory.
pub struct AccountBankRepository {}

impl Default for AccountBankRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl Repository<AccountBankKey, AccountBank> for AccountBankRepository {
    fn get(&self, key: &AccountBankKey) -> Option<AccountBank> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccountBankKey, value: AccountBank) -> Option<AccountBank> {
        DB.with(|m| m.borrow_mut().insert(key, value))
    }

    fn remove(&self, key: &AccountBankKey) -> Option<AccountBank> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

/// Enables the initialization of the AccountBankRepository repository.
impl AccountBankRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find_by_account_id(
        &self,
        account_id: &UUID,
    ) -> Result<Vec<AccountBank>, AccountBankRepositoryError> {
        DB.with(|m| {
            let start_key = AccountBank::key(&min_principal_id(), account_id);
            let end_key = AccountBank::key(&&max_principal_id(), account_id);

            let banks = m
                .borrow()
                .range(start_key..=end_key)
                .map(|(_, account_bank)| account_bank)
                .collect::<Vec<AccountBank>>();

            Ok(banks)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn init_account_bank_repository() {
        let repository = AccountBankRepository::default();
        let key = AccountBankKey::default();
        let account_bank = repository.get(&key);
        assert!(account_bank.is_none());
    }

    #[test]
    fn insert_account_bank() {
        let repository = AccountBankRepository::default();
        let key = AccountBankKey::default();
        let account_bank = AccountBank::default();
        let result = repository.insert(key.clone(), account_bank.clone());
        let inserted_record = repository.get(&key);

        assert!(result.is_none());
        assert!(inserted_record.is_some());
        assert_eq!(account_bank, inserted_record.unwrap());
    }

    #[test]
    fn insert_bank_with_same_key_overrides_and_returns_previous() {
        let repository = AccountBankRepository::default();
        let key = AccountBankKey::default();
        let previous_record = AccountBank::default();

        repository.insert(key.clone(), previous_record.clone());

        let mut new_record = AccountBank::default();
        new_record.name = Some(String::from("test"));
        let result = repository.insert(key.clone(), new_record.clone());

        assert!(result.is_some());
        assert_eq!(previous_record, result.unwrap());
    }

    #[test]
    fn removes_inserted_account_bank() {
        let repository = AccountBankRepository::default();
        let key = AccountBankKey::default();
        let account_bank = AccountBank::default();

        repository.insert(key.clone(), account_bank.clone());

        let before_delete_result = repository.get(&key);
        let removed_value = repository.remove(&key);
        let after_delete_result = repository.get(&key);

        assert!(before_delete_result.is_some());
        assert!(after_delete_result.is_none());
        assert_eq!(removed_value.unwrap(), account_bank);
    }

    #[test]
    fn finds_all_banks_by_account_id() {
        let repository = AccountBankRepository::default();
        let account_id: UUID = [2u8; 16];
        let different_account_id: UUID = [3u8; 16];
        let key = AccountBank::key(&Principal::from_slice(&[0u8; 29]), &account_id);
        let second_key = AccountBank::key(&Principal::from_slice(&[1u8; 29]), &account_id);
        let different_key =
            AccountBank::key(&Principal::from_slice(&[2u8; 29]), &different_account_id);
        let mut record = AccountBank::default();
        let mut second_record = AccountBank::default();
        let mut different_record = AccountBank::default();

        record.account_id = account_id.clone();
        second_record.account_id = account_id.clone();
        different_record.account_id = different_account_id.clone();

        repository.insert(key.clone(), record.clone());
        repository.insert(second_key.clone(), second_record.clone());
        repository.insert(different_key.clone(), different_record.clone());

        let banks = repository.find_by_account_id(&account_id);

        assert!(banks.is_ok());
        assert_eq!(banks.unwrap().len(), 2);
    }
}
