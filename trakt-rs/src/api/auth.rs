//! Authentication endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/authentication-oauth>
//! <https://trakt.docs.apiary.io/#reference/authentication-devices>

pub mod token {
    //! Exchange authorization code for an access & refresh token
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-oauth/get-token/exchange-code-for-access_token>

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
            auth: crate::AuthRequirement::None,
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
    //! Exchange refresh token for a new access token
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-oauth/revoke-token/revoke-an-access_token>

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
            auth: crate::AuthRequirement::None,
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
    //! Revoke an access token
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-oauth/revoke-token>

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
            auth: crate::AuthRequirement::None,
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

pub mod device_code {
    //! Generate a device code
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-devices/device-code/generate-new-device-codes>

    use bytes::BufMut;

    use crate::{Context, IntoHttpError, Metadata};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
    pub struct Request;

    impl crate::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/device/code",
            method: http::Method::POST,
            auth: crate::AuthRequirement::None,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let url = crate::url::construct_url(ctx.base_url, Self::METADATA.endpoint, &(), &())?;

            let body = T::default();
            let mut writer = body.writer();

            let json = serde_json::json!({
                "client_id": ctx.client_id,
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
        pub device_code: String,
        pub user_code: String,
        pub verification_url: String,
        pub expires_in: i64,
        pub interval: i64,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            crate::utils::handle_response_body(&response, http::StatusCode::OK)
        }
    }
}

pub mod poll_token {
    //! Poll for an access token
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-devices/device-code/poll-for-the-access_token>

    use bytes::BufMut;

    use crate::{Context, IntoHttpError, Metadata};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Request {
        pub device_code: String,
        pub client_secret: String,
    }

    impl crate::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/device/token",
            method: http::Method::POST,
            auth: crate::AuthRequirement::None,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let url = crate::url::construct_url(ctx.base_url, Self::METADATA.endpoint, &(), &())?;

            let body = T::default();
            let mut writer = body.writer();

            let json = serde_json::json!({
                "code": self.device_code,
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

    /// Poll Response
    ///
    /// Will [`ApiError::BadRequest`] if the device code has not been authorized by the user yet.
    ///
    /// [`ApiError::BadRequest`]: crate::ApiError::BadRequest
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
