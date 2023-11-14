use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;

use crate::{
    models::{Upgrade, UpgradeExecutionPlan},
    transport::{UpgradeDTO, UpgradeExecutionScheduleDTO, UpgradeMetadataDTO},
};

#[derive(Default, Clone, Debug)]
pub struct UpgradeMapper {}

impl Upgrade {
    pub fn to_dto(&self) -> UpgradeDTO {
        UpgradeMapper::to_dto(self.clone())
    }
}

impl UpgradeMapper {
    pub fn to_dto(upgrade: Upgrade) -> UpgradeDTO {
        UpgradeDTO {
            id: Uuid::from_slice(&upgrade.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            metadata: upgrade
                .metadata
                .iter()
                .map(|(k, v)| UpgradeMetadataDTO {
                    key: k.to_owned(),
                    value: v.to_owned(),
                })
                .collect(),
            expiration_dt: timestamp_to_rfc3339(&upgrade.expiration_dt),
            execution_plan: match upgrade.execution_plan {
                UpgradeExecutionPlan::Immediate => UpgradeExecutionScheduleDTO::Immediate,
                UpgradeExecutionPlan::Scheduled { execution_time } => {
                    UpgradeExecutionScheduleDTO::Scheduled {
                        execution_time: timestamp_to_rfc3339(&execution_time),
                    }
                }
            },
            status: upgrade.status.into(),
        }
    }
}
