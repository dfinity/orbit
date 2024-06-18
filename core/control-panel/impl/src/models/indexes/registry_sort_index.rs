use crate::models::{RegistryEntry, RegistryValue};
use orbit_essentials::{storable, types::Timestamp};

/// The main index for registry entries.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegistrySortIndex {
    /// The registry entry's creation timestamp.
    pub created_at: Timestamp,
    /// The registry entry's last modification timestamp.
    pub modified_at: Option<Timestamp>,
    /// Version of the registry entry.
    pub version: Option<String>,
}

impl RegistryEntry {
    pub fn to_sort_index(&self) -> RegistrySortIndex {
        RegistrySortIndex {
            created_at: self.created_at,
            modified_at: self.updated_at,
            version: match self.value.clone() {
                RegistryValue::WasmModule(value) => Some(value.version),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::registry_entry_test_utils::create_registry_entry;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = RegistrySortIndex {
            created_at: 1,
            modified_at: Some(2),
            version: Some("1.0.0".to_string()),
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = RegistrySortIndex::from_bytes(serialized_model);

        assert_eq!(model.created_at, deserialized_model.created_at);
        assert_eq!(model.modified_at, deserialized_model.modified_at);
        assert_eq!(model.version, deserialized_model.version);
    }

    #[test]
    fn valid_to_sort_index() {
        let mut entry = create_registry_entry();
        entry.created_at = 1;
        entry.updated_at = Some(2);
        let index = entry.to_sort_index();

        assert_eq!(
            index,
            RegistrySortIndex {
                created_at: entry.created_at,
                modified_at: entry.updated_at,
                version: Some("1.0.0".to_string())
            }
        );
    }
}
