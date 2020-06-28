/*
 * Kitsu API Docs
 *
 * Kitsu is a modern anime discovery platform that helps you track the anime you're watching, discover new anime and socialize with other fans.  With the Kitsu API you can do everything the client can do and much more.  API path: `https://kitsu.io/api/edge`  <!-- # Versioning  Every year, we release a new version of the API, numbered by the last two digits of the current year. For example, the root URL of this (the 2017) release is `https://kitsu.io/api/17/`.  No fields, endpoints, or resources will be removed until the next year's release, but may be changed to return empty values (arrays, empty strings, etc.) before then. Fields, endpoints, and resources may be added throughout the lifetime of a release.  In addition to these versioned APIs, there is access to the same API our website uses. However, this offers no guarantees: anything could change at any time. We suggest you don't use this, but if you need to, it can be accessed at `https://kitsu.io/api/edge/`.  ## Life Cycle  Upon release of a new version, the previous version will be maintained for one year or until usage drops below 2% of API traffic.  During this period, it will not be updated to have any new fields, endpoints, or resources. You are expected to keep your applications running on the latest version of the API. For most applications, upgrading should take no more than a few hours of work. -->  # JSON API  The Kitsu API implements the JSON API specification. This means there are some notable semantics to how you consume it, but understanding it will take a lot of the work of using it out of your hands.  These docs will include a short overview of the capabilities, but you can consult the [JSON API Specification][jsonapi] for more information.  You can be more specific about the data you want to retrieve by using URL parameters and are outlined below.  **Note:** This documentation will display parameters with brackets (`[` and `]`) for readability, but actual URLs will need to be percent-encoded (`%5B` and `%5D`).  ## Request Headers  As per the JSON API specification, all requests to the API should contain these headers:  ``` Accept: application/vnd.api+json Content-Type: application/vnd.api+json ```  ## Filtering and Search  Filtering lets you query data that contains certain matching attributes or relationships. These take the form of `filter[attribute]=value`. For example, you can request all the anime of the Adventure category:  ``` /anime?filter[categories]=adventure ```  For some models, you can also search based on the query text:  ``` /anime?filter[text]=cowboy%20bebop ```  ## Pagination  You can choose how much of a resource to receive by specifying pagination parameters. Pagination is supported via `limit` and `offset`. Resources are paginated in groups of `10` by default and can be increased to a maximum of `20`.  `/anime?page[limit]=5&page[offset]=0`  The response will include URLs for the first, next and last page of resources in the `links` object based on your request.  ``` \"links\": {     \"first\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=0\",     \"next\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=5\",     \"last\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=12062\" } ```  ## Sorting  Sorting by attributes is also supported. By default, sorts are applied in ascending order. You can request a descending order by prepending `-` to the parameter. You can use a comma-delimited list to sort by multiple attributes.  `/users?sort=-followersCount,-followingCount`  ## Includes  You can include related resources with `include=[relationship]`. You can also specify successive relationships using a `.`. A comma-delimited list can be used to request multiple relationships.  `/anime?include=categories,mediaRelationships.destination`  ## Sparse Fieldsets  You can request a resource to only return a specific set of fields in its response. For example, to only receive a user's name and creation date:  `/users?fields[users]=name,createdAt`  ## Client Implementations  JSON API has a great advantage in that since its standardised, API-agnostic tools can be made to abstract away the semantics of consuming and working with the data. It is recommended that you use a JSON API client to implement the Kitsu API for this reason.  Many implementations in over 13 languages can be found on the [JSON API website][jsonapi-client].  # HTTP Methods  Method   | Description -------- | ----------- `GET`    | Fetch - retrieve resources `POST`   | Create - create new resources `PATCH`  | Update - (partially) modify existing resources `DELETE` | Delete - remove resources  # Status Codes  Code  | Description ----- | ----------- `200` | OK - request succeeded (GET, PATCH, DELETE) `201` | Created - new resource created (POST) `204` | No Content - request succeeded (DELETE) `400` | Bad Request - malformed request `401` | Unauthorized - invalid or no authentication details provided `404` | Not Found - resource does not exist `406` | Not Acceptable - invalid `Accept` header `5xx` | Server Error  # Tutorials  - [You and your Kitsu Anime library](https://github.com/pheyvaer/kitsu-tutorial/blob/master/index.md)  # Questions?  If you have any questions you can:  - Join our [Discord server][discord]  - Join our Slack by sending an email to josh@kitsu.io  - Ping [@wopian][wopian], [@matthewdias][matthewdias] or [@nuck][nuck] on Kitsu.  [jsonapi]:http://jsonapi.org/format/ [jsonapi-client]:http://jsonapi.org/implementations/#client-libraries [wopian]:https://kitsu.io/users/wopian [matthewdias]:https://kitsu.io/users/matthewdias [nuck]:https://kitsu.io/users/nuck [discord]:https://invite.gg/kitsu
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2004DataAttributesRatingFrequencies {
    #[serde(rename = "2", skip_serializing_if = "Option::is_none")]
    pub var_2: Option<String>,
    #[serde(rename = "3", skip_serializing_if = "Option::is_none")]
    pub var_3: Option<String>,
    #[serde(rename = "4", skip_serializing_if = "Option::is_none")]
    pub var_4: Option<String>,
    #[serde(rename = "5", skip_serializing_if = "Option::is_none")]
    pub var_5: Option<String>,
    #[serde(rename = "6", skip_serializing_if = "Option::is_none")]
    pub var_6: Option<String>,
    #[serde(rename = "7", skip_serializing_if = "Option::is_none")]
    pub var_7: Option<String>,
    #[serde(rename = "8", skip_serializing_if = "Option::is_none")]
    pub var_8: Option<String>,
    #[serde(rename = "9", skip_serializing_if = "Option::is_none")]
    pub var_9: Option<String>,
    #[serde(rename = "10", skip_serializing_if = "Option::is_none")]
    pub var_10: Option<String>,
    #[serde(rename = "11", skip_serializing_if = "Option::is_none")]
    pub var_11: Option<String>,
    #[serde(rename = "12", skip_serializing_if = "Option::is_none")]
    pub var_12: Option<String>,
    #[serde(rename = "13", skip_serializing_if = "Option::is_none")]
    pub var_13: Option<String>,
    #[serde(rename = "14", skip_serializing_if = "Option::is_none")]
    pub var_14: Option<String>,
    #[serde(rename = "15", skip_serializing_if = "Option::is_none")]
    pub var_15: Option<String>,
    #[serde(rename = "16", skip_serializing_if = "Option::is_none")]
    pub var_16: Option<String>,
    #[serde(rename = "17", skip_serializing_if = "Option::is_none")]
    pub var_17: Option<String>,
    #[serde(rename = "18", skip_serializing_if = "Option::is_none")]
    pub var_18: Option<String>,
    #[serde(rename = "19", skip_serializing_if = "Option::is_none")]
    pub var_19: Option<String>,
    #[serde(rename = "20", skip_serializing_if = "Option::is_none")]
    pub var_20: Option<String>,
}

impl InlineResponse2004DataAttributesRatingFrequencies {
    pub fn new() -> InlineResponse2004DataAttributesRatingFrequencies {
        InlineResponse2004DataAttributesRatingFrequencies {
            var_2: None,
            var_3: None,
            var_4: None,
            var_5: None,
            var_6: None,
            var_7: None,
            var_8: None,
            var_9: None,
            var_10: None,
            var_11: None,
            var_12: None,
            var_13: None,
            var_14: None,
            var_15: None,
            var_16: None,
            var_17: None,
            var_18: None,
            var_19: None,
            var_20: None,
        }
    }
}


