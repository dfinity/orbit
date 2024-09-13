use std::{cell::RefCell, sync::Mutex};

mod assets;
mod canister_call;
mod me;
mod review;
mod setup;
mod util;

thread_local! {static PORT: RefCell<u16> = const { RefCell::new(4943) };}
static AGENT_MUTEX: Mutex<()> = Mutex::new(());

const DFX_ROOT: &str = "DFX_CONFIG_ROOT";

// TODO: Integration test for update settings
// TODO: Use the arguments in the system tests to cover more code of the actual tool
