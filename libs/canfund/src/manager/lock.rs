use std::{cell::RefCell, collections::HashSet, rc::Rc};

#[derive(Debug, Clone)]
pub struct ProcessExecutionLock {
    processes: Rc<RefCell<HashSet<Vec<u8>>>>,
}

impl ProcessExecutionLock {
    pub fn new() -> Self {
        Self {
            processes: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn is_executing(&self, process_id: &[u8]) -> bool {
        self.processes.borrow().contains(process_id)
    }

    pub fn lock(&mut self, process_id: Vec<u8>) -> Option<RunningProcess> {
        if self.is_executing(&process_id) {
            return None;
        }

        self.processes.borrow_mut().insert(process_id.clone());

        Some(RunningProcess::new(Rc::clone(&self.processes), process_id))
    }

    pub fn clear(&mut self) {
        self.processes.borrow_mut().clear();
    }
}

impl Default for ProcessExecutionLock {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct RunningProcess {
    processes: Rc<RefCell<HashSet<Vec<u8>>>>,
    process_id: Vec<u8>,
}

impl RunningProcess {
    pub fn new(processes: Rc<RefCell<HashSet<Vec<u8>>>>, process_id: Vec<u8>) -> Self {
        Self {
            processes,
            process_id,
        }
    }
}

impl Drop for RunningProcess {
    fn drop(&mut self) {
        self.processes.borrow_mut().remove(&self.process_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locks_same_process_id() {
        let mut lock = ProcessExecutionLock::new();

        let process_id = vec![1, 2, 3];

        let running_process = lock.lock(process_id.clone()).unwrap();

        assert!(lock.is_executing(&process_id));

        assert!(lock.lock(process_id.clone()).is_none());

        drop(running_process);

        assert!(!lock.is_executing(&process_id));
    }

    #[test]
    fn test_locks_different_process_id() {
        let mut lock = ProcessExecutionLock::new();

        let process_id_1 = vec![1, 2, 3];
        let process_id_2 = vec![4, 5, 6];

        let running_process_1 = lock.lock(process_id_1.clone()).unwrap();

        assert!(lock.is_executing(&process_id_1));
        assert!(!lock.is_executing(&process_id_2));

        let running_process_2 = lock.lock(process_id_2.clone()).unwrap();

        drop(running_process_1);

        assert!(!lock.is_executing(&process_id_1));
        assert!(lock.is_executing(&process_id_2));

        drop(running_process_2);

        assert!(!lock.is_executing(&process_id_1));
        assert!(!lock.is_executing(&process_id_2));
    }

    #[test]
    fn test_locks_processes_indepedently() {
        let mut lock = ProcessExecutionLock::new();

        let process_id_1 = vec![1, 2, 3];
        let process_id_2 = vec![4, 5, 6];

        let running_process_1 = lock.lock(process_id_1.clone()).unwrap();
        let running_process_2 = lock.lock(process_id_2.clone()).unwrap();

        assert!(lock.is_executing(&process_id_1));
        assert!(lock.is_executing(&process_id_2));

        drop(running_process_1);

        assert!(!lock.is_executing(&process_id_1));
        assert!(lock.is_executing(&process_id_2));

        drop(running_process_2);

        assert!(!lock.is_executing(&process_id_1));
        assert!(!lock.is_executing(&process_id_2));
    }
}
