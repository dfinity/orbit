use super::MacroDefinition;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::{parse::Parser, parse2, DeriveInput, Error, Token};

/// The arguments passed to the `storable` macro.
///
/// The macro accepts a list of arguments separated by `,`.
#[derive(Debug)]
struct MacroArguments {
    /// The maximum byte size that the object can be serialized to.
    ///
    /// This should only be used when the object is of a fixed size and the size is known at compile time, otherwise,
    /// the bytes will be reserved but not used.
    pub size: Option<u32>,

    /// The name of the serializer to use for the object.
    ///
    /// This should be the serialization format that the object should be serialized to and deserialized from.
    ///
    /// If this is not provided, the object will be serialized to and deserialized from the default serialization format.
    pub serializer: SerializerFormat,

    /// Wether or not deserialize should be implemented for the object.
    ///
    /// This is useful to provide a custom deserialize implementation for the object.
    pub skip_deserialize: bool,
}

impl Default for MacroArguments {
    fn default() -> Self {
        Self {
            size: None,
            serializer: SerializerFormat::Cbor,
            skip_deserialize: false,
        }
    }
}

#[derive(Debug)]
pub struct StorableMacro {
    input_args: TokenStream,
    input: TokenStream,
}

impl MacroDefinition for StorableMacro {
    const MACRO_NAME: &'static str = "storable";

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

impl StorableMacro {
    const MACRO_ARG_KEY_SIZE: &'static str = "size";
    const MACRO_ARG_KEY_SERIALIZER: &'static str = "serializer";
    const MACRO_ARG_KEY_SKIP_DESERIALIZE: &'static str = "skip_deserialize";

    fn expand_implementation(&self, args: &MacroArguments) -> Result<TokenStream, Error> {
        let parsed_input: DeriveInput = parse2(self.input.clone())?;

        match parsed_input.data {
            syn::Data::Struct(_) | syn::Data::Enum(_) => {
                let input = parsed_input.clone();

                match &args.serializer {
                    SerializerFormat::Candid => expand_candid_impl(&input, args),
                    SerializerFormat::Cbor => expand_cbor_impl(&input, args),
                }
            }
            _ => Err(Error::new_spanned(
                parsed_input,
                "Only structs and enums are supported by the storable macro",
            )),
        }
    }

    fn parse_input_arguments(&self) -> Result<MacroArguments, Error> {
        let parser = syn::punctuated::Punctuated::<syn::ExprAssign, Token![,]>::parse_terminated;
        let raw_args = parser.parse(self.input_args.clone().into())?;
        let mut args = MacroArguments::default();

        for expr in raw_args {
            let syn::ExprAssign {
                left,
                right,
                attrs: _,
                eq_token: _,
            } = expr;

            if let syn::Expr::Path(expr_path) = *left {
                match expr_path.path.get_ident().unwrap().to_string().as_str() {
                    Self::MACRO_ARG_KEY_SIZE => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Int(lit_int),
                            ..
                        }) = *right
                        {
                            args.size = Some(lit_int.base10_parse()?);
                        }
                    }

                    Self::MACRO_ARG_KEY_SERIALIZER => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = *right
                        {
                            args.serializer = SerializerFormat::from_str(&lit_str.value())
                                .map_err(|err| {
                                    Error::new(
                                        lit_str.span(),
                                        format!(
                                            "Invalid value for the \"{}\" argument: {}",
                                            Self::MACRO_ARG_KEY_SERIALIZER,
                                            err
                                        ),
                                    )
                                })?;
                        }
                    }

                    Self::MACRO_ARG_KEY_SKIP_DESERIALIZE => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Bool(lit_bool),
                            ..
                        }) = *right
                        {
                            args.skip_deserialize = lit_bool.value;
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

        Ok(args)
    }
}

