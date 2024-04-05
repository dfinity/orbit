//! # Canfund Library
//!
//! `canfund` is a library that provides a set of features for managing cycles of a canister.
//!
//! Those features include:
//!
//! - Monitoring of canister cycles.
//! - Adding cycles to a canister.

pub mod errors;
pub mod fetch;
pub mod manager;
pub mod types;
pub mod utils;

pub use manager::FundManager;
