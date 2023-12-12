use super::{AccountService, UserService};
use crate::{
    core::CallContext,
    errors::{AccountError, TransferError},
    mappers::HelperMapper,
    models::{Account, Transfer, TransferId},
    repositories::{AccountRepository, TransferRepository},
};
use ic_canister_core::repository::Repository;
use ic_canister_core::{api::ServiceResult, utils::rfc3339_to_timestamp};
use uuid::Uuid;
use wallet_api::ListAccountTransfersInput;

#[derive(Default, Debug)]
pub struct TransferService {
    user_service: UserService,
    account_service: AccountService,
    account_repository: AccountRepository,
    transfer_repository: TransferRepository,
}

impl TransferService {
    pub fn get_transfer(&self, id: &TransferId, ctx: &CallContext) -> ServiceResult<Transfer> {
        let transfer_key = Transfer::key(*id);
        let transfer = self.transfer_repository.get(&transfer_key).ok_or({
            TransferError::TransferNotFound {
                transfer_id: Uuid::from_bytes(*id).hyphenated().to_string(),
            }
        })?;

        self.assert_transfer_access(&transfer, ctx)?;

        Ok(transfer)
    }

    pub fn get_transfers(
        &self,
        transfer_ids: Vec<TransferId>,
        ctx: &CallContext,
    ) -> ServiceResult<Vec<Transfer>> {
        if transfer_ids.len() > 50 {
            Err(TransferError::GetTransfersBatchNotAllowed { max: 50 })?
        }

        let mut transfers = Vec::new();
        for transfer_id in transfer_ids.iter() {
            let transfer = self.get_transfer(transfer_id, ctx)?;
            self.assert_transfer_access(&transfer, ctx)?;
            transfers.push(transfer);
        }

        Ok(transfers)
    }

    pub fn list_account_transfers(
        &self,
        input: ListAccountTransfersInput,
    ) -> ServiceResult<Vec<Transfer>> {
        let account = self
            .account_service
            .get_account(HelperMapper::to_uuid(input.account_id)?.as_bytes())?;

        let transfers = self.transfer_repository.find_by_account(
            account.id,
            input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
            input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
            input.status,
        );

        Ok(transfers)
    }

    fn assert_transfer_access(&self, transfer: &Transfer, ctx: &CallContext) -> ServiceResult<()> {
        let caller_user = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;
        let account_key = Account::key(transfer.from_account);
        let account = self.account_repository.get(&account_key).ok_or({
            AccountError::AccountNotFound {
                id: Uuid::from_bytes(transfer.from_account)
                    .hyphenated()
                    .to_string(),
            }
        })?;
        let is_transfer_creator = caller_user.id == transfer.initiator_user;
        let is_account_owner = account.owners.contains(&caller_user.id);
        if !is_transfer_creator && !is_account_owner {
            Err(AccountError::Forbidden)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;

    use super::*;
    use crate::{
        core::test_utils,
        models::{
            account_test_utils::mock_account, transfer_test_utils::mock_transfer,
            user_test_utils::mock_user, User,
        },
        repositories::UserRepository,
    };

    struct TestContext {
        repository: TransferRepository,
        service: TransferService,
        caller_user: User,
        account: Account,
        call_context: CallContext,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));
        let mut user = mock_user();
        user.identities = vec![call_context.caller()];

        UserRepository::default().insert(user.to_key(), user.clone());

        let mut account = mock_account();
        account.owners.push(user.id);

        AccountRepository::default().insert(account.to_key(), account.clone());

        TestContext {
            repository: TransferRepository::default(),
            service: TransferService::default(),
            caller_user: user,
            account,
            call_context,
        }
    }

    #[test]
    fn get_transfer() {
        let ctx = setup();
        let mut transfer = mock_transfer();
        transfer.from_account = ctx.account.id;
        transfer.initiator_user = ctx.caller_user.id;

        ctx.repository.insert(transfer.to_key(), transfer.clone());

        let result = ctx.service.get_transfer(&transfer.id, &ctx.call_context);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_get_transfer_not_allowed() {
        let ctx = setup();
        let mut user = mock_user();
        user.identities = vec![Principal::anonymous()];
        UserRepository::default().insert(user.to_key(), user.clone());
        let mut transfer = mock_transfer();
        transfer.from_account = ctx.account.id;
        transfer.initiator_user = user.id;

        ctx.repository.insert(transfer.to_key(), transfer.clone());

        let result = ctx.service.get_transfer(&transfer.id, &ctx.call_context);

        assert!(result.is_err());
    }
}
