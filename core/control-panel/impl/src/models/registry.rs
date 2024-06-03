use super::artifact::ArtifactId;
use orbit_essentials::storable;
use orbit_essentials::types::{Timestamp, UUID};
use std::collections::BTreeMap;

/// When adding tags to registry entries, if the tag is in this list then it is unique across all the 
/// entries of the same namespace and name. If the tag exists in other entries, then it will be removed from
/// older entries.
/// 
/// This value is hardcoded, and it should be updated or moved to a configurable value in the future if needed.
pub const _UNIQUE_TAGS: [&str; 2] = ["latest", "stable"];

/// The registry entry id, which is a UUID.
pub type RegistryEntryId = UUID;

/// The registry entry is a record that is stored in the registry repository.
/// 
/// It stores entries about wasm modules and can be extended to other entry types. When adding new entry types,
/// the `RegistryEntryValue` enum should be updated to include the new entry type.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RegistryEntry {
    /// The UUID that identifies the entry in the registry.
    pub id: RegistryEntryId,
    /// Wether or not the entry is public, if the entry is public then it is readable by anyone.
    pub is_public: bool,
    /// The namespace of the entry, which is used to group entries together (e.g. "@orbit").
    pub namespace: String,
    /// The name of the entry, which is used to identify it (e.g. station), within each namespace the
    /// name should refer to a the same type of entry, but many entries can exist with the same name.
    ///
    /// e.g. if the namespace is "@orbit" and the name is "station", then all the entries
    /// will refer to a wasm module that is a station.
    pub name: String,
    /// The tags are used to tag the entry with specific search terms (e.g. "latest", "stable").
    ///
    /// Tags are grouped within the same `namespace/name` (e.g. "@orbit/station").
    pub tags: Vec<String>,
    /// The category is used to associate the entry with a specific category (e.g. "chain-fusion")
    /// across all the entries in the registry.
    pub categories: Vec<String>,
    /// The description of the entry, which is a human-readable description of the entry.
    pub description: String,
    /// The content of the entry in the registry, which can be a Wasm module.
    pub value: RegistryEntryValue,
    /// The timestamp when the entry was created.
    pub created_at: Timestamp,
    /// The timestamp when the entry was last updated.
    pub updated_at: Timestamp,
    /// The metadata of the entry in the registry.
    ///
    /// This is a key-value map that can be used to store additional information about the entry,
    /// such as the author, license, repository, docs, etc.
    pub metadata: BTreeMap<String, String>,
}

/// The registry entry value, which is the content of the registry entry.
/// 
/// When adding new entry types to the registry, if the new entry contains artifacts, then the artifact repository
/// should be used to store them efficiently.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RegistryEntryValue {
    WasmModule(WasmModuleRegistryEntryValue),
}

/// The wasm module registry entry value, which is the content of the wasm module and its version.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WasmModuleRegistryEntryValue {
    /// The id of the wasm module that is stored in the artifact repository.
    pub wasm_artifact_id: ArtifactId,
    /// The version of the wasm module.
    pub version: String,
    /// The dependencies of the wasm module, which are other wasm modules that this wasm module depends on.
    ///
    /// This registry ids should only reference registry entries that are of type `WasmModule`, others will be ignored.
    pub dependencies: Vec<RegistryEntryId>,
}
