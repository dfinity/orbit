use std::sync::Arc;

use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use uuid::Uuid;

use super::{SystemService, UserService, USER_SERVICE};
use crate::{
    core::observer::Observer,
    errors::DisasterRecoveryError,
    models::{Account, User, UserStatus},
    repositories::{AccountRepository, ACCOUNT_REPOSITORY},
    services::SYSTEM_SERVICE,
};
use orbit_essentials::repository::Repository;
lazy_static! {
    pub static ref DISASTER_RECOVERY_SERVICE: Arc<DisasterRecoveryService> =
        Arc::new(DisasterRecoveryService {
            system_service: Arc::clone(&SYSTEM_SERVICE),
            user_service: Arc::clone(&USER_SERVICE),
            account_repository: Arc::clone(&ACCOUNT_REPOSITORY),
        });
}

pub struct DisasterRecoveryService {
    system_service: Arc<SystemService>,
    user_service: Arc<UserService>,
    account_repository: Arc<AccountRepository>,
}

impl DisasterRecoveryService {
    pub async fn sync_accounts(&self) -> ServiceResult<()> {
        let upgrader_canister_id = self.system_service.get_upgrader_canister_id();

        let accounts = self.account_repository.list();

        ic_cdk::call(
            upgrader_canister_id,
            "set_disaster_recovery_accounts",
            (upgrader_api::SetDisasterRecoveryAccountsInput {
                accounts: accounts
                    .iter()
                    .map(|account| upgrader_api::Account {
                        id: Uuid::from_bytes(account.id).hyphenated().to_string(),
                        blockchain: account.blockchain.to_string(),
                        address: account.address.clone(),
                        standard: account.standard.to_string(),
                        symbol: account.symbol.clone(),
                        decimals: account.decimals,
                        name: account.name.clone(),
                        metadata: account.metadata.clone().into(),
                    })
                    .collect(),
            },),
        )
        .await
        .map_err(|(_, err)| DisasterRecoveryError::AccountSyncFailed {
            reason: err.to_string(),
        })?;

        Ok(())
    }

    pub async fn sync_committee(&self) -> ServiceResult<()> {
        let upgrader_canister_id = self.system_service.get_upgrader_canister_id();

        let (users, quorum) = self
            .system_service
            .get_system_info()
            .get_disaster_recovery_committee()
            .map(|committee| {
                (
                    self.user_service
                        .get_active_users_in_groups(&[committee.user_group_id]),
                    committee.quorum,
                )
            })
            .unwrap_or_default();

        ic_cdk::call(
            upgrader_canister_id,
            "set_disaster_recovery_committee",
            (upgrader_api::SetDisasterRecoveryCommitteeInput {
                committee: upgrader_api::DisasterRecoveryCommittee {
                    users: users
                        .iter()
                        .map(|user| upgrader_api::AdminUser {
                            id: Uuid::from_bytes(user.id).hyphenated().to_string(),
                            name: user.name.clone(),
                            identities: user.identities.clone(),
                        })
                        .collect(),
                    quorum,
                },
            },),
        )
        .await
        .map_err(|(_, err)| DisasterRecoveryError::CommitteeSyncFailed {
            reason: err.to_string(),
        })?;

        Ok(())
    }

    pub async fn sync_all(&self) {
        if let Err(error) = DISASTER_RECOVERY_SERVICE.sync_committee().await {
            crate::core::ic_cdk::api::print(format!("Failed to sync committee: {}", error,));
        }
        if let Err(error) = DISASTER_RECOVERY_SERVICE.sync_accounts().await {
            crate::core::ic_cdk::api::print(format!("Failed to sync accounts: {}", error,));
        }
    }
}

pub fn disaster_recovery_observes_insert_user(observer: &mut Observer<(User, Option<User>)>) {
    observer.add_listener(Box::new(|(user, prev)| {
        if !SYSTEM_SERVICE.is_healthy() {
            // Skip syncing committee during system init
            return;
        }

        if let Some(committee) = SYSTEM_SERVICE
            .get_system_info()
            .get_disaster_recovery_committee()
            .cloned()
        {
            let user_was_previously_in_committee = prev
                .as_ref()
                .map(|user| {
                    user.groups.contains(&committee.user_group_id)
                        && user.status == UserStatus::Active
                })
                .unwrap_or(false);

            let user_is_in_committee =
                user.status == UserStatus::Active && user.groups.contains(&committee.user_group_id);

            if user_is_in_committee && !user_was_previously_in_committee
                || !user_is_in_committee && user_was_previously_in_committee
            {
                crate::core::ic_cdk::spawn(async {
                    if let Err(error) = DISASTER_RECOVERY_SERVICE.sync_committee().await {
                        crate::core::ic_cdk::api::print(format!(
                            "Failed to sync committee: {}",
                            error,
                        ));
                    }
                });
            }
        }
    }));
}

pub fn disaster_recovery_observes_remove_user(observer: &mut Observer<User>) {
    observer.add_listener(Box::new(|prev_user| {
        if !SYSTEM_SERVICE.is_healthy() {
            // Skip syncing committee during system init
            return;
        }

        if let Some(committee) = SYSTEM_SERVICE
            .get_system_info()
            .get_disaster_recovery_committee()
            .cloned()
        {
            let user_was_previously_in_committee =
                prev_user.groups.contains(&committee.user_group_id)
                    && prev_user.status == UserStatus::Active;

            if user_was_previously_in_committee {
                crate::core::ic_cdk::spawn(async {
                    if let Err(error) = DISASTER_RECOVERY_SERVICE.sync_committee().await {
                        crate::core::ic_cdk::api::print(format!(
                            "Failed to sync committee: {}",
                            error,
                        ));
                    }
                });
            }
        }
    }));
}

pub fn disaster_recovery_observes_insert_account(
    observer: &mut Observer<(Account, Option<Account>)>,
) {
    observer.add_listener(Box::new(|(_account, _prev)| {
        if !SYSTEM_SERVICE.is_healthy() {
            // Skip syncing accounts during system init
            return;
        }

        crate::core::ic_cdk::spawn(async {
            if let Err(error) = DISASTER_RECOVERY_SERVICE.sync_accounts().await {
                crate::core::ic_cdk::api::print(format!("Failed to sync accounts: {}", error,));
            }
        });
    }));
}
