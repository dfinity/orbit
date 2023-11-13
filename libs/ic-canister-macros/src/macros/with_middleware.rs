use crate::interface::MacroDefinition;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parser, parse2, Error, Token};

/// The arguments passed to the macro.
///
/// The macro accepts a list of arguments separated by `,`.
#[derive(Clone, Debug)]
struct MacroArguments {
    pub attach: String,
    pub when: String,
}

#[derive(Debug)]
pub struct WithMiddlewareMacro {
    input_args: TokenStream,
    input: TokenStream,
}

impl MacroDefinition for WithMiddlewareMacro {
    const MACRO_NAME: &'static str = "with_middleware";

    fn new(input_args: TokenStream, input: TokenStream) -> Self {
        Self { input, input_args }
    }

    fn build(&self) -> Result<TokenStream, Error> {
        let args: MacroArguments = self.parse_input_arguments()?;
        let expanded_input = self.expand_implementation(&args)?;

        Ok(expanded_input)
    }
}

impl WithMiddlewareMacro {
    /// The name of the middleware function to call.
    const MACRO_ARG_KEY_ATTACH: &str = "attach";
    /// Specifies when to call the middleware function, possible values are "before" and "after".
    const MACRO_ARG_KEY_WHEN: &str = "when";

    fn expand_implementation(&self, args: &MacroArguments) -> Result<TokenStream, Error> {
        let parsed_input: syn::Item = parse2(self.input.clone().into())?;

        match &parsed_input {
            syn::Item::Fn(syn::ItemFn {
                attrs,
                vis,
                sig,
                block,
            }) => {
                // let fn_name = &sig.ident;
                let middleware_fn = syn::Ident::new(&args.attach, Span::call_site());
                let mut middleware_before_fn = quote! {};
                let mut middleware_after_fn = quote! {};

                match args.when.as_str() {
                    "before" => {
                        middleware_before_fn = quote! {
                            #middleware_fn ().expect("Middleware failed");
                        };
                    }
                    "after" => {
                        middleware_after_fn = quote! {
                            #middleware_fn (&result).expect("Middleware failed");
                        };
                    }
                    _ => {
                        return Err(Error::new_spanned(
                            parsed_input,
                            format!(
                                "Unknown value \"{}\" passed to the \"{}\" macro `when` argument",
                                args.when,
                                Self::MACRO_NAME
                            ),
                        ));
                    }
                };

                // Extract the return type from the signature
                let return_type = match &sig.output {
                    syn::ReturnType::Default => quote! { () },
                    syn::ReturnType::Type(_, ty) => quote! { #ty },
                };

                let expanded = quote! {
                    #(#attrs)* #vis #sig {
                        #middleware_before_fn

                        // The async block should be directly within the async function
                        let result: #return_type = async move #block.await;

                        #middleware_after_fn

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

        let mut attach_fn: Option<String> = None;
        let mut attach_when: Option<String> = Some(String::from("before"));

        for expr in args {
            let syn::ExprAssign {
                left,
                right,
                attrs: _,
                eq_token: _,
            } = expr;

            if let syn::Expr::Path(expr_path) = *left {
                match expr_path.path.get_ident().unwrap().to_string().as_str() {
                    Self::MACRO_ARG_KEY_ATTACH => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = *right
                        {
                            attach_fn = Some(lit_str.value());
                        }
                    }
                    Self::MACRO_ARG_KEY_WHEN => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = *right
                        {
                            attach_when = Some(lit_str.value());
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

        match (attach_fn, attach_when) {
            (Some(attach_fn), Some(attach_when)) => Ok(MacroArguments {
                attach: attach_fn,
                when: attach_when,
            }),
            (None, _) => Err(Error::new(
                Span::call_site(),
                format!(
                    "Missing argument \"{}\" passed to the \"{}\" macro",
                    Self::MACRO_ARG_KEY_ATTACH,
                    Self::MACRO_NAME
                ),
            )),
            (_, None) => Err(Error::new(
                Span::call_site(),
                format!(
                    "Missing argument \"{}\" passed to the \"{}\" macro",
                    Self::MACRO_ARG_KEY_WHEN,
                    Self::MACRO_NAME
                ),
            )),
        }
    }
}
