use std::sync::Arc;

use ic_cdk::query;
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ApiResult,
    cdk::{api::is_controller, caller},
};

use crate::{
    errors::UpgraderApiError,
    services::{
        DisasterRecoveryService, GetLogsResult, LoggerService, DISASTER_RECOVERY_SERVICE,
        LOGGER_SERVICE,
    },
};

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: LogsController = LogsController {
        disaster_recover_service: DISASTER_RECOVERY_SERVICE.clone(),
        logger_service: LOGGER_SERVICE.clone(),
    };
}

#[query]
fn get_logs(input: upgrader_api::GetLogsInput) -> ApiResult<upgrader_api::GetLogsResponse> {
    CONTROLLER.get_logs(input)
}

pub struct LogsController {
    disaster_recover_service: Arc<DisasterRecoveryService>,
    logger_service: Arc<LoggerService>,
}

impl LogsController {
    pub fn get_logs(
        &self,
        input: upgrader_api::GetLogsInput,
    ) -> ApiResult<upgrader_api::GetLogsResponse> {
        let caller = caller();

        if is_controller(&caller) || self.disaster_recover_service.is_committee_member(&caller) {
            let GetLogsResult {
                logs,
                next_offset,
                total,
            } = self.logger_service.get_logs(
                input.pagination.as_ref().and_then(|p| p.offset),
                input.pagination.as_ref().and_then(|p| p.limit),
            );

            Ok(upgrader_api::GetLogsResponse {
                logs: logs.into_iter().map(|l| l.into()).collect(),
                total,
                next_offset,
            })
        } else {
            Err(UpgraderApiError::Unauthorized.into())
        }
    }
}
