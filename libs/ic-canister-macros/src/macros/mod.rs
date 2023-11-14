mod stable_object;
use proc_macro::TokenStream;
pub use stable_object::*;
use syn::Error;

pub mod with_middleware;

pub trait MacroDefinition {
    const MACRO_NAME: &'static str;

    fn new(input_args: TokenStream, input: TokenStream) -> Self;
    fn build(&self) -> Result<TokenStream, Error>;
}
