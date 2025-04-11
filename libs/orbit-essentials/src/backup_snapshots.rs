use orbit_essentials_macros::storable;
use std::collections::VecDeque;

#[storable]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupSnapshots {
    max_backup_snapshots: usize,
    backup_snapshots: VecDeque<Vec<u8>>,
}

impl BackupSnapshots {
    pub fn new(max_backup_snapshots: usize) -> Self {
        Self {
            max_backup_snapshots,
            backup_snapshots: VecDeque::new(),
        }
    }

    pub fn replace_snapshot(&self) -> Option<Vec<u8>> {
        if self.backup_snapshots.len() >= self.max_backup_snapshots {
            self.backup_snapshots.front().cloned()
        } else {
            None
        }
    }

    pub fn insert_snapshot(&mut self, snapshot_id: Vec<u8>) {
        if self.backup_snapshots.len() >= self.max_backup_snapshots {
            let _ = self.backup_snapshots.pop_front();
        }
        self.backup_snapshots.push_back(snapshot_id);
    }
}
