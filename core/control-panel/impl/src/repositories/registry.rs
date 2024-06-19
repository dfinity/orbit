use super::indexes::{
    registry_index::RegistryIndexRepository, registry_sort_index::RegistrySortIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, REGISTRY_MEMORY_ID},
    mappers::HelperMapper,
    models::{
        indexes::registry_index::{RegistryIndex, RegistryIndexCriteria, RegistryIndexKind},
        RegistryEntry, RegistryEntryId, RegistryValueKind,
    },
};
use control_panel_api::RegistryEntrySortBy;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::{
    repository::{IndexRepository, OrSelectionFilter, Repository},
    types::UUID,
};
use orbit_essentials::{
    repository::{RefreshIndexMode, SelectionFilter, SortDirection, SortingStrategy},
    types::Timestamp,
};
use std::{cell::RefCell, collections::HashSet, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<RegistryEntryId, RegistryEntry, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REGISTRY_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref REGISTRY_REPOSITORY: Arc<RegistryRepository> =
        Arc::new(RegistryRepository::default());
}

/// A repository that enables managing registry entries in stable memory.
#[derive(Default, Debug)]
pub struct RegistryRepository {
    indexes: RegistryIndexRepository,
    sort_index: RegistrySortIndexRepository,
}

impl Repository<RegistryEntryId, RegistryEntry> for RegistryRepository {
    fn list(&self) -> Vec<RegistryEntry> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &RegistryEntryId) -> Option<RegistryEntry> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: RegistryEntryId, value: RegistryEntry) -> Option<RegistryEntry> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.indexes
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev: RegistryEntry| prev.indexes()),
                    current: value.indexes(),
                });

            self.sort_index.insert(key, value.to_sort_index());

            prev
        })
    }

    fn remove(&self, key: &RegistryEntryId) -> Option<RegistryEntry> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            self.indexes
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev.clone().map_or(Vec::new(), |prev| prev.indexes()),
                });

            self.sort_index.remove(key);

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl RegistryRepository {
    /// Finds the ids of the registry entries by the given where clause.
    pub fn find_ids_where(
        &self,
        condition: RegistryWhere,
        sort_by: Option<RegistryEntrySortBy>,
    ) -> Vec<RegistryEntryId> {
        let filters = self.build_where_filtering_strategy(condition);
        let entry_ids = self.find_with_filters(filters);
        let mut ids = entry_ids.into_iter().collect::<Vec<_>>();

        self.sort_ids_with_strategy(&mut ids, &sort_by);

        ids
    }

    /// Sorts the registry entry IDs based on the provided sort strategy.
    ///
    /// If no sort strategy is provided, it defaults to sorting by creation timestamp descending.
    fn sort_ids_with_strategy(
        &self,
        entry_ids: &mut [UUID],
        sort_by: &Option<RegistryEntrySortBy>,
    ) {
        match sort_by {
            Some(RegistryEntrySortBy::CreatedAt(direction)) => {
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Creation,
                    direction: match direction {
                        control_panel_api::SortDirection::Asc => Some(SortDirection::Ascending),
                        control_panel_api::SortDirection::Desc => Some(SortDirection::Descending),
                    },
                };

                sort_strategy.sort(entry_ids);
            }
            Some(RegistryEntrySortBy::Version(direction)) => {
                let sort_strategy = VersionSortingStrategy {
                    index: &self.sort_index,
                    direction: match direction {
                        control_panel_api::SortDirection::Asc => SortDirection::Ascending,
                        control_panel_api::SortDirection::Desc => SortDirection::Descending,
                    },
                };

                sort_strategy.sort(entry_ids);
            }
            None => {
                // Default sort by creation timestamp descending
                let sort_strategy = TimestampSortingStrategy {
                    index: &self.sort_index,
                    timestamp_type: TimestampType::Creation,
                    direction: Some(SortDirection::Descending),
                };

                sort_strategy.sort(entry_ids);
            }
        }
    }

    fn build_where_filtering_strategy<'a>(
        &'a self,
        condition: RegistryWhere,
    ) -> Vec<Box<dyn SelectionFilter<'a, IdType = UUID> + 'a>> {
        let mut filters: Vec<Box<dyn SelectionFilter<IdType = UUID> + 'a>> = Vec::new();

        if let Some(fullname) = condition.fullname {
            filters.push(Box::new(FullnameSelectionFilter {
                repository: &self.indexes,
                fullname,
            }));
        }

        if let Some(namespace) = condition.namespace {
            filters.push(Box::new(NamespaceSelectionFilter {
                repository: &self.indexes,
                namespace,
            }));
        }

        if let Some(kind) = condition.kind {
            filters.push(Box::new(KindSelectionFilter {
                repository: &self.indexes,
                kind,
            }));
        }

        if let Some(version) = condition.version {
            filters.push(Box::new(VersionSelectionFilter {
                repository: &self.indexes,
                version,
            }));
        }

        if !condition.categories.is_empty() {
            let includes_categories = Box::new(OrSelectionFilter {
                filters: condition
                    .categories
                    .iter()
                    .map(|category| {
                        Box::new(CategorySelectionFilter {
                            repository: &self.indexes,
                            category: category.clone(),
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            });

            filters.push(includes_categories);
        }

        if !condition.tags.is_empty() {
            let includes_tags = Box::new(OrSelectionFilter {
                filters: condition
                    .tags
                    .iter()
                    .map(|tag| {
                        Box::new(TagSelectionFilter {
                            repository: &self.indexes,
                            tag: tag.clone(),
                        }) as Box<dyn SelectionFilter<IdType = UUID>>
                    })
                    .collect(),
            });

            filters.push(includes_tags);
        }

        // If no filters are provided, use the default namespace
        if filters.is_empty() {
            filters.push(Box::new(NamespaceSelectionFilter {
                repository: &self.indexes,
                namespace: RegistryEntry::DEFAULT_NAMESPACE.to_string(),
            }));
        }

        filters
    }
}

#[derive(Debug, Clone)]
enum TimestampType {
    Creation,
}

#[derive(Debug, Clone)]
struct TimestampSortingStrategy<'a> {
    index: &'a RegistrySortIndexRepository,
    timestamp_type: TimestampType,
    direction: Option<SortDirection>,
}

impl<'a> SortingStrategy<'a> for TimestampSortingStrategy<'a> {
    type IdType = UUID;

    fn sort(&self, ids: &mut [Self::IdType]) {
        let direction = self.direction.unwrap_or(SortDirection::Ascending);
        let mut id_with_timestamps: Vec<(Timestamp, Self::IdType)> = ids
            .iter()
            .map(|id| {
                let timestamp = self
                    .index
                    .get(id)
                    .map(|index| match self.timestamp_type {
                        TimestampType::Creation => index.created_at,
                    })
                    .unwrap_or_default();
                (timestamp, *id)
            })
            .collect();

        id_with_timestamps.sort_by(|a, b| {
            {
                let ord = a.0.cmp(&b.0); // Compare timestamps
                match direction {
                    SortDirection::Ascending => ord,
                    SortDirection::Descending => ord.reverse(),
                }
            }
            .then_with(|| match direction {
                // Compare IDs if timestamps are equal
                SortDirection::Ascending => a.1.cmp(&b.1),
                SortDirection::Descending => b.1.cmp(&a.1),
            })
        });

        let sorted_ids: Vec<UUID> = id_with_timestamps.into_iter().map(|(_, id)| id).collect();
        ids.copy_from_slice(&sorted_ids);
    }
}

#[derive(Debug, Clone)]
struct VersionSortingStrategy<'a> {
    index: &'a RegistrySortIndexRepository,
    direction: SortDirection,
}

