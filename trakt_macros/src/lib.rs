mod paginated;
mod request;

use proc_macro::TokenStream;

#[proc_macro_derive(Request, attributes(trakt, serde))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    request::derive_request(input)
}

#[proc_macro_derive(Paginated, attributes(trakt))]
pub fn derive_paginated(input: TokenStream) -> TokenStream {
    paginated::derive_paginated(input)
}
