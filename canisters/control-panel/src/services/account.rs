use super::{AccountBankService, AccountIdentityService};
use crate::{
    core::{canister_config, generate_uuid_v4, ApiError, Repository, ServiceResult},
    entities::{Account, AccountBank, AccountIdentity},
    errors::AccountManagementError,
    mappers::{AccountBankMapper, AccountIdentityMapper, AccountMapper},
    repositories::{AccountBankRepository, AccountIdentityRepository, AccountRepository},
    transport::{
        AccountBankDTO, AccountDetailsDTO, AssociateIdentityWithAccountInput, ManageAccountInput,
        RegisterAccountInput,
    },
};
use candid::Principal;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Default)]
pub struct AccountService {
    account_repository: AccountRepository,
    account_identity_repository: AccountIdentityRepository,
    account_mapper: AccountMapper,
    account_identity_mapper: AccountIdentityMapper,
    account_bank_repository: AccountBankRepository,
    account_bank_mapper: AccountBankMapper,
    account_identity_service: AccountIdentityService,
    account_bank_service: AccountBankService,
}

impl AccountService {
    fn assert_identity_has_no_account(&self, identity: &Principal) -> Result<(), ApiError> {
        let account_identity = self
            .account_identity_repository
            .find_by_identity_id(identity)?;

        if let Some(entry) = account_identity {
            let formatted_account_id = Uuid::from_bytes(entry.account_id)
                .as_hyphenated()
                .to_string();

            return Err(
                AccountManagementError::IdentityAssociatedWithAnotherAccount {
                    account_id: formatted_account_id,
                }
                .into(),
            );
        }

        Ok(())
    }

    /// Registers a new account for the caller.
    pub async fn register_account(
        &self,
        caller: &Principal,
        input: &RegisterAccountInput,
    ) -> ServiceResult<Account, ApiError> {
        self.assert_identity_has_no_account(caller)?;

        let account_id = generate_uuid_v4().await.as_bytes().to_owned();
        if self.account_repository.find_by_id(&account_id)?.is_some() {
            return Err(AccountManagementError::DuplicatedAccountId.into());
        }

        self.account_repository.find_by_id(&account_id)?;

        let account = self.account_mapper.map_register_account_input_to_account(
            input.clone(),
            account_id,
            *caller,
            canister_config().shared_bank_canister,
        );
        let account_identity = self
            .account_identity_mapper
            .map_account_identity_for_registration(account_id, *caller);

        self.account_repository
            .insert(Account::key(&account_id), account.clone());
        self.account_identity_repository.insert(
            AccountIdentity::key(&account_identity.identity, &account_identity.account_id),
            account_identity,
        );
        let bank_entries = self
            .account_bank_mapper
            .map_account_to_account_bank_entries(&account);

        bank_entries.iter().for_each(|entry| {
            self.account_bank_repository.insert(
                AccountBank::key(&entry.account_id, &entry.canister_id),
                entry.clone(),
            );
        });

        Ok(account)
    }

    fn get_identity_account(&self, identity: &Principal) -> ServiceResult<Account, ApiError> {
        let maybe_account_identity = self
            .account_identity_repository
            .find_by_identity_id(identity)?;

        if maybe_account_identity.is_none() {
            return Err(AccountManagementError::NoAccountAssociatedWithCallerIdentity.into());
        }

        let account_identity = maybe_account_identity.unwrap();
        let maybe_account = self
            .account_repository
            .find_by_id(&account_identity.account_id)?;

        if maybe_account.is_none() {
            let formatted_account_id = Uuid::from_bytes(account_identity.account_id)
                .as_hyphenated()
                .to_string();

            return Err(AccountManagementError::MissingAccountDetails {
                account_id: formatted_account_id,
            }
            .into());
        }

        Ok(maybe_account.unwrap())
    }

    pub async fn get_account_details(
        &self,
        identity: &Principal,
    ) -> ServiceResult<AccountDetailsDTO, ApiError> {
        let account = self.get_identity_account(identity)?;
        let identities = account
            .identities
            .iter()
            .map(|identity| {
                self.account_identity_repository
                    .find_by_identity_id(identity)
                    .unwrap()
                    .unwrap()
            })
            .collect::<Vec<AccountIdentity>>();

        let banks = self.account_bank_repository.find_by_account_id(&account.id);

        let account_details =
            self.account_mapper
                .map_to_account_details_dto(&account, &banks, &identities);

        Ok(account_details)
    }

