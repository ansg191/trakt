use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

use crate::response::{check_pagination, Pagination};

pub fn derive_paginated(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    // Disallow Generic structs
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "Paginated Response structs cannot be generic",
        ));
    }

    let Some(Pagination { field }) = check_pagination(input)? else {
        return Err(syn::Error::new(
            Span::call_site(),
            "missing #[trakt(pagination)] attribute",
        ));
    };
    let tp = extract_item(&field.ty)?;

    let Some(i_field) = &field.ident else {
        return Err(syn::Error::new(Span::call_site(), "missing field name"));
    };

    let expanded = quote! {
        #[automatically_derived]
        impl _trakt_core::PaginatedResponse for #name {
            type Item = #tp;

            fn items(&self) -> &[Self::Item] {
                &self.#i_field.items
            }

            fn next_page(&self) -> Option<_trakt_core::Pagination> {
                self.#i_field.next_page()
            }
        }
    };

    let wrap = quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate trakt_core as _trakt_core;
            #expanded
        };
    };

    Ok(wrap)
}

/// Extracts the inner type of `PaginationResponse<T>` type.
fn extract_item(tp: &syn::Type) -> syn::Result<&syn::Type> {
    let syn::Type::Path(type_path) = tp else {
        return Err(syn::Error::new(tp.span(), "expected a type path"));
    };
    let syn::Path { segments, .. } = &type_path.path;

    let mut tp = None;
    for segment in segments {
        if segment.ident != "PaginationResponse" {
            continue;
        }

        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
            return Err(syn::Error::new(
                segment.ident.span(),
                "expected angle-bracketed arguments",
            ));
        };
        let args = &args.args;

        if args.len() != 1 {
            return Err(syn::Error::new(
                args.span(),
                "expected exactly one argument",
            ));
        }

        if let syn::GenericArgument::Type(t) = &args[0] {
            tp = Some(t);
            break;
        }

        return Err(syn::Error::new(args.span(), "expected a type argument"));
    }

    tp.map_or_else(
        || {
            Err(syn::Error::new(
                tp.span(),
                "expected a PaginationResponse type",
            ))
        },
        Ok,
    )
}
