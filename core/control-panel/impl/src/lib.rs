//! # Control panel canister
//!
//! The control panel canister is responsible for providing helper functions for the Orbit Wallet user interface.

pub const SERVICE_NAME: &str = "control_panel";
pub const SYSTEM_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod controllers;
pub mod core;
pub mod errors;
pub mod mappers;
pub mod models;
pub mod repositories;
pub mod services;
