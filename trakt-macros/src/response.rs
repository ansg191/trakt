use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Error, Field, Fields, Ident, Result};

pub fn derive_response(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;

    let expected = get_expected(input)?;

    let pagination = check_pagination(input)?;

    let DeriveResponse { body, extra } = match pagination {
        Some(pagination) => derive_pagination(input, pagination, &expected)?,
        None => derive_normal(input, &expected)?,
    };

    let expanded = quote! {
        #[automatically_derived]
        impl _trakt_core::Response for #name {
            fn try_from_http_response<T: AsRef<[u8]>>(
                response: http::Response<T>,
            ) -> Result<Self, _trakt_core::error::FromHttpError> {
                #body
            }
        }
        #extra
    };

    let wrap = quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate http as _http;
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate trakt_core as _trakt_core;
            #expanded
        };
    };

    Ok(wrap)
}

fn get_expected(input: &DeriveInput) -> Result<Ident> {
    let mut expected = None;
    for attr in &input.attrs {
        if attr.path().is_ident("trakt") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("expected") {
                    let value = meta.value()?;
                    expected = Some(value.parse()?);
                    Ok(())
                } else {
                    Err(meta.error("unknown attribute"))
                }
            })?;
        }
    }

    Ok(expected.unwrap_or_else(|| Ident::new("OK", Span::call_site())))
}

#[derive(Copy, Clone)]
pub struct Pagination<'a> {
    /// The field containing the `PaginationResponse`
    pub field: &'a Field,
}

pub fn check_pagination(input: &DeriveInput) -> Result<Option<Pagination>> {
    let syn::Data::Struct(data) = &input.data else {
        return Err(Error::new(input.ident.span(), "Must be a struct"));
    };
    let Fields::Named(fields) = &data.fields else {
        return Ok(None);
    };

    let mut ret = None;

    for field in &fields.named {
        for attr in &field.attrs {
            if attr.path().is_ident("trakt") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("pagination") {
                        ret = Some(field);
                        Ok(())
                    } else {
                        Err(meta.error("unknown attribute"))
                    }
                })?;
            }
        }
    }

    Ok(ret.map(|field| Pagination { field }))
}

#[derive(Debug)]
struct DeriveResponse {
    body: TokenStream,
    extra: TokenStream,
}

fn derive_pagination(
    input: &DeriveInput,
    pagination: Pagination,
    expected: &Ident,
) -> Result<DeriveResponse> {
    let Pagination { field } = pagination;
    let ident = field.ident.as_ref().unwrap();

    let body = quote! {
        let body = _trakt_core::handle_response_body(&response, _http::StatusCode::#expected)?;
        let #ident = _trakt_core::PaginationResponse::from_headers(body, response.headers())?;
        Ok(Self { #ident })
    };

    let extra = crate::paginated::derive_paginated::<false>(input)?;

    Ok(DeriveResponse { body, extra })
}

fn derive_normal(input: &DeriveInput, expected: &Ident) -> Result<DeriveResponse> {
    let syn::Data::Struct(data) = &input.data else {
        return Err(Error::new(input.ident.span(), "Must be a struct"));
    };

    Ok(match &data.fields {
        Fields::Named(_) => derive_struct(expected),
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() != 1 {
                return Err(Error::new(
                    fields.unnamed.span(),
                    "Expected exactly one field",
                ));
            }
            derive_newtype(expected)
        }
        Fields::Unit => derive_unit(expected),
    })
}

fn derive_newtype(expected: &Ident) -> DeriveResponse {
    let body = quote! {
        Ok(Self(_trakt_core::handle_response_body(&response, _http::StatusCode::#expected)?))
    };
    DeriveResponse {
        body,
        extra: TokenStream::default(),
    }
}

fn derive_struct(expected: &Ident) -> DeriveResponse {
    let body = quote! {
        _trakt_core::handle_response_body(&response, _http::StatusCode::#expected)
    };
    DeriveResponse {
        body,
        extra: TokenStream::default(),
    }
}

fn derive_unit(expected: &Ident) -> DeriveResponse {
    let body = quote! {
        _trakt_core::handle_response_body(&response, http::StatusCode::#expected)?;
        Ok(Self)
    };
    DeriveResponse {
        body,
        extra: TokenStream::default(),
    }
}
