pub mod token {
    use bytes::BufMut;

    use crate::{Context, FromHttpError, IntoHttpError, Metadata};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Request {
        pub code: String,
        pub client_secret: String,
        pub redirect_uri: String,
    }

    impl crate::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/token",
            method: http::Method::POST,
            auth: false,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let url = crate::url::construct_url(ctx.base_url, Self::METADATA.endpoint, &(), &())?;

            let body = T::default();
            let mut writer = body.writer();

            let json = serde_json::json!({
                "code": self.code,
                "client_id": ctx.client_id,
                "client_secret": self.client_secret,
                "redirect_uri": self.redirect_uri,
                "grant_type": "authorization_code",
            });
            serde_json::to_writer(&mut writer, &json)?;

            let request = http::Request::builder()
                .method(Self::METADATA.method)
                .uri(url)
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", ctx.client_id)
                .body(writer.into_inner())?;
            Ok(request)
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct Response {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: i64,
        pub refresh_token: String,
        pub scope: String,
        pub created_at: i64,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, FromHttpError> {
            crate::utils::handle_response_body(&response, http::StatusCode::OK)
        }
    }
}

pub mod exchange {
    use bytes::BufMut;

    use crate::{Context, IntoHttpError, Metadata};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Request {
        pub refresh_token: String,
        pub client_secret: String,
        pub redirect_uri: String,
    }

    impl crate::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/token",
            method: http::Method::POST,
            auth: false,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let url = crate::url::construct_url(ctx.base_url, Self::METADATA.endpoint, &(), &())?;

            let body = T::default();
            let mut writer = body.writer();

            let json = serde_json::json!({
                "refresh_token": self.refresh_token,
                "client_id": ctx.client_id,
                "client_secret": self.client_secret,
                "redirect_uri": self.redirect_uri,
                "grant_type": "refresh_token",
            });
            serde_json::to_writer(&mut writer, &json)?;

            let request = http::Request::builder()
                .method(Self::METADATA.method)
                .uri(url)
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", ctx.client_id)
                .body(writer.into_inner())?;
            Ok(request)
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct Response {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: i64,
        pub refresh_token: String,
        pub scope: String,
        pub created_at: i64,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            crate::utils::handle_response_body(&response, http::StatusCode::OK)
        }
    }
}

pub mod revoke {
    use bytes::BufMut;

    use crate::{utils::handle_response_body, Context, IntoHttpError, Metadata};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Request {
        pub token: String,
        pub client_secret: String,
    }

    impl crate::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/revoke",
            method: http::Method::POST,
            auth: false,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let url = crate::url::construct_url(ctx.base_url, Self::METADATA.endpoint, &(), &())?;

            let body = T::default();
            let mut writer = body.writer();

            let json = serde_json::json!({
                "token": self.token,
                "client_id": ctx.client_id,
                "client_secret": self.client_secret,
            });
            serde_json::to_writer(&mut writer, &json)?;

            let request = http::Request::builder()
                .method(Self::METADATA.method)
                .uri(url)
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", ctx.client_id)
                .body(writer.into_inner())?;
            Ok(request)
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Response;

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            handle_response_body(&response, http::StatusCode::OK)?;
            Ok(Self)
        }
    }
}
