use std::sync::Arc;

use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use uuid::Uuid;

use super::{SystemService, UserService, USER_SERVICE};
use crate::{
    core::observer::Observer,
    errors::DisasterRecoveryError,
    models::{Account, Asset, User, UserStatus},
    repositories::{
        AccountRepository, AssetRepository, InsertEntryObserverArgs, ACCOUNT_REPOSITORY,
        ASSET_REPOSITORY,
    },
    services::SYSTEM_SERVICE,
};
use orbit_essentials::repository::Repository;
lazy_static! {
    pub static ref DISASTER_RECOVERY_SERVICE: Arc<DisasterRecoveryService> =
        Arc::new(DisasterRecoveryService {
            system_service: Arc::clone(&SYSTEM_SERVICE),
            user_service: Arc::clone(&USER_SERVICE),
            account_repository: Arc::clone(&ACCOUNT_REPOSITORY),
            asset_repository: Arc::clone(&ASSET_REPOSITORY),
        });
}

pub struct DisasterRecoveryService {
    system_service: Arc<SystemService>,
    user_service: Arc<UserService>,
    account_repository: Arc<AccountRepository>,
    asset_repository: Arc<AssetRepository>,
}

impl DisasterRecoveryService {
    pub async fn sync_accounts_and_assets(&self) -> ServiceResult<()> {
        let upgrader_canister_id = self.system_service.get_upgrader_canister_id();

        let accounts = self.account_repository.list();
        let assets = self.asset_repository.list();

        ic_cdk::call(
            upgrader_canister_id,
            "set_disaster_recovery_accounts_and_assets",
            (upgrader_api::SetDisasterRecoveryAccountsAndAssetsInput {
                accounts: accounts.into_iter().map(Into::into).collect(),
                assets: assets.into_iter().map(Into::into).collect(),
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
        if let Err(error) = DISASTER_RECOVERY_SERVICE.sync_accounts_and_assets().await {
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

#[cfg(test)]
thread_local! {
    static SYNC_CALLED: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
}

pub fn disaster_recovery_sync_accounts_and_assets_on_remove(observer: &mut Observer<()>) {
    observer.add_listener(Box::new(|_| {
        if !SYSTEM_SERVICE.is_healthy() {
            // Skip syncing during system init
            return;
        }

        #[cfg(test)]
        SYNC_CALLED.with(|sync_called| {
            *sync_called.borrow_mut() += 1;
        });

        crate::core::ic_cdk::spawn(async {
            if let Err(error) = DISASTER_RECOVERY_SERVICE.sync_accounts_and_assets().await {
                crate::core::ic_cdk::api::print(format!(
                    "Failed to sync accounts and assets: {}",
                    error,
                ));
            }
        });
    }));
}

/// A trait for comparing two values for equality in the context of Disaster Recovery.
/// Two values are considered equal if they are the same when serialized into the format
/// stored by the Upgrader.
pub trait SyncEq {
    fn sync_eq(&self) -> bool;
}

impl SyncEq for InsertEntryObserverArgs<Account> {
    fn sync_eq(&self) -> bool {
        if let Some(prev) = &self.prev {
            let current_synced: upgrader_api::MultiAssetAccount = self.current.clone().into();
            let prev_synced: upgrader_api::MultiAssetAccount = prev.clone().into();

            current_synced == prev_synced
        } else {
            false
        }
    }
}

impl SyncEq for InsertEntryObserverArgs<Asset> {
    fn sync_eq(&self) -> bool {
        if let Some(prev) = &self.prev {
            let current_synced: upgrader_api::Asset = self.current.clone().into();
            let prev_synced: upgrader_api::Asset = prev.clone().into();

            current_synced == prev_synced
        } else {
            false
        }
    }
}

pub fn disaster_recovery_sync_accounts_and_assets_on_insert<T>(observer: &mut Observer<T>)
where
    T: SyncEq,
{
    observer.add_listener(Box::new(|sync_cmp| {
        if !SYSTEM_SERVICE.is_healthy() {
            // Skip syncing during system init
            return;
        }

        if sync_cmp.sync_eq() {
            // Skip syncing if the account or asset hasn't changed
            return;
        }

        #[cfg(test)]
        SYNC_CALLED.with(|sync_called| {
            *sync_called.borrow_mut() += 1;
        });

        crate::core::ic_cdk::spawn(async {
            if let Err(error) = DISASTER_RECOVERY_SERVICE.sync_accounts_and_assets().await {
                crate::core::ic_cdk::api::print(format!(
                    "Failed to sync accounts and assets: {}",
                    error,
                ));
            }
        });
    }));
}

#[cfg(test)]
mod tests {

    use orbit_essentials::{model::ModelKey, repository::Repository};

    use crate::{
        core::test_utils::init_canister_system,
        models::{
            account_test_utils::mock_account, asset_test_utils::mock_asset, AccountAsset,
            AccountBalance,
        },
        repositories::{InsertEntryObserverArgs, ACCOUNT_REPOSITORY, ASSET_REPOSITORY},
        services::SyncEq,
    };

    use super::SYNC_CALLED;

    #[test]
    fn test_account_eq() {
        let prev_account = mock_account();
        let mut current_account = prev_account.clone();

        assert!(!InsertEntryObserverArgs {
            current: current_account.clone(),
            prev: None,
        }
        .sync_eq());

        assert!(InsertEntryObserverArgs {
            current: current_account.clone(),
            prev: Some(prev_account.clone()),
        }
        .sync_eq());

        current_account.assets[0].balance = Some(AccountBalance {
            balance: 1000u64.into(),
            last_modification_timestamp: 1,
        });

        // Account has not changed as far as the sync is concerned
        assert!(InsertEntryObserverArgs {
            current: current_account.clone(),
            prev: Some(prev_account.clone()),
        }
        .sync_eq());

        current_account.assets.push(AccountAsset {
            asset_id: [1; 16],
            balance: None,
        });

        // Account has changed
        assert!(!InsertEntryObserverArgs {
            current: current_account.clone(),
            prev: Some(prev_account.clone()),
        }
        .sync_eq());
    }

    #[test]
    fn test_asset_eq() {
        let prev_asset = mock_asset();
        let mut current_asset = prev_asset.clone();

        assert!(!InsertEntryObserverArgs {
            current: current_asset.clone(),
            prev: None,
        }
        .sync_eq());

        assert!(InsertEntryObserverArgs {
            current: current_asset.clone(),
            prev: Some(prev_asset.clone()),
        }
        .sync_eq());

        current_asset
            .metadata
            .change(crate::models::ChangeMetadata::RemoveKeys(vec![
                "index_canister_id".to_string(),
            ]));

        // Asset has changed
        assert!(!InsertEntryObserverArgs {
            current: current_asset.clone(),
            prev: Some(prev_asset.clone()),
        }
        .sync_eq());
    }

    #[test]
    fn test_sync_call() {
        init_canister_system();

        let mut asset = mock_asset();
        ASSET_REPOSITORY.insert(asset.key(), asset.clone());
        assert_eq!(SYNC_CALLED.with(|sync_called| *sync_called.borrow()), 1);

        let mut account = mock_account();
        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());
        assert_eq!(SYNC_CALLED.with(|sync_called| *sync_called.borrow()), 2);

        account.assets[0].balance = Some(AccountBalance {
            balance: 1000u64.into(),
            last_modification_timestamp: 1,
        });
        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());
        // Account has not changed as far as the sync is concerned
        assert_eq!(SYNC_CALLED.with(|sync_called| *sync_called.borrow()), 2);

        account.assets.push(AccountAsset {
            asset_id: [1; 16],
            balance: None,
        });
        ACCOUNT_REPOSITORY.insert(account.to_key(), account.clone());
        // Account has changed
        assert_eq!(SYNC_CALLED.with(|sync_called| *sync_called.borrow()), 3);

        asset
            .metadata
            .change(crate::models::ChangeMetadata::RemoveKeys(vec![
                "index_canister_id".to_string(),
            ]));
        ASSET_REPOSITORY.insert(asset.key(), asset.clone());
        // Asset has changed
        assert_eq!(SYNC_CALLED.with(|sync_called| *sync_called.borrow()), 4);

        ASSET_REPOSITORY.remove(&asset.key());
        assert_eq!(SYNC_CALLED.with(|sync_called| *sync_called.borrow()), 5);
    }
}
