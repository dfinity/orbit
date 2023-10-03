//! # IC Canister Macros Library
//!
//! Common macros for IC canisters to expose reusable features.
//!
//! Some of the features include:
//!
//! - Entity generation for stable structures.

extern crate proc_macro;

use proc_macro::TokenStream;

mod constants;
mod macros;
mod utils;

#[proc_macro_attribute]
pub fn stable_object(metadata: TokenStream, input: TokenStream) -> TokenStream {
    utils::handle_macro_errors(
        macros::dfn_stable_object_macro,
        "stable_object",
        metadata,
        input,
    )
}
