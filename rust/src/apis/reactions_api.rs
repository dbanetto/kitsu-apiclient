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


/// struct for typed errors of method `delete_resource1234`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteResource1234Error {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_fetch_collection1234567`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFetchCollection1234567Error {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_fetch_resource1234567`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFetchResource1234567Error {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `media_reactions_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaReactionsCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `media_reactions_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaReactionsDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `media_reactions_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaReactionsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `media_reactions_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaReactionsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `media_reactions_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaReactionsUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `patch_update_resource123`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchUpdateResource123Error {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `post_create_resource123`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateResource123Error {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `review_likes_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewLikesCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `review_likes_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewLikesDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `review_likes_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewLikesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `review_likes_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewLikesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `review_likes_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewLikesUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `reviews_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewsCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `reviews_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewsDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `reviews_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `reviews_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `reviews_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewsUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}


    pub async fn delete_resource1234(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<DeleteResource1234Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reaction-votes/{id}", configuration.base_path, id=id);
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
            let entity: Option<DeleteResource1234Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_fetch_collection1234567(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FetchCollectionresponse8, Error<GetFetchCollection1234567Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reaction-votes", configuration.base_path);
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
            let entity: Option<GetFetchCollection1234567Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_fetch_resource1234567(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FetchResourceresponse8, Error<GetFetchResource1234567Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reaction-votes/{id}", configuration.base_path, id=id);
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
            let entity: Option<GetFetchResource1234567Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn media_reactions_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<MediaReactionsCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reactions", configuration.base_path);
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
            let entity: Option<MediaReactionsCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn media_reactions_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<MediaReactionsDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reactions/{id}", configuration.base_path, id=id);
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
            let entity: Option<MediaReactionsDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn media_reactions_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::MediaReactionsFetchCollectionresponse, Error<MediaReactionsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reactions", configuration.base_path);
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
            let entity: Option<MediaReactionsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn media_reactions_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::MediaReactionsFetchResourceresponse, Error<MediaReactionsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reactions/{id}", configuration.base_path, id=id);
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
            let entity: Option<MediaReactionsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn media_reactions_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<MediaReactionsUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reactions/{id}", configuration.base_path, id=id);
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
            let entity: Option<MediaReactionsUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn patch_update_resource123(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<PatchUpdateResource123Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reaction-votes/{id}", configuration.base_path, id=id);
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
            let entity: Option<PatchUpdateResource123Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn post_create_resource123(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<PostCreateResource123Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/media-reaction-votes", configuration.base_path);
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
            let entity: Option<PostCreateResource123Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn review_likes_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<ReviewLikesCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/review-likes", configuration.base_path);
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
            let entity: Option<ReviewLikesCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn review_likes_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<ReviewLikesDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/review-likes/{id}", configuration.base_path, id=id);
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
            let entity: Option<ReviewLikesDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn review_likes_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::ReviewLikesFetchCollectionresponse, Error<ReviewLikesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/review-likes", configuration.base_path);
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
            let entity: Option<ReviewLikesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn review_likes_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::ReviewLikesFetchResourceresponse, Error<ReviewLikesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/review-likes/{id}", configuration.base_path, id=id);
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
            let entity: Option<ReviewLikesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn review_likes_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<ReviewLikesUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/review-likes/{id}", configuration.base_path, id=id);
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
            let entity: Option<ReviewLikesUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn reviews_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<ReviewsCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/reviews", configuration.base_path);
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
            let entity: Option<ReviewsCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn reviews_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<ReviewsDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/reviews/{id}", configuration.base_path, id=id);
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
            let entity: Option<ReviewsDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn reviews_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::ReviewsFetchCollectionresponse, Error<ReviewsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/reviews", configuration.base_path);
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
            let entity: Option<ReviewsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn reviews_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::ReviewsFetchResourceresponse, Error<ReviewsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/reviews/{id}", configuration.base_path, id=id);
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
            let entity: Option<ReviewsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn reviews_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<ReviewsUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/reviews/{id}", configuration.base_path, id=id);
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
            let entity: Option<ReviewsUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

