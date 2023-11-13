//! # IC Canister Macros Library
//!
//! Common macros for IC canisters to expose reusable features.
//!
//! Some of the features include:
//!
//! - Entity generation for stable structures.

extern crate proc_macro;

mod macros;
mod utils;

use crate::macros::MacroDefinition;
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

/// The `with_middleware` procedural macro is designed to inject middleware functionality
/// into Rust functions. It enables pre- or post-execution of specified middleware,
/// allowing for additional processing such as logging, authorization, or metrics gathering
/// without intruding into the function's core logic.
///
/// # Parameters
/// - `function`: The name of the middleware function to be invoked. This function should
///   return a `()` and can optionally take a context object and/or the
///   function result (for `when="after"`). The first argument of this function is always
///   the name of the function to which the macro is attached.
/// - `when`: Specifies when the middleware is to be executed, either `"before"` or `"after"`
///   the function's execution. Defaults to `"before"` if not specified.
/// - `context`: (Optional) The name of a function that generates a context to be passed
///   to the middleware function. This function should return a context object.
///
/// # Usage
/// Annotate functions with `#[with_middleware]`, specifying the middleware function, the
/// execution time (before/after), and optionally a context generator function.
///
/// # Examples
///
/// Basic usage with a middleware function executed before the function body:
/// ```ignore
/// #[with_middleware(function = "my_middleware")]
/// async fn my_function() {
///     // Function body
/// }
/// ```
///
/// Using a context function and executing middleware after the function body:
/// ```ignore
/// #[with_middleware(function = "my_middleware", when = "after", context = "create_context")]
/// async fn my_function() {
///     // Function body
/// }
/// ```
///
/// # Middleware Function Signatures
/// - For `when="before"`:
///   ```ignore
///   fn my_middleware(fn_name: &'static str, ctx: MyContext) -> Result<(), String>;
///   ```
/// - For `when="after"`:
///   ```ignore
///   fn my_middleware(fn_name: &'static str, ctx: MyContext, result: &FunctionResultType);
///   ```
///   Replace `MyContext` and `FunctionResultType` with appropriate types as per your application's design.
///
/// # Context Function Signature
/// If a context function is specified, it should have the following signature:
/// ```ignore
/// fn build_context() -> MyContext;
/// ```
///
/// # Notes
/// - Ensure that the middleware and context functions are available in the scope where
///   the macro is used.
/// - The macro currently supports only function items. Other items like structs or enums
///   are not supported.
/// - The macro is designed to work with async functions.
#[proc_macro_attribute]
pub fn with_middleware(input_args: TokenStream, input: TokenStream) -> TokenStream {
    utils::handle_macro_errors(
        |input_args, input| {
            let macro_impl = macros::with_middleware::WithMiddlewareMacro::new(input_args, input);

            macro_impl.build()
        },
        macros::with_middleware::WithMiddlewareMacro::MACRO_NAME,
        input_args,
        input,
    )
}
