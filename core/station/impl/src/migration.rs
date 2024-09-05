use crate::core::ic_cdk::api::trap;
use crate::core::{read_system_info, write_system_info};

use crate::STABLE_MEMORY_VERSION;

/// Handles stable memory schema migrations for the station canister.
///
/// Stable memory migration conditions:
///
/// - The migration is only applied once per each version.
/// - Stable memory versions can't be skipped, the upgrade must be sequential.
/// - The migration is applied is the previous version is `STABLE_MEMORY_VERSION - 1`.
pub struct MigrationHandler;

impl MigrationHandler {
    /// Run migrations for the station canister to ensure the stable memory schema is up-to-date.
    ///
    /// WARNING: This needs to be run before any other access to stable memory happens.
    pub fn run() {
        let mut system_info = read_system_info();
        let stored_version = system_info.get_stable_memory_version();

        if stored_version == STABLE_MEMORY_VERSION {
            return;
        }

        if stored_version > STABLE_MEMORY_VERSION {
            trap(&format!(
                "Cannot downgrade the station from memory layout version {} to {}",
                stored_version, STABLE_MEMORY_VERSION
            ));
        }

        apply_migration();

        // Update the stable memory version to the latest version.
        system_info.set_stable_memory_version(STABLE_MEMORY_VERSION);
        write_system_info(system_info);
    }
}

/// The migration to apply to the station canister stable memory.
///
/// Please include the migration steps in the `apply_migration` function.
fn apply_migration() {
    // Add migration steps here.
}
