use std::sync::Arc;

use crate::upgrader_ic_cdk::{api::is_controller, caller};
use crate::{
    errors::UpgraderApiError,
    services::{DisasterRecoveryService, DISASTER_RECOVERY_SERVICE},
};
use candid::Principal;
use ic_cdk::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;

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

    fn can_query_state(&self, caller: &Principal) -> bool {
        is_controller(caller) || self.disaster_recovery_service.is_committee_member(caller)
    }

    fn get_disaster_recovery_accounts(
        &self,
    ) -> ApiResult<upgrader_api::GetDisasterRecoveryAccountsResponse> {
        let caller = caller();
        if !self.can_query_state(&caller) {
            Err(UpgraderApiError::Unauthorized)?
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

    fn get_disaster_recovery_committee(
        &self,
    ) -> ApiResult<upgrader_api::GetDisasterRecoveryCommitteeResponse> {
        let caller = caller();
        if !self.can_query_state(&caller) {
            Err(UpgraderApiError::Unauthorized)?
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
        if !self.can_query_state(&caller) {
            Err(UpgraderApiError::Unauthorized)?
        } else {
            Ok(self.disaster_recovery_service.get_state().into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::tests::mock_non_committee_member;
    use crate::upgrader_ic_cdk::set_caller;
    use crate::{
        model::tests::{mock_committee, mock_committee_member},
        upgrader_ic_cdk::TEST_CONTROLLER_ID,
    };
    use candid::Principal;

    use super::CONTROLLER;
    use crate::services::DISASTER_RECOVERY_SERVICE;

    #[test]
    fn committee_members_and_controllers_are_authorized() {
        let member_principal = mock_committee_member();

        DISASTER_RECOVERY_SERVICE
            .set_committee(mock_committee())
            .expect("set committee should succeed");

        assert!(CONTROLLER.can_query_state(&member_principal));
        assert!(CONTROLLER.can_query_state(&TEST_CONTROLLER_ID));

        assert!(!CONTROLLER.can_query_state(&Principal::anonymous()));
        assert!(!CONTROLLER.can_query_state(&Principal::from_slice(&[128; 29])));
    }

    #[test]
    fn authorized_callers_can_query_state() {
        let member_principal = mock_committee_member();

        DISASTER_RECOVERY_SERVICE
            .set_committee(mock_committee())
            .expect("set committee should succeed");

        set_caller(member_principal);
        assert!(CONTROLLER.get_disaster_recovery_accounts().is_ok());
    }

    #[test]
    fn unauthorized_callers_cannot_query_state() {
        DISASTER_RECOVERY_SERVICE
            .set_committee(mock_committee())
            .expect("set committee should succeed");

        set_caller(mock_non_committee_member());
        assert!(CONTROLLER.get_disaster_recovery_accounts().is_err());
    }
}
