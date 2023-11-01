use crate::{
    core::{canister_config, generate_uuid_v4, CallContext, WithCallContext},
    errors::AccountError,
    mappers::AccountMapper,
    models::{Account, AccountBank, AccountId},
    repositories::AccountRepository,
    transport::{ManageAccountInput, RegisterAccountInput},
};
use candid::Principal;
use ic_canister_core::repository::Repository;
use ic_canister_core::{
    api::{ApiError, ServiceResult},
    model::ModelValidator,
};
use uuid::Uuid;

#[derive(Default)]
pub struct AccountService {
    call_context: CallContext,
    account_repository: AccountRepository,
}

impl WithCallContext for AccountService {
    fn with_call_context(call_context: CallContext) -> Self {
        Self {
            call_context: call_context.clone(),
            ..Default::default()
        }
    }
}

impl AccountService {
    /// Returns the account associated with the given account id.
    pub fn get_account(&self, account_id: &AccountId) -> ServiceResult<Account> {
        let account = self
            .account_repository
            .get(&Account::key(account_id))
            .ok_or(AccountError::NotFound {
                account: Uuid::from_bytes(account_id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?;

        self.assert_account_access(&account)?;

        Ok(account)
    }

    /// Returns the account associated with the given user identity.
    pub fn get_account_by_identity(&self, identity: &Principal) -> ServiceResult<Account> {
        let account = self
            .account_repository
            .find_account_by_identity(identity)
            .ok_or(AccountError::AssociatedAccountIdentityNotFound {
                identity: identity.to_text(),
            })?;

        self.assert_account_access(&account)?;

        Ok(account)
    }

    pub fn get_main_bank(&self) -> ServiceResult<Option<AccountBank>> {
        let account = self.get_account_by_identity(&self.call_context.caller())?;

        match account.main_bank {
            Some(main_bank) => {
                let main_bank = account
                    .banks
                    .into_iter()
                    .find(|bank| bank.canister_id == main_bank)
                    .ok_or(AccountError::MainBankNotFound)?;

                Ok(Some(main_bank))
            }
            None => Ok(None),
        }
    }

    /// Associates the caller identity with the given account if it exists.
    pub async fn associate_identity_with_account(
        &self,
        account_id: AccountId,
    ) -> ServiceResult<Account, ApiError> {
        let caller = self.call_context.caller();
        self.assert_identity_is_unregistered(&caller)?;
        let mut account = self.get_account(&account_id)?;

        let unconfirmed_identity = account
            .unconfirmed_identities
            .clone()
            .into_iter()
            .find(|identity| identity.identity == caller)
            .ok_or(AccountError::Forbidden {
                account: Uuid::from_bytes(account_id).hyphenated().to_string(),
            })?;

        let unconfirmed_identities = account
            .unconfirmed_identities
            .iter()
            .filter(|identity| identity.identity != caller)
            .map(|identity| identity.to_owned())
            .collect();

        account.unconfirmed_identities = unconfirmed_identities;
        account.identities.push(unconfirmed_identity);

        account.validate()?;
        self.account_repository
            .insert(account.to_key(), account.clone());

        Ok(account)
    }

    /// Registers a new account for the caller identity.
    pub async fn register_account(
        &self,
        input: RegisterAccountInput,
    ) -> ServiceResult<Account, ApiError> {
        self.assert_identity_is_unregistered(&self.call_context.caller())?;

        let account_id = generate_uuid_v4().await;
        let account = AccountMapper::from_register_input(
            input.clone(),
            *account_id.as_bytes(),
            self.call_context.caller(),
            canister_config().shared_bank_canister,
        );

        account.validate()?;
        self.account_repository
            .insert(account.to_key(), account.clone());

        Ok(account)
    }

    pub async fn remove_account(&self, account_id: &AccountId) -> ServiceResult<Account> {
        let account = self.get_account(account_id)?;

        self.assert_account_access(&account)?;

        self.account_repository.remove(&account.to_key());

        Ok(account)
    }

    pub async fn manage_account(&self, input: ManageAccountInput) -> ServiceResult<Account> {
        let mut account = self.get_account_by_identity(&self.call_context.caller())?;

        account.update_with(input, &self.call_context.caller())?;
        account.validate()?;

        self.account_repository
            .insert(account.to_key(), account.clone());

        Ok(account)
    }

    /// Checks if the caller has access to the given account.
    ///
    /// Admins have access to all accounts.
    fn assert_account_access(&self, account: &Account) -> ServiceResult<()> {
        let is_account_owner = account
            .identities
            .iter()
            .any(|identity| identity.identity == self.call_context.caller())
            || account
                .unconfirmed_identities
                .iter()
                .any(|identity| identity.identity == self.call_context.caller());
        if !is_account_owner && !self.call_context.is_admin() {
            Err(AccountError::Forbidden {
                account: Uuid::from_bytes(account.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }

    /// Validates that the given identity has no associated account.
    ///
    /// If the identity has an associated account, an error is returned.
    pub fn assert_identity_is_unregistered(&self, identity: &Principal) -> ServiceResult<()> {
        let maybe_account = self.account_repository.find_account_by_identity(identity);

        if let Some(account) = maybe_account {
            Err(AccountError::IdentityAlreadyHasAccount {
                account: Uuid::from_bytes(account.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::AccountIdentity, transport::RegisterAccountBankInput};

    #[test]
    fn get_account_returns_not_found_err() {
        let service = AccountService::default();
        let account_id = *Uuid::new_v4().as_bytes();

        let result = service.get_account(&account_id);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(AccountError::NotFound {
                account: Uuid::from_bytes(account_id).hyphenated().to_string()
            })
        );
    }

    #[test]
    fn success_fetch_existing_account() {
        let service = AccountService::default();
        let account_id = *Uuid::new_v4().as_bytes();
        let account = Account {
            id: account_id,
            identities: vec![AccountIdentity {
                identity: Principal::anonymous(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        service
            .account_repository
            .insert(account.to_key(), account.clone());

        let result = service.get_account(&account_id);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), account);
    }

    #[test]
    fn success_fetch_existing_account_by_identity() {
        let service = AccountService::default();
        let account_id = *Uuid::new_v4().as_bytes();
        let account = Account {
            id: account_id,
            identities: vec![AccountIdentity {
                identity: Principal::anonymous(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        service
            .account_repository
            .insert(account.to_key(), account.clone());

        let result = service.get_account_by_identity(&Principal::anonymous());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), account);
    }

    #[tokio::test]
    async fn success_register_new_account() {
        crate::core::test_utils::init_canister_config();

        let service = AccountService::default();
        let input = RegisterAccountInput {
            name: Some("Account".to_string()),
            bank: RegisterAccountBankInput::PrivateBank {
                id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                use_shared_bank: None,
            },
        };

        let result = service.register_account(input.clone()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, input.name);
    }

    #[tokio::test]
    async fn failed_registering_new_account_with_same_identity() {
        crate::core::test_utils::init_canister_config();

        let service = AccountService::default();
        let input = RegisterAccountInput {
            name: Some("Account".to_string()),
            bank: RegisterAccountBankInput::PrivateBank {
                id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                use_shared_bank: None,
            },
        };
        let duplicated_account_input = RegisterAccountInput {
            name: Some("Account 2".to_string()),
            bank: RegisterAccountBankInput::PrivateBank {
                id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                use_shared_bank: None,
            },
        };

        let result = service.register_account(input.clone()).await;
        let duplicated_account_result = service
            .register_account(duplicated_account_input.clone())
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, input.name);
        assert!(duplicated_account_result.is_err());
    }

    #[tokio::test]
    async fn correctly_associates_identity_with_account() {
        crate::core::test_utils::init_canister_config();
        let service = AccountService {
            call_context: CallContext::new(
                Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            ),
            ..Default::default()
        };

        let account_id = *Uuid::new_v4().as_bytes();
        let account = Account {
            id: account_id,
            identities: vec![],
            unconfirmed_identities: vec![AccountIdentity {
                identity: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                name: None,
            }],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        service
            .account_repository
            .insert(account.to_key(), account.clone());

        let result = service.associate_identity_with_account(account_id).await;

        assert!(result.is_ok());
        let account = result.unwrap();

        assert_eq!(account.identities.len(), 1);
        assert_eq!(account.unconfirmed_identities.len(), 0);
    }

    #[tokio::test]
    async fn can_remove_account() {
        crate::core::test_utils::init_canister_config();
        let service = AccountService {
            call_context: CallContext::new(
                Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            ),
            ..Default::default()
        };

        let account_id = *Uuid::new_v4().as_bytes();
        let account = Account {
            id: account_id,
            identities: vec![AccountIdentity {
                identity: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        service
            .account_repository
            .insert(account.to_key(), account.clone());

        let result = service.remove_account(&account_id).await;

        assert!(result.is_ok());
        assert!(service
            .account_repository
            .get(&Account::key(&account_id))
            .is_none());
    }
}
