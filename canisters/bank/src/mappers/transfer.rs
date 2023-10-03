use super::HelperMapper;
use crate::{
    errors::MapperError,
    models::{
        AccountId, PolicySnapshot, Transfer, TransferExecutionPlan, TransferId, TransferStatus,
    },
    transport::{
        NetworkDTO, TransferDTO, TransferExecutionScheduleDTO, TransferInput, TransferListItemDTO,
        TransferMetadataDTO, TransferStatusDTO,
    },
};
use candid::Nat;
use ic_canister_core::{
    cdk::api::time,
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct TransferMapper {
    helper_mapper: HelperMapper,
}

impl TransferMapper {
    pub fn new_transfer_from_input(
        &self,
        input: TransferInput,
        transfer_id: TransferId,
        initiator_account: AccountId,
        default_fee: Nat,
        blockchain_network: String,
        default_expiration_dt: u64,
    ) -> Result<Transfer, MapperError> {
        Ok(Transfer {
            id: transfer_id,
            initiator_account,
            from_wallet: *self
                .helper_mapper
                .uuid_from_str(input.from_wallet_id)?
                .as_bytes(),
            expiration_dt: input.expiration_dt.map_or(default_expiration_dt, |dt| {
                rfc3339_to_timestamp(dt.as_str())
            }),
            fee: input.fee.unwrap_or(default_fee),
            metadata: input.metadata.map_or(vec![], |data| {
                data.iter()
                    .map(|entry| (entry.key.to_owned(), entry.value.to_owned()))
                    .collect()
            }),
            amount: input.amount,
            to_address: input.to,
            status: TransferStatus::Pending,
            blockchain_network,
            execution_plan: match input.execution_plan {
                Some(plan) => match plan {
                    TransferExecutionScheduleDTO::Immediate => TransferExecutionPlan::Immediate,
                    TransferExecutionScheduleDTO::Scheduled { execution_time } => {
                        TransferExecutionPlan::Scheduled {
                            execution_time: rfc3339_to_timestamp(execution_time.as_str()),
                        }
                    }
                },
                None => TransferExecutionPlan::Immediate,
            },
            policy_snapshot: PolicySnapshot { min_approvals: 1 },
            last_modification_timestamp: time(),
            created_timestamp: time(),
        })
    }

    pub fn transfer_to_dto(&self, transfer: Transfer) -> TransferDTO {
        TransferDTO {
            id: Uuid::from_slice(&transfer.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            amount: transfer.amount,
            fee: transfer.fee,
            metadata: transfer
                .metadata
                .iter()
                .map(|(k, v)| TransferMetadataDTO {
                    key: k.to_owned(),
                    value: v.to_owned(),
                })
                .collect(),
            network: NetworkDTO {
                id: transfer.blockchain_network.to_owned(),
                name: transfer.blockchain_network.to_owned(),
            },
            from_wallet_id: Uuid::from_slice(&transfer.from_wallet)
                .unwrap()
                .hyphenated()
                .to_string(),
            to: transfer.to_address,
            expiration_dt: timestamp_to_rfc3339(&transfer.expiration_dt),
            execution_plan: match transfer.execution_plan {
                TransferExecutionPlan::Immediate => TransferExecutionScheduleDTO::Immediate,
                TransferExecutionPlan::Scheduled { execution_time } => {
                    TransferExecutionScheduleDTO::Scheduled {
                        execution_time: timestamp_to_rfc3339(&execution_time),
                    }
                }
            },
            status: self.to_transfer_status_dto(transfer.status),
        }
    }

    pub fn to_transfer_status_dto(&self, status: TransferStatus) -> TransferStatusDTO {
        match status {
            TransferStatus::Cancelled { reason } => TransferStatusDTO::Cancelled {
                reason: reason.map(|r| r.to_owned()),
            },
            TransferStatus::Processing { started_at } => TransferStatusDTO::Processing {
                started_at: timestamp_to_rfc3339(&started_at),
            },
            TransferStatus::Submitted => TransferStatusDTO::Submitted,
            TransferStatus::Pending => TransferStatusDTO::Pending,
            TransferStatus::Completed {
                signature,
                hash,
                completed_at,
            } => TransferStatusDTO::Completed {
                signature: signature.map(|s| s.to_owned()),
                hash: hash.map(|h| h.to_owned()),
                completed_at: timestamp_to_rfc3339(&completed_at),
            },
            TransferStatus::Approved => TransferStatusDTO::Approved,
            TransferStatus::Rejected { reason } => TransferStatusDTO::Rejected {
                reason: reason.to_owned(),
            },
        }
    }

    pub fn transfer_to_list_item_dto(&self, transfer: Transfer) -> TransferListItemDTO {
        TransferListItemDTO {
            transfer_id: Uuid::from_slice(&transfer.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            amount: transfer.amount,
            to: transfer.to_address,
            created_at: timestamp_to_rfc3339(&transfer.created_timestamp),
            status: self.to_transfer_status_dto(transfer.status),
        }
    }
}
