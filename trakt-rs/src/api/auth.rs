//! Authentication endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/authentication-oauth>
//! <https://trakt.docs.apiary.io/#reference/authentication-devices>

pub mod token {
    //! Exchange authorization code for an access & refresh token
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-oauth/get-token/exchange-code-for-access_token>

    use bytes::BufMut;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Request {
        pub code: String,
        pub client_secret: String,
        pub redirect_uri: String,
    }

    impl trakt_core::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/token",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::None,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
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

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: i64,
        pub refresh_token: String,
        pub scope: String,
        pub created_at: i64,
    }
}

pub mod exchange {
    //! Exchange refresh token for a new access token
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-oauth/revoke-token/revoke-an-access_token>

    use bytes::BufMut;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Request {
        pub refresh_token: String,
        pub client_secret: String,
        pub redirect_uri: String,
    }

    impl trakt_core::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/token",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::None,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
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

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: i64,
        pub refresh_token: String,
        pub scope: String,
        pub created_at: i64,
    }
}

pub mod revoke {
    //! Revoke an access token
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-oauth/revoke-token>

    use bytes::BufMut;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Request {
        pub token: String,
        pub client_secret: String,
    }

    impl trakt_core::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/revoke",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::None,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let body = T::default();
            let mut writer = body.writer();

            let json = serde_json::json!({
                "token": self.token,
                "client_id": ctx.client_id,
                "client_secret": self.client_secret,
            });
            serde_json::to_writer(&mut writer, &json)?;

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, trakt_macros::Response)]
    pub struct Response;
}

pub mod device_code {
    //! Generate a device code
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-devices/device-code/generate-new-device-codes>

    use bytes::BufMut;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
    pub struct Request;

    impl trakt_core::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/device/code",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::None,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let body = T::default();
            let mut writer = body.writer();

            let json = serde_json::json!({
                "client_id": ctx.client_id,
            });
            serde_json::to_writer(&mut writer, &json)?;

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub device_code: String,
        pub user_code: String,
        pub verification_url: String,
        pub expires_in: i64,
        pub interval: i64,
    }
}

pub mod poll_token {
    //! Poll for an access token
    //!
    //! <https://trakt.docs.apiary.io/#reference/authentication-devices/device-code/poll-for-the-access_token>

    use bytes::BufMut;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Request {
        pub device_code: String,
        pub client_secret: String,
    }

    impl trakt_core::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/oauth/device/token",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::None,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let body = T::default();
            let mut writer = body.writer();

            let json = serde_json::json!({
                "code": self.device_code,
                "client_id": ctx.client_id,
                "client_secret": self.client_secret,
            });
            serde_json::to_writer(&mut writer, &json)?;

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }

    /// Poll Response
    ///
    /// Will [`ApiError::BadRequest`] if the device code has not been authorized by the user yet.
    ///
    /// [`ApiError::BadRequest`]: crate::error::ApiError::BadRequest
    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: i64,
        pub refresh_token: String,
        pub scope: String,
        pub created_at: i64,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use trakt_core::Context;

    use super::*;
    use crate::test::assert_request;

    const CTX: Context = Context {
        base_url: "https://api.trakt.tv",
        client_id: "client_id",
        oauth_token: None,
    };

    #[test]
    fn test_token_request() {
        let expected = json!({
            "code": "code",
            "client_id": CTX.client_id,
            "client_secret": "secret",
            "redirect_uri": "https://localhost:8080",
            "grant_type": "authorization_code",
        });
        let req = token::Request {
            code: "code".to_owned(),
            client_secret: "secret".to_owned(),
            redirect_uri: "https://localhost:8080".to_owned(),
        };
        assert_request(CTX, req, "https://api.trakt.tv/oauth/token", &expected);
    }

    #[test]
    fn test_exchange_request() {
        let expected = json!({
            "refresh_token": "token",
            "client_id": CTX.client_id,
            "client_secret": "secret",
            "redirect_uri": "https://localhost:8080",
            "grant_type": "refresh_token",
        });
        let req = exchange::Request {
            refresh_token: "token".to_owned(),
            client_secret: "secret".to_owned(),
            redirect_uri: "https://localhost:8080".to_owned(),
        };
        assert_request(CTX, req, "https://api.trakt.tv/oauth/token", &expected);
    }

    #[test]
    fn test_revoke_request() {
        let expected = json!({
            "token": "token",
            "client_id": CTX.client_id,
            "client_secret": "secret",
        });
        let req = revoke::Request {
            token: "token".to_owned(),
            client_secret: "secret".to_owned(),
        };
        assert_request(CTX, req, "https://api.trakt.tv/oauth/revoke", &expected);
    }

    #[test]
    fn test_device_code_request() {
        let expected = json!({
            "client_id": CTX.client_id,
        });
        let req = device_code::Request;
        assert_request(
            CTX,
            req,
            "https://api.trakt.tv/oauth/device/code",
            &expected,
        );
    }

    #[test]
    fn test_poll_token_request() {
        let expected = json!({
            "code": "code",
            "client_id": CTX.client_id,
            "client_secret": "secret",
        });
        let req = poll_token::Request {
            device_code: "code".to_owned(),
            client_secret: "secret".to_owned(),
        };
        assert_request(
            CTX,
            req,
            "https://api.trakt.tv/oauth/device/token",
            &expected,
        );
    }
}
