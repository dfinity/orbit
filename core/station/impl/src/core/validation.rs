use std::{
    fmt::{Display, Formatter},
    hash::Hash,
    sync::Arc,
};

#[cfg(test)]
use std::cell::RefCell;

use crate::{
    errors::{
        ExternalCanisterValidationError, FieldValidationError, RecordValidationError,
        ValidationError,
    },
    models::{
        resource::{Resource, ResourceId, ResourceIds},
        AccountKey, AddressBookEntryKey, NamedRuleKey, NotificationKey, RequestKey, TokenStandard,
        UserKey,
    },
    repositories::{
        permission::PERMISSION_REPOSITORY, request_policy::REQUEST_POLICY_REPOSITORY,
        ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY, ASSET_REPOSITORY, NAMED_RULE_REPOSITORY,
        NOTIFICATION_REPOSITORY, REQUEST_REPOSITORY, USER_GROUP_REPOSITORY, USER_REPOSITORY,
    },
    services::SYSTEM_SERVICE,
};
use candid::Principal;
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

        !(is_ledger_canister_id
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

pub trait ValidateField<T>: Send + Sync {
    fn validate_field(&self, value: &T) -> Result<(), ValidationError>;
}

#[derive(Clone)]
pub enum StringCharacterSet {
    Alphanumeric,
}

impl StringCharacterSet {
    pub fn validate(&self, str: &str) -> bool {
        match self {
            StringCharacterSet::Alphanumeric => str.chars().all(|c| c.is_alphanumeric()),
        }
    }
}

#[derive(Clone)]
pub struct StringFieldValidator {
    field_name: String,
    min_length: Option<usize>,
    max_length: Option<usize>,
    char_set: Option<StringCharacterSet>,
}

pub struct StringFieldValidatorBuilder {
    validator: StringFieldValidator,
}

impl Display for StringCharacterSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StringCharacterSet::Alphanumeric => write!(f, "alphanumeric"),
        }
    }
}

impl StringFieldValidatorBuilder {
    pub fn new(field_name: String) -> Self {
        Self {
            validator: StringFieldValidator {
                field_name,
                min_length: None,
                max_length: None,
                char_set: None,
            },
        }
    }

    pub fn min_length(mut self, min_length: usize) -> Self {
        self.validator.min_length = Some(min_length);
        self
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.validator.max_length = Some(max_length);
        self
    }

    pub fn char_set(mut self, char_set: StringCharacterSet) -> Self {
        self.validator.char_set = Some(char_set);
        self
    }

    pub fn build(self) -> StringFieldValidator {
        self.validator
    }
}

impl ValidateField<String> for StringFieldValidator {
    fn validate_field(&self, value: &String) -> Result<(), ValidationError> {
        if let Some(min_length) = self.min_length {
            if value.len() < min_length {
                return Err(ValidationError::FieldValidationError(
                    FieldValidationError::InvalidRecord {
                        field_name: self.field_name.clone(),
                        error: format!("Length cannot be shorter than {}", min_length),
                    },
                ));
            }
        }

        if let Some(max_length) = self.max_length {
            if value.len() > max_length {
                return Err(ValidationError::FieldValidationError(
                    FieldValidationError::InvalidRecord {
                        field_name: self.field_name.clone(),
                        error: format!("Length cannot be longer than {}", max_length),
                    },
                ));
            }
        }

        if let Some(char_set) = &self.char_set {
            if !char_set.validate(value) {
                return Err(ValidationError::FieldValidationError(
                    FieldValidationError::InvalidRecord {
                        field_name: self.field_name.clone(),
                        error: format!("Allowed characters: {}", char_set),
                    },
                ));
            }
        }

        Ok(())
    }
}

pub struct NumberFieldValidatorBuilder<T: PartialOrd + Display> {
    validator: NumberFieldValidator<T>,
}

impl<T: PartialOrd + Display> NumberFieldValidatorBuilder<T> {
    pub fn new(field_name: String) -> Self {
        Self {
            validator: NumberFieldValidator::new(field_name),
        }
    }

    pub fn min(mut self, min: T) -> Self {
        self.validator.min = Some(min);
        self
    }

    pub fn max(mut self, max: T) -> Self {
        self.validator.max = Some(max);
        self
    }

    pub fn build(self) -> NumberFieldValidator<T> {
        self.validator
    }
}

pub struct NumberFieldValidator<T: PartialOrd + Display> {
    field_name: String,
    min: Option<T>,
    max: Option<T>,
}

impl<T: PartialOrd + Display> NumberFieldValidator<T> {
    pub fn new(field_name: String) -> Self {
        Self {
            field_name,
            min: None,
            max: None,
        }
    }

    pub fn validate_field(&self, value: T) -> Result<(), ValidationError> {
        if let Some(min) = &self.min {
            if &value < min {
                return Err(ValidationError::FieldValidationError(
                    FieldValidationError::InvalidRecord {
                        field_name: self.field_name.clone(),
                        error: format!("Cannot be less than {}", min),
                    },
                ));
            }
        }

        if let Some(max) = &self.max {
            if &value > max {
                return Err(ValidationError::FieldValidationError(
                    FieldValidationError::InvalidRecord {
                        field_name: self.field_name.clone(),
                        error: format!("Cannot be greater than {}", max),
                    },
                ));
            }
        }

        Ok(())
    }
}

pub struct VecFieldValidator<T> {
    field_name: String,
    min_length: Option<usize>,
    max_length: Option<usize>,
    item_validator: Arc<dyn ValidateField<T>>,
}

pub struct VecFieldValidatorBuilder<T> {
    validator: VecFieldValidator<T>,
}

impl<T> VecFieldValidator<T> {
    pub fn new(field_name: String, item_validator: Arc<dyn ValidateField<T>>) -> Self {
        Self {
            field_name,
            min_length: None,
            max_length: None,
            item_validator,
        }
    }

    pub fn validate_field(&self, value: &Vec<T>) -> Result<(), ValidationError> {
        if let Some(min_length) = self.min_length {
            if value.len() < min_length {
                return Err(ValidationError::FieldValidationError(
                    FieldValidationError::InvalidRecord {
                        field_name: self.field_name.clone(),
                        error: format!("Cannot have fewer than {} items", min_length),
                    },
                ));
            }
        }

        if let Some(max_length) = self.max_length {
            if value.len() > max_length {
                return Err(ValidationError::FieldValidationError(
                    FieldValidationError::InvalidRecord {
                        field_name: self.field_name.clone(),
                        error: format!("Cannot have more than {} items", max_length),
                    },
                ));
            }
        }

        for item in value {
            self.item_validator.validate_field(item)?;
        }

        Ok(())
    }
}

impl<T> VecFieldValidatorBuilder<T> {
    pub fn new(field_name: String, item_validator: Arc<dyn ValidateField<T>>) -> Self {
        Self {
            validator: VecFieldValidator::new(field_name, item_validator),
        }
    }

    pub fn min_length(mut self, min_length: usize) -> Self {
        self.validator.min_length = Some(min_length);
        self
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.validator.max_length = Some(max_length);
        self
    }

    pub fn build(self) -> VecFieldValidator<T> {
        self.validator
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use candid::Principal;
    use orbit_essentials::{model::ModelKey, repository::Repository};

    use crate::{
        core::test_utils::init_canister_system,
        models::{asset_test_utils::mock_asset, TokenStandard},
        repositories::ASSET_REPOSITORY,
    };

    use super::EnsureExternalCanister;

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
}
