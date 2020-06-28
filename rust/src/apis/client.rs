use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    anime_api: Box<dyn crate::apis::AnimeApi>,
    categories_api: Box<dyn crate::apis::CategoriesApi>,
    characters_api: Box<dyn crate::apis::CharactersApi>,
    comments_api: Box<dyn crate::apis::CommentsApi>,
    groups_api: Box<dyn crate::apis::GroupsApi>,
    manga_api: Box<dyn crate::apis::MangaApi>,
    media_follows_api: Box<dyn crate::apis::MediaFollowsApi>,
    media_relations_api: Box<dyn crate::apis::MediaRelationsApi>,
    posts_api: Box<dyn crate::apis::PostsApi>,
    producers_staff_api: Box<dyn crate::apis::ProducersStaffApi>,
    reactions_api: Box<dyn crate::apis::ReactionsApi>,
    reports_api: Box<dyn crate::apis::ReportsApi>,
    site_announcements_api: Box<dyn crate::apis::SiteAnnouncementsApi>,
    streamers_api: Box<dyn crate::apis::StreamersApi>,
    user_libraries_api: Box<dyn crate::apis::UserLibrariesApi>,
    users_api: Box<dyn crate::apis::UsersApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            anime_api: Box::new(crate::apis::AnimeApiClient::new(rc.clone())),
            categories_api: Box::new(crate::apis::CategoriesApiClient::new(rc.clone())),
            characters_api: Box::new(crate::apis::CharactersApiClient::new(rc.clone())),
            comments_api: Box::new(crate::apis::CommentsApiClient::new(rc.clone())),
            groups_api: Box::new(crate::apis::GroupsApiClient::new(rc.clone())),
            manga_api: Box::new(crate::apis::MangaApiClient::new(rc.clone())),
            media_follows_api: Box::new(crate::apis::MediaFollowsApiClient::new(rc.clone())),
            media_relations_api: Box::new(crate::apis::MediaRelationsApiClient::new(rc.clone())),
            posts_api: Box::new(crate::apis::PostsApiClient::new(rc.clone())),
            producers_staff_api: Box::new(crate::apis::ProducersStaffApiClient::new(rc.clone())),
            reactions_api: Box::new(crate::apis::ReactionsApiClient::new(rc.clone())),
            reports_api: Box::new(crate::apis::ReportsApiClient::new(rc.clone())),
            site_announcements_api: Box::new(crate::apis::SiteAnnouncementsApiClient::new(rc.clone())),
            streamers_api: Box::new(crate::apis::StreamersApiClient::new(rc.clone())),
            user_libraries_api: Box::new(crate::apis::UserLibrariesApiClient::new(rc.clone())),
            users_api: Box::new(crate::apis::UsersApiClient::new(rc.clone())),
        }
    }

    pub fn anime_api(&self) -> &dyn crate::apis::AnimeApi{
        self.anime_api.as_ref()
    }

    pub fn categories_api(&self) -> &dyn crate::apis::CategoriesApi{
        self.categories_api.as_ref()
    }

    pub fn characters_api(&self) -> &dyn crate::apis::CharactersApi{
        self.characters_api.as_ref()
    }

    pub fn comments_api(&self) -> &dyn crate::apis::CommentsApi{
        self.comments_api.as_ref()
    }

    pub fn groups_api(&self) -> &dyn crate::apis::GroupsApi{
        self.groups_api.as_ref()
    }

    pub fn manga_api(&self) -> &dyn crate::apis::MangaApi{
        self.manga_api.as_ref()
    }

    pub fn media_follows_api(&self) -> &dyn crate::apis::MediaFollowsApi{
        self.media_follows_api.as_ref()
    }

    pub fn media_relations_api(&self) -> &dyn crate::apis::MediaRelationsApi{
        self.media_relations_api.as_ref()
    }

    pub fn posts_api(&self) -> &dyn crate::apis::PostsApi{
        self.posts_api.as_ref()
    }

    pub fn producers_staff_api(&self) -> &dyn crate::apis::ProducersStaffApi{
        self.producers_staff_api.as_ref()
    }

    pub fn reactions_api(&self) -> &dyn crate::apis::ReactionsApi{
        self.reactions_api.as_ref()
    }

    pub fn reports_api(&self) -> &dyn crate::apis::ReportsApi{
        self.reports_api.as_ref()
    }

    pub fn site_announcements_api(&self) -> &dyn crate::apis::SiteAnnouncementsApi{
        self.site_announcements_api.as_ref()
    }

    pub fn streamers_api(&self) -> &dyn crate::apis::StreamersApi{
        self.streamers_api.as_ref()
    }

    pub fn user_libraries_api(&self) -> &dyn crate::apis::UserLibrariesApi{
        self.user_libraries_api.as_ref()
    }

    pub fn users_api(&self) -> &dyn crate::apis::UsersApi{
        self.users_api.as_ref()
    }

}
