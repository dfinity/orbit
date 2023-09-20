//! # Control panel canister
//!
//! The control panel canister is responsible for providing helper functions for the Orbit Wallet user interface.

#[macro_use]
extern crate lazy_static;

pub mod core;
pub mod controllers;
pub mod entities;
pub mod errors;
pub mod mappers;
pub mod repositories;
pub mod services;
pub mod transport;
