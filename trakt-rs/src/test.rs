use std::sync::OnceLock;

use bytes::Bytes;
use reqwest::blocking::Client;
use trakt_core::{
    error::{FromHttpError, IntoHttpError},
    Context, Request, Response,
};

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn execute<R: Request>(ctx: Context, req: R) -> Result<R::Response, Error> {
    let client = CLIENT.get_or_init(Client::new);

    let request: http::Request<Vec<u8>> = req.try_into_http_request(ctx)?;

    let reqwest_res = client.execute(request.try_into()?)?;
    let http_res = reqwest_to_http(reqwest_res)?;

    Ok(Response::try_from_http_response(http_res)?)
}

fn reqwest_to_http(
    mut response: reqwest::blocking::Response,
) -> Result<http::Response<Bytes>, Error> {
    let mut builder = http::Response::builder()
        .status(response.status())
        .version(response.version());

    std::mem::swap(
        builder.headers_mut().expect("response should be valid"),
        response.headers_mut(),
    );

    Ok(builder
        .body(response.bytes()?)
        .expect("response should be valid"))
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    IntoHttp(IntoHttpError),
    FromHttp(FromHttpError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<IntoHttpError> for Error {
    fn from(e: IntoHttpError) -> Self {
        Self::IntoHttp(e)
    }
}

impl From<FromHttpError> for Error {
    fn from(e: FromHttpError) -> Self {
        Self::FromHttp(e)
    }
}