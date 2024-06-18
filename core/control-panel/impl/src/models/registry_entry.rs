use super::artifact::ArtifactId;
use crate::repositories::{RegistryWhere, REGISTRY_REPOSITORY};
use crate::{core::ic_cdk::next_time, errors::RegistryError};
use orbit_essentials::model::{ModelValidator, ModelValidatorResult};
use orbit_essentials::repository::Repository;
use orbit_essentials::storable;
use orbit_essentials::types::{Timestamp, UUID};
use std::collections::{BTreeMap, HashSet};
use std::fmt::Display;
use uuid::Uuid;

/// The latest tag, which is used to tag the latest version of the entry if applicable.
pub const LATEST_TAG: &str = "latest";

/// The entry id in the registry, which is a UUID.
pub type RegistryEntryId = UUID;

/// The registry is a record that is stored in the registry repository.
///
/// It stores entries about wasm modules and can be extended to other entry types. When adding new entry types,
/// the `RegistryValue` enum should be updated to include the new entry type.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RegistryEntry {
    /// The UUID that identifies the entry in the registry.
    pub id: RegistryEntryId,
    /// The namespace of the entry.
    ///
    /// Restrictions:
    ///
    /// - Namespaces must be between 2 and 32 characters long.
    /// - Namespaces can only contain lowercase letters, numbers, and hyphens.
    pub namespace: String,
    /// The name of the entry, which is used to identify it (e.g. station). Within each namespace
    /// the name should refer to the same type of entry, but many entries can exist with the same name.
    ///
    /// e.g. if the namespace is "@orbit" and the name is "station", then all the entries will refer to a wasm module.
    ///
    /// Restrictions:
    ///
    /// - Names must be between 2 and 48 characters long.
    /// - Names that are not namespaced, are put in the default namespace `default`.
    pub name: String,
    /// The description of the entry, which is a human-readable description of the entry.
    ///
    /// Restrictions:
    ///
    /// - Descriptions must be between 24 and 512 characters long.
    pub description: String,
    /// The tags are used to tag the entry with specific search terms (e.g. "latest", "stable").
    ///
    /// Tags are grouped within the same `namespace/name` (e.g. "@orbit/station").
    ///
    /// Restrictions:
    ///
    /// - Tags must be between 2 and 32 characters long.
    /// - There can be up to 10 tags per entry.
    pub tags: Vec<String>,
    /// The category is used to associate the entry with a specific category (e.g. "chain-fusion")
    /// across all the entries in the registry.
    ///
    /// Restrictions:
    ///
    /// - Categories must be between 2 and 32 characters long.
    /// - There can be up to 10 categories per entry.
    pub categories: Vec<String>,
    /// The content of the entry in the registry, which can be a Wasm module.
    pub value: RegistryValue,
    /// The timestamp when the entry was created.
    pub created_at: Timestamp,
    /// The timestamp when the entry was last updated.
    pub updated_at: Option<Timestamp>,
    /// The metadata of the entry in the registry.
    ///
    /// This is a key-value map that can be used to store additional information about the entry,
    /// such as the author, license, repository, docs, etc.
    ///
    /// Restrictions:
    ///
    /// - The key must be between 1 and 32 characters long.
    /// - The value must be between 1 and 512 characters long.
    /// - There can be up to 10 metadata entries per entry in the registry.
    pub metadata: BTreeMap<String, String>,
}

/// The registry value, which is the content of the registry.
///
/// When adding new entry types to the registry, if the new entry contains artifacts, then the artifact repository
/// should be used to store them efficiently.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RegistryValue {
    WasmModule(WasmModuleRegistryValue),
}

/// The registry type, which is the type of the registry.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum RegistryValueKind {
    WasmModule = 1,
}

/// The wasm module registry value, which is the content of the wasm module and its version.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WasmModuleRegistryValue {
    /// The id of the wasm module that is stored in the artifact repository.
    pub wasm_artifact_id: ArtifactId,
    /// The version of the wasm module.
    ///
    /// Restrictions:
    ///
    /// - Versions must be between 1 and 32 characters long.
    pub version: String,
    /// The dependencies of the wasm module, which are other wasm modules that this wasm module depends on.
    ///
    /// Restrictions:
    ///
    /// - There can be up to 25 dependencies per wasm module.
    pub dependencies: Vec<WasmModuleRegistryEntryDependency>,
}

