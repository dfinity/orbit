use super::MacroDefinition;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parser, parse2, Error, Token};

/// The arguments passed to the `with_middleware` macro.
///
/// The macro accepts a list of arguments separated by `,`.
#[derive(Clone, Debug)]
struct MacroArguments {
    /// The name of the middleware function to call.
    ///
    /// This is a function that returns a `Result<(), String>` and it can take as an argument the
    /// `context` if set and the result of the function when set to "after". The first argument is always
    /// the function name that it was attached to.
    pub function: String,
    /// When to call the middleware function, possible values are "before" and "after".
    ///
    /// Default value is "before".
    pub when: String,
    /// The context to pass to the middleware function.
    ///
    /// This is a function that creates a context to pass to the middleware function.
    pub context: Option<String>,
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
    const MACRO_ARG_KEY_GUARD: &str = "guard";
    const MACRO_ARG_KEY_WHEN: &str = "when";
    const MACRO_ARG_KEY_CONTEXT: &str = "context";

    fn expand_implementation(&self, args: &MacroArguments) -> Result<TokenStream, Error> {
        let parsed_input: syn::Item = parse2(self.input.clone().into())?;

        match &parsed_input {
            syn::Item::Fn(syn::ItemFn {
                attrs,
                vis,
                sig,
                block,
            }) => {
                let fn_name = &sig.ident;
                let middleware_fn = syn::Ident::new(&args.function, Span::call_site());
                let mut use_before = false;
                let mut use_after = false;
                let mut with_context = false;

                match args.when.as_str() {
                    "before" => {
                        use_before = true;
                    }
                    "after" => {
                        use_after = true;
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

                let context_expansion = match &args.context {
                    Some(context_fn_name) => {
                        with_context = true;
                        let context_fn = syn::Ident::new(context_fn_name, Span::call_site());
                        quote! { let context = #context_fn (); }
                    }
                    None => quote! {},
                };

                let before_expansion = match (use_before, with_context) {
                    (true, true) => quote! { #middleware_fn (stringify!(#fn_name), context); },
                    (true, false) => quote! { #middleware_fn (stringify!(#fn_name)); },
                    (_, _) => quote! {},
                };

                let after_expansion = match (use_after, with_context) {
                    (true, true) => {
                        quote! { #middleware_fn (stringify!(#fn_name), context, &result); }
                    }
                    (true, false) => quote! { #middleware_fn (stringify!(#fn_name), &result); },
                    (_, _) => quote! {},
                };

                let expanded = quote! {
                    #(#attrs)* #vis #sig {
                        // The context should be created before anything else as it can be used by to add additional
                        // information such as the execution time of the function.
                        #context_expansion

                        #before_expansion

                        // The async block should be directly within the async function
                        let result: #return_type = async move #block.await;

                        #after_expansion

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
        let mut attach_context: Option<String> = None;

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

                    Self::MACRO_ARG_KEY_CONTEXT => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = *right
                        {
                            attach_context = Some(lit_str.value());
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

        match (attach_fn, attach_when, attach_context) {
            (Some(attach_fn), Some(attach_when), ctx) => Ok(MacroArguments {
                function: attach_fn,
                when: attach_when,
                context: ctx,
            }),
            (None, _, _) => Err(Error::new(
                Span::call_site(),
                format!(
                    "Missing argument \"{}\" passed to the \"{}\" macro",
                    Self::MACRO_ARG_KEY_GUARD,
                    Self::MACRO_NAME
                ),
            )),
            (_, None, _) => Err(Error::new(
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