impl<'a> SortingStrategy<'a> for VersionSortingStrategy<'a> {
    type IdType = UUID;

    fn sort(&self, ids: &mut [Self::IdType]) {
        let mut id_with_versions: Vec<(semver::Version, Self::IdType)> = ids
            .iter()
            .map(|id| {
                let version = self
                    .index
                    .get(id)
                    .map(|index| match &index.version {
                        Some(version) => HelperMapper::to_semver(version),
                        None => semver::Version::new(0, 0, 0),
                    })
                    .unwrap_or(semver::Version::new(0, 0, 0));
                (version, *id)
            })
            .collect();

        id_with_versions.sort_by(|a, b| {
            {
                let ord = a.0.cmp(&b.0); // Compare versions
                match self.direction {
                    SortDirection::Ascending => ord,
                    SortDirection::Descending => ord.reverse(),
                }
            }
            .then_with(|| match self.direction {
                // Compare IDs if versions are equal
                SortDirection::Ascending => a.1.cmp(&b.1),
                SortDirection::Descending => b.1.cmp(&a.1),
            })
        });

        let sorted_ids: Vec<UUID> = id_with_versions.into_iter().map(|(_, id)| id).collect();
        ids.copy_from_slice(&sorted_ids);
    }
}

#[derive(Debug, Clone)]
pub struct RegistryWhere {
    pub fullname: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub version: Option<String>,
    pub kind: Option<RegistryValueKind>,
}

