//! # Station canister
//!
//! This canister provides a comphehensive set of APIs for managing digital assets.

pub const SERVICE_NAME: &str = "station";
pub const SYSTEM_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod controllers;
pub mod core;
pub mod errors;
pub mod factories;
pub mod jobs;
pub mod mappers;
pub mod models;
pub mod repositories;
pub mod services;
