use std::collections::BTreeMap;

use orbit_essentials::storable;
use orbit_essentials::types::UUID;

/// The registry id, which is a UUID.
pub type RegistryId = UUID;

/// The item in the registry.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Registry {
    /// The UUID that identifies the entry in the registry.
    pub id: RegistryId,
    /// The category is used to associate the entry with a specific category (e.g. "canister_module").
    pub categories: Vec<String>,
    /// The name of the entry, which is used to identify it.
    pub name: String,
    /// The description of the entry, which is a human-readable description of the entry.
    pub description: String,
    /// The content of the entry in the registry, which can be a Wasm module.
    pub entry: RegistryEntry,
    /// The metadata of the entry in the registry.
    ///
    /// This is a key-value map that can be used to store additional information about the entry,
    /// such as the version, the author, the description, etc.
    pub metadata: BTreeMap<String, String>,
}

#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RegistryEntry {
    WasmModule(WasmModuleRegistryEntry),
}

#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WasmModuleRegistryEntry {
    /// The version of the wasm module.
    pub version: String,
    /// The sha256 hash of the wasm module.
    pub wasm_module_hash: Vec<u8>,
    /// The wasm module itself.
    pub wasm_module: Vec<u8>,
    /// The dependencies of the wasm module, which are other wasm modules that this wasm module depends on.
    /// 
    /// This registry ids should only reference registry entries that are of type `WasmModule`, others will be ignored.
    pub dependencies: Vec<RegistryId>,
}
