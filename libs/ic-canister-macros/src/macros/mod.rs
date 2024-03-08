use proc_macro::TokenStream;
use syn::Error;

pub mod storable;
pub mod with_middleware;

pub trait MacroDefinition {
    const MACRO_NAME: &'static str;

    fn new(input_args: TokenStream, input: TokenStream) -> Self;
    fn build(&self) -> Result<TokenStream, Error>;
}