impl RegistryWhere {
    pub fn clause() -> Self {
        Self {
            fullname: None,
            name: None,
            namespace: None,
            categories: Vec::new(),
            tags: Vec::new(),
            version: None,
            kind: None,
        }
    }

    pub fn and_fullname(mut self, fullname: &str) -> Self {
        self.fullname = Some(fullname.to_string());
        self
    }

    pub fn and_namespace(mut self, namespace: &str) -> Self {
        self.namespace = Some(namespace.to_string());
        self
    }

    pub fn and_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn and_categories(mut self, categories: Vec<String>) -> Self {
        self.categories.extend(categories);
        self
    }

    pub fn and_tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }

    pub fn and_version(mut self, version: &str) -> Self {
        self.version = Some(version.to_string());
        self
    }

    pub fn and_kind(mut self, kind: RegistryValueKind) -> Self {
        self.kind = Some(kind);
        self
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FullnameSelectionFilter<'a> {
    repository: &'a RegistryIndexRepository,
    fullname: String,
}

impl<'a> SelectionFilter<'a> for FullnameSelectionFilter<'a> {
    type IdType = RegistryEntryId;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RegistryIndex {
            index: RegistryIndexKind::Fullname(self.fullname.clone()),
            registry_entry_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Fullname(self.fullname.clone()),
            to: RegistryIndexKind::Fullname(self.fullname.clone()),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NamespaceSelectionFilter<'a> {
    repository: &'a RegistryIndexRepository,
    namespace: String,
}

impl<'a> SelectionFilter<'a> for NamespaceSelectionFilter<'a> {
    type IdType = RegistryEntryId;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RegistryIndex {
            index: RegistryIndexKind::Namespace(self.namespace.clone()),
            registry_entry_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Namespace(self.namespace.clone()),
            to: RegistryIndexKind::Namespace(self.namespace.clone()),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct KindSelectionFilter<'a> {
    repository: &'a RegistryIndexRepository,
    kind: RegistryValueKind,
}

impl<'a> SelectionFilter<'a> for KindSelectionFilter<'a> {
    type IdType = RegistryEntryId;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RegistryIndex {
            index: RegistryIndexKind::ValueKind(self.kind.clone()),
            registry_entry_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::ValueKind(self.kind.clone()),
            to: RegistryIndexKind::ValueKind(self.kind.clone()),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CategorySelectionFilter<'a> {
    repository: &'a RegistryIndexRepository,
    category: String,
}

