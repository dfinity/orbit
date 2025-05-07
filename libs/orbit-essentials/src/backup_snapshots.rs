use candid::Principal;
use ic_cdk::api::management_canister::main::{self as mgmt, DeleteCanisterSnapshotArgs};
use orbit_essentials_macros::storable;
use std::collections::VecDeque;

#[storable]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupSnapshots {
    max_backup_snapshots: u64,
    backup_snapshots: VecDeque<Vec<u8>>,
}

impl Default for BackupSnapshots {
    fn default() -> Self {
        Self::new(1)
    }
}

impl BackupSnapshots {
    pub fn new(max_backup_snapshots: u64) -> Self {
        Self {
            max_backup_snapshots,
            backup_snapshots: VecDeque::new(),
        }
    }

    pub fn get_max_backup_snapshots(&self) -> u64 {
        self.max_backup_snapshots
    }

    pub async fn set_max_backup_snapshots(
        &mut self,
        max_backup_snapshots: u64,
        canister_id: Principal,
    ) -> Result<(), String> {
        while self.backup_snapshots.len() as u64 > max_backup_snapshots {
            let snapshot_id = self.backup_snapshots.front().unwrap().clone();
            mgmt::delete_canister_snapshot(DeleteCanisterSnapshotArgs {
                canister_id,
                snapshot_id,
            })
            .await
            .map_err(|(_, err)| err)?;
            let _ = self.backup_snapshots.pop_front();
        }
        self.max_backup_snapshots = max_backup_snapshots;

        Ok(())
    }

    pub fn get_snapshot_to_replace(&self) -> Option<Vec<u8>> {
        if self.backup_snapshots.len() as u64 >= self.max_backup_snapshots {
            self.backup_snapshots.front().cloned()
        } else {
            None
        }
    }

    pub fn insert_snapshot(&mut self, snapshot_id: Vec<u8>) {
        if self.backup_snapshots.len() as u64 >= self.max_backup_snapshots {
            let _ = self.backup_snapshots.pop_front();
        }
        self.backup_snapshots.push_back(snapshot_id);
    }
}
