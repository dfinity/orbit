use crate::models::{RegistryEntry, RegistryEntryId, RegistryValue, RegistryValueKind};
use orbit_essentials::storable;

/// The main index for registry entries.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegistryIndex {
    /// An indexed value of the registry entry.
    pub index: RegistryIndexKind,
    /// The registry entry id, which is a UUID.
    pub registry_entry_id: RegistryEntryId,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegistryIndexKind {
    ValueKind(RegistryValueKind),
    Fullname(String),
    Namespace(String),
    Category(String),
    Tag(String),
    Version(String),
}

#[derive(Clone, Debug)]
pub struct RegistryIndexCriteria {
    pub from: RegistryIndexKind,
    pub to: RegistryIndexKind,
}

impl RegistryEntry {
    pub fn to_index_by_fullname(&self) -> RegistryIndex {
        RegistryIndex {
            index: RegistryIndexKind::Fullname(self.fullname()),
            registry_entry_id: self.id,
        }
    }

    pub fn to_index_by_namespace(&self) -> RegistryIndex {
        RegistryIndex {
            index: RegistryIndexKind::Namespace(self.namespace.to_string()),
            registry_entry_id: self.id,
        }
    }

    pub fn to_index_by_categories(&self) -> Vec<RegistryIndex> {
        self.categories
            .iter()
            .map(|category| RegistryIndex {
                index: RegistryIndexKind::Category(category.to_string()),
                registry_entry_id: self.id,
            })
            .collect()
    }

    pub fn to_index_by_tags(&self) -> Vec<RegistryIndex> {
        self.tags
            .iter()
            .map(|tag| RegistryIndex {
                index: RegistryIndexKind::Tag(tag.to_string()),
                registry_entry_id: self.id,
            })
            .collect()
    }

    pub fn to_index_by_version(&self) -> Option<RegistryIndex> {
        match &self.value {
            RegistryValue::WasmModule(module) => Some(RegistryIndex {
                index: RegistryIndexKind::Version(module.version.to_string()),
                registry_entry_id: self.id,
            }),
        }
    }

    pub fn to_index_by_value_kind(&self) -> RegistryIndex {
        RegistryIndex {
            index: RegistryIndexKind::ValueKind(RegistryValueKind::from(&self.value)),
            registry_entry_id: self.id,
        }
    }

    /// Converts the registry entry to a list of indexes for searching.
    pub fn indexes(&self) -> Vec<RegistryIndex> {
        let mut indexes = vec![self.to_index_by_fullname()];
        indexes.push(self.to_index_by_namespace());
        indexes.extend(self.to_index_by_categories());
        indexes.extend(self.to_index_by_tags());
        indexes.push(self.to_index_by_value_kind());

        if let Some(index) = self.to_index_by_version() {
            indexes.push(index);
        }

        indexes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::registry_entry_test_utils::create_registry_entry;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model: RegistryIndex = RegistryIndex {
            index: RegistryIndexKind::Namespace("test".to_string()),
            registry_entry_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = RegistryIndex::from_bytes(serialized_model);

        assert_eq!(model.index, deserialized_model.index);
        assert_eq!(
            model.registry_entry_id,
            deserialized_model.registry_entry_id
        );
    }

    #[test]
    fn valid_to_namespace() {
        let mut entry = create_registry_entry();
        entry.namespace = "mynamespace".to_string();

        let index = entry.to_index_by_namespace();

        assert_eq!(index.index, RegistryIndexKind::Namespace(entry.namespace));
        assert_eq!(index.registry_entry_id, entry.id);
    }

    #[test]
    fn valid_to_categories() {
        let entry = create_registry_entry();
        let index = entry.to_index_by_categories();

        assert_eq!(index.len(), entry.categories.len());
        for (i, category) in entry.categories.iter().enumerate() {
            assert_eq!(
                index[i].index,
                RegistryIndexKind::Category(category.to_string())
            );
            assert_eq!(index[i].registry_entry_id, entry.id);
        }
    }

    #[test]
    fn valid_to_tags() {
        let entry = create_registry_entry();
        let index = entry.to_index_by_tags();

        assert_eq!(index.len(), entry.tags.len());
        for (i, tag) in entry.tags.iter().enumerate() {
            assert_eq!(index[i].index, RegistryIndexKind::Tag(tag.to_string()));
            assert_eq!(index[i].registry_entry_id, entry.id);
        }
    }

    #[test]
    fn valid_to_fullname() {
        let mut entry = create_registry_entry();
        entry.namespace = "mynamespace".to_string();
        entry.name = "test".to_string();
        let index = entry.to_index_by_fullname();

        assert_eq!(
            index.index,
            RegistryIndexKind::Fullname(entry.fullname().to_string())
        );
        assert_eq!(index.registry_entry_id, entry.id);
    }

    #[test]
    fn valid_to_value_kind() {
        let entry = create_registry_entry();
        let index = entry.to_index_by_value_kind();

        assert_eq!(
            index.index,
            RegistryIndexKind::ValueKind(RegistryValueKind::from(&entry.value))
        );
        assert_eq!(index.registry_entry_id, entry.id);
    }
}
