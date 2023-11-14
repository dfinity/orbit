use ic_canister_core::api::ServiceResult;

use crate::{
    core::CallContext,
    models::Upgrade,
    repositories::{AccountRepository, ProposalRepository, TransferRepository},
    transport::UpgradeInput,
};

use super::{AccountService, NotificationService, UserService};

#[derive(Default, Debug)]
pub struct UpgradeService {
    call_context: CallContext,
    user_service: UserService,
    account_service: AccountService,
    account_repository: AccountRepository,
    transfer_repository: TransferRepository,
    proposal_repository: ProposalRepository,
    notification_service: NotificationService,
}

impl UpgradeService {
    pub async fn create_upgrade(
        &self,
        input: UpgradeInput,
        ctx: &CallContext,
    ) -> ServiceResult<Upgrade> {
        Ok(Upgrade {
            id: todo!(),
            initiator_user: todo!(),
            status: todo!(),
            expiration_dt: todo!(),
            execution_plan: todo!(),
            metadata: todo!(),
            last_modification_timestamp: todo!(),
            created_timestamp: todo!(),
        })
    }
}
