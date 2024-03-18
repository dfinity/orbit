//! # IC Canister Macros Library
//!
//! Common macros for IC canisters to expose reusable features.
//!
//! Some of the features include:
//!
//! - Entity generation for stable structures.
//! - Middleware injection for functions.

extern crate proc_macro;

mod macros;
mod utils;

use crate::macros::MacroDefinition;
use proc_macro::TokenStream;

/// The `with_middleware` procedural macro is designed to inject middleware functionality
/// into functions. It enables pre- or post-execution of specified middleware,
/// allowing for additional processing such as logging, authorization, or metrics gathering
/// without intruding into the function's core logic.
///
/// # Parameters
/// - `guard`: The name of the middleware function to be invoked. This function should
///   return a `()` and can optionally take a context object, additional arguments, and/or
///   the function result (for `when="after"`). The first argument of this function is always a tuple with
///   the name of the function to which the macro is attached and a list of the `args` added to the middleware.
/// - `when`: Specifies when the middleware is to be executed, either `"before"` or `"after"`
///   the function's execution. Defaults to `"before"` if not specified.
/// - `context`: (Optional) The name of a function that generates a context to be passed
///   to the middleware function. This function should return a context object.
/// - `args`: (Optional) Additional arguments to be passed to the middleware function. These
///   should be specified as an array of strings (e.g., `args = ["read:user", "write:data"]`).
/// - `is_async`: (Optional) Specifies whether the middleware function is asynchronous. Defaults to `false`
///   if not specified.
///
/// # Usage
/// Annotate functions with `#[with_middleware]`, specifying the middleware function, the
/// execution time (before/after), optionally a context generator function, and any additional arguments.
///
/// # Examples
///
/// Basic usage with a middleware function executed before the function body:
/// ```ignore
/// #[with_middleware(guard = "my_middleware")]
/// async fn my_function() {
///     // Function body
/// }
/// ```
///
/// Using a context function and executing middleware after the function body with additional arguments:
/// ```ignore
/// #[with_middleware(guard = "my_middleware", when = "after", context = "create_context", args = ["read:user", "write:data"])]
/// async fn my_function() {
///     // Function body
/// }
/// ```
///
/// # Middleware Function Signatures
/// - For `when="before"` without additional arguments:
///   ```ignore
///   fn my_middleware(middleware: (&'static str, &Vec<&'static str>), ctx: MyContext);
///   ```
/// - For `when="after"` with additional arguments:
///   ```ignore
///   fn my_middleware(middleware: (&'static str, &Vec<&'static str>), ctx: MyContext, args: Vec<String>, result: &FunctionResultType);
///   ```
///   Replace `MyContext` and `FunctionResultType` with appropriate types as per your application's design.
///
/// # Context Function Signature
/// If a context function is specified, it should have the following signature:
/// ```ignore
/// fn create_context() -> MyContext;
/// ```
///
/// # Notes
/// - Ensure that the middleware and context functions, as well as any additional arguments, are available in the scope where
///   the macro is used.
/// - The macro currently supports only function items. Other items like structs or enums
///   are not supported.
/// - The macro is designed to work with asynchronous functions.
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

/// The `storable` procedural macro is designed to generate serialization and deserialization
/// implementations for a given struct or enum. It allows for the serialization format to be
/// specified, as well as, optionally the maximum byte size that the object can be serialized to.
///
/// This macro is compatible with the `ic_stable_strcutures` crate and it can be used to generate
/// objects that can be stored in the stable memory of a canister.
///
/// # Parameters
///
/// - `size`: The maximum byte size that the object can be serialized to.
/// - `format`: The serialization format to be used. Supported formats are `cbor` and `candid`.
///
/// # Usage
///
/// Annotate a struct or enum with `#[storable]`.
///
/// # Examples
///
/// Basic usage with default parameters:
///
/// ```ignore
/// #[storable]
/// struct MyStruct {
///     field1: u32,
///     field2: String,
/// }
/// ```
///
/// Usage with custom parameters:
///
/// ```ignore
/// #[storable(size = 1000, format = "cbor")]
/// struct MyStruct {
///     field1: u32,
///     field2: String,
/// }
/// ```
///
/// # Notes
///
/// - The macro currently supports only struct and enum items.
///
/// # Notes on Serialization Formats
///
/// ## Candid
///
/// - Only accepts schema evolution without breaking changes in struct fields if the new field is `optional`.
///
/// ## CBOR
///
/// - Accepts schema evolution with field addition in structs, but only if serde's `default` attribute is used.
#[proc_macro_attribute]
pub fn storable(input_args: TokenStream, input: TokenStream) -> TokenStream {
    utils::handle_macro_errors(
        |input_args, input| {
            let macro_impl = macros::storable::StorableMacro::new(input_args, input);

            macro_impl.build()
        },
        macros::storable::StorableMacro::MACRO_NAME,
        input_args,
        input,
    )
}
