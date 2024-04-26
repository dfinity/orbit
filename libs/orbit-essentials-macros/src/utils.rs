use proc_macro::TokenStream;
use syn::Error;

pub fn handle_macro_errors<F>(
    cb: F,
    _name: &str,
    metadata: TokenStream,
    item: TokenStream,
) -> TokenStream
where
    F: FnOnce(TokenStream, TokenStream) -> Result<TokenStream, Error>,
{
    let result = cb(metadata, item);

    result.map_or_else(|e| e.to_compile_error().into(), Into::into)
}
