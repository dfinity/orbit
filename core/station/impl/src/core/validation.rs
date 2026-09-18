use std::{hash::Hash, sync::Arc};

#[cfg(test)]
use std::cell::RefCell;

use crate::{
    errors::{AssetValidationError, ExternalCanisterValidationError, RecordValidationError},
    models::{
        resource::{Resource, ResourceId, ResourceIds},
        AccountKey, AddressBookEntryKey, AssetId, ChangeMetadata, NamedRuleKey, NotificationKey,
        RequestKey, TokenStandard, UserKey,
    },
    repositories::{
        permission::PERMISSION_REPOSITORY, request_policy::REQUEST_POLICY_REPOSITORY,
        ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY, ASSET_REPOSITORY, NAMED_RULE_REPOSITORY,
        NOTIFICATION_REPOSITORY, REQUEST_REPOSITORY, USER_GROUP_REPOSITORY, USER_REPOSITORY,
    },
    services::{MAINNET_CYCLES_LEDGER_CANISTER_ID, SYSTEM_SERVICE},
};
use candid::Principal;
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use ic_stable_structures::{Memory, Storable};

use orbit_essentials::repository::Repository;
use orbit_essentials::types::UUID;
use uuid::Uuid;

thread_local! {
  /// Switch for tests to enable validation if needed.
  #[cfg(test)]
  static MOCK_RESOURCE_VALIDATION_ON: RefCell<bool> = const { RefCell::new(true) };
}

#[cfg(test)]
pub fn disable_mock_resource_validation() {
    MOCK_RESOURCE_VALIDATION_ON.with(|v| *v.borrow_mut() = false);
}

#[cfg(test)]
pub fn enable_mock_resource_validation() {
    MOCK_RESOURCE_VALIDATION_ON.with(|v| *v.borrow_mut() = true);
}

fn ensure_entry_exists<K, V, M, R>(repository: Arc<R>, key: K) -> Option<()>
where
    R: Repository<K, V, M>,
    K: Hash + Clone + Eq + Ord + Storable,
    V: Clone + Storable,
    M: Memory,
{
    #[cfg(test)]
    if MOCK_RESOURCE_VALIDATION_ON.with(|v| *v.borrow()) {
        return Some(());
    }

    repository.get(&key).map(|_| ())
}

pub trait EnsureIdExists<Key> {
    fn id_exists(id: &Key) -> Result<(), RecordValidationError>;

    fn id_list_exists(ids: &[Key]) -> Result<(), RecordValidationError> {
        for id in ids {
            Self::id_exists(id)?;
        }
        Ok(())
    }
}

pub trait EnsureResourceIdExists: EnsureIdExists<UUID> {
    fn resource_id_exists(resource_id: &ResourceId) -> Result<(), RecordValidationError> {
        match resource_id {
            ResourceId::Any => Ok(()),
            ResourceId::Id(id) => Self::id_exists(id),
        }
    }
    fn resource_ids_exist(resource_ids: &ResourceIds) -> Result<(), RecordValidationError> {
        match resource_ids {
            ResourceIds::Any => Ok(()),
            ResourceIds::Ids(ids) => {
                Self::id_list_exists(ids)?;
                Ok(())
            }
        }
    }
}

pub struct EnsureUser {}

impl EnsureIdExists<UUID> for EnsureUser {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(USER_REPOSITORY.to_owned(), UserKey { id: *id }).ok_or(
            RecordValidationError::NotFound {
                model_name: "User".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureUser {}

pub struct EnsureUserGroup {}

impl EnsureIdExists<UUID> for EnsureUserGroup {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(USER_GROUP_REPOSITORY.to_owned(), *id).ok_or(
            RecordValidationError::NotFound {
                model_name: "UserGroup".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureUserGroup {}

pub struct EnsureAccount {}

impl EnsureIdExists<UUID> for EnsureAccount {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(ACCOUNT_REPOSITORY.to_owned(), AccountKey { id: *id }).ok_or(
            RecordValidationError::NotFound {
                model_name: "Account".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureAccount {}

pub struct EnsureAddressBookEntry {}

impl EnsureIdExists<UUID> for EnsureAddressBookEntry {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(
            ADDRESS_BOOK_REPOSITORY.to_owned(),
            AddressBookEntryKey { id: *id },
        )
        .ok_or(RecordValidationError::NotFound {
            model_name: "AddressBookEntry".to_string(),
            id: Uuid::from_bytes(*id).hyphenated().to_string(),
        })
    }
}

impl EnsureResourceIdExists for EnsureAddressBookEntry {}

pub struct EnsureRequest {}

impl EnsureIdExists<UUID> for EnsureRequest {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(REQUEST_REPOSITORY.to_owned(), RequestKey { id: *id }).ok_or(
            RecordValidationError::NotFound {
                model_name: "Request".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureRequest {}

pub struct EnsureRequestPolicy {}

impl EnsureIdExists<UUID> for EnsureRequestPolicy {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(REQUEST_POLICY_REPOSITORY.to_owned(), *id).ok_or(
            RecordValidationError::NotFound {
                model_name: "RequestPolicy".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureRequestPolicy {}

pub struct EnsurePermission {}

impl EnsureIdExists<Resource> for EnsurePermission {
    fn id_exists(key: &Resource) -> Result<(), RecordValidationError> {
        ensure_entry_exists(PERMISSION_REPOSITORY.to_owned(), key.to_owned()).ok_or(
            RecordValidationError::NotFound {
                model_name: "Permission".to_string(),
                id: key.to_string(),
            },
        )
    }
}

pub struct EnsureExternalCanister {}

impl EnsureExternalCanister {
    // Known ledger canisters, the management canister, the orbit station, and the upgrader are NOT external canisters.
    pub fn is_external_canister(principal: Principal) -> bool {
        // Check if the target canister is a ledger canister of an asset.
        let principal_str = principal.to_text();
        let is_ledger_canister_id = ASSET_REPOSITORY.list().iter().any(|asset| {
            asset
                .metadata
                .get(TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID)
                .is_some_and(|canister_id| canister_id == principal_str)
        });

        // The asset-derived check above only covers ledgers that some asset currently points at,
        // so it stops covering a ledger once its last asset is edited or removed. The system
        // ledgers hold station funds directly, so they are pinned here and never depend on what
        // the asset registry happens to contain.
        !(is_ledger_canister_id
            || principal == MAINNET_LEDGER_CANISTER_ID
            || principal == MAINNET_CYCLES_LEDGER_CANISTER_ID
            || principal == Principal::management_canister()
            || principal == crate::core::ic_cdk::api::id()
            || principal == SYSTEM_SERVICE.get_upgrader_canister_id())
    }

    pub fn ensure_external_canister(
        principal: Principal,
    ) -> Result<(), ExternalCanisterValidationError> {
        if !Self::is_external_canister(principal) {
            return Err(ExternalCanisterValidationError::InvalidExternalCanister { principal });
        }

        Ok(())
    }
}

pub struct EnsureNotification {}

impl EnsureIdExists<UUID> for EnsureNotification {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(
            NOTIFICATION_REPOSITORY.to_owned(),
            NotificationKey { id: *id },
        )
        .ok_or(RecordValidationError::NotFound {
            model_name: "Notification".to_string(),
            id: Uuid::from_bytes(*id).hyphenated().to_string(),
        })
    }
}

impl EnsureResourceIdExists for EnsureNotification {}

pub struct EnsureAsset {}

impl EnsureIdExists<UUID> for EnsureAsset {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(ASSET_REPOSITORY.to_owned(), *id).ok_or(
            RecordValidationError::NotFound {
                model_name: "Asset".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureAsset {}

impl EnsureAsset {
    /// The ledger canister id is resolved at call time by transfers, balance reads and fee
    /// lookups, so repointing it on an existing asset silently changes which token those
    /// operations act on. Rejecting it here keeps a doomed request from being created and
    /// approved before the service refuses it at execution.
    pub fn ledger_canister_id_preserved(
        asset_id: &AssetId,
        change_metadata: &Option<ChangeMetadata>,
    ) -> Result<(), AssetValidationError> {
        let Some(change_metadata) = change_metadata else {
            return Ok(());
        };

        let Some(asset) = ASSET_REPOSITORY.get(asset_id) else {
            return Ok(());
        };

        if asset.changes_ledger_canister_id(change_metadata) {
            return Err(AssetValidationError::ImmutableField {
                field: TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
            });
        }

        Ok(())
    }
}

pub struct EnsureNamedRule {}

impl EnsureIdExists<UUID> for EnsureNamedRule {
    fn id_exists(id: &UUID) -> Result<(), RecordValidationError> {
        ensure_entry_exists(NAMED_RULE_REPOSITORY.to_owned(), NamedRuleKey { id: *id }).ok_or(
            RecordValidationError::NotFound {
                model_name: "NamedRule".to_string(),
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )
    }
}

impl EnsureResourceIdExists for EnsureNamedRule {}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use candid::Principal;
    use orbit_essentials::{model::ModelKey, repository::Repository};

    use crate::{
        core::test_utils::init_canister_system,
        models::{asset_test_utils::mock_asset, ChangeMetadata, TokenStandard},
        repositories::ASSET_REPOSITORY,
    };

    use super::{EnsureAsset, EnsureExternalCanister, MAINNET_CYCLES_LEDGER_CANISTER_ID};
    use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;

    fn insert_asset_with_ledger(ledger: Principal) -> crate::models::Asset {
        let mut asset = mock_asset();
        asset
            .metadata
            .change(ChangeMetadata::OverrideSpecifiedBy(BTreeMap::from([(
                TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
                ledger.to_text(),
            )])));

        ASSET_REPOSITORY.insert(asset.key(), asset.clone());

        asset
    }

    #[test]
    fn test_is_external_canister() {
        init_canister_system();

        let principal = Principal::from_slice(&[1; 29]);

        let is_external_canister = EnsureExternalCanister::is_external_canister(principal);
        assert!(is_external_canister);
        let ensure_external_canister = EnsureExternalCanister::ensure_external_canister(principal);
        assert!(ensure_external_canister.is_ok());

        let mut asset = mock_asset();

        asset
            .metadata
            .change(crate::models::ChangeMetadata::OverrideSpecifiedBy(
                BTreeMap::from([(
                    TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
                    principal.to_text(),
                )]),
            ));

        ASSET_REPOSITORY.insert(asset.key(), asset);

        let is_external_canister = EnsureExternalCanister::is_external_canister(principal);
        assert!(!is_external_canister);
        let ensure_external_canister = EnsureExternalCanister::ensure_external_canister(principal);
        assert!(ensure_external_canister.is_err());
    }

    #[test]
    fn test_system_ledgers_are_never_external_canisters() {
        init_canister_system();

        // These hold station funds directly, so they stay off limits even when no asset points at
        // them, which is the case after the last asset naming them is edited or removed.
        for ledger in [
            MAINNET_LEDGER_CANISTER_ID,
            MAINNET_CYCLES_LEDGER_CANISTER_ID,
        ] {
            assert!(!EnsureExternalCanister::is_external_canister(ledger));
            assert!(EnsureExternalCanister::ensure_external_canister(ledger).is_err());
        }
    }

    #[test]
    fn test_edit_can_set_initial_ledger_canister_id() {
        init_canister_system();

        let asset = crate::models::asset_test_utils::mock_asset_b();
        assert!(asset.ledger_canister_id().is_none());
        ASSET_REPOSITORY.insert(asset.key(), asset.clone());

        let result = EnsureAsset::ledger_canister_id_preserved(
            &asset.id,
            &Some(ChangeMetadata::OverrideSpecifiedBy(BTreeMap::from([(
                TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
                Principal::from_slice(&[3; 29]).to_text(),
            )]))),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_edit_cannot_repoint_ledger_canister_id() {
        init_canister_system();

        let asset = insert_asset_with_ledger(Principal::from_slice(&[1; 29]));

        let result = EnsureAsset::ledger_canister_id_preserved(
            &asset.id,
            &Some(ChangeMetadata::OverrideSpecifiedBy(BTreeMap::from([(
                TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
                Principal::from_slice(&[2; 29]).to_text(),
            )]))),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_edit_cannot_drop_ledger_canister_id() {
        init_canister_system();

        let asset = insert_asset_with_ledger(Principal::from_slice(&[1; 29]));

        let result = EnsureAsset::ledger_canister_id_preserved(
            &asset.id,
            &Some(ChangeMetadata::RemoveKeys(vec![
                TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
            ])),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_edit_allows_unrelated_metadata_changes() {
        init_canister_system();

        let ledger = Principal::from_slice(&[1; 29]);
        let asset = insert_asset_with_ledger(ledger);

        let result = EnsureAsset::ledger_canister_id_preserved(
            &asset.id,
            &Some(ChangeMetadata::OverrideSpecifiedBy(BTreeMap::from([
                ("description".to_string(), "updated".to_string()),
                (
                    TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
                    ledger.to_text(),
                ),
            ]))),
        );

        assert!(result.is_ok());
    }
}
