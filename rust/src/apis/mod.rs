use reqwest;
use serde_json;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod anime_api;
pub mod categories_api;
pub mod characters_api;
pub mod comments_api;
pub mod groups_api;
pub mod manga_api;
pub mod media_follows_api;
pub mod media_relations_api;
pub mod posts_api;
pub mod producers_staff_api;
pub mod reactions_api;
pub mod reports_api;
pub mod site_announcements_api;
pub mod streamers_api;
pub mod user_libraries_api;
pub mod users_api;

pub mod configuration;