#[storable]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WasmModuleRegistryEntryDependency {
    pub name: String,
    pub version: String,
}

impl Display for RegistryValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryValueKind::WasmModule => write!(f, "WasmModule"),
        }
    }
}

impl Default for RegistryEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryEntry {
    pub const NAMESPACE_PREFIX: &'static str = "@";
    pub const DEFAULT_NAMESPACE: &'static str = "default";

    pub const MIN_NAMESPACE_LENGTH: usize = 2;
    pub const MAX_NAMESPACE_LENGTH: usize = 32;

    pub const MIN_NAME_LENGTH: usize = 2;
    pub const MAX_NAME_LENGTH: usize = 48;

    pub const MIN_DESCRIPTION_LENGTH: usize = 24;
    pub const MAX_DESCRIPTION_LENGTH: usize = 512;

    pub const MAX_METADATA_ENTRIES: usize = 10;
    pub const MIN_METADATA_KEY_LENGTH: usize = 1;
    pub const MAX_METADATA_KEY_LENGTH: usize = 32;
    pub const MIN_METADATA_VALUE_LENGTH: usize = 1;
    pub const MAX_METADATA_VALUE_LENGTH: usize = 512;

    pub const MAX_CATEGORIES: usize = 10;
    pub const MIN_CATEGORY_LENGTH: usize = 2;
    pub const MAX_CATEGORY_LENGTH: usize = 32;

    pub const MAX_TAGS: usize = 10;
    pub const MIN_TAG_LENGTH: usize = 2;
    pub const MAX_TAG_LENGTH: usize = 32;

    /// Creates a new registry entry with a random id and default values.
    ///
    /// The value of the entry is a wasm module with a random id and default values.
    pub fn new() -> Self {
        Self {
            id: *Uuid::new_v4().as_bytes(),
            namespace: Self::DEFAULT_NAMESPACE.to_string(),
            name: Default::default(),
            description: Default::default(),
            tags: Default::default(),
            categories: Default::default(),
            value: RegistryValue::WasmModule(WasmModuleRegistryValue {
                wasm_artifact_id: [0; 16],
                version: Default::default(),
                dependencies: Default::default(),
            }),
            metadata: Default::default(),
            created_at: next_time(),
            updated_at: None,
        }
    }

    /// Returns the full name of the entry, which is the namespace and the name separated by a `/`.
    pub fn fullname(&self) -> String {
        format!("{}{}/{}", Self::NAMESPACE_PREFIX, self.namespace, self.name)
    }

    pub fn to_kind(&self) -> RegistryValueKind {
        match &self.value {
            RegistryValue::WasmModule(_) => RegistryValueKind::WasmModule,
        }
    }
}

impl WasmModuleRegistryValue {
    pub const MIN_VERSION_LENGTH: usize = 1;
    pub const MAX_VERSION_LENGTH: usize = 32;

    pub const MAX_DEPENDENCIES: usize = 25;
}

fn validate_wasm_module_version(version: &str) -> ModelValidatorResult<RegistryError> {
    if (version.len() < WasmModuleRegistryValue::MIN_VERSION_LENGTH)
        || (version.len() > WasmModuleRegistryValue::MAX_VERSION_LENGTH)
    {
        return Err(RegistryError::ValidationError {
            info: format!(
                "Version length must be between {} and {}",
                WasmModuleRegistryValue::MIN_VERSION_LENGTH,
                WasmModuleRegistryValue::MAX_VERSION_LENGTH,
            ),
        });
    }

    Ok(())
}

