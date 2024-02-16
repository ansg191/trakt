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

mod emoji_str;
pub mod error;
mod request;
mod response;
mod url;
mod utils;

pub use emoji_str::*;
pub use request::*;
pub use response::*;
pub use url::*;
pub use utils::*;
