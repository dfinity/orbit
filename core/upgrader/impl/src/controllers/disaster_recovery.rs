use candid::Principal;
use ic_cdk::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ApiResult,
    cdk::{api::is_controller, caller},
};
use upgrader_api::{
    GetDisasterRecoveryAccountsResponse, GetDisasterRecoveryCommitteeResponse,
    IsCommitteeMemberResponse,
};

use crate::{
    errors::UpgraderApiError,
    services::{DisasterRecoveryService, DISASTER_RECOVERY_SERVICE},
};

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: DisasterRecoveryController =
        DisasterRecoveryController::new(DISASTER_RECOVERY_SERVICE.clone());
}

#[update]
async fn set_disaster_recovery_committee(
    input: upgrader_api::SetDisasterRecoveryCommitteeInput,
) -> ApiResult {
    CONTROLLER.set_disaster_recovery_committee(input).await
}

#[update]
async fn set_disaster_recovery_accounts(
    input: upgrader_api::SetDisasterRecoveryAccountsInput,
) -> ApiResult {
    CONTROLLER.set_disaster_recovery_accounts(input).await
}

#[update]
async fn request_disaster_recovery(input: upgrader_api::RequestDisasterRecoveryInput) -> ApiResult {
    CONTROLLER.request_disaster_recovery(input).await
}

#[query]
async fn is_committee_member() -> ApiResult<IsCommitteeMemberResponse> {
    CONTROLLER.is_committee_member().await
}

#[query]
async fn get_disaster_recovery_accounts() -> ApiResult<GetDisasterRecoveryAccountsResponse> {
    CONTROLLER.get_disaster_recovery_accounts().await
}

#[query]
async fn get_disaster_recovery_committee() -> ApiResult<GetDisasterRecoveryCommitteeResponse> {
    CONTROLLER.get_disaster_recovery_committee().await
}

pub struct DisasterRecoveryController {
    disaster_recovery_service: DisasterRecoveryService,
}

impl DisasterRecoveryController {
    pub fn new(disaster_recovery_service: DisasterRecoveryService) -> Self {
        Self {
            disaster_recovery_service,
        }
    }

    async fn set_disaster_recovery_committee(
        &self,
        input: upgrader_api::SetDisasterRecoveryCommitteeInput,
    ) -> ApiResult {
        let caller = caller();
        if !is_controller(&caller) {
            Err(UpgraderApiError::NotController)?
        } else {
            self.disaster_recovery_service
                .set_committee(input.committee.into())
        }
    }

    async fn set_disaster_recovery_accounts(
        &self,

        input: upgrader_api::SetDisasterRecoveryAccountsInput,
    ) -> ApiResult {
        let caller = caller();
        if !is_controller(&caller) {
            Err(UpgraderApiError::NotController)?
        } else {
            self.disaster_recovery_service
                .set_accounts(input.accounts.into_iter().map(Into::into).collect())
        }
    }

    async fn request_disaster_recovery(
        &self,

        input: upgrader_api::RequestDisasterRecoveryInput,
    ) -> ApiResult {
        let caller = caller();
        if !self.disaster_recovery_service.is_committee_member(&caller) {
            Err(UpgraderApiError::Unauthorized)?
        } else {
            self.disaster_recovery_service
                .request_recovery(caller, input);

            self.disaster_recovery_service.check_requests();

            Ok(())
        }
    }

    async fn is_committee_member(&self) -> ApiResult<IsCommitteeMemberResponse> {
        let caller = ic_cdk::caller();

        if caller == Principal::anonymous() {
            Err(UpgraderApiError::Unauthorized)?
        } else {
            Ok(IsCommitteeMemberResponse {
                is_committee_member: self.disaster_recovery_service.is_committee_member(&caller),
            })
        }
    }

    async fn get_disaster_recovery_accounts(
        &self,
    ) -> ApiResult<GetDisasterRecoveryAccountsResponse> {
        let caller = caller();
        if !is_controller(&caller) {
            Err(UpgraderApiError::NotController)?
        } else {
            Ok(GetDisasterRecoveryAccountsResponse {
                accounts: self
                    .disaster_recovery_service
                    .storage
                    .get()
                    .accounts
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            })
        }
    }

    async fn get_disaster_recovery_committee(
        &self,
    ) -> ApiResult<GetDisasterRecoveryCommitteeResponse> {
        let caller = caller();
        if !is_controller(&caller) {
            Err(UpgraderApiError::NotController)?
        } else {
            Ok(GetDisasterRecoveryCommitteeResponse {
                committee: self
                    .disaster_recovery_service
                    .storage
                    .get()
                    .committee
                    .map(Into::into),
            })
        }
    }
}
