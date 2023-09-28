//! # Bank canister
//!
//! The bank canister provides a comphehensive set of APIs for managing wallets and crypto assets.

pub mod blockchains;
pub mod controllers;
pub mod core;
pub mod errors;
pub mod generated;
pub mod mappers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod transport;
pub mod types;
