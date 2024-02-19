//! Follower related endpoints

pub mod pending_requests {
    //! Get pending follow requests
    //!
    //! <https://trakt.docs.apiary.io/#reference/users/settings/get-pending-following-requests>

    use time::OffsetDateTime;

    use crate::smo::User;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/users/requests/following",
    auth = Required,
    )]
    pub struct Request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response)]
    pub struct Response(Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub id: u64,
        #[serde(with = "time::serde::iso8601")]
        pub requested_at: OffsetDateTime,
        pub user: User,
    }
}

pub mod requests {
    //! Get follow requests
    //!
    //! <https://trakt.docs.apiary.io/#reference/users/follower-requests/get-follow-requests>

    use time::OffsetDateTime;

    use crate::smo::User;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/users/requests",
    auth = Required,
    )]
    pub struct Request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response)]
    pub struct Response(Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub id: u64,
        #[serde(with = "time::serde::iso8601")]
        pub requested_at: OffsetDateTime,
        pub user: User,
    }
}

pub mod approve {
    //! Approve a follow request
    //!
    //! <https://trakt.docs.apiary.io/#reference/users/follower-requests/approve-follow-request>

    use time::OffsetDateTime;

    use crate::smo::User;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/users/requests/{id}",
    method = POST,
    auth = Required,
    )]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        #[serde(with = "time::serde::iso8601")]
        pub followed_at: OffsetDateTime,
        pub user: User,
    }
}

pub mod deny {
    //! Deny a follow request
    //!
    //! <https://trakt.docs.apiary.io/#reference/users/approve-or-deny-follower-requests/deny-follow-request>

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/users/requests/{id}",
    method = DELETE,
    auth = Required,
    )]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = NO_CONTENT)]
    pub struct Response;
}
