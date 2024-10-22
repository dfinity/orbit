use std::{cell::RefCell, sync::Mutex};

mod assets;
mod canister_call;
mod install;
mod manage_settings;
mod me;
mod review;
mod setup;
mod util;

thread_local! {static PORT: RefCell<u16> = const { RefCell::new(4943) };}
static AGENT_MUTEX: Mutex<()> = Mutex::new(());

const DFX_ROOT: &str = "DFX_CONFIG_ROOT";
