use crate::core::ic_cdk::api::trap;
use crate::core::{read_system_info, write_system_info};
use crate::repositories::{
    ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY, EXTERNAL_CANISTER_REPOSITORY,
    USER_GROUP_REPOSITORY, USER_REPOSITORY,
};
use crate::STABLE_MEMORY_VERSION;
use crate::{core::with_memory_manager, repositories::REQUEST_REPOSITORY};
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::Memory;
use orbit_essentials::repository::RebuildRepository;

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
    // step 1: clear unused memory ids
    with_memory_manager(|memory_manager| {
        for memory_id in [
            MemoryId::new(3),  // USER_IDENTITY_INDEX_MEMORY_ID,
            MemoryId::new(5),  // REQUEST_EXPIRATION_TIME_INDEX_MEMORY_ID
            MemoryId::new(8),  // REQUEST_APPROVER_INDEX_MEMORY_ID
            MemoryId::new(9),  // REQUEST_STATUS_INDEX_MEMORY_ID
            MemoryId::new(10), // REQUEST_SCHEDULED_INDEX_MEMORY_ID
            MemoryId::new(15), // USER_GROUP_NAME_INDEX_MEMORY_ID
            MemoryId::new(18), // USER_STATUS_GROUP_INDEX_MEMORY_ID
            MemoryId::new(20), // ADDRESS_BOOK_INDEX_MEMORY_ID
            MemoryId::new(21), // REQUEST_REQUESTER_INDEX_MEMORY_ID
            MemoryId::new(22), // REQUEST_CREATION_TIME_INDEX_MEMORY_ID
            MemoryId::new(23), // REQUEST_KEY_CREATION_TIME_INDEX_MEMORY_ID
            MemoryId::new(24), // REQUEST_KEY_EXPIRATION_TIME_INDEX_MEMORY_ID
            MemoryId::new(25), // REQUEST_SORT_INDEX_MEMORY_ID
            MemoryId::new(26), // REQUEST_STATUS_MODIFICATION_INDEX_MEMORY_ID
            MemoryId::new(27), // NAME_TO_ACCOUNT_ID_INDEX_MEMORY_ID
            MemoryId::new(28), // NAME_TO_USER_ID_INDEX_MEMORY_ID
            MemoryId::new(29), // OPERATION_TYPE_TO_REQUEST_ID_INDEX_MEMORY_ID
            MemoryId::new(34), // EXTERNAL_CANISTER_INDEX_MEMORY_ID
        ] {
            // This cleans up the memory by writing a single zero byte to the memory id,
            // this will make the memory id available for reuse in the future.
            //
            // This makes sure that if `init` is called on the memory id, it will make sure
            // it can be reused with a different datatype.
            let memory = memory_manager.get(memory_id);
            if memory.size() > 0 {
                // This marks the memory as unused, this is because the StableBTreeMap
                // implementation uses the first three bytes of the memory to store the MAGIC value [66, 84, 82]
                // that indicates that the memory is used by the StableBTreeMap, so adding a single different byte
                // in those first three bytes will make the memory available for reuse.
                memory.write(0, &[0]);
            }
        }
    });

    // step 2: rebuilds the repositories to ensure the data is up-to-date
    USER_GROUP_REPOSITORY.rebuild();
    USER_REPOSITORY.rebuild();
    ACCOUNT_REPOSITORY.rebuild();
    EXTERNAL_CANISTER_REPOSITORY.rebuild();
    ADDRESS_BOOK_REPOSITORY.rebuild();
    REQUEST_REPOSITORY.rebuild();
}
