use crate::{
    core::ic_cdk::api::trap,
    models::{Account, ProposalOperation, Transfer},
    repositories::{AccountRepository, TransferRepository},
    transport::{ProposalOperationDTO, ProposedTransferOperationDTO},
};
use ic_canister_core::repository::Repository;
use uuid::Uuid;

pub struct ProposalMapperContext {
    pub transfer: Option<Transfer>,
    pub account: Option<Account>,
}

impl From<ProposalOperation> for ProposalMapperContext {
    fn from(operation: ProposalOperation) -> ProposalMapperContext {
        match operation {
            ProposalOperation::Transfer(ctx) => {
                let transfer = TransferRepository::default()
                    .get(&Transfer::key(ctx.transfer_id))
                    .unwrap_or_else(|| {
                        trap(&format!(
                            "MapperError: Transfer not found: {}",
                            Uuid::from_bytes(ctx.transfer_id).hyphenated()
                        ))
                    });

                let account = AccountRepository::default()
                    .get(&Account::key(transfer.from_account))
                    .unwrap_or_else(|| {
                        trap(&format!(
                            "MapperError: Account not found: {}",
                            Uuid::from_bytes(transfer.from_account).hyphenated()
                        ))
                    });

                Self {
                    transfer: Some(transfer),
                    account: Some(account),
                }
            }
        }
    }
}

impl From<ProposalOperation> for ProposalOperationDTO {
    fn from(operation: ProposalOperation) -> ProposalOperationDTO {
        let context = ProposalMapperContext::from(operation.clone());
        match operation {
            ProposalOperation::Transfer(_) => {
                let transfer = context.transfer.expect("Missing transfer context");
                let account = context.account.expect("Missing account context");

                ProposalOperationDTO::Transfer(ProposedTransferOperationDTO {
                    transfer: transfer.to_dto(),
                    account: account.to_dto(),
                })
            }
        }
    }
}
