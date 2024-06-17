use trakt_core::{
    error::{FromHttpError, IntoHttpError},
    Context, Request, Response,
};

macro_rules! assert_req {
    ($ctx: expr, $req: expr, $exp_url: expr, $exp_body: expr $(,)?) => {
        $crate::test::assert_request($ctx, $req, $exp_url, $exp_body, true);
    };
    ($ctx: expr, $req: expr, $exp_url: expr, $exp_body: expr, $json: expr $(,)?) => {
        $crate::test::assert_request($ctx, $req, $exp_url, $exp_body, $json);
    };
}
pub(crate) use assert_req;

pub fn assert_request<R, T>(ctx: Context, req: R, exp_url: &str, exp_body: &T, json: bool)
where
    R: Request,
    T: ToString + ?Sized,
{
    let http_req = req.try_into_http_request::<Vec<u8>>(ctx).unwrap();

    assert_eq!(http_req.method(), R::METADATA.method);
    assert_eq!(http_req.uri(), exp_url);
    assert_eq!(
        http_req.headers().get("Content-Type").unwrap(),
        "application/json"
    );
    assert_eq!(http_req.headers().get("trakt-api-version").unwrap(), "2");
    assert_eq!(
        http_req.headers().get("trakt-api-key").unwrap(),
        ctx.client_id
    );
    if let Some(token) = ctx.oauth_token {
        assert_eq!(
            *http_req.headers().get("Authorization").unwrap(),
            format!("Bearer {token}")
        );
    }

    if json {
        let actual = serde_json::from_slice::<serde_json::Value>(http_req.body()).unwrap();
        let expected = serde_json::from_str::<serde_json::Value>(&exp_body.to_string()).unwrap();
        assert_eq!(actual, expected);
    } else {
        assert_eq!(
            String::from_utf8_lossy(http_req.body()),
            exp_body.to_string()
        );
    }
}

pub fn execute<R: Request>(ctx: Context, req: R) -> Result<R::Response, Error> {
    let request: http::Request<Vec<u8>> = req.try_into_http_request(ctx)?;
    let (parts, body) = request.into_parts();
    let request = ureq::Request::from(parts);

    let response = request.send_bytes(&body)?;
    let http_res: http::Response<Vec<u8>> = http::Response::from(response);

    Ok(Response::try_from_http_response(http_res)?)
}

#[derive(Debug)]
pub enum Error {
    Reqwest(Box<ureq::Error>),
    IntoHttp(IntoHttpError),
    FromHttp(FromHttpError),
}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Self::Reqwest(Box::new(e))
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
