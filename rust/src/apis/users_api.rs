/*
 * Kitsu API Docs
 *
 * Kitsu is a modern anime discovery platform that helps you track the anime you're watching, discover new anime and socialize with other fans.  With the Kitsu API you can do everything the client can do and much more.  API path: `https://kitsu.io/api/edge`  <!-- # Versioning  Every year, we release a new version of the API, numbered by the last two digits of the current year. For example, the root URL of this (the 2017) release is `https://kitsu.io/api/17/`.  No fields, endpoints, or resources will be removed until the next year's release, but may be changed to return empty values (arrays, empty strings, etc.) before then. Fields, endpoints, and resources may be added throughout the lifetime of a release.  In addition to these versioned APIs, there is access to the same API our website uses. However, this offers no guarantees: anything could change at any time. We suggest you don't use this, but if you need to, it can be accessed at `https://kitsu.io/api/edge/`.  ## Life Cycle  Upon release of a new version, the previous version will be maintained for one year or until usage drops below 2% of API traffic.  During this period, it will not be updated to have any new fields, endpoints, or resources. You are expected to keep your applications running on the latest version of the API. For most applications, upgrading should take no more than a few hours of work. -->  # JSON API  The Kitsu API implements the JSON API specification. This means there are some notable semantics to how you consume it, but understanding it will take a lot of the work of using it out of your hands.  These docs will include a short overview of the capabilities, but you can consult the [JSON API Specification][jsonapi] for more information.  You can be more specific about the data you want to retrieve by using URL parameters and are outlined below.  **Note:** This documentation will display parameters with brackets (`[` and `]`) for readability, but actual URLs will need to be percent-encoded (`%5B` and `%5D`).  ## Request Headers  As per the JSON API specification, all requests to the API should contain these headers:  ``` Accept: application/vnd.api+json Content-Type: application/vnd.api+json ```  ## Filtering and Search  Filtering lets you query data that contains certain matching attributes or relationships. These take the form of `filter[attribute]=value`. For example, you can request all the anime of the Adventure category:  ``` /anime?filter[categories]=adventure ```  For some models, you can also search based on the query text:  ``` /anime?filter[text]=cowboy%20bebop ```  ## Pagination  You can choose how much of a resource to receive by specifying pagination parameters. Pagination is supported via `limit` and `offset`. Resources are paginated in groups of `10` by default and can be increased to a maximum of `20`.  `/anime?page[limit]=5&page[offset]=0`  The response will include URLs for the first, next and last page of resources in the `links` object based on your request.  ``` \"links\": {     \"first\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=0\",     \"next\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=5\",     \"last\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=12062\" } ```  ## Sorting  Sorting by attributes is also supported. By default, sorts are applied in ascending order. You can request a descending order by prepending `-` to the parameter. You can use a comma-delimited list to sort by multiple attributes.  `/users?sort=-followersCount,-followingCount`  ## Includes  You can include related resources with `include=[relationship]`. You can also specify successive relationships using a `.`. A comma-delimited list can be used to request multiple relationships.  `/anime?include=categories,mediaRelationships.destination`  ## Sparse Fieldsets  You can request a resource to only return a specific set of fields in its response. For example, to only receive a user's name and creation date:  `/users?fields[users]=name,createdAt`  ## Client Implementations  JSON API has a great advantage in that since its standardised, API-agnostic tools can be made to abstract away the semantics of consuming and working with the data. It is recommended that you use a JSON API client to implement the Kitsu API for this reason.  Many implementations in over 13 languages can be found on the [JSON API website][jsonapi-client].  # HTTP Methods  Method   | Description -------- | ----------- `GET`    | Fetch - retrieve resources `POST`   | Create - create new resources `PATCH`  | Update - (partially) modify existing resources `DELETE` | Delete - remove resources  # Status Codes  Code  | Description ----- | ----------- `200` | OK - request succeeded (GET, PATCH, DELETE) `201` | Created - new resource created (POST) `204` | No Content - request succeeded (DELETE) `400` | Bad Request - malformed request `401` | Unauthorized - invalid or no authentication details provided `404` | Not Found - resource does not exist `406` | Not Acceptable - invalid `Accept` header `5xx` | Server Error  # Tutorials  - [You and your Kitsu Anime library](https://github.com/pheyvaer/kitsu-tutorial/blob/master/index.md)  # Questions?  If you have any questions you can:  - Join our [Discord server][discord]  - Join our Slack by sending an email to josh@kitsu.io  - Ping [@wopian][wopian], [@matthewdias][matthewdias] or [@nuck][nuck] on Kitsu.  [jsonapi]:http://jsonapi.org/format/ [jsonapi-client]:http://jsonapi.org/implementations/#client-libraries [wopian]:https://kitsu.io/users/wopian [matthewdias]:https://kitsu.io/users/matthewdias [nuck]:https://kitsu.io/users/nuck [discord]:https://invite.gg/kitsu
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;

use std::option::Option;

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `delete_resource12`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteResource12Error {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `favorites_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FavoritesCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `favorites_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FavoritesDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `favorites_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FavoritesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `favorites_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FavoritesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `favorites_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FavoritesUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `follows_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FollowsCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `follows_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FollowsDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `follows_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FollowsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `follows_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FollowsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `follows_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FollowsUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_fetch_collection12345`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFetchCollection12345Error {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_fetch_resource12345`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFetchResource12345Error {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `linked_accounts_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkedAccountsCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `linked_accounts_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkedAccountsDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `linked_accounts_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkedAccountsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `linked_accounts_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkedAccountsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `linked_accounts_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkedAccountsUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `patch_update_resource1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchUpdateResource1Error {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `post_create_resource1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateResource1Error {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_link_sites_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileLinkSitesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_link_sites_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileLinkSitesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_links_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileLinksCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_links_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileLinksDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_links_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileLinksFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_links_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileLinksFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_links_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileLinksUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `roles_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolesCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `roles_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolesDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `roles_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `roles_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `roles_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolesUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `stats_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatsDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `stats_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `stats_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `user_roles_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserRolesCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `user_roles_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserRolesDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `user_roles_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserRolesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `user_roles_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserRolesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}


    pub async fn delete_resource12(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<DeleteResource12Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/blocks/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<DeleteResource12Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn favorites_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<FavoritesCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/favorites", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<FavoritesCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn favorites_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<FavoritesDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/favorites/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<FavoritesDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn favorites_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FavoritesFetchCollectionresponse, Error<FavoritesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/favorites", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<FavoritesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn favorites_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FavoritesFetchResourceresponse, Error<FavoritesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/favorites/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<FavoritesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn favorites_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<FavoritesUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/favorites/{id}", configuration.base_path, id=id);
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<FavoritesUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn follows_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<FollowsCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/follows", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<FollowsCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn follows_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<FollowsDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/follows/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<FollowsDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn follows_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FollowsFetchCollectionresponse, Error<FollowsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/follows", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<FollowsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn follows_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FollowsFetchResourceresponse, Error<FollowsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/follows/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<FollowsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn follows_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<FollowsUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/follows/{id}", configuration.base_path, id=id);
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<FollowsUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_fetch_collection12345(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FetchCollectionresponse6, Error<GetFetchCollection12345Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/blocks", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetFetchCollection12345Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_fetch_resource12345(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FetchResourceresponse6, Error<GetFetchResource12345Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/blocks/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetFetchResource12345Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn linked_accounts_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<LinkedAccountsCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/linked-accounts", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<LinkedAccountsCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn linked_accounts_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<LinkedAccountsDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/linked-accounts/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<LinkedAccountsDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn linked_accounts_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::LinkedAccountsFetchCollectionresponse, Error<LinkedAccountsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/linked-accounts", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<LinkedAccountsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn linked_accounts_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::LinkedAccountsFetchResourceresponse, Error<LinkedAccountsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/linked-accounts/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<LinkedAccountsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn linked_accounts_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<LinkedAccountsUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/linked-accounts/{id}", configuration.base_path, id=id);
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<LinkedAccountsUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn patch_update_resource1(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<PatchUpdateResource1Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/blocks/{id}", configuration.base_path, id=id);
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<PatchUpdateResource1Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn post_create_resource1(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<PostCreateResource1Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/blocks", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<PostCreateResource1Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn profile_link_sites_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::ProfileLinkSitesFetchCollectionresponse, Error<ProfileLinkSitesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/profile-link-sites", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ProfileLinkSitesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn profile_link_sites_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::ProfileLinkSitesFetchResourceresponse, Error<ProfileLinkSitesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/profile-link-sites/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ProfileLinkSitesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn profile_links_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<ProfileLinksCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/profile-links", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<ProfileLinksCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn profile_links_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<ProfileLinksDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/profile-links/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<ProfileLinksDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn profile_links_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::ProfileLinksFetchCollectionresponse, Error<ProfileLinksFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/profile-links", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ProfileLinksFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn profile_links_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::ProfileLinksFetchResourceresponse, Error<ProfileLinksFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/profile-links/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ProfileLinksFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn profile_links_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<ProfileLinksUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/profile-links/{id}", configuration.base_path, id=id);
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<ProfileLinksUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn roles_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<RolesCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/roles", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<RolesCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn roles_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<RolesDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/roles/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<RolesDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn roles_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::RolesFetchCollectionresponse, Error<RolesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/roles", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<RolesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn roles_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::RolesFetchResourceresponse, Error<RolesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/roles/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<RolesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn roles_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<RolesUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/roles/{id}", configuration.base_path, id=id);
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<RolesUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn stats_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<StatsDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/stats/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<StatsDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn stats_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::StatsFetchCollectionresponse, Error<StatsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/stats", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<StatsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn stats_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::StatsFetchResourceresponse, Error<StatsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/stats/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<StatsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn user_roles_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<UserRolesCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/user-roles", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<UserRolesCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn user_roles_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<UserRolesDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/user-roles/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<UserRolesDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn user_roles_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::UserRolesFetchCollectionresponse, Error<UserRolesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/user-roles", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UserRolesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn user_roles_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::UserRolesFetchResourceresponse, Error<UserRolesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/user-roles/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UserRolesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<UsersCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/users", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<UsersCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<UsersDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/users/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<UsersDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::UsersFetchCollectionresponse, Error<UsersFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/users", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("id", &id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UsersFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::UsersFetchResourceresponse, Error<UsersFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/users/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UsersFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<UsersUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/users/{id}", configuration.base_path, id=id);
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<UsersUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

