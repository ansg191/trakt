use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, DeriveInput, Field, Fields,
    LitStr, Token, Type,
};

pub fn derive_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Disallow Generic structs
    if !input.generics.params.is_empty() {
        return syn::Error::new(Span::call_site(), "Request structs cannot be generic")
            .into_compile_error()
            .into();
    }

    let RequestAttrs {
        endpoint,
        method,
        auth,
        response,
    } = match derive_request_attrs(&input) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };

    let Some(response) = response else {
        return syn::Error::new(
            Span::call_site(),
            "missing #[trakt(response = \"...\")] attribute",
        )
        .into_compile_error()
        .into();
    };

    let SerializeStructs {
        q_ident,
        p_ident,
        stream,
    } = match derive_request_structs(&input, &endpoint.value()) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };

    let expanded = quote! {
        #stream
        #[automatically_derived]
        impl _trakt_core::Request for #name {
            type Response = #response;

            const METADATA: _trakt_core::Metadata = _trakt_core::Metadata {
                endpoint: #endpoint,
                method: _http::Method::#method,
                auth: _trakt_core::AuthRequirement::#auth,
            };

            fn try_into_http_request<T: Default + _bytes::BufMut>(
                self,
                ctx: _trakt_core::Context,
            ) -> Result<_http::Request<T>, _trakt_core::error::IntoHttpError> {
                let (path, query): (#p_ident, #q_ident) = self.into();

                let url = _trakt_core::construct_url(
                    ctx.base_url,
                    #endpoint,
                    &path,
                    &query,
                )?;

                let request = _http::Request::builder()
                    .method(Self::METADATA.method)
                    .uri(url)
                    .header("Content-Type", "application/json")
                    .header("trakt-api-version", "2")
                    .header("trakt-api-key", ctx.client_id);

                let request = match (Self::METADATA.auth, ctx.oauth_token) {
                    (_trakt_core::AuthRequirement::None, _) | (_trakt_core::AuthRequirement::Optional, None) => request,
                    (_trakt_core::AuthRequirement::Optional | _trakt_core::AuthRequirement::Required, Some(token)) => {
                        request.header("Authorization", format!("Bearer {}", token))
                    }
                    (_trakt_core::AuthRequirement::Required, None) => {
                        return Err(_trakt_core::error::IntoHttpError::MissingToken);
                    }
                };

                Ok(request.body(T::default())?)
            }
        }
    };

    let wrap = quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate http as _http;
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate bytes as _bytes;
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate trakt_core as _trakt_core;
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #expanded
        };
    };

    TokenStream::from(wrap)
}

fn parse_url_params(endpoint: &str) -> Vec<&str> {
    let mut params = vec![];
    for (i, c) in endpoint.char_indices() {
        if c == '{' {
            let end = endpoint[i..].find('}').unwrap();
            params.push(&endpoint[i + 1..i + end]);
        }
    }
    params
}

struct RequestAttrs {
    endpoint: LitStr,
    method: Ident,
    auth: Ident,
    response: Option<Type>,
}

fn derive_request_attrs(input: &DeriveInput) -> syn::Result<RequestAttrs> {
    let mut ret = RequestAttrs {
        endpoint: LitStr::new("/", Span::call_site()),
        method: format_ident!("GET"),
        auth: format_ident!("None"),
        response: None,
    };

    for attr in &input.attrs {
        if attr.path().is_ident("trakt") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("response") {
                    let value = meta.value()?;
                    ret.response = Some(value.parse()?);
                    Ok(())
                } else if meta.path.is_ident("endpoint") {
                    let value = meta.value()?;
                    ret.endpoint = value.parse()?;
                    Ok(())
                } else if meta.path.is_ident("method") {
                    let value = meta.value()?;
                    ret.method = value.parse()?;
                    Ok(())
                } else if meta.path.is_ident("auth") {
                    ret.auth = meta.value()?.parse()?;
                    Ok(())
                } else {
                    Err(meta.error("unsupported attribute"))
                }
            })?;
        }
    }

    Ok(ret)
}

struct SerializeStructs {
    q_ident: Ident,
    p_ident: Ident,
    stream: proc_macro2::TokenStream,
}

fn derive_request_structs(input: &DeriveInput, endpoint: &str) -> syn::Result<SerializeStructs> {
    let syn::Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "Request structs must be structs",
        ));
    };
    match &data.fields {
        Fields::Named(f) => make_structs(&input.ident, &f.named, endpoint),
        Fields::Unnamed(_) => Err(syn::Error::new(
            Span::call_site(),
            "Request structs cannot have unnamed fields",
        )),
        Fields::Unit => make_structs(&input.ident, &Punctuated::new(), endpoint),
    }
}

fn make_structs(
    ident: &Ident,
    fields: &Punctuated<Field, Token![,]>,
    endpoint: &str,
) -> syn::Result<SerializeStructs> {
    let mut path_params_str = parse_url_params(endpoint);

    let mut path_params = Punctuated::<_, Token![,]>::new();
    let mut query_params = Punctuated::<_, Token![,]>::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap();

        let idx = path_params_str
            .iter()
            .position(|&s| s == &*ident.to_string());
        if let Some(idx) = idx {
            path_params_str.swap_remove(idx);
            path_params.push(field);
        } else {
            query_params.push(field);
        }
    }

    if !path_params_str.is_empty() {
        return Err(syn::Error::new(
            fields.span(),
            format!(
                "missing path parameter{}: {}",
                if path_params_str.len() == 1 { "" } else { "s" },
                path_params_str.join(", ")
            ),
        ));
    }

    let q_ident = format_ident!("{}QueryParams", ident);
    let p_ident = format_ident!("{}PathParams", ident);

    let p_names = path_params.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let q_names = query_params.iter().map(|f| &f.ident).collect::<Vec<_>>();

    let stream = quote! {
        #[doc(hidden)]
        #[derive(Debug, Clone, _serde::Serialize)]
        struct #q_ident {
            #query_params
        }

        #[doc(hidden)]
        #[derive(Debug, Clone, _serde::Serialize)]
        struct #p_ident {
            #path_params
        }

        impl std::convert::From<#ident> for (#p_ident, #q_ident) {
            fn from(req: #ident) -> Self {
                let #ident { #(#p_names,)* #(#q_names,)* } = req;
                (#p_ident { #(#p_names,)* }, #q_ident { #(#q_names,)* })
            }
        }
    };

    Ok(SerializeStructs {
        q_ident,
        p_ident,
        stream,
    })
}
