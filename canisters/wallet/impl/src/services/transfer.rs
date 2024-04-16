use super::{AccountService, UserService};
use crate::{
    core::{authorization::Authorization, CallContext},
    errors::{AccountError, TransferError},
    mappers::HelperMapper,
    models::{
        resource::{AccountResourceAction, Resource, ResourceId},
        Transfer, TransferId,
    },
    repositories::TransferRepository,
};
use ic_canister_core::repository::Repository;
use ic_canister_core::{api::ServiceResult, model::ModelValidator, utils::rfc3339_to_timestamp};
use uuid::Uuid;
use wallet_api::ListAccountTransfersInput;

#[derive(Default, Debug)]
pub struct TransferService {
    user_service: UserService,
    account_service: AccountService,
    transfer_repository: TransferRepository,
}

impl TransferService {
    pub fn add_transfer(&self, transfer: Transfer) -> ServiceResult<Transfer> {
        transfer.validate()?;

        self.transfer_repository
            .insert(transfer.to_key(), transfer.to_owned());

        Ok(transfer)
    }

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
        let caller_user = self.user_service.get_user_by_identity(&ctx.caller())?;
        let is_transfer_creator = caller_user.id == transfer.initiator_user;
        let has_account_access = Authorization::is_allowed(
            ctx,
            &Resource::Account(AccountResourceAction::Read(ResourceId::Id(
                transfer.from_account,
            ))),
        );
        if !is_transfer_creator && !has_account_access {
            Err(AccountError::Forbidden)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{
            account_test_utils::mock_account, proposal_test_utils::mock_proposal,
            transfer_test_utils::mock_transfer, user_test_utils::mock_user, Account, User,
        },
        repositories::{
            ACCOUNT_REPOSITORY, PROPOSAL_REPOSITORY, TRANSFER_REPOSITORY, USER_REPOSITORY,
        },
    };
    use candid::Principal;

    struct TestContext {
        repository: TransferRepository,
        service: TransferService,
        caller_user: User,
        account: Account,
        call_context: CallContext,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_system();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));
        let mut user = mock_user();
        user.identities = vec![call_context.caller()];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let account = mock_account();

        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());

        let mut proposal = mock_proposal();
        proposal.id = [2; 16];

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        TestContext {
            repository: TransferRepository::default(),
            service: TransferService::default(),
            caller_user: user,
            account,
            call_context,
        }
    }

    #[test]
    fn add_transfer_successfully() {
        let ctx = setup();

        let mut transfer = mock_transfer();
        transfer.initiator_user = ctx.caller_user.id;
        transfer.from_account = ctx.account.id;

        let result = ctx.service.add_transfer(transfer);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_add_transfer_missing_initiator_user() {
        let ctx = setup();

        let mut transfer = mock_transfer();
        transfer.initiator_user = [0; 16];
        transfer.from_account = ctx.account.id;

        let result = ctx.service.add_transfer(transfer);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().details.unwrap().get("info"),
            Some(&"The initiator_user does not exist".to_owned())
        );
    }

    #[test]
    fn fail_add_transfer_missing_from_account() {
        let ctx = setup();

        let mut transfer = mock_transfer();
        transfer.initiator_user = ctx.caller_user.id;
        transfer.from_account = [0; 16];

        let result = ctx.service.add_transfer(transfer);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().details.unwrap().get("info"),
            Some(&"The from_account does not exist".to_owned())
        );
    }

    #[test]
    fn fail_add_transfer_missing_proposal_id() {
        let ctx = setup();

        let mut transfer = mock_transfer();
        transfer.initiator_user = ctx.caller_user.id;
        transfer.from_account = ctx.account.id;
        transfer.proposal_id = [0; 16];

        let result = ctx.service.add_transfer(transfer);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().details.unwrap().get("info"),
            Some(&"The proposal_id does not exist".to_owned())
        );
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
        user.identities = vec![Principal::from_slice(&[10; 29])];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let account = mock_account();

        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());

        let mut transfer = mock_transfer();
        transfer.from_account = account.id;
        transfer.initiator_user = user.id;

        TRANSFER_REPOSITORY.insert(transfer.to_key(), transfer.clone());

        let result = ctx.service.get_transfer(&transfer.id, &ctx.call_context);

        assert!(result.is_err());
    }
}
