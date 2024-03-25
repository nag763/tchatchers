//! # tchatchers macro
//!
//! This module contains procedural macros for internal use.


extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Converts an identifier to snake case.
///
/// This function takes an identifier and converts it to snake case.
///
/// # Arguments
///
/// * `ident` - The identifier to be converted to snake case.
///
/// # Returns
///
/// Returns the converted string in snake case.
fn to_snake_case(ident: &proc_macro2::Ident) -> String {
    let ident_str = ident.to_string();
    let mut snake_case = String::with_capacity(ident_str.len() * 2);
    let mut prev_was_upper = false;

    for (i, c) in ident_str.chars().enumerate() {
        if c.is_uppercase() && i > 0 && !prev_was_upper {
            snake_case.push('_');
        }
        prev_was_upper = c.is_uppercase();
        snake_case.push(c.to_lowercase().next().unwrap());
    }

    snake_case
}

/// Procedural macro for generating an enum that represents different response kinds.
///
/// This macro generates an enum that represents different response kinds based on the provided attributes.
///
/// # Attributes
///
/// This macro supports the following attributes:
///
/// - `response`: Defines the response details such as status code, error messages, etc.
///
/// # Example
///
/// ```ignore
/// #[derive(IntoApiResponse)]
/// pub enum ApiGenericResponse {
///     #[response(status=UNAUTHORIZED, simple)]
///     AuthenticationRequired,
///     // More variants...
/// }
/// ```
#[proc_macro_derive(IntoApiResponse, attributes(response))]
pub fn into_api_response(item: TokenStream) -> TokenStream {
    let DeriveInput { data, .. } = parse_macro_input!(item);

    let mut variant_generic_generator = TokenStream2::new();
    let mut variant_from_status_code = TokenStream2::new();
    let mut variant_into_response = TokenStream2::new();

    let Data::Enum(data_enum) = data else {
        panic!("Macro can only be used on data enums");
    };
    for variant in &data_enum.variants {
        let (variant_name, variant_attrs) = (&variant.ident, &variant.attrs);

        variant_generic_generator.extend(quote! {
            #variant_name ,
        });

        for attr in variant_attrs {
            if attr.path().is_ident("response") {
                let meta = &attr.meta;
                if let Ok(list) = meta.require_list() {
                    let mut token_iter =
                        <proc_macro2::TokenStream as Clone>::clone(&list.tokens).into_iter();
                    while let Some(token) = token_iter.next() {
                        match token {
                            proc_macro2::TokenTree::Ident(ident) => {
                                let ident_name = ident.to_string();
                                match ident_name.as_str() {
                                    "status" => {
                                        let Some(next_token) = token_iter.next() else {
                                            panic!("Status need to be associated with a status code value")
                                        };
                                        let proc_macro2::TokenTree::Punct(punct) = next_token
                                        else {
                                            panic!("Status has to be followed by a punctuation")
                                        };
                                        if punct.as_char() == '=' {
                                            let Some(next_token) = token_iter.next() else {
                                                panic!("No expression following equal operator.");
                                            };
                                            let proc_macro2::TokenTree::Ident(ident) = next_token
                                            else {
                                                panic!("No identifer found following equal operator for {variant_name} status")
                                            };
                                            variant_from_status_code.extend(quote! {
                                                                    ApiResponseKind::#variant_name => StatusCode::#ident,
                                                                });
                                        } else {
                                            panic!("Equal operator is the only supported type (provided {punct} for {variant_name}");
                                        }
                                    }
                                    "error" => {
                                        if let Some(next_token) = token_iter.next() {
                                            if let proc_macro2::TokenTree::Group(group) = next_token
                                            {
                                                if let Some(token) =
                                                    <proc_macro2::TokenStream as Clone>::clone(
                                                        &group.stream(),
                                                    )
                                                    .into_iter()
                                                    .next()
                                                {
                                                    let proc_macro2::TokenTree::Literal(lit) =
                                                        token
                                                    else {
                                                        panic!("Not supported");
                                                    };
                                                    variant_into_response.extend(quote!{
                                                                ApiGenericResponse::#variant_name(errors) => ApiResponse::errors(
                                                                    ApiResponseKind::#variant_name,
                                                                    #lit,
                                                                    vec![errors]
                                                                ),
                                                            });
                                                }
                                            }
                                        } else {
                                            let variant_name_snake_case = to_snake_case(&ident);
                                            variant_into_response.extend(quote!{
                                                        ApiGenericResponse::#variant_name(errors) => ApiResponse::errors(                                                                    ApiResponseKind::#variant_name,
                                                            ApiResponseKind::#variant_name,
                                                            #variant_name_snake_case
                                                            vec![errors]
                                                        ),
                                                    });
                                        }
                                    }
                                    "errors" => {
                                        if let Some(next_token) = token_iter.next() {
                                            if let proc_macro2::TokenTree::Group(group) = next_token
                                            {
                                                if let Some(token) =
                                                    <proc_macro2::TokenStream as Clone>::clone(
                                                        &group.stream(),
                                                    )
                                                    .into_iter()
                                                    .next()
                                                {
                                                    let proc_macro2::TokenTree::Literal(lit) =
                                                        token
                                                    else {
                                                        panic!("Not supported");
                                                    };
                                                    variant_into_response.extend(quote!{
                                                                ApiGenericResponse::#variant_name(errors) => ApiResponse::errors(
                                                                    ApiResponseKind::#variant_name,
                                                                    #lit,
                                                                    errors
                                                                ),
                                                            });
                                                }
                                            }
                                        } else {
                                            let variant_name_snake_case = to_snake_case(&ident);
                                            variant_into_response.extend(quote!{
                                                        ApiGenericResponse::#variant_name(errors) => ApiResponse::errors(                                                                    ApiResponseKind::#variant_name,
                                                            ApiResponseKind::#variant_name_snake_case,
                                                            errors
                                                        ),
                                                    });
                                        }
                                    }
                                    "simple" => {
                                        if let Some(next_token) = token_iter.next() {
                                            if let proc_macro2::TokenTree::Group(group) = next_token
                                            {
                                                if let Some(token) =
                                                    <proc_macro2::TokenStream as Clone>::clone(
                                                        &group.stream(),
                                                    )
                                                    .into_iter()
                                                    .next()
                                                {
                                                    let proc_macro2::TokenTree::Literal(lit) =
                                                        token
                                                    else {
                                                        panic!("Not supported");
                                                    };
                                                    variant_into_response.extend(quote!{
                                                                ApiGenericResponse::#variant_name => ApiResponse::new(
                                                                    ApiResponseKind::#variant_name,
                                                                    #lit
                                                                ),
                                                            });
                                                }
                                            }
                                        } else {
                                            let variant_name_snake_case = to_snake_case(&ident);

                                            variant_into_response.extend(quote!{
                                                        ApiGenericResponse::#variant_name => ApiResponse::new(
                                                            ApiResponseKind::#variant_name,
                                                            #variant_name_snake_case,
                                                        ),
                                                    });
                                        }
                                    }
                                    _ => {
                                        panic!("{ident_name} not supported");
                                    }
                                }
                            }
                            proc_macro2::TokenTree::Punct(punc) => {
                                if punc.as_char() == ',' {
                                    continue;
                                }
                            }
                            _ => panic!("Token type not supported"),
                        }
                    }
                }
            }
        }
    }
    TokenStream::from(quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        pub enum ApiResponseKind {
            #variant_generic_generator
        }

        #[cfg(feature = "back")]
        impl From<ApiResponseKind> for StatusCode {
            fn from(value: ApiResponseKind) -> StatusCode {
                match value {
                    #variant_from_status_code
                }
            }
        }

        impl From<ApiGenericResponse> for ApiResponse {
            fn from(value: ApiGenericResponse) -> Self {
                match value {
                    #variant_into_response
                }
            }
        }
    })
}

