use super::HelperMapper;
use crate::{
    errors::MapperError,
    models::{
        AccountId, PolicySnapshot, Transfer, TransferExecutionPlan, TransferId, TransferStatus,
    },
    transport::{
        NetworkDTO, TransferDTO, TransferExecutionScheduleDTO, TransferInput, TransferListItemDTO,
        TransferMetadataDTO,
    },
};
use candid::Nat;
use ic_canister_core::{
    cdk::api::time,
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct TransferMapper {}

impl TransferMapper {
    pub fn from_create_input(
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
            from_wallet: *HelperMapper::to_uuid(input.from_wallet_id)?.as_bytes(),
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

    pub fn to_dto(transfer: Transfer) -> TransferDTO {
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
            status: transfer.status.into(),
        }
    }

    pub fn to_list_item_dto(transfer: Transfer) -> TransferListItemDTO {
        TransferListItemDTO {
            transfer_id: Uuid::from_slice(&transfer.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            amount: transfer.amount,
            to: transfer.to_address,
            created_at: timestamp_to_rfc3339(&transfer.created_timestamp),
            status: transfer.status.into(),
        }
    }
}

impl Transfer {
    pub fn to_dto(&self) -> TransferDTO {
        TransferMapper::to_dto(self.clone())
    }

    pub fn to_list_item_dto(&self) -> TransferListItemDTO {
        TransferMapper::to_list_item_dto(self.clone())
    }
}