    pub async fn associate_identity_with_account(
        &self,
        caller: &Principal,
        input: &AssociateIdentityWithAccountInput,
    ) -> ServiceResult<Account, ApiError> {
        self.assert_identity_has_no_account(caller)?;
        let account_id = *Uuid::from_str(input.account_id.as_str())
            .map_err(|_| AccountManagementError::MalformedAccountId {
                account_id: input.account_id.clone(),
            })?
            .as_bytes();

        let maybe_account = self.account_repository.find_by_id(&account_id)?;
        if maybe_account.is_none() {
            return Err(AccountManagementError::NoAccountAssociatedWithCallerIdentity)?;
        }

        let mut account = maybe_account.unwrap();
        account
            .unconfirmed_identities
            .iter()
            .find(|identity| identity == &caller)
            .ok_or(AccountManagementError::NoAccountAssociatedWithCallerIdentity)?;

        let unconfirmed_identities = account
            .unconfirmed_identities
            .iter()
            .filter(|identity| identity != &caller)
            .copied()
            .collect();

        account.unconfirmed_identities = unconfirmed_identities;
        account.identities.push(*caller);

        let account_identity = self
            .account_identity_mapper
            .map_account_identity_for_registration(account_id, *caller);

        self.account_repository
            .insert(Account::key(&account_id), account.clone());
        self.account_identity_repository.insert(
            AccountIdentity::key(&account_identity.identity, &account_identity.account_id),
            account_identity,
        );

        Ok(account)
    }

    pub async fn remove_account(&self, identity: &Principal) -> ServiceResult<Account, ApiError> {
        let maybe_account_identity = self
            .account_identity_repository
            .find_by_identity_id(identity)?;

        if maybe_account_identity.is_none() {
            return Err(AccountManagementError::NoAccountAssociatedWithCallerIdentity)?;
        }

        let account_identity = maybe_account_identity.unwrap();
        let maybe_removed_account = self
            .account_repository
            .remove(&Account::key(&account_identity.account_id));

        if maybe_removed_account.is_none() {
            return Err(AccountManagementError::NoAccountAssociatedWithCallerIdentity)?;
        }

        let removed_account = maybe_removed_account.unwrap();
        removed_account.identities.iter().for_each(|identity| {
            self.account_identity_repository
                .remove(&AccountIdentity::key(identity, &removed_account.id));
        });
        removed_account.banks.iter().for_each(|bank| {
            self.account_bank_repository
                .remove(&AccountBank::key(&removed_account.id, bank));
        });

        Ok(removed_account)
    }

    pub async fn manage_account(
        &self,
        identity: &Principal,
        input: &ManageAccountInput,
    ) -> ServiceResult<AccountDetailsDTO, ApiError> {
        let current_account = self.get_identity_account(identity)?;
        let account_identities = match &input.identities {
            Some(identities) => {
                self.account_identity_service
                    .update_account_identities(&current_account, identities, Some(identity))
                    .await?
            }
            None => self
                .account_identity_service
                .get_account_identities(&current_account)?,
        };
        let account_banks = match (input.banks.to_owned(), input.main_bank) {
            (Some(banks), Some(main_bank)) => {
                self.account_bank_service
                    .update_account_banks(&current_account.id, &Some(main_bank), &banks)
                    .await?
            }
            (None, Some(main_bank)) => {
                let current_bank_dtos = self
                    .account_bank_service
                    .get_account_banks_dtos(&current_account.id)?;
                self.account_bank_service
                    .update_account_banks(&current_account.id, &Some(main_bank), &current_bank_dtos)
                    .await?
            }
            (Some(banks), None) => {
                self.account_bank_service
                    .update_account_banks(&current_account.id, &current_account.main_bank, &banks)
                    .await?
            }
            (_, _) => self
                .account_bank_service
                .get_account_banks(&current_account.id)?,
        };

        let updated_account = self.account_mapper.update_account_with_input(
            input,
            &current_account,
            &account_identities,
            &account_banks,
        );

        if updated_account.unconfirmed_identities.len()
            > Account::MAX_ACCOUNT_UNCONFIRMED_IDENTITIES as usize
        {
            return Err(
                AccountManagementError::TooManyUnconfirmedIdentitiesForAccount {
                    max_identities: Account::MAX_ACCOUNT_UNCONFIRMED_IDENTITIES,
                },
            )?;
        }

        let total_identities =
            updated_account.unconfirmed_identities.len() + account_identities.len();

        if total_identities > Account::MAX_ACCOUNT_IDENTITIES as usize {
            return Err(AccountManagementError::TooManyIdentitiesForAccount {
                max_identities: Account::MAX_ACCOUNT_IDENTITIES,
            })?;
        }

        self.account_repository
            .insert(Account::key(&updated_account.id), updated_account.clone());

        let account_details = self.account_mapper.map_to_account_details_dto(
            &updated_account,
            &account_banks,
            &account_identities,
        );

        Ok(account_details)
    }

    pub async fn get_account_main_bank(
        &self,
        identity: &Principal,
    ) -> ServiceResult<Option<AccountBankDTO>> {
        let account = self.get_identity_account(identity)?;

        match account.main_bank {
            Some(main_bank) => {
                let account_banks = self.account_bank_repository.find_by_account_id(&account.id);
                let account_bank = account_banks
                    .iter()
                    .find(|bank| bank.canister_id == main_bank)
                    .ok_or(AccountManagementError::AccountMissingMainBankDetails)?;
                let dto = self.account_bank_mapper.map_to_dto(account_bank);

                Ok(Some(dto))
            }
            None => Ok(None),
        }
    }
}
