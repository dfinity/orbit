use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse2, DeriveInput, Error, Token};

/// The arguments passed to the `stable_object` macro.
///
/// The macro accepts a list of arguments separated by `,`.
#[derive(Clone, Debug)]
struct StableObjectArguments {
    pub size: Option<u32>,
}

/// The name of the argument that specifies the size of the stable memory layout.
const STABLE_OBJECT_ARGUMENT_SIZE_KEY: &str = "size";

pub fn dfn_stable_object_macro(
    args: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, Error> {
    let args: StableObjectArguments = parse_stable_object_macro_arguments(args)?;
    let expanded_input = stable_object_impl(input.clone(), args.clone())?;

    Ok(expanded_input)
}

fn stable_object_impl(
    input: TokenStream,
    args: StableObjectArguments,
) -> Result<TokenStream, Error> {
    let parsed_input: DeriveInput = parse2(input.into())?;

    match parsed_input.data {
        syn::Data::Struct(_) | syn::Data::Enum(_) => {
            let object_input = parsed_input.clone();
            let object_name = object_input.ident.clone();
            let size_value: Option<u32> = args.size;

            let expanded = match size_value {
                Some(size) => quote! {
                    #object_input

                    impl ic_stable_structures::Storable for #object_name {
                        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                            use candid::Encode;

                            std::borrow::Cow::Owned(candid::Encode!(self).unwrap())
                        }

                        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                            use candid::Decode;

                            candid::Decode!(bytes.as_ref(), Self).unwrap()
                        }

                        const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
                            max_size: #size,
                            is_fixed_size: false,
                        };
                    }
                },
                None => quote! {
                    #object_input

                    impl ic_stable_structures::Storable for #object_name {
                        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                            use candid::Encode;

                            std::borrow::Cow::Owned(candid::Encode!(self).unwrap())
                        }

                        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                            use candid::Decode;

                            candid::Decode!(bytes.as_ref(), Self).unwrap()
                        }

                        const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
                    }
                },
            };

            Ok(expanded.into())
        }
        _ => Err(Error::new_spanned(
            parsed_input,
            "Only structs and enums are supported by the stable_object macro",
        )),
    }
}

fn parse_stable_object_macro_arguments(args: TokenStream) -> Result<StableObjectArguments, Error> {
    let parser = syn::punctuated::Punctuated::<syn::ExprAssign, Token![,]>::parse_terminated;
    let args = parser.parse(args)?;

    // The byte size to allocate for the stable memory layout.
    //
    // This is the default value, it can be overridden by the user using the `size` argument.
    let mut size: Option<u32> = None;

    for expr in args {
        let syn::ExprAssign {
            left,
            right,
            attrs: _,
            eq_token: _,
        } = expr;

        if let syn::Expr::Path(expr_path) = *left {
            match expr_path.path.get_ident().unwrap().to_string().as_str() {
                STABLE_OBJECT_ARGUMENT_SIZE_KEY => {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(lit_int),
                        ..
                    }) = *right
                    {
                        size = Some(lit_int.base10_parse()?);
                    }
                }
                unknown_arg => {
                    return Err(Error::new(
                        expr_path.path.get_ident().unwrap().span(),
                        format!(
                            "Unknown argument \"{}\" passed to the stable_object macro",
                            unknown_arg
                        ),
                    ));
                }
            }
        }
    }

    Ok(StableObjectArguments { size })
}