fn validate_uniqueness(entry: &RegistryEntry) -> ModelValidatorResult<RegistryError> {
    match &entry.value {
        RegistryValue::WasmModule(value) => {
            let mut ids = REGISTRY_REPOSITORY.find_ids_where(
                RegistryWhere::clause()
                    .and_fullname(&entry.fullname())
                    .and_version(&value.version),
                None,
            );

            ids.retain(|id| id != &entry.id);

            if !ids.is_empty() {
                return Err(RegistryError::ValidationError {
                    info: format!(
                        "The entry is a duplicate of {}",
                        Uuid::from_bytes(ids[0]).hyphenated()
                    ),
                });
            }
        }
    }

    Ok(())
}

fn validate_is_of_same_kind(entry: &RegistryEntry) -> ModelValidatorResult<RegistryError> {
    let entries_of_kind = REGISTRY_REPOSITORY.find_ids_where(
        RegistryWhere::clause()
            .and_fullname(&entry.fullname())
            .and_kind(entry.to_kind()),
        None,
    );

    let all_entries = REGISTRY_REPOSITORY.find_ids_where(
        RegistryWhere::clause().and_fullname(&entry.fullname()),
        None,
    );

    if entries_of_kind.len() != all_entries.len() {
        return Err(RegistryError::UpdateKindMismatch {
            kind: entry.to_kind().to_string(),
        });
    }

    Ok(())
}

/// Performs a depth-first search to check for circular dependencies in the entry.
///
/// The function returns an error if a circular dependency is detected.
fn dfs_check_entry_dependencies(
    entry: &RegistryEntry,
    visited: &mut HashSet<RegistryEntryId>,
) -> ModelValidatorResult<RegistryError> {
    if visited.contains(&entry.id) {
        return Err(RegistryError::ValidationError {
            info: "Circular dependency detected".to_string(),
        });
    }

    visited.insert(entry.id);

    match &entry.value {
        RegistryValue::WasmModule(value) => {
            for dependency in value.dependencies.iter() {
                let found = REGISTRY_REPOSITORY.find_ids_where(
                    RegistryWhere::clause()
                        .and_fullname(&dependency.name)
                        .and_version(&dependency.version)
                        .and_kind(RegistryValueKind::WasmModule),
                    None,
                );

                if found.is_empty() {
                    // The dependency is not found in the registry and can be ignored.
                    continue;
                }

                if let Some(dependency_entry) = REGISTRY_REPOSITORY.get(&found[0]) {
                    dfs_check_entry_dependencies(&dependency_entry, visited)?;
                }
            }
        }
    }

    Ok(())
}

fn validate_dependencies(entry: &RegistryEntry) -> ModelValidatorResult<RegistryError> {
    match &entry.value {
        RegistryValue::WasmModule(value) => {
            if value.dependencies.len() > WasmModuleRegistryValue::MAX_DEPENDENCIES {
                return Err(RegistryError::ValidationError {
                    info: format!(
                        "Too many dependencies, expected at most {} but got {}",
                        WasmModuleRegistryValue::MAX_DEPENDENCIES,
                        value.dependencies.len()
                    ),
                });
            }

            for dependency in value.dependencies.iter() {
                let found = REGISTRY_REPOSITORY.find_ids_where(
                    RegistryWhere::clause()
                        .and_fullname(&dependency.name)
                        .and_version(&dependency.version)
                        .and_kind(RegistryValueKind::WasmModule),
                    None,
                );

                if found.is_empty() {
                    return Err(RegistryError::ValidationError {
                        info: format!("The dependency {} is not valid", dependency.name),
                    });
                }
            }
        }
    }

    // Check for circular dependencies
    dfs_check_entry_dependencies(entry, &mut HashSet::new())?;

    Ok(())
}

impl ModelValidator<RegistryError> for WasmModuleRegistryValue {
    fn validate(&self) -> ModelValidatorResult<RegistryError> {
        validate_wasm_module_version(&self.version)?;

        Ok(())
    }
}

fn validate_chars(field: &str, content: &str) -> ModelValidatorResult<RegistryError> {
    if !content
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_numeric() || c == '-')
    {
        return Err(RegistryError::ValidationError {
            info: format!(
                "{} can only contain lowercase letters, numbers, and hyphens",
                field
            ),
        });
    }

    if content.starts_with('-') || content.ends_with('-') {
        return Err(RegistryError::ValidationError {
            info: format!("{} cannot start or end with a hyphen", field),
        });
    }

    Ok(())
}

