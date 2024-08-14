use crate::core::ic_cdk::api::trap;
use crate::core::{read_system_info, write_system_info};
use crate::STABLE_MEMORY_VERSION;
use crate::{
    core::{with_memory_manager, Memory},
    repositories::REQUEST_REPOSITORY,
};
use ic_stable_structures::{
    memory_manager::{MemoryId, VirtualMemory},
    StableBTreeMap,
};

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

const USER_IDENTITY_INDEX_MEMORY_ID: MemoryId = MemoryId::new(3);
const USER_STATUS_GROUP_INDEX_MEMORY_ID: MemoryId = MemoryId::new(18);
const NAME_TO_USER_ID_INDEX_MEMORY_ID: MemoryId = MemoryId::new(28);
// Old request indexes
const REQUEST_EXPIRATION_TIME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(5);
const REQUEST_APPROVER_INDEX_MEMORY_ID: MemoryId = MemoryId::new(8);
const REQUEST_STATUS_MEMORY_ID: MemoryId = MemoryId::new(9);
const REQUEST_SCHEDULED_INDEX_MEMORY_ID: MemoryId = MemoryId::new(10);
const REQUEST_REQUESTER_INDEX_MEMORY_ID: MemoryId = MemoryId::new(21);
const REQUEST_CREATION_TIME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(22);
const REQUEST_KEY_CREATION_TIME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(23);
const REQUEST_KEY_EXPIRATION_TIME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(24);
const REQUEST_SORT_INDEX_MEMORY_ID: MemoryId = MemoryId::new(25);
const REQUEST_STATUS_MODIFICATION_INDEX_MEMORY_ID: MemoryId = MemoryId::new(26);
const OPERATION_TYPE_TO_REQUEST_ID_INDEX_MEMORY_ID: MemoryId = MemoryId::new(29);
// Old user group indexes
const USER_GROUP_NAME_INDEX_MEMORY_ID: MemoryId = MemoryId::new(15);
// Old account indexes
const NAME_TO_ACCOUNT_ID_INDEX_MEMORY_ID: MemoryId = MemoryId::new(27);

/// The migration to apply to the station canister stable memory.
///
/// Please include the migration steps in the `apply_migration` function.
fn apply_migration() {
    with_memory_manager(|memory_manager| {
        // step 1: clear unused memory ids
        for memory_id in vec![
            USER_IDENTITY_INDEX_MEMORY_ID,
            USER_STATUS_GROUP_INDEX_MEMORY_ID,
            NAME_TO_USER_ID_INDEX_MEMORY_ID,
            REQUEST_EXPIRATION_TIME_INDEX_MEMORY_ID,
            REQUEST_APPROVER_INDEX_MEMORY_ID,
            REQUEST_STATUS_MEMORY_ID,
            REQUEST_SCHEDULED_INDEX_MEMORY_ID,
            REQUEST_REQUESTER_INDEX_MEMORY_ID,
            REQUEST_CREATION_TIME_INDEX_MEMORY_ID,
            REQUEST_KEY_CREATION_TIME_INDEX_MEMORY_ID,
            REQUEST_KEY_EXPIRATION_TIME_INDEX_MEMORY_ID,
            REQUEST_SORT_INDEX_MEMORY_ID,
            REQUEST_STATUS_MODIFICATION_INDEX_MEMORY_ID,
            OPERATION_TYPE_TO_REQUEST_ID_INDEX_MEMORY_ID,
            USER_GROUP_NAME_INDEX_MEMORY_ID,
            NAME_TO_ACCOUNT_ID_INDEX_MEMORY_ID,
        ] {
            let mut unused_memory: StableBTreeMap<(), (), VirtualMemory<Memory>> =
                StableBTreeMap::new(memory_manager.get(memory_id));

            unused_memory.clear_new();
        }

        // step 2: recreates the indexes for all requests
        REQUEST_REPOSITORY.rebuild_indexes();
    })
}
