use super::indexes::registry_index::RegistryIndexRepository;
use crate::{
    core::{with_memory_manager, Memory, REGISTRY_MEMORY_ID},
    models::{
        indexes::registry_index::{RegistryIndex, RegistryIndexCriteria, RegistryIndexKind},
        RegistryEntry, RegistryEntryId, RegistryValueKind,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::{IndexRepository, Repository};
use orbit_essentials::repository::{RefreshIndexMode, SelectionFilter};
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

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl RegistryRepository {
    /// Finds the ids of the registry entries by the given name.
    pub fn find_ids_by_fullname(&self, name: &str) -> Vec<RegistryEntryId> {
        let entries = self.indexes.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Fullname(name.to_string()),
            to: RegistryIndexKind::Fullname(name.to_string()),
        });

        entries.into_iter().collect()
    }

    /// Finds the registry entries by the given name.
    pub fn find_by_fullname(&self, name: &str) -> Vec<RegistryEntry> {
        self.find_ids_by_fullname(name)
            .into_iter()
            .filter_map(|id| self.get(&id))
            .collect()
    }

    /// Finds the ids of the registry entries by the given namespace.
    pub fn find_ids_by_namespace(&self, namespace: &str) -> Vec<RegistryEntryId> {
        let entries = self.indexes.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Namespace(namespace.to_string()),
            to: RegistryIndexKind::Namespace(namespace.to_string()),
        });

        entries.into_iter().collect()
    }

    /// Finds the registry entries by the given namespace.
    pub fn find_by_namespace(&self, namespace: &str) -> Vec<RegistryEntry> {
        self.find_ids_by_namespace(namespace)
            .into_iter()
            .filter_map(|id| self.get(&id))
            .collect()
    }

    /// Finds the ids of the registry entries by the given unnamespaced name.
    pub fn find_ids_by_name(&self, unnamespaced_name: &str) -> Vec<RegistryEntryId> {
        let entries = self.indexes.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Name(unnamespaced_name.to_string()),
            to: RegistryIndexKind::Name(unnamespaced_name.to_string()),
        });

        entries.into_iter().collect()
    }

    /// Finds the registry entries by the given unnamespaced name.
    pub fn find_by_name(&self, unnamespaced_name: &str) -> Vec<RegistryEntry> {
        self.find_ids_by_name(unnamespaced_name)
            .into_iter()
            .filter_map(|id| self.get(&id))
            .collect()
    }

    /// Finds the ids of the registry entries by the given category.
    pub fn find_ids_by_category(&self, category: &str) -> Vec<RegistryEntryId> {
        let entries = self.indexes.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Category(category.to_string()),
            to: RegistryIndexKind::Category(category.to_string()),
        });

        entries.into_iter().collect()
    }

    /// Finds the registry entries by the given category.
    pub fn find_by_category(&self, category: &str) -> Vec<RegistryEntry> {
        self.find_ids_by_category(category)
            .into_iter()
            .filter_map(|id| self.get(&id))
            .collect()
    }

    /// Find by tags the registry entries by the given tag.
    pub fn find_ids_by_tag(&self, tag: &str) -> Vec<RegistryEntryId> {
        let entries = self.indexes.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Tag(tag.to_string()),
            to: RegistryIndexKind::Tag(tag.to_string()),
        });

        entries.into_iter().collect()
    }

    /// Finds the registry entries by the given tag.
    pub fn find_by_tag(&self, tag: &str) -> Vec<RegistryEntry> {
        self.find_ids_by_tag(tag)
            .into_iter()
            .filter_map(|id| self.get(&id))
            .collect()
    }

    /// Finds the ids of the registry entries by the given value kind.
    pub fn find_ids_by_kind(&self, kind: &RegistryValueKind) -> Vec<RegistryEntryId> {
        let entries = self.indexes.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::ValueKind(kind.clone()),
            to: RegistryIndexKind::ValueKind(kind.clone()),
        });

        entries.into_iter().collect()
    }

    /// Finds the registry entries by the given value kind.
    pub fn find_by_kind(&self, kind: &RegistryValueKind) -> Vec<RegistryEntry> {
        self.find_ids_by_kind(kind)
            .into_iter()
            .filter_map(|id| self.get(&id))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct RegistryWhereClause {
    pub fullname: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub kind: Option<RegistryValueKind>,
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
pub(crate) struct NameSelectionFilter<'a> {
    repository: &'a RegistryIndexRepository,
    name: String,
}

impl<'a> SelectionFilter<'a> for NameSelectionFilter<'a> {
    type IdType = RegistryEntryId;

    fn matches(&self, id: &Self::IdType) -> bool {
        self.repository.exists(&RegistryIndex {
            index: RegistryIndexKind::Name(self.name.clone()),
            registry_entry_id: *id,
        })
    }

    fn select(&self) -> HashSet<Self::IdType> {
        self.repository.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Name(self.name.clone()),
            to: RegistryIndexKind::Name(self.name.clone()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::registry_entry_test_utils::{
        create_registry_entry, create_wasm_module_registry_entry_value,
    };
    use orbit_essentials::model::ModelValidator;

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
    fn test_find_by_name() {
        let repository = RegistryRepository::default();
        for i in 0..10 {
            let mut entry = create_registry_entry();
            entry.name = format!("entry-{}", i);
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result = repository.find_by_name("entry-2");

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = result.first().unwrap();

        assert_eq!(entry.name, "entry-2");

        let result = repository.find_by_name("entry-");

        assert!(result.is_empty());
    }

    #[test]
    fn test_find_by_namespace() {
        let repository = RegistryRepository::default();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.name = format!("@orbit/entry-{}", i);
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result = repository.find_by_namespace("orbit");

        assert!(!result.is_empty());
        assert_eq!(result.len(), 5);
        assert!(result.iter().any(|entry| entry.name == "@orbit/entry-0"));

        let result = repository.find_by_namespace("orbi");

        assert!(result.is_empty());
    }

    #[test]
    fn test_find_by_fullname() {
        let repository = RegistryRepository::default();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.name = format!("@orbit/entry-{}", i);
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result = repository.find_by_fullname("@orbit/entry-2");

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = result.first().unwrap();

        assert_eq!(entry.name, "@orbit/entry-2");

        let result = repository.find_by_fullname("@orbit/entry-");

        assert!(result.is_empty());
    }

    #[test]
    fn test_find_by_fullname_when_namespace_not_set() {
        let repository = RegistryRepository::default();
        for i in 0..5 {
            let mut entry = create_registry_entry();
            entry.name = format!("entry-{}", i);
            entry.validate().unwrap();

            repository.insert(entry.id, entry);
        }

        let result = repository
            .find_by_fullname(format!("@{}/entry-2", RegistryEntry::DEFAULT_NAMESPACE).as_str());

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = result.first().unwrap();

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

        let result = repository.find_by_category("category-2");

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = result.first().unwrap();

        assert_eq!(entry.categories.len(), 1);
        assert_eq!(entry.categories.first().unwrap(), "category-2");

        let result = repository.find_by_category("category-");

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

        let result = repository.find_by_tag("tag-2");

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);

        let entry = result.first().unwrap();

        assert_eq!(entry.tags.len(), 1);
        assert_eq!(entry.tags.first().unwrap(), "tag-2");

        let result = repository.find_by_tag("tag-");

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

        let result = repository.find_by_kind(&RegistryValueKind::WasmModule);

        assert!(!result.is_empty());
        assert_eq!(result.len(), 5);
    }
}
