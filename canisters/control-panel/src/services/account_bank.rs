use crate::{
    core::{ApiError, Repository, ServiceResult, UUID},
    entities::{Account, AccountBank},
    errors::AccountManagementError,
    mappers::AccountBankMapper,
    repositories::AccountBankRepository,
    transport::AccountBankDTO,
};
use candid::Principal;

#[derive(Default)]
pub struct AccountBankService {
    account_bank_repository: AccountBankRepository,
    account_bank_mapper: AccountBankMapper,
}

impl AccountBankService {
    pub fn get_account_banks(
        &self,
        account_id: &UUID,
    ) -> ServiceResult<Vec<AccountBank>, ApiError> {
        let account_banks = self.account_bank_repository.find_by_account_id(account_id);

        Ok(account_banks)
    }

    pub fn get_account_banks_dtos(
        &self,
        account_id: &UUID,
    ) -> ServiceResult<Vec<AccountBankDTO>, ApiError> {
        let account_banks = self.get_account_banks(account_id)?;
        let dtos = account_banks
            .iter()
            .map(|bank| self.account_bank_mapper.map_to_dto(bank))
            .collect();

        Ok(dtos)
    }

    pub async fn update_account_banks(
        &self,
        account_id: &UUID,
        main_bank: &Option<Principal>,
        new_banks: &[AccountBankDTO],
    ) -> ServiceResult<Vec<AccountBank>, ApiError> {
        let current_banks = self.get_account_banks(account_id)?;
        let banks_to_delete = current_banks
            .iter()
            .filter(|current_bank| {
                !new_banks
                    .iter()
                    .any(|new_bank| new_bank.canister_id == current_bank.canister_id)
            })
            .map(|entry| entry.canister_id)
            .collect::<Vec<Principal>>();

        let mut new_banks = new_banks.to_owned();
        if main_bank.is_some() {
            let main_bank = main_bank.as_ref().unwrap();
            let has_bank = new_banks.iter().any(|bank| bank.canister_id == *main_bank);
            if !has_bank {
                new_banks.push(AccountBankDTO {
                    canister_id: *main_bank,
                    name: None,
                });
            }
        }

        let total_banks = new_banks.len() - banks_to_delete.len();
        if total_banks > Account::MAX_ACCOUNT_BANKS as usize {
            return Err(AccountManagementError::TooManyBanksForAccount {
                max_banks: Account::MAX_ACCOUNT_BANKS,
            })?;
        }

        banks_to_delete.iter().for_each(|bank_canister_id| {
            self.account_bank_repository
                .remove(&AccountBank::key(account_id, bank_canister_id));
        });

        let mut updated_banks: Vec<AccountBank> = Vec::with_capacity(new_banks.len());

        new_banks.iter().for_each(|entry| {
            let new_bank = self
                .account_bank_mapper
                .from_dto(entry.clone(), *account_id);

            self.account_bank_repository.insert(
                AccountBank::key(&new_bank.account_id, &new_bank.canister_id),
                new_bank.clone(),
            );

            updated_banks.push(new_bank);
        });

        Ok(updated_banks)
    }
}
