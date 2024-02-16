#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::as_underscore,
    clippy::clone_on_ref_ptr,
    clippy::format_push_string,
    clippy::mod_module_files,
    clippy::str_to_string
)]
#![allow(clippy::module_name_repetitions)]

mod paginated;
mod request;
mod response;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Request, attributes(trakt, serde))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    request::derive_request(input)
}

#[proc_macro_derive(Response, attributes(trakt))]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    response::derive_response(&input)
        .map_or_else(|e| e.into_compile_error().into(), TokenStream::from)
}

#[proc_macro_derive(Paginated, attributes(trakt))]
pub fn derive_paginated(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    paginated::derive_paginated(&input)
        .map_or_else(|e| e.into_compile_error().into(), TokenStream::from)
}
