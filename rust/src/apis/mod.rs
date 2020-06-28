use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod anime_api;
pub use self::anime_api::{ AnimeApi, AnimeApiClient };
mod categories_api;
pub use self::categories_api::{ CategoriesApi, CategoriesApiClient };
mod characters_api;
pub use self::characters_api::{ CharactersApi, CharactersApiClient };
mod comments_api;
pub use self::comments_api::{ CommentsApi, CommentsApiClient };
mod groups_api;
pub use self::groups_api::{ GroupsApi, GroupsApiClient };
mod manga_api;
pub use self::manga_api::{ MangaApi, MangaApiClient };
mod media_follows_api;
pub use self::media_follows_api::{ MediaFollowsApi, MediaFollowsApiClient };
mod media_relations_api;
pub use self::media_relations_api::{ MediaRelationsApi, MediaRelationsApiClient };
mod posts_api;
pub use self::posts_api::{ PostsApi, PostsApiClient };
mod producers_staff_api;
pub use self::producers_staff_api::{ ProducersStaffApi, ProducersStaffApiClient };
mod reactions_api;
pub use self::reactions_api::{ ReactionsApi, ReactionsApiClient };
mod reports_api;
pub use self::reports_api::{ ReportsApi, ReportsApiClient };
mod site_announcements_api;
pub use self::site_announcements_api::{ SiteAnnouncementsApi, SiteAnnouncementsApiClient };
mod streamers_api;
pub use self::streamers_api::{ StreamersApi, StreamersApiClient };
mod user_libraries_api;
pub use self::user_libraries_api::{ UserLibrariesApi, UserLibrariesApiClient };
mod users_api;
pub use self::users_api::{ UsersApi, UsersApiClient };

pub mod configuration;
pub mod client;