fn validate_name(name: &str) -> ModelValidatorResult<RegistryError> {
    if name.is_empty() {
        return Err(RegistryError::ValidationError {
            info: "Name cannot be empty".to_string(),
        });
    }

    if (name.len() < RegistryEntry::MIN_NAME_LENGTH)
        || (name.len() > RegistryEntry::MAX_NAME_LENGTH)
    {
        return Err(RegistryError::ValidationError {
            info: format!(
                "Name length must be between {} and {}",
                RegistryEntry::MIN_NAME_LENGTH,
                RegistryEntry::MAX_NAME_LENGTH,
            ),
        });
    }

    validate_chars("Name", name)?;

    Ok(())
}

fn validate_namespace(namespace: &str) -> ModelValidatorResult<RegistryError> {
    if namespace.is_empty() {
        return Err(RegistryError::ValidationError {
            info: "Namespace cannot be empty".to_string(),
        });
    }

    if (namespace.len() < RegistryEntry::MIN_NAMESPACE_LENGTH)
        || (namespace.len() > RegistryEntry::MAX_NAMESPACE_LENGTH)
    {
        return Err(RegistryError::ValidationError {
            info: format!(
                "Namespace length must be between {} and {}",
                RegistryEntry::MIN_NAME_LENGTH,
                RegistryEntry::MAX_NAME_LENGTH,
            ),
        });
    }

    validate_chars("Namespace", namespace)?;

    Ok(())
}

fn validate_description(description: &str) -> ModelValidatorResult<RegistryError> {
    if (description.len() < RegistryEntry::MIN_DESCRIPTION_LENGTH)
        || (description.len() > RegistryEntry::MAX_DESCRIPTION_LENGTH)
    {
        return Err(RegistryError::ValidationError {
            info: format!(
                "Description length must be between {} and {}",
                RegistryEntry::MIN_DESCRIPTION_LENGTH,
                RegistryEntry::MAX_DESCRIPTION_LENGTH,
            ),
        });
    }

    Ok(())
}

fn validate_categories(categories: &[String]) -> ModelValidatorResult<RegistryError> {
    if categories.len() > RegistryEntry::MAX_CATEGORIES {
        return Err(RegistryError::ValidationError {
            info: format!(
                "Too many categories, expected at most {} but got {}",
                RegistryEntry::MAX_CATEGORIES,
                categories.len()
            ),
        });
    }

    for category in categories.iter() {
        if (category.len() < RegistryEntry::MIN_CATEGORY_LENGTH)
            || (category.len() > RegistryEntry::MAX_CATEGORY_LENGTH)
        {
            return Err(RegistryError::ValidationError {
                info: format!(
                    "Category length must be between {} and {}",
                    RegistryEntry::MIN_CATEGORY_LENGTH,
                    RegistryEntry::MAX_CATEGORY_LENGTH,
                ),
            });
        }
    }

    if categories.len()
        != categories
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    {
        return Err(RegistryError::ValidationError {
            info: "Categories must be unique".to_string(),
        });
    }

    Ok(())
}

fn validate_tags(tags: &[String]) -> ModelValidatorResult<RegistryError> {
    if tags.len() > RegistryEntry::MAX_TAGS {
        return Err(RegistryError::ValidationError {
            info: format!(
                "Too many tags, expected at most {} but got {}",
                RegistryEntry::MAX_TAGS,
                tags.len()
            ),
        });
    }

    for tag in tags.iter() {
        if (tag.len() < RegistryEntry::MIN_TAG_LENGTH)
            || (tag.len() > RegistryEntry::MAX_TAG_LENGTH)
        {
            return Err(RegistryError::ValidationError {
                info: format!(
                    "Tag length must be between {} and {}",
                    RegistryEntry::MIN_TAG_LENGTH,
                    RegistryEntry::MAX_TAG_LENGTH,
                ),
            });
        }
    }

    if tags.len() != tags.iter().collect::<std::collections::BTreeSet<_>>().len() {
        return Err(RegistryError::ValidationError {
            info: "Tags must be unique".to_string(),
        });
    }

    Ok(())
}