/// Procedural macro for generating error wrappers for the enum variants.
///
/// This macro generates error wrappers for the enum variants based on the provided attributes.
///
/// # Attributes
///
/// This macro supports the following attribute:
///
/// - `from_err`: Defines the error types to be wrapped.
///
/// # Example
///
/// ```ignore
/// #[derive(ErrorWrapper)]
/// pub enum MyError {
///     #[from_err(std::io::Error)]
///     IoError,
///     // More variants...
/// }
/// ```
#[proc_macro_derive(ErrorWrapper, attributes(from_err))]
pub fn from_error(item: TokenStream) -> TokenStream {
    let DeriveInput { data, .. } = parse_macro_input!(item);

    let Data::Enum(data_enum) = data else {
        panic!("Macro can only be used on data enums");
    };

    let mut from_impl_generator = TokenStream2::new();

    for variant in &data_enum.variants {
        let (variant_name, variant_attrs) = (&variant.ident, &variant.attrs);

        for attr in variant_attrs {
            if attr.path().is_ident("from_err") {
                let meta = &attr.meta;
                if let Ok(list) = meta.require_list() {
                    let mut token_iter =
                        <proc_macro2::TokenStream as Clone>::clone(&list.tokens).into_iter();
                    while let Some(token) = token_iter.next() {
                        match token {
                            proc_macro2::TokenTree::Ident(ident) => {
                                let mut current_token = TokenStream2::new();
                                current_token.extend(quote! {#ident});
                                loop {
                                    let Some(next_token) = token_iter.next() else {
                                        break;
                                    };
                                    match next_token {
                                        proc_macro2::TokenTree::Punct(punct)
                                            if punct.as_char() == ':' =>
                                        {
                                            current_token.extend(quote! {#punct})
                                        }
                                        proc_macro2::TokenTree::Ident(ident) => {
                                            current_token.extend(quote! {#ident})
                                        }
                                        _ => break,
                                    }
                                }

                                from_impl_generator.extend(quote! {
                                    impl From<#current_token> for ApiGenericResponse {
                                        fn from(value: #current_token) -> Self {
                                            Self::#variant_name(value.to_string())
                                        }
                                    }
                                });
                            }
                            proc_macro2::TokenTree::Punct(punct) if punct.to_string() == "," => {
                                continue
                            }
                            _ => panic!("The provided token type is not supported {token:?}"),
                        }
                    }
                }
            }
        }
    }

    TokenStream::from(quote! {
        #from_impl_generator
    })
}
