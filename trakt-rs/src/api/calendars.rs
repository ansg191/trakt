//! API endpoints for calendars
//!
//! <https://trakt.docs.apiary.io/#reference/calendars>

pub mod my {
    //! My calendars

    pub mod shows {
        //! Get shows
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/my-shows/get-shows>

        use time::Date;

        use crate::smo::EpisodeAirEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/my/shows/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<EpisodeAirEvent>);
    }

    pub mod new_shows {
        //! Get new show premieres
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/my-new-shows/get-new-shows>

        use time::Date;

        use crate::smo::EpisodeAirEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/my/shows/new/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<EpisodeAirEvent>);
    }

    pub mod season_premiers {
        //! Get season premieres
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/my-new-shows/get-season-premieres>

        use time::Date;

        use crate::smo::EpisodeAirEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/my/shows/premieres/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<EpisodeAirEvent>);
    }

    pub mod finales {
        //! Get show finales
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/my-finales/get-finales>

        use time::Date;

        use crate::smo::EpisodeAirEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/my/shows/finales/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<EpisodeAirEvent>);
    }

    pub mod movies {
        //! Get movies
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/my-movies/get-movies>

        use time::Date;

        use crate::smo::MovieReleaseEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/my/movies/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<MovieReleaseEvent>);
    }

    pub mod dvd_releases {
        //! Get DVD releases
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/my-dvd/get-dvd-releases>

        use time::Date;

        use crate::smo::MovieReleaseEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/my/dvd/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<MovieReleaseEvent>);
    }
}

pub mod all {
    //! All calendars

    pub mod new_shows {
        //! Get all new show premieres
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/all-shows/get-new-shows>

        use time::Date;

        use crate::smo::EpisodeAirEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/all/shows/new/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<EpisodeAirEvent>);
    }

    pub mod season_premiers {
        //! Get all season premieres
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/all-season-premieres/get-season-premieres>

        use time::Date;

        use crate::smo::EpisodeAirEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/all/shows/premieres/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<EpisodeAirEvent>);
    }

    pub mod finales {
        //! Get all show finales
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/all-season-premieres/get-finales>

        use time::Date;

        use crate::smo::EpisodeAirEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/all/shows/finales/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<EpisodeAirEvent>);
    }

    pub mod movies {
        //! Get all movies
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/all-movies/get-movies>

        use time::Date;

        use crate::smo::MovieReleaseEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/all/movies/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<MovieReleaseEvent>);
    }

    pub mod dvd_releases {
        //! Get all DVD releases
        //!
        //! <https://trakt.docs.apiary.io/#reference/calendars/all-dvd/get-dvd-releases>

        use time::Date;

        use crate::smo::MovieReleaseEvent;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/calendars/all/dvd/{start_date}/{days}",
        auth = Required
        )]
        pub struct Request {
            #[serde(with = "crate::iso8601_date")]
            pub start_date: Date,
            pub days: u64,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
        pub struct Response(pub Vec<MovieReleaseEvent>);
    }
}
