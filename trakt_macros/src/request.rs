use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Field, Fields, LitStr, Token, Type};

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
        impl crate::Request for #name {
            type Response = #response;

            const METADATA: crate::Metadata = crate::Metadata {
                endpoint: #endpoint,
                method: ::http::Method::#method,
                auth: #auth,
            };

            fn try_into_http_request<T: Default + ::bytes::BufMut>(
                self,
                ctx: crate::Context,
            ) -> Result<::http::Request<T>, crate::IntoHttpError> {
                let (path, query): (#p_ident, #q_ident) = self.into();

                let url = crate::url::construct_url(
                    ctx.base_url,
                    #endpoint,
                    &path,
                    &query,
                )?;

                let request = ::http::Request::builder()
                    .method(Self::METADATA.method)
                    .uri(url)
                    .header("Content-Type", "application/json")
                    .header("trakt-api-version", "2")
                    .header("trakt-api-key", ctx.client_id)
                    .body(T::default())?;
                Ok(request)
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_url_params(endpoint: &str) -> Vec<&str> {
    let mut params = vec![];
    for c in endpoint.chars() {
        if c == '{' {
            let start = endpoint.find(c).unwrap();
            let end = endpoint[start..].find('}').unwrap();
            params.push(&endpoint[start + 1..start + end]);
        }
    }
    params
}

struct RequestAttrs {
    endpoint: LitStr,
    method: Ident,
    auth: bool,
    response: Option<Type>,
}

fn derive_request_attrs(input: &DeriveInput) -> syn::Result<RequestAttrs> {
    let mut ret = RequestAttrs {
        endpoint: LitStr::new("/", Span::call_site()),
        method: format_ident!("GET"),
        auth: false,
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
                    ret.auth = true;
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
        Fields::Named(f) => Ok(make_structs(&input.ident, &f.named, endpoint)),
        Fields::Unnamed(_) => Err(syn::Error::new(
            Span::call_site(),
            "Request structs cannot have unnamed fields",
        )),
        Fields::Unit => Ok(make_structs(&input.ident, &Punctuated::new(), endpoint)),
    }
}

fn make_structs(
    ident: &Ident,
    fields: &Punctuated<Field, Token![,]>,
    endpoint: &str,
) -> SerializeStructs {
    let path_params_str = parse_url_params(endpoint);

    let mut path_params = Punctuated::<_, Token![,]>::new();
    let mut query_params = Punctuated::<_, Token![,]>::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        if path_params_str.contains(&&*ident.to_string()) {
            path_params.push(field);
        } else {
            query_params.push(field);
        }
    }

    let q_ident = format_ident!("{}QueryParams", ident);
    let p_ident = format_ident!("{}PathParams", ident);

    let p_names = path_params.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let q_names = query_params.iter().map(|f| &f.ident).collect::<Vec<_>>();

    let stream = quote! {
        #[doc(hidden)]
        #[derive(Debug, Clone, ::serde::Serialize)]
        struct #q_ident {
            #query_params
        }

        #[doc(hidden)]
        #[derive(Debug, Clone, ::serde::Serialize)]
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

    SerializeStructs {
        q_ident,
        p_ident,
        stream,
    }
}
