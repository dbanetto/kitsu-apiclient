/*
 * Kitsu API Docs
 *
 * Kitsu is a modern anime discovery platform that helps you track the anime you're watching, discover new anime and socialize with other fans.  With the Kitsu API you can do everything the client can do and much more.  API path: `https://kitsu.io/api/edge`  <!-- # Versioning  Every year, we release a new version of the API, numbered by the last two digits of the current year. For example, the root URL of this (the 2017) release is `https://kitsu.io/api/17/`.  No fields, endpoints, or resources will be removed until the next year's release, but may be changed to return empty values (arrays, empty strings, etc.) before then. Fields, endpoints, and resources may be added throughout the lifetime of a release.  In addition to these versioned APIs, there is access to the same API our website uses. However, this offers no guarantees: anything could change at any time. We suggest you don't use this, but if you need to, it can be accessed at `https://kitsu.io/api/edge/`.  ## Life Cycle  Upon release of a new version, the previous version will be maintained for one year or until usage drops below 2% of API traffic.  During this period, it will not be updated to have any new fields, endpoints, or resources. You are expected to keep your applications running on the latest version of the API. For most applications, upgrading should take no more than a few hours of work. -->  # JSON API  The Kitsu API implements the JSON API specification. This means there are some notable semantics to how you consume it, but understanding it will take a lot of the work of using it out of your hands.  These docs will include a short overview of the capabilities, but you can consult the [JSON API Specification][jsonapi] for more information.  You can be more specific about the data you want to retrieve by using URL parameters and are outlined below.  **Note:** This documentation will display parameters with brackets (`[` and `]`) for readability, but actual URLs will need to be percent-encoded (`%5B` and `%5D`).  ## Request Headers  As per the JSON API specification, all requests to the API should contain these headers:  ``` Accept: application/vnd.api+json Content-Type: application/vnd.api+json ```  ## Filtering and Search  Filtering lets you query data that contains certain matching attributes or relationships. These take the form of `filter[attribute]=value`. For example, you can request all the anime of the Adventure category:  ``` /anime?filter[categories]=adventure ```  For some models, you can also search based on the query text:  ``` /anime?filter[text]=cowboy%20bebop ```  ## Pagination  You can choose how much of a resource to receive by specifying pagination parameters. Pagination is supported via `limit` and `offset`. Resources are paginated in groups of `10` by default and can be increased to a maximum of `20`.  `/anime?page[limit]=5&page[offset]=0`  The response will include URLs for the first, next and last page of resources in the `links` object based on your request.  ``` \"links\": {     \"first\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=0\",     \"next\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=5\",     \"last\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=12062\" } ```  ## Sorting  Sorting by attributes is also supported. By default, sorts are applied in ascending order. You can request a descending order by prepending `-` to the parameter. You can use a comma-delimited list to sort by multiple attributes.  `/users?sort=-followersCount,-followingCount`  ## Includes  You can include related resources with `include=[relationship]`. You can also specify successive relationships using a `.`. A comma-delimited list can be used to request multiple relationships.  `/anime?include=categories,mediaRelationships.destination`  ## Sparse Fieldsets  You can request a resource to only return a specific set of fields in its response. For example, to only receive a user's name and creation date:  `/users?fields[users]=name,createdAt`  ## Client Implementations  JSON API has a great advantage in that since its standardised, API-agnostic tools can be made to abstract away the semantics of consuming and working with the data. It is recommended that you use a JSON API client to implement the Kitsu API for this reason.  Many implementations in over 13 languages can be found on the [JSON API website][jsonapi-client].  # HTTP Methods  Method   | Description -------- | ----------- `GET`    | Fetch - retrieve resources `POST`   | Create - create new resources `PATCH`  | Update - (partially) modify existing resources `DELETE` | Delete - remove resources  # Status Codes  Code  | Description ----- | ----------- `200` | OK - request succeeded (GET, PATCH, DELETE) `201` | Created - new resource created (POST) `204` | No Content - request succeeded (DELETE) `400` | Bad Request - malformed request `401` | Unauthorized - invalid or no authentication details provided `404` | Not Found - resource does not exist `406` | Not Acceptable - invalid `Accept` header `5xx` | Server Error  # Tutorials  - [You and your Kitsu Anime library](https://github.com/pheyvaer/kitsu-tutorial/blob/master/index.md)  # Questions?  If you have any questions you can:  - Join our [Discord server][discord]  - Join our Slack by sending an email to josh@kitsu.io  - Ping [@wopian][wopian], [@matthewdias][matthewdias] or [@nuck][nuck] on Kitsu.  [jsonapi]:http://jsonapi.org/format/ [jsonapi-client]:http://jsonapi.org/implementations/#client-libraries [wopian]:https://kitsu.io/users/wopian [matthewdias]:https://kitsu.io/users/matthewdias [nuck]:https://kitsu.io/users/nuck [discord]:https://invite.gg/kitsu
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct GroupsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> GroupsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> GroupsApiClient<C> {
        GroupsApiClient {
            configuration,
        }
    }
}

pub trait GroupsApi {
    fn create_resource(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_0(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_1(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_10(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_2(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_3(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_4(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_5(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_6(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_7(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_8(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_resource_9(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource_0(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource_1(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource_2(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource_3(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource_4(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource_5(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource_6(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource_7(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_resource_8(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn fetch_collection(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_0(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_1(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_10(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_11(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_2(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_3(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_4(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_5(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_6(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_7(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_8(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_collection_9(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn fetch_resource(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20017, Error = Error<serde_json::Value>>>;
    fn fetch_resource_0(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20018, Error = Error<serde_json::Value>>>;
    fn fetch_resource_1(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20019, Error = Error<serde_json::Value>>>;
    fn fetch_resource_10(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20028, Error = Error<serde_json::Value>>>;
    fn fetch_resource_11(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20030, Error = Error<serde_json::Value>>>;
    fn fetch_resource_2(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20020, Error = Error<serde_json::Value>>>;
    fn fetch_resource_3(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20021, Error = Error<serde_json::Value>>>;
    fn fetch_resource_4(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20022, Error = Error<serde_json::Value>>>;
    fn fetch_resource_5(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20023, Error = Error<serde_json::Value>>>;
    fn fetch_resource_6(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20024, Error = Error<serde_json::Value>>>;
    fn fetch_resource_7(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20025, Error = Error<serde_json::Value>>>;
    fn fetch_resource_8(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20026, Error = Error<serde_json::Value>>>;
    fn fetch_resource_9(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20027, Error = Error<serde_json::Value>>>;
    fn update_resource(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_resource_0(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_resource_1(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_resource_2(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_resource_3(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_resource_4(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_resource_5(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_resource_6(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_resource_7(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>GroupsApi for GroupsApiClient<C> {
    fn create_resource(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-bans".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_0(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-categories".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_1(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-invites".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_10(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/leader-chat-messages".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_2(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-member-notes".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_3(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-members".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_4(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-neighbors".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_5(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-permissions".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_6(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-reports".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_7(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-ticket-messages".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_8(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/group-tickets".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn create_resource_9(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/groups".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/group-bans/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource_0(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/group-categories/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource_1(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/group-invites/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource_2(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/group-member-notes/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource_3(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/group-members/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource_4(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/group-neighbors/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource_5(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/group-permissions/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource_6(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/group-ticket-messages/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource_7(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/groups/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_resource_8(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/leader-chat-messages/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-action-logs".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_0(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-bans".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_1(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-categories".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_10(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/groups".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_11(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/leader-chat-messages".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_2(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-invites".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_3(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-member-notes".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_4(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-members".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_5(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-neighbors".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_6(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-permissions".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_7(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-reports".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_8(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-ticket-messages".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_collection_9(&self, ) -> Box<dyn Future<Item = crate::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-tickets".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20017, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-action-logs/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_0(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20018, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-bans/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_1(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20019, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-categories/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_10(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20028, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/groups/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_11(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20030, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/leader-chat-messages/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_2(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20020, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-invites/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_3(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20021, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-member-notes/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_4(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20022, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-members/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_5(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20023, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-neighbors/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_6(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20024, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-permissions/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_7(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20025, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-reports/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_8(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20026, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-ticket-messages/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn fetch_resource_9(&self, id: f32) -> Box<dyn Future<Item = crate::models::InlineResponse20027, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/group-tickets/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn update_resource(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/group-categories/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn update_resource_0(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/group-invites/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn update_resource_1(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/group-member-notes/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn update_resource_2(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/group-members/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn update_resource_3(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/group-reports/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn update_resource_4(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/group-ticket-messages/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn update_resource_5(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/group-tickets/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn update_resource_6(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/groups/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn update_resource_7(&self, id: f32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/leader-chat-messages/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
