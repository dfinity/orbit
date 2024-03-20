//! This module contains the error types that are used throughout the project.
//!
//! Error types should be specific and provide the necessary contextual information to understand the error.

mod account;
pub use account::*;

mod address_book;
pub use address_book::*;

mod metadata;
pub use metadata::*;

mod user;
pub use user::*;

mod user_group;
pub use user_group::*;

mod mapper;
pub use mapper::*;

mod notification;
pub use notification::*;

mod factory;
pub use factory::*;

mod blockchain_api;
pub use blockchain_api::*;

mod repository;
pub use repository::*;

mod transfer;
pub use transfer::*;

mod proposal;
pub use proposal::*;

mod proposal_execute;
pub use proposal_execute::*;

mod evaluate;
pub use evaluate::*;

mod specifier;
pub use specifier::*;

mod authorization;
pub use authorization::*;

mod pagination;
pub use pagination::*;

mod install;
pub use install::*;

mod change_canister;
pub use change_canister::*;
