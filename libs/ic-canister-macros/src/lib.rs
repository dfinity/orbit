//! # IC Canister Macros Library
//!
//! Common macros for IC canisters to expose reusable features.
//!
//! Some of the features include:
//!
//! - Entity generation for stable structures.

extern crate proc_macro;

mod constants;
mod interface;
mod macros;
mod utils;

use interface::MacroDefinition;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn stable_object(metadata: TokenStream, input: TokenStream) -> TokenStream {
    utils::handle_macro_errors(
        macros::dfn_stable_object_macro,
        "stable_object",
        metadata,
        input,
    )
}

/// A procedural macro to automatically log the start and end of async functions.
/// This macro is designed to be used on async functions to provide consistent logging
/// for function entry and exit points, including the result of the function.
///
/// # Parameters
/// - `prefix`: An optional string literal that will be prepended to log messages for
///   identification. If not provided, no prefix will be used.
///
/// # Usage
/// Apply this macro to async functions in an implementation block where you want
/// automatic logging.
///
/// # Examples
///
/// Without a prefix:
/// ```ignore
/// #[with_logs]
/// async fn my_async_function(&self) -> Result<MyType, MyError> {
///     // Function implementation
/// }
/// ```
///
/// With a prefix:
/// ```ignore
/// #[with_logs(prefix = "testing")]
/// async fn my_async_function(&self) -> Result<MyType, MyError> {
///     // Function implementation
/// }
/// ```
///
/// # Notes
/// - This macro is intended for use with `async` functions only.
/// - The macro will log the start of the function with "Function <function_name> started"
///   and the end with "Function <function_name> completed with result <result>".
/// - The result of the function is expected to implement `std::fmt::Debug` to be logged.
///
/// # Compatibility
/// This macro is not compatible with the `async_trait` macro.
///
/// # Errors
/// If the macro is applied to a non-async function, or if the result type does not
/// implement `Debug`, it will result in a compilation error.
#[proc_macro_attribute]
pub fn with_logs(input_args: TokenStream, input: TokenStream) -> TokenStream {
    utils::handle_macro_errors(
        |input_args, input| {
            let macro_impl = macros::with_logs::WithLogsMacro::new(input_args, input);

            macro_impl.build()
        },
        macros::with_logs::WithLogsMacro::MACRO_NAME,
        input_args,
        input,
    )
}
