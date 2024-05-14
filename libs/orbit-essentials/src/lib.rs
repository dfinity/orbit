//! # IC Canister Core Library
//!
//! Common features and utilities for IC canisters that are built on top of the CDK and expose reusable features.
//!
//! Some of the features include:
//!
//! - Ramdon number generation.
//! - UUID generation.

pub mod api;
pub mod cdk;
pub mod deserialize;
pub mod metrics;
pub mod model;
pub mod repository;
pub mod timers;

pub mod types;
pub mod utils;

pub use ic_stable_structures;
pub use orbit_essentials_macros::*;
