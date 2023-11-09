use crate::interface::MacroDefinition;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse2, Error, Token};

/// The arguments passed to the macro.
///
/// The macro accepts a list of arguments separated by `,`.
#[derive(Clone, Debug)]
struct MacroArguments {
    pub prefix: Option<String>,
}

#[derive(Debug)]
pub struct WithLogsMacro {
    input_args: TokenStream,
    input: TokenStream,
}

impl MacroDefinition for WithLogsMacro {
    const MACRO_NAME: &'static str = "with_logs";

    fn new(input_args: TokenStream, input: TokenStream) -> Self {
        Self { input, input_args }
    }

    fn build(&self) -> Result<TokenStream, Error> {
        let args: MacroArguments = self.parse_input_arguments()?;
        let expanded_input = self.expand_implementation(&args)?;

        Ok(expanded_input)
    }
}

impl WithLogsMacro {
    /// The name of the argument that specifies an optional prefix to add to the logs.
    const MACRO_ARG_KEY_LOG_PREFIX: &str = "prefix";

    fn expand_implementation(&self, args: &MacroArguments) -> Result<TokenStream, Error> {
        let parsed_input: syn::Item = parse2(self.input.clone().into())?;

        match parsed_input {
            syn::Item::Fn(syn::ItemFn {
                attrs,
                vis,
                sig,
                block,
            }) => {
                let fn_name = &sig.ident;
                let log_prefix = match &args.prefix {
                    Some(prefix) => format!("[{}]: ", prefix),
                    None => String::new(),
                };

                // Extract the return type from the signature
                let return_type = match &sig.output {
                    syn::ReturnType::Default => quote! { () },
                    syn::ReturnType::Type(_, ty) => quote! { #ty },
                };

                let expanded = quote! {
                    #(#attrs)* #vis #sig {
                        #[cfg(not(test))]
                        use ic_canister_core::cdk as ic_cdk;
                        #[cfg(test)]
                        use ic_canister_core::cdk::mocks as ic_cdk;

                        ic_cdk::api::print(serde_json::to_string(&ic_canister_core::types::LogMessage {
                            function: stringify!(#fn_name).to_string(),
                            message: format!("{}started execution", #log_prefix),
                            timestamp: ic_cdk::api::time(),
                        }).expect("Failed to serialize log message"));

                        // The async block should be directly within the async function
                        let result: #return_type = async move #block.await;

                        ic_cdk::api::print(serde_json::to_string(&ic_canister_core::types::LogMessage {
                            function: stringify!(#fn_name).to_string(),
                            message: format!("{}completed execution with result {:?}", #log_prefix, result),
                            timestamp: ic_cdk::api::time(),
                        }).expect("Failed to serialize log message"));

                        result
                    }
                };

                Ok(expanded.into())
            }
            _ => Err(Error::new_spanned(
                parsed_input,
                format!(
                    "Only functions are supported by the \"{}\" macro",
                    Self::MACRO_NAME
                ),
            )),
        }
    }

    fn parse_input_arguments(&self) -> Result<MacroArguments, Error> {
        let parser = syn::punctuated::Punctuated::<syn::ExprAssign, Token![,]>::parse_terminated;
        let args = parser.parse(self.input_args.clone())?;

        let mut log_prefix: Option<String> = None;

        for expr in args {
            let syn::ExprAssign {
                left,
                right,
                attrs: _,
                eq_token: _,
            } = expr;

            if let syn::Expr::Path(expr_path) = *left {
                match expr_path.path.get_ident().unwrap().to_string().as_str() {
                    Self::MACRO_ARG_KEY_LOG_PREFIX => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = *right
                        {
                            log_prefix = Some(lit_str.value());
                        }
                    }
                    unknown_arg => {
                        return Err(Error::new(
                            expr_path.path.get_ident().unwrap().span(),
                            format!(
                                "Unknown argument \"{}\" passed to the \"{}\" macro",
                                unknown_arg,
                                Self::MACRO_NAME
                            ),
                        ));
                    }
                }
            }
        }

        Ok(MacroArguments { prefix: log_prefix })
    }
}
