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

    fn expand_implementation(&self, args: &MacroArguments) -> Result<TokenStream, Error> {
        let parsed_input: DeriveInput = parse2(self.input.clone())?;

        match parsed_input.data {
            syn::Data::Struct(_) | syn::Data::Enum(_) => {
                let input = parsed_input.clone();
                let size_value: Option<u32> = args.size;

                match args.serializer {
                    SerializerFormat::Candid => expand_candid_impl(&input, size_value),
                    SerializerFormat::Cbor => expand_cbor_impl(&input, size_value),
                    SerializerFormat::CborOrDefault => {
                        expand_cbor_or_default_impl(&input, size_value)
                    }
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
        let args = parser.parse(self.input_args.clone().into())?;

        let mut size: Option<u32> = None;
        let mut serializer: SerializerFormat = SerializerFormat::Cbor;

        for expr in args {
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
                            size = Some(lit_int.base10_parse()?);
                        }
                    }

                    Self::MACRO_ARG_KEY_SERIALIZER => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = *right
                        {
                            serializer =
                                SerializerFormat::from_str(&lit_str.value()).map_err(|err| {
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

        Ok(MacroArguments { size, serializer })
    }
}

fn expand_candid_impl(
    input: &DeriveInput,
    size: Option<u32>,
) -> Result<proc_macro2::TokenStream, Error> {
    let object_name = input.ident.clone();
    let storage_bounds = storage_bounds(size);

    let expanded = quote! {
        #[derive(candid::CandidType, candid::Deserialize)]
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
    size: Option<u32>,
) -> Result<proc_macro2::TokenStream, Error> {
    let object_name = input.ident.clone();
    let storage_bounds = storage_bounds(size);

    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
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

fn expand_cbor_or_default_impl(
    input: &DeriveInput,
    size: Option<u32>,
) -> Result<proc_macro2::TokenStream, Error> {
    let object_name = input.ident.clone();
    let storage_bounds = storage_bounds(size);

    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #input

        impl orbit_essentials::ic_stable_structures::Storable for #object_name {
            fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                std::borrow::Cow::Owned(serde_cbor::to_vec(self).unwrap())
            }

            fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                serde_cbor::from_slice(bytes.as_ref()).unwrap_or_default()
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
    CborOrDefault,
}

impl std::str::FromStr for SerializerFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "candid" => Ok(Self::Candid),
            "cbor" => Ok(Self::Cbor),
            "cbor_or_default" => Ok(Self::CborOrDefault),
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

        let expanded = expand_cbor_impl(&input, None).unwrap();

        assert_eq!(
            expanded.to_string(),
            quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
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
    fn test_expand_cbor_or_default_impl() {
        let input: DeriveInput = parse2(quote! {
            pub struct MyStruct {
                pub id: u32,
            }
        })
        .unwrap();

        let expanded = expand_cbor_or_default_impl(&input, None).unwrap();

        assert_eq!(
            expanded.to_string(),
            quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
                pub struct MyStruct {
                    pub id: u32,
                }

                impl orbit_essentials::ic_stable_structures::Storable for MyStruct {
                    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                        std::borrow::Cow::Owned(serde_cbor::to_vec(self).unwrap())
                    }

                    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                        serde_cbor::from_slice(bytes.as_ref()).unwrap_or_default()
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

        let expanded = expand_candid_impl(&input, None).unwrap();

        assert_eq!(
            expanded.to_string(),
            quote! {
                #[derive(candid::CandidType, candid::Deserialize)]
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
