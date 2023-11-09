use proc_macro::TokenStream;
use syn::Error;

pub trait MacroDefinition {
    const MACRO_NAME: &'static str;

    fn new(input_args: TokenStream, input: TokenStream) -> Self;
    fn build(&self) -> Result<TokenStream, Error>;
}
