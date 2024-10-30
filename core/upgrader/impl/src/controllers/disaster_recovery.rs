use std::sync::Arc;

use candid::Principal;
use ic_cdk::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ApiResult,
    cdk::{api::is_controller, caller},
};

use crate::{
    errors::UpgraderApiError,
    services::{DisasterRecoveryService, DISASTER_RECOVERY_SERVICE},
};

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: Arc<DisasterRecoveryController> = Arc::new(DisasterRecoveryController {
        disaster_recovery_service: DISASTER_RECOVERY_SERVICE.clone()
    });
}

#[update]
fn set_disaster_recovery_committee(
    input: upgrader_api::SetDisasterRecoveryCommitteeInput,
) -> ApiResult {
    CONTROLLER.set_disaster_recovery_committee(input)
}

#[update]
fn set_disaster_recovery_accounts(
    input: upgrader_api::SetDisasterRecoveryAccountsInput,
) -> ApiResult {
    CONTROLLER.set_disaster_recovery_accounts(input)
}

#[update]
fn set_disaster_recovery_accounts_and_assets(
    input: upgrader_api::SetDisasterRecoveryAccountsAndAssetsInput,
) -> ApiResult {
    CONTROLLER.set_disaster_recovery_accounts_and_assets(input)
}

#[update]
fn request_disaster_recovery(input: upgrader_api::RequestDisasterRecoveryInput) -> ApiResult {
    CONTROLLER.request_disaster_recovery(input)
}

#[query]
fn is_committee_member() -> ApiResult<upgrader_api::IsCommitteeMemberResponse> {
    CONTROLLER.is_committee_member()
}

#[query]
fn get_disaster_recovery_accounts() -> ApiResult<upgrader_api::GetDisasterRecoveryAccountsResponse>
{
    CONTROLLER.get_disaster_recovery_accounts()
}

#[query]
fn get_disaster_recovery_accounts_and_assets(
) -> ApiResult<upgrader_api::GetDisasterRecoveryAccountsAndAssetsResponse> {
    CONTROLLER.get_disaster_recovery_accounts_and_assets()
}

#[query]
fn get_disaster_recovery_committee() -> ApiResult<upgrader_api::GetDisasterRecoveryCommitteeResponse>
{
    CONTROLLER.get_disaster_recovery_committee()
}

#[query]
fn get_disaster_recovery_state() -> ApiResult<upgrader_api::GetDisasterRecoveryStateResponse> {
    CONTROLLER.get_disaster_recovery_state()
}

pub struct DisasterRecoveryController {
    disaster_recovery_service: Arc<DisasterRecoveryService>,
}

impl DisasterRecoveryController {
    fn set_disaster_recovery_committee(
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

    fn set_disaster_recovery_accounts(
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

    fn set_disaster_recovery_accounts_and_assets(
        &self,

        input: upgrader_api::SetDisasterRecoveryAccountsAndAssetsInput,
    ) -> ApiResult {
        let caller = caller();
        if !is_controller(&caller) {
            Err(UpgraderApiError::NotController)?
        } else {
            self.disaster_recovery_service.set_accounts_and_assets(
                input.accounts.into_iter().map(Into::into).collect(),
                input.assets.into_iter().map(Into::into).collect(),
            )
        }
    }

    fn request_disaster_recovery(
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

    fn is_committee_member(&self) -> ApiResult<upgrader_api::IsCommitteeMemberResponse> {
        let caller = ic_cdk::caller();

        if caller == Principal::anonymous() {
            Err(UpgraderApiError::Unauthorized)?
        } else {
            Ok(upgrader_api::IsCommitteeMemberResponse {
                is_committee_member: self.disaster_recovery_service.is_committee_member(&caller),
            })
        }
    }

    fn get_disaster_recovery_accounts(
        &self,
    ) -> ApiResult<upgrader_api::GetDisasterRecoveryAccountsResponse> {
        let caller = caller();
        if !is_controller(&caller) {
            Err(UpgraderApiError::NotController)?
        } else {
            Ok(upgrader_api::GetDisasterRecoveryAccountsResponse {
                accounts: self
                    .disaster_recovery_service
                    .get_accounts()
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            })
        }
    }

    fn get_disaster_recovery_accounts_and_assets(
        &self,
    ) -> ApiResult<upgrader_api::GetDisasterRecoveryAccountsAndAssetsResponse> {
        let caller = caller();
        if !is_controller(&caller) {
            Err(UpgraderApiError::NotController)?
        } else {
            Ok(upgrader_api::GetDisasterRecoveryAccountsAndAssetsResponse {
                accounts: self
                    .disaster_recovery_service
                    .get_multi_asset_accounts()
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                assets: self
                    .disaster_recovery_service
                    .get_assets()
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            })
        }
    }

    fn get_disaster_recovery_committee(
        &self,
    ) -> ApiResult<upgrader_api::GetDisasterRecoveryCommitteeResponse> {
        let caller = caller();
        if !is_controller(&caller) {
            Err(UpgraderApiError::NotController)?
        } else {
            Ok(upgrader_api::GetDisasterRecoveryCommitteeResponse {
                committee: self
                    .disaster_recovery_service
                    .get_committee()
                    .map(Into::into),
            })
        }
    }

    fn get_disaster_recovery_state(
        &self,
    ) -> ApiResult<upgrader_api::GetDisasterRecoveryStateResponse> {
        let caller = caller();
        if !is_controller(&caller) {
            Err(UpgraderApiError::NotController)?
        } else {
            Ok(self.disaster_recovery_service.get_state().into())
        }
    }
}
