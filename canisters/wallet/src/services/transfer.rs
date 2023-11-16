use super::{AccountService, NotificationService, UserService};
use crate::{
    core::CallContext,
    errors::{AccountError, TransferError},
    mappers::HelperMapper,
    models::{Account, Transfer, TransferId},
    repositories::{AccountRepository, ProposalRepository, TransferRepository},
    transport::ListAccountTransfersInput,
};
use ic_canister_core::repository::Repository;
use ic_canister_core::{api::ServiceResult, utils::rfc3339_to_timestamp};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct TransferService {
    user_service: UserService,
    account_service: AccountService,
    account_repository: AccountRepository,
    transfer_repository: TransferRepository,
    notification_service: NotificationService,
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

    // pub async fn create_transfer(
    //     &self,
    //     input: TransferInput,
    //     ctx: &CallContext,
    // ) -> ServiceResult<Transfer> {
    //     // validate user is owner of account
    //     let caller_user = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;
    //     let account_id = HelperMapper::to_uuid(input.from_account_id.clone())?;
    //     let account_key = Account::key(*account_id.as_bytes());
    //     let account =
    //         self.account_repository
    //             .get(&account_key)
    //             .ok_or(AccountError::AccountNotFound {
    //                 id: account_id.hyphenated().to_string(),
    //             })?;
    //     let is_account_owner = account.owners.contains(&caller_user.id);
    //     if !is_account_owner {
    //         Err(AccountError::Forbidden)?
    //     }

    //     // create transfer
    //     let blockchain_api = BlockchainApiFactory::build(&account.blockchain, &account.standard)?;
    //     let default_fee = blockchain_api.transaction_fee(&account).await?;
    //     let transfer_id = generate_uuid_v4().await;

    //     let transfer = TransferMapper::from_create_input(
    //         input,
    //         *transfer_id.as_bytes(),
    //         caller_user.id,
    //         Nat(default_fee.fee),
    //         blockchain_api.default_network(),
    //         Transfer::default_expiration_dt(),
    //     )?;

    //     transfer.validate()?;

    //     // save transfer to stable memory
    //     self.transfer_repository
    //         .insert(transfer.to_key(), transfer.to_owned());

    //     // this await is within the canister so a trap inside create_transfer_proposal will revert the canister state
    //     let mut proposal = self.create_transfer_proposal(&account, &transfer).await?;

    //     proposal.post_process()?;

    //     Ok(transfer)
    // }

    // async fn create_transfer_proposal(
    //     &self,
    //     account: &Account,
    //     transfer: &Transfer,
    // ) -> ServiceResult<Proposal> {
    //     let proposal_id = generate_uuid_v4().await;
    //     let mut proposal = Proposal {
    //         id: *proposal_id.as_bytes(),
    //         status: ProposalStatus::Created,
    //         created_timestamp: time(),
    //         proposed_by: Some(transfer.initiator_user),
    //         operation: ProposalOperation::Transfer(TransferOperationContext {
    //             transfer_id: transfer.id,
    //             account_id: account.id,
    //         }),
    //         metadata: vec![],
    //         last_modification_timestamp: time(),
    //         votes: Vec::new(),
    //     };

    //     for owner in account.owners.iter() {
    //         proposal.votes.push(ProposalVote {
    //             user_id: *owner,
    //             status: match transfer.initiator_user == *owner {
    //                 true => ProposalVoteStatus::Adopted,
    //                 false => ProposalVoteStatus::Pending,
    //             },
    //             decided_dt: match transfer.initiator_user == *owner {
    //                 true => Some(time()),
    //                 false => None,
    //             },
    //             last_modification_timestamp: time(),
    //             status_reason: None,
    //         });

    //         if transfer.initiator_user != *owner {
    //             self.notification_service
    //                 .send_notification(
    //                     *owner,
    //                     NotificationType::TransferProposalCreated(
    //                         TransferProposalCreatedNotification {
    //                             account_id: account.id,
    //                             proposal_id: proposal.id,
    //                             transfer_id: transfer.id,
    //                         },
    //                     ),
    //                     None,
    //                     None,
    //                 )
    //                 .await?;
    //         }
    //     }

    //     self.proposal_repository
    //         .insert(proposal.to_key(), proposal.clone());

    //     Ok(proposal)
    // }

    pub fn list_account_transfers(
        &self,
        input: ListAccountTransfersInput,
        ctx: &CallContext,
    ) -> ServiceResult<Vec<Transfer>> {
        let account = self
            .account_service
            .get_account(HelperMapper::to_uuid(input.account_id)?.as_bytes(), ctx)?;

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

    // #[tokio::test]
    // async fn create_transfer_happy_path() {
    //     let ctx = setup();
    //     let transfer_input = TransferInput {
    //         from_account_id: Uuid::from_bytes(ctx.account.id).to_string(),
    //         amount: candid::Nat::from(100),
    //         fee: None,
    //         network: None,
    //         expiration_dt: None,
    //         execution_plan: None,
    //         metadata: None,
    //         to: "03e252ebe920437d7aaff019b78a40bca50e24e42aebff00384d62038482ac81".to_string(),
    //     };

    //     let result = ctx
    //         .service
    //         .create_transfer(transfer_input.clone(), &ctx.call_context)
    //         .await;

    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap().from_account, ctx.account.id);
    // }

    // #[tokio::test]
    // async fn fail_create_transfer_from_unknown_account() {
    //     let ctx = setup();
    //     let transfer_input = TransferInput {
    //         from_account_id: Uuid::new_v4().to_string(),
    //         amount: candid::Nat::from(100),
    //         fee: None,
    //         network: None,
    //         expiration_dt: None,
    //         execution_plan: None,
    //         metadata: None,
    //         to: "03e252ebe920437d7aaff019b78a40bca50e24e42aebff00384d62038482ac81".to_string(),
    //     };

    //     let result = ctx
    //         .service
    //         .create_transfer(transfer_input.clone(), &ctx.call_context)
    //         .await;

    //     assert!(result.is_err());
    // }
}