fn expand_candid_impl(
    input: &DeriveInput,
    args: &MacroArguments,
) -> Result<proc_macro2::TokenStream, Error> {
    let object_name = input.ident.clone();
    let storage_bounds = storage_bounds(args.size);
    let derive_serialize = quote! { #[derive(candid::CandidType)] };
    let derive_deserialize = if args.skip_deserialize {
        quote! {}
    } else {
        quote! { #[derive(candid::Deserialize)] }
    };

    let expanded = quote! {
        #derive_serialize
        #derive_deserialize
        #input

        impl orbit_essentials::ic_stable_structures::Storable for #object_name {
            fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                use candid::Encode;

                std::borrow::Cow::Owned(candid::Encode!(self).unwrap())
            }

            fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                use candid::Decode;

                candid::Decode!(bytes.as_ref(), Self).unwrap()
            }

            #storage_bounds
        }
    };

    Ok(expanded)
}

fn expand_cbor_impl(
    input: &DeriveInput,
    args: &MacroArguments,
) -> Result<proc_macro2::TokenStream, Error> {
    let object_name = input.ident.clone();
    let storage_bounds = storage_bounds(args.size);
    let derive_serialize = quote! { #[derive(serde::Serialize)] };
    let derive_deserialize = if args.skip_deserialize {
        quote! {}
    } else {
        quote! { #[derive(serde::Deserialize)] }
    };

    let expanded = quote! {
        #derive_serialize
        #derive_deserialize
        #input

        impl orbit_essentials::ic_stable_structures::Storable for #object_name {
            fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                std::borrow::Cow::Owned(serde_cbor::to_vec(self).unwrap())
            }

            fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                serde_cbor::from_slice(bytes.as_ref()).unwrap()
            }

            #storage_bounds
        }
    };

    Ok(expanded)
}

fn storage_bounds(size: Option<u32>) -> proc_macro2::TokenStream {
    match size {
        Some(size) => quote! {
            const BOUND: orbit_essentials::ic_stable_structures::storable::Bound = orbit_essentials::ic_stable_structures::storable::Bound::Bounded {
                max_size: #size,
                is_fixed_size: false,
            };
        },
        None => quote! {
            const BOUND: orbit_essentials::ic_stable_structures::storable::Bound = orbit_essentials::ic_stable_structures::storable::Bound::Unbounded;
        },
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SerializerFormat {
    Candid,
    Cbor,
}

impl std::str::FromStr for SerializerFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "candid" => Ok(Self::Candid),
            "cbor" => Ok(Self::Cbor),
            _ => Err(format!("Unknown serializer format \"{}\"", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_expand_cbor_impl() {
        let input: DeriveInput = parse2(quote! {
            pub struct MyStruct {
                pub id: u32,
            }
        })
        .unwrap();

        let expanded = expand_cbor_impl(&input, &MacroArguments::default()).unwrap();

        assert_eq!(
            expanded.to_string(),
            quote! {
                #[derive(serde::Serialize)]
                #[derive(serde::Deserialize)]
                pub struct MyStruct {
                    pub id: u32,
                }

                impl orbit_essentials::ic_stable_structures::Storable for MyStruct {
                    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                        std::borrow::Cow::Owned(serde_cbor::to_vec(self).unwrap())
                    }

                    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                        serde_cbor::from_slice(bytes.as_ref()).unwrap()
                    }

                    const BOUND: orbit_essentials::ic_stable_structures::storable::Bound = orbit_essentials::ic_stable_structures::storable::Bound::Unbounded;
                }
            }
            .to_string()
        );
    }

    #[test]
    fn test_expand_candid_impl() {
        let input: DeriveInput = parse2(quote! {
            pub struct MyStruct {
                pub id: u32,
            }
        })
        .unwrap();

        let expanded = expand_candid_impl(&input, &MacroArguments::default()).unwrap();

        assert_eq!(
            expanded.to_string(),
            quote! {
                #[derive(candid::CandidType)]
                #[derive(candid::Deserialize)]
                pub struct MyStruct {
                    pub id: u32,
                }

                impl orbit_essentials::ic_stable_structures::Storable for MyStruct {
                    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                        use candid::Encode;

                        std::borrow::Cow::Owned(candid::Encode!(self).unwrap())
                    }

                    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                        use candid::Decode;

                        candid::Decode!(bytes.as_ref(), Self).unwrap()
                    }

                    const BOUND: orbit_essentials::ic_stable_structures::storable::Bound = orbit_essentials::ic_stable_structures::storable::Bound::Unbounded;
                }
            }
            .to_string()
        );
    }
}
