use super::MacroDefinition;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parser, parse2, Error, Token};

/// The arguments passed to the `with_middleware` macro.
///
/// The macro accepts a list of arguments separated by `,`.
#[derive(Debug)]
struct MacroArguments {
    /// The guard function to call.
    ///
    /// It is executed before the target function that it is attached to.
    pub guard: Option<TokenStream>,
    /// The tail function to call.
    ///
    /// It is executed after the target function that it is attached to, it has access to the
    /// result of the target function.
    pub tail: Option<TokenStream>,
    /// The context function to call before the middleware function.
    pub context: Option<TokenStream>,
}

#[derive(Debug)]
pub struct WithMiddlewareMacro {
    input_args: TokenStream,
    input: TokenStream,
}

impl MacroDefinition for WithMiddlewareMacro {
    const MACRO_NAME: &'static str = "with_middleware";

    fn new(input_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> Self {
        Self {
            input: input.into(),
            input_args: input_args.into(),
        }
    }

    fn build(&self) -> Result<proc_macro::TokenStream, Error> {
        let args: MacroArguments = self.parse_input_arguments()?;
        let expanded_input = self.expand_implementation(&args)?;

        Ok(expanded_input.into())
    }
}

impl WithMiddlewareMacro {
    const MACRO_ARG_KEY_CONTEXT: &'static str = "context";
    const MACRO_ARG_KEY_GUARD: &'static str = "guard";
    const MACRO_ARG_KEY_TAIL: &'static str = "tail";

    fn expand_implementation(&self, args: &MacroArguments) -> Result<TokenStream, Error> {
        let parsed_input: syn::Item = parse2(self.input.clone())?;

        match &parsed_input {
            syn::Item::Fn(syn::ItemFn {
                attrs,
                vis,
                sig,
                block,
            }) => {
                let target_fn_name = &sig.ident;

                // Extract the return type from the signature
                let return_type = match &sig.output {
                    syn::ReturnType::Default => quote! { () },
                    syn::ReturnType::Type(_, ty) => quote! { #ty },
                };

                let with_context = match &args.context {
                    Some(context_fn) => {
                        quote! { let context = #context_fn; }
                    }
                    None => quote! {},
                };

                if args.guard.is_none() && args.tail.is_none() {
                    return Err(Error::new_spanned(
                        parsed_input,
                        format!(
                            "At least one of the \"{}\" or \"{}\" arguments must be passed to the \"{}\" macro",
                            Self::MACRO_ARG_KEY_GUARD,
                            Self::MACRO_ARG_KEY_TAIL,
                            Self::MACRO_NAME
                        ),
                    ));
                }

                let guard = match &args.guard {
                    Some(guard_fn) => quote! { #guard_fn; },
                    _ => quote! {},
                };

                let tail = match &args.tail {
                    Some(tail_fn) => quote! { #tail_fn; },
                    _ => quote! {},
                };

                let expanded = quote! {
                    #(#attrs)* #vis #sig {
                        let __target_fn = stringify!(#target_fn_name);

                        // The context should be created before anything else as it can be used by to add additional
                        // information such as the execution time of the function.
                        #with_context

                        // Executes the middleware function before the function
                        #guard

                        // The async block should be directly within the async function
                        let result: #return_type = async move #block.await;

                        // Executes the middleware function after the function, has access to the result and the context
                        #tail

                        result
                    }
                };

                Ok(expanded)
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
        let args = parser.parse(self.input_args.clone().into())?;

        let mut attach_guard: Option<TokenStream> = None;
        let mut attach_tail: Option<TokenStream> = None;
        let mut attach_context: Option<TokenStream> = None;

        for expr in args {
            let syn::ExprAssign {
                left,
                right,
                attrs: _,
                eq_token: _,
            } = expr;

            if let syn::Expr::Path(expr_path) = *left {
                match expr_path.path.get_ident().unwrap().to_string().as_str() {
                    Self::MACRO_ARG_KEY_GUARD => {
                        attach_guard = Some(right.to_token_stream());
                    }
                    Self::MACRO_ARG_KEY_CONTEXT => {
                        attach_context = Some(right.to_token_stream());
                    }
                    Self::MACRO_ARG_KEY_TAIL => {
                        attach_tail = Some(right.to_token_stream());
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

        Ok(MacroArguments {
            guard: attach_guard,
            tail: attach_tail,
            context: attach_context,
        })
    }
}
