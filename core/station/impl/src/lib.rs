//! # Station canister
//!
//! This canister provides a comphehensive set of APIs for managing digital assets.

pub const SERVICE_NAME: &str = "station";
pub const SYSTEM_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const STABLE_MEMORY_VERSION: u32 = 3;

pub mod controllers;
pub mod core;
pub mod errors;
pub mod factories;
pub mod jobs;
mod macros;
pub mod mappers;
pub mod migration;
pub mod migration_tests;
pub mod models;
pub mod repositories;
pub mod services;