fn validate_metadata(metadata: &BTreeMap<String, String>) -> ModelValidatorResult<RegistryError> {
    if metadata.len() > RegistryEntry::MAX_METADATA_ENTRIES {
        return Err(RegistryError::ValidationError {
            info: format!(
                "Too many metadata entries, expected at most {} but got {}",
                RegistryEntry::MAX_METADATA_ENTRIES,
                metadata.len()
            ),
        });
    }

    for (key, value) in metadata.iter() {
        if (key.len() < RegistryEntry::MIN_METADATA_KEY_LENGTH)
            || (key.len() > RegistryEntry::MAX_METADATA_KEY_LENGTH)
        {
            return Err(RegistryError::ValidationError {
                info: format!(
                    "Metadata key length must be between {} and {}",
                    RegistryEntry::MIN_METADATA_KEY_LENGTH,
                    RegistryEntry::MAX_METADATA_KEY_LENGTH,
                ),
            });
        }

        if (value.len() < RegistryEntry::MIN_METADATA_VALUE_LENGTH)
            || (value.len() > RegistryEntry::MAX_METADATA_VALUE_LENGTH)
        {
            return Err(RegistryError::ValidationError {
                info: format!(
                    "Metadata value length must be between {} and {}",
                    RegistryEntry::MIN_METADATA_VALUE_LENGTH,
                    RegistryEntry::MAX_METADATA_VALUE_LENGTH,
                ),
            });
        }
    }

    Ok(())
}

fn validate_timestamps(
    date_added: Timestamp,
    date_updated: Option<Timestamp>,
) -> ModelValidatorResult<RegistryError> {
    if date_added > date_updated.unwrap_or(date_added) {
        return Err(RegistryError::ValidationError {
            info: "The date added must be before the date updated".to_string(),
        });
    }

    Ok(())
}

impl ModelValidator<RegistryError> for RegistryEntry {
    fn validate(&self) -> ModelValidatorResult<RegistryError> {
        validate_namespace(&self.namespace)?;
        validate_name(&self.name)?;
        validate_description(&self.description)?;
        validate_categories(&self.categories)?;
        validate_tags(&self.tags)?;
        validate_metadata(&self.metadata)?;
        validate_timestamps(self.created_at, self.updated_at)?;

        match &self.value {
            RegistryValue::WasmModule(value) => value.validate(),
        }?;

        validate_uniqueness(self)?;
        validate_is_of_same_kind(self)?;
        validate_dependencies(self)?;

        Ok(())
    }
}

#[cfg(test)]
pub mod registry_entry_test_utils {
    use super::*;
    use uuid::Uuid;

    pub fn create_registry_entry() -> RegistryEntry {
        RegistryEntry {
            id: *Uuid::new_v4().as_bytes(),
            namespace: RegistryEntry::DEFAULT_NAMESPACE.to_string(),
            name: Uuid::new_v4().to_string().as_str()[..8].to_string(),
            description: "This is a test entry to the registry.".to_string(),
            tags: Vec::new(),
            categories: Vec::new(),
            value: create_wasm_module_registry_entry_value(),
            created_at: 0,
            updated_at: None,
            metadata: BTreeMap::new(),
        }
    }