impl<'a> SelectionFilter<'a> for CategorySelectionFilter<'a> {
    type IdType = RegistryEntryId;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RegistryIndex {
            index: RegistryIndexKind::Category(self.category.clone()),
            registry_entry_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Category(self.category.clone()),
            to: RegistryIndexKind::Category(self.category.clone()),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TagSelectionFilter<'a> {
    repository: &'a RegistryIndexRepository,
    tag: String,
}

impl<'a> SelectionFilter<'a> for TagSelectionFilter<'a> {
    type IdType = RegistryEntryId;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RegistryIndex {
            index: RegistryIndexKind::Tag(self.tag.clone()),
            registry_entry_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Tag(self.tag.clone()),
            to: RegistryIndexKind::Tag(self.tag.clone()),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VersionSelectionFilter<'a> {
    repository: &'a RegistryIndexRepository,
    version: String,
}

impl<'a> SelectionFilter<'a> for VersionSelectionFilter<'a> {
    type IdType = RegistryEntryId;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RegistryIndex {
            index: RegistryIndexKind::Version(self.version.clone()),
            registry_entry_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Version(self.version.clone()),
            to: RegistryIndexKind::Version(self.version.clone()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        registry_entry_test_utils::{
            create_registry_entry, create_wasm_module_registry_entry_value,
        },
        RegistryValue, WasmModuleRegistryValue,
    };
    use orbit_essentials::model::ModelValidator;
    use uuid::Uuid;

    #[test]
    fn check_crud_operations() {
        let repository = RegistryRepository::default();
        let entry = create_registry_entry();

        assert!(repository.get(&entry.id).is_none());

        repository.insert(entry.id, entry.clone());
        assert_eq!(repository.len(), 1);
        assert_eq!(repository.get(&entry.id), Some(entry.clone()));

        repository.remove(&entry.id);

        assert!(repository.get(&entry.id).is_none());

        assert_eq!(repository.len(), 0);
    }

    #[test]
    fn test_find_by_namespace() {
        let repository = RegistryRepository::default();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.namespace = "orbit".to_string();
            entry.name = format!("entry-{}", i);
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result =
            repository.find_ids_where(RegistryWhere::clause().and_namespace("orbit"), None);

        assert!(!result.is_empty());
        assert_eq!(result.len(), 5);

        let result = result
            .into_iter()
            .map(|id| repository.get(&id).unwrap())
            .collect::<Vec<_>>();

        assert!(result.iter().any(|entry| entry.name == "entry-0"));

        let result = repository.find_ids_where(RegistryWhere::clause().and_namespace("orbi"), None);

        assert!(result.is_empty());
    }

    #[test]
    fn test_find_by_fullname() {
        let repository = RegistryRepository::default();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.namespace = "orbit".to_string();
            entry.name = format!("entry-{}", i);
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result =
            repository.find_ids_where(RegistryWhere::clause().and_fullname("@orbit/entry-2"), None);

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = repository.get(result.first().unwrap()).unwrap();

        assert_eq!(entry.namespace, "orbit");
        assert_eq!(entry.name, "entry-2");

        let result =
            repository.find_ids_where(RegistryWhere::clause().and_fullname("@orbit/entry-"), None);

        assert!(result.is_empty());
    }

    #[test]
    fn test_find_by_fullname_when_namespace_not_set() {
        let repository = RegistryRepository::default();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.namespace = RegistryEntry::DEFAULT_NAMESPACE.to_string();
            entry.name = format!("entry-{}", i);
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result = repository.find_ids_where(
            RegistryWhere::clause()
                .and_fullname(format!("@{}/entry-2", RegistryEntry::DEFAULT_NAMESPACE).as_str()),
            None,
        );

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = repository.get(result.first().unwrap()).unwrap();

        assert_eq!(entry.name, "entry-2");
    }

    #[test]
    fn test_find_by_category() {
        let repository = RegistryRepository::default();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.categories.push(format!("category-{}", i));
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result = repository.find_ids_where(
            RegistryWhere::clause().and_categories(vec!["category-2".to_string()]),
            None,
        );

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = repository.get(result.first().unwrap()).unwrap();

        assert_eq!(entry.categories.len(), 1);
        assert_eq!(entry.categories.first().unwrap(), "category-2");

        let result = repository.find_ids_where(
            RegistryWhere::clause().and_categories(vec!["category-".to_string()]),
            None,
        );

        assert!(result.is_empty());
    }

    #[test]
    fn test_find_by_tag() {
        let repository = RegistryRepository::default();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.tags.push(format!("tag-{}", i));
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result = repository.find_ids_where(
            RegistryWhere::clause().and_tags(vec!["tag-2".to_string()]),
            None,
        );

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = repository.get(result.first().unwrap()).unwrap();

        assert_eq!(entry.tags.len(), 1);
        assert_eq!(entry.tags.first().unwrap(), "tag-2");

        let result = repository.find_ids_where(
            RegistryWhere::clause().and_tags(vec!["tag-".to_string()]),
            None,
        );

        assert!(result.is_empty());
    }

    #[test]
    fn test_find_by_kind() {
        let repository = RegistryRepository::default();
        for _ in 0..5 {
            let mut entry = create_registry_entry();
            entry.value = create_wasm_module_registry_entry_value();
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result = repository.find_ids_where(
            RegistryWhere::clause().and_kind(RegistryValueKind::WasmModule),
            None,
        );

        assert!(!result.is_empty());
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_find_by_version() {
        let repository = RegistryRepository::default();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.namespace = "orbit".to_string();
            entry.name = "station".to_string();
            entry.value = RegistryValue::WasmModule(WasmModuleRegistryValue {
                wasm_artifact_id: *Uuid::new_v4().as_bytes(),
                version: format!("1.0.{}", i),
                dependencies: Vec::new(),
            });
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result = repository.find_ids_where(
            RegistryWhere::clause()
                .and_fullname("@orbit/station")
                .and_kind(RegistryValueKind::WasmModule)
                .and_version("1.0.2"),
            None,
        );

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = repository.get(result.first().unwrap()).unwrap();

        match entry.value {
            RegistryValue::WasmModule(ref value) => {
                assert_eq!(value.version, "1.0.2");
            }
        }

        let result = repository.find_ids_where(
            RegistryWhere::clause()
                .and_fullname("@orbit/station")
                .and_kind(RegistryValueKind::WasmModule)
                .and_version("1.0"),
            None,
        );

        assert!(result.is_empty());
    }

    #[test]
    fn sort_search_by_created_dt_asc_and_desc() {
        let repository = RegistryRepository::default();
        let mut expected_sorted_ids = Vec::new();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.created_at = i;
            entry.validate().unwrap();

            expected_sorted_ids.push(entry.id);
            repository.insert(entry.id, entry);
        }

        let sorted_result = repository.find_ids_where(
            RegistryWhere::clause(),
            Some(RegistryEntrySortBy::CreatedAt(
                control_panel_api::SortDirection::Asc,
            )),
        );

        assert!(!sorted_result.is_empty());
        assert_eq!(sorted_result.len(), 5);

        assert_eq!(sorted_result, expected_sorted_ids);

        let sorted_result = repository.find_ids_where(
            RegistryWhere::clause(),
            Some(RegistryEntrySortBy::CreatedAt(
                control_panel_api::SortDirection::Desc,
            )),
        );

        assert!(!sorted_result.is_empty());
        assert_eq!(sorted_result.len(), 5);

        // first make sure the ascending order is not equal to the descending order
        assert_ne!(sorted_result, expected_sorted_ids);

        expected_sorted_ids.reverse();

        assert_eq!(sorted_result, expected_sorted_ids);
    }
}
