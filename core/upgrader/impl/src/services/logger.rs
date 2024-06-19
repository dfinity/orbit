use std::{cell::RefCell, sync::Arc};

use ic_stable_structures::{memory_manager::MemoryId, Log};
use lazy_static::lazy_static;

use crate::{
    model::{LogEntry, LogEntryType},
    Memory, MEMORY_ID_LOG_DATA, MEMORY_ID_LOG_INDEX, MEMORY_MANAGER,
};

thread_local! {

  static STORAGE: RefCell<Log<LogEntry, Memory, Memory>> = RefCell::new(
      Log::init(
          MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID_LOG_INDEX))),
          MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID_LOG_DATA))),
      ).expect("Failed to initialize log storage")
  );

}

lazy_static! {
    pub static ref LOGGER_SERVICE: Arc<LoggerService> = Arc::new(LoggerService::default());
}

#[derive(Clone, Default)]
pub struct LoggerService {}

impl LoggerService {
    pub fn try_log(&self, entry_type: LogEntryType) -> Result<(), String> {
        let entry = LogEntry::try_from_entry_type(entry_type)?;
        STORAGE.with(|storage| {
            storage
                .borrow_mut()
                .append(&entry)
                .map_err(|err| format!("Failed to log entry: {:?}", err))
        })?;
        Ok(())
    }

    pub fn log(&self, entry_type: LogEntryType) {
        if let Err(err) = self.try_log(entry_type) {
            crate::upgrader_ic_cdk::api::print(format!("Failed to log entry: {}", err));
        }
    }
}