    pub fn create_wasm_module_registry_entry_value() -> RegistryValue {
        RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbit_essentials::repository::Repository;
    use registry_entry_test_utils::create_registry_entry;
    use rstest::rstest;
    use uuid::Uuid;

    #[rstest]
    #[case::empty_namespace(&"")]
    #[case::empty_namespace_with_space(&" ")]
    #[case::starts_with_space(&" orbit")]
    #[case::ends_with_space(&"orbit ")]
    #[case::namespace_to_small(&"a")]
    #[case::namespace_too_big(&"a".repeat(RegistryEntry::MAX_NAMESPACE_LENGTH + 1))]
    #[case::namespace_contains_invalid_characters(&"orbit!")]
    #[case::namespace_contains_invalid_characters(&"orbit.1")]
    #[case::namespace_contains_invalid_characters(&"orbit_1")]
    #[case::namespace_ends_with_hyphen(&"orbit-")]
    fn invalid_namespace(#[case] namespace: &str) {
        let mut entry = create_registry_entry();
        entry.namespace = namespace.to_string();

        assert!(validate_namespace(&entry.namespace).is_err());
    }

    #[rstest]
    #[case::common_namespace(&"orbit")]
    #[case::short_namespace(&"a".repeat(RegistryEntry::MIN_NAMESPACE_LENGTH))]
    #[case::long_namespace(&"a".repeat(RegistryEntry::MAX_NAMESPACE_LENGTH))]
    fn valid_namespace(#[case] namespace: &str) {
        let mut entry = create_registry_entry();
        entry.namespace = namespace.to_string();

        validate_namespace(&entry.namespace).unwrap();
    }

    #[rstest]
    #[case::empty_name(&"")]
    #[case::empty_name_with_space(&" ")]
    #[case::starts_with_space(&" entry")]
    #[case::ends_with_space(&"entry ")]
    #[case::name_too_small(&"a")]
    #[case::name_too_big(&"a".repeat(RegistryEntry::MAX_NAME_LENGTH + 1))]
    #[case::name_contains_invalid_characters(&"test!")]
    #[case::name_contains_invalid_characters(&"test.1")]
    #[case::name_contains_invalid_characters(&"test_1")]
    #[case::name_contains_invalid_characters(&"test@1")]
    #[case::name_contains_invalid_characters(&"test#1")]
    #[case::name_contains_invalid_characters(&"test 1")]
    fn invalid_name(#[case] name: &str) {
        let mut entry = create_registry_entry();
        entry.name = name.to_string();

        assert!(validate_name(&entry.name).is_err());
    }

    #[rstest]
    #[case::common_name(&"test")]
    #[case::short_name(&"a".repeat(RegistryEntry::MIN_NAME_LENGTH))]
    #[case::long_name(&"a".repeat(RegistryEntry::MAX_NAME_LENGTH))]
    fn valid_name(#[case] name: &str) {
        let mut entry = create_registry_entry();
        entry.name = name.to_string();

        validate_name(&entry.name).unwrap();
    }

    #[rstest]
    #[case::empty_description(&"")]
    #[case::description_too_small(&"a")]
    #[case::description_too_big(&"a".repeat(RegistryEntry::MAX_DESCRIPTION_LENGTH + 1))]
    fn invalid_description(#[case] description: &str) {
        let mut entry = create_registry_entry();
        entry.description = description.to_string();

        assert!(validate_description(&entry.description).is_err());
    }

    #[rstest]
    #[case::common_description(&"This is a test entry to the registry.")]
    #[case::short_description(&"a".repeat(RegistryEntry::MIN_DESCRIPTION_LENGTH))]
    #[case::long_description(&"a".repeat(RegistryEntry::MAX_DESCRIPTION_LENGTH))]
    fn valid_description(#[case] description: &str) {
        let mut entry = create_registry_entry();
        entry.description = description.to_string();

        validate_description(&entry.description).unwrap();
    }

    #[rstest]
    #[case::too_many_categories((0..RegistryEntry::MAX_CATEGORIES + 1).map(|i| format!("test-{}", i).to_string()).collect())]
    #[case::category_too_small(vec!["a".to_string()])]
    #[case::category_too_big(vec!["a".repeat(RegistryEntry::MAX_CATEGORY_LENGTH + 1)])]
    #[case::duplicate_categories(vec!["test".to_string(), "test".to_string()])]
    fn invalid_categories(#[case] categories: Vec<String>) {
        let mut entry = create_registry_entry();
        entry.categories = categories;

        assert!(validate_categories(&entry.categories).is_err());
    }

    #[rstest]
    #[case::no_categories(vec![])]
    #[case::common_categories(vec!["test".to_string()])]
    #[case::short_categories(vec!["a".repeat(RegistryEntry::MIN_CATEGORY_LENGTH)])]
    #[case::long_categories(vec!["a".repeat(RegistryEntry::MAX_CATEGORY_LENGTH)])]
    fn valid_categories(#[case] categories: Vec<String>) {
        let mut entry = create_registry_entry();
        entry.categories = categories;

        validate_categories(&entry.categories).unwrap();
    }

    #[rstest]
    #[case::too_many_tags((0..RegistryEntry::MAX_TAGS + 1).map(|i| i.to_string()).collect())]
    #[case::tag_too_small(vec!["a".to_string()])]
    #[case::tag_too_big(vec!["a".repeat(RegistryEntry::MAX_TAG_LENGTH + 1)])]
    #[case::duplicate_tags(vec!["test".to_string(), "test".to_string()])]
    fn invalid_tags(#[case] tags: Vec<String>) {
        let mut entry = create_registry_entry();
        entry.tags = tags;

        assert!(validate_tags(&entry.tags).is_err());
    }

    #[rstest]
    #[case::no_tags(vec![])]
    #[case::common_tags(vec!["test".to_string()])]
    #[case::short_tags(vec!["a".repeat(RegistryEntry::MIN_TAG_LENGTH)])]
    #[case::long_tags(vec!["a".repeat(RegistryEntry::MAX_TAG_LENGTH)])]
    fn valid_tags(#[case] tags: Vec<String>) {
        let mut entry = create_registry_entry();
        entry.tags = tags;

        validate_tags(&entry.tags).unwrap();
    }

    #[rstest]
    #[case::too_many_metadata(
        (0..RegistryEntry::MAX_METADATA_ENTRIES + 1)
            .map(|i| (i.to_string(), i.to_string()))
            .collect()
    )]
    #[case::metadata_key_too_small(vec![("".to_string(), "test".to_string())])]
    #[case::metadata_key_too_big(vec![("a".repeat(RegistryEntry::MAX_METADATA_KEY_LENGTH + 1), "test".to_string())])]
    #[case::metadata_value_too_small(vec![("test".to_string(), "".to_string())])]
    #[case::metadata_value_too_big(vec![("test".to_string(), "a".repeat(RegistryEntry::MAX_METADATA_VALUE_LENGTH + 1))])]
    fn invalid_metadata(#[case] metadata: Vec<(String, String)>) {
        let mut entry = create_registry_entry();
        entry.metadata = metadata.into_iter().collect();

        assert!(validate_metadata(&entry.metadata).is_err());
    }

    #[rstest]
    #[case::no_metadata(Vec::new())]
    #[case::common_metadata(vec![
        ("author".to_string(), "test".to_string()),
        ("license".to_string(), "MIT".to_string())
    ])]
    #[case::short_metadata(vec![("a".to_string(), "a".to_string())])]
    #[case::long_metadata(vec![(
        "a".repeat(RegistryEntry::MAX_METADATA_KEY_LENGTH),
        "a".repeat(RegistryEntry::MAX_METADATA_VALUE_LENGTH)
    )])]
    fn valid_metadata(#[case] metadata: Vec<(String, String)>) {
        let mut entry = create_registry_entry();
        entry.metadata = metadata.into_iter().collect();

        validate_metadata(&entry.metadata).unwrap();
    }

    #[rstest]
    #[case::date_added_after_date_updated(1, Some(0))]
    fn invalid_timestamps(#[case] date_added: Timestamp, #[case] date_updated: Option<Timestamp>) {
        let mut entry = create_registry_entry();
        entry.created_at = date_added;
        entry.updated_at = date_updated;

        assert!(validate_timestamps(entry.created_at, entry.updated_at).is_err());
    }

    #[rstest]
    #[case::date_added_before_date_updated(0, Some(1))]
    #[case::date_added_equal_to_date_updated(0, Some(0))]
    #[case::date_added_and_no_date_updated(0, None)]
    fn valid_timestamps(#[case] date_added: Timestamp, #[case] date_updated: Option<Timestamp>) {
        let mut entry = create_registry_entry();
        entry.created_at = date_added;
        entry.updated_at = date_updated;

        validate_timestamps(entry.created_at, entry.updated_at).unwrap();
    }

    #[rstest]
    #[case::empty_version(&"")]
    #[case::version_too_big(&"a".repeat(WasmModuleRegistryValue::MAX_VERSION_LENGTH + 1))]
    fn invalid_version(#[case] version: &str) {
        let mut entry = create_registry_entry();
        entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: version.to_string(),
            dependencies: Vec::new(),
        });

        assert!(entry.validate().is_err());
    }

    #[rstest]
    #[case::common_version(&"1.0.0")]
    #[case::short_version(&"1")]
    #[case::long_version(&"1".repeat(WasmModuleRegistryValue::MAX_VERSION_LENGTH))]
    fn valid_version(#[case] version: &str) {
        let mut entry = create_registry_entry();
        entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: version.to_string(),
            dependencies: Vec::new(),
        });

        entry.validate().unwrap();
    }

    #[test]
    fn invalid_dependencies() {
        let dependencies = (0..WasmModuleRegistryValue::MAX_DEPENDENCIES + 1)
            .map(|i| WasmModuleRegistryEntryDependency {
                name: i.to_string(),
                version: format!("1.0.{}", i),
            })
            .collect();

        let mut entry = create_registry_entry();
        entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies,
        });

        assert!(entry.validate().is_err());
    }

    #[rstest]
    #[case::no_dependencies(Vec::new())]
    #[case::some_dependencies(vec![("test".to_string(), "1.0.0".to_string())])]
    fn valid_dependencies(#[case] dependencies: Vec<(String, String)>) {
        for (name, version) in dependencies.iter() {
            let mut entry = create_registry_entry();
            entry.namespace = RegistryEntry::DEFAULT_NAMESPACE.to_string();
            entry.name = name.to_string();
            entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
                wasm_artifact_id: *Uuid::new_v4().as_bytes(),
                version: version.to_string(),
                dependencies: Vec::new(),
            });

            REGISTRY_REPOSITORY.insert(entry.id, entry.clone());
        }

        let mut entry = create_registry_entry();
        entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies: dependencies
                .into_iter()
                .map(|(name, version)| WasmModuleRegistryEntryDependency {
                    name: format!("@{}/{}", RegistryEntry::DEFAULT_NAMESPACE, name),
                    version,
                })
                .collect(),
        });

        entry.validate().unwrap();
    }

    #[test]
    fn detects_circular_dependencies() {
        let mut sub_package = create_registry_entry();
        sub_package.name = "sub-package".to_string();

        let mut package = create_registry_entry();
        package.name = "package".to_string();

        sub_package.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies: vec![WasmModuleRegistryEntryDependency {
                name: package.fullname(),
                version: "1.0.0".to_string(),
            }],
        });

        package.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies: vec![WasmModuleRegistryEntryDependency {
                name: sub_package.fullname(),
                version: "1.0.0".to_string(),
            }],
        });

        REGISTRY_REPOSITORY.insert(sub_package.id, sub_package.clone());
        REGISTRY_REPOSITORY.insert(package.id, package.clone());

        let result = validate_dependencies(&package);

        assert!(result.unwrap_err().to_string().contains("dependency"));
    }

    #[test]
    fn correctly_checks_dependencies() {
        let mut sub_sub_package = create_registry_entry();
        sub_sub_package.name = "sub-sub-package".to_string();

        let mut sub_package = create_registry_entry();
        sub_package.name = "sub-package".to_string();

        let mut package = create_registry_entry();
        package.name = "package".to_string();

        sub_sub_package.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
        });

        sub_package.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies: vec![WasmModuleRegistryEntryDependency {
                name: sub_sub_package.fullname(),
                version: "1.0.0".to_string(),
            }],
        });

        package.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
            wasm_artifact_id: *Uuid::new_v4().as_bytes(),
            version: "1.0.0".to_string(),
            dependencies: vec![WasmModuleRegistryEntryDependency {
                name: sub_package.fullname(),
                version: "1.0.0".to_string(),
            }],
        });

        REGISTRY_REPOSITORY.insert(sub_sub_package.id, sub_sub_package.clone());
        REGISTRY_REPOSITORY.insert(sub_package.id, sub_package.clone());
        REGISTRY_REPOSITORY.insert(package.id, package.clone());

        validate_dependencies(&package).unwrap();
    }
}
