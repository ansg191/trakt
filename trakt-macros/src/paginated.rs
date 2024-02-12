use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

pub fn derive_paginated(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Disallow Generic structs
    if !input.generics.params.is_empty() {
        return syn::Error::new(
            Span::call_site(),
            "Paginated Response structs cannot be generic",
        )
        .into_compile_error()
        .into();
    }

    let field = match handle_field_attrs(&input) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };

    let tp = match extract_item(&field.ty) {
        Ok(tp) => tp,
        Err(e) => return e.to_compile_error().into(),
    };

    let Some(i_field) = &field.ident else {
        return syn::Error::new(Span::call_site(), "missing field name")
            .into_compile_error()
            .into();
    };

    let expanded = quote! {
        #[automatically_derived]
        impl crate::PaginatedResponse for #name {
            type Item = #tp;

            fn items(&self) -> &[Self::Item] {
                &self.#i_field.items
            }

            fn next_page(&self) -> Option<usize> {
                self.#i_field.next_page()
            }
        }
    };

    TokenStream::from(expanded)
}

fn handle_field_attrs(input: &DeriveInput) -> syn::Result<&syn::Field> {
    let syn::Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "Paginated Response must be a struct",
        ));
    };

    let mut ret = None;

    for field in &data.fields {
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
                break;
            }
        }
    }

    ret.map_or_else(
        || {
            Err(syn::Error::new(
                Span::call_site(),
                "missing #[trakt(pagination)] attribute",
            ))
        },
        Ok,
    )
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
