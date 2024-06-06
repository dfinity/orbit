use crate::models::{RegistryValue, RegistryValueKind};

impl From<&RegistryValue> for RegistryValueKind {
    fn from(value: &RegistryValue) -> Self {
        match value {
            RegistryValue::WasmModule(_) => RegistryValueKind::WasmModule,
        }
    }
}
