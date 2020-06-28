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
pub struct UsersAttributes {
    /// Max length of 500 characters
    #[serde(rename = "about", skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    /// Logged in user only. Aozora user imports that had Aozora Pro - treated as Kitsu Pro
    #[serde(rename = "aoPro", skip_serializing_if = "Option::is_none")]
    pub ao_pro: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<crate::models::InlineResponse20063DataAttributesAvatar>,
    #[serde(rename = "birthday", skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(rename = "commentsCount", skip_serializing_if = "Option::is_none")]
    pub comments_count: Option<f32>,
    /// Logged in user only. Email confirmed
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
    /// Logged in user only.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "coverImage", skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<crate::models::InlineResponse20063DataAttributesCoverImage>,
    /// ISO 8601 date and time
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Logged in user only
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Logged in user only
    #[serde(rename = "facebookId", skip_serializing_if = "Option::is_none")]
    pub facebook_id: Option<String>,
    #[serde(rename = "favoritesCount", skip_serializing_if = "Option::is_none")]
    pub favorites_count: Option<f32>,
    /// Completed feeds onboarding
    #[serde(rename = "feedCompleted", skip_serializing_if = "Option::is_none")]
    pub feed_completed: Option<bool>,
    #[serde(rename = "followersCount", skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<f32>,
    #[serde(rename = "followingCount", skip_serializing_if = "Option::is_none")]
    pub following_count: Option<f32>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Logged in user only
    #[serde(rename = "hasPassword", skip_serializing_if = "Option::is_none")]
    pub has_password: Option<bool>,
    /// Logged in user only
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Deprecated, use the `stats` relationship
    #[serde(rename = "lifeSpentOnAnime", skip_serializing_if = "Option::is_none")]
    pub life_spent_on_anime: Option<f32>,
    #[serde(rename = "likesGivenCount", skip_serializing_if = "Option::is_none")]
    pub likes_given_count: Option<f32>,
    #[serde(rename = "likesReceivedCount", skip_serializing_if = "Option::is_none")]
    pub likes_received_count: Option<f32>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "mediaReactionsCount", skip_serializing_if = "Option::is_none")]
    pub media_reactions_count: Option<f32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Logged in user only. Used to set new password, always displays null
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "pastNames", skip_serializing_if = "Option::is_none")]
    pub past_names: Option<Vec<crate::models::AnyType>>,
    #[serde(rename = "postsCount", skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<f32>,
    /// Logged in user only
    #[serde(rename = "previousEmail", skip_serializing_if = "Option::is_none")]
    pub previous_email: Option<String>,
    #[serde(rename = "proExpiresAt", skip_serializing_if = "Option::is_none")]
    pub pro_expires_at: Option<String>,
    #[serde(rename = "proTier", skip_serializing_if = "Option::is_none")]
    pub pro_tier: Option<String>,
    /// Completed profile onboarding
    #[serde(rename = "profileCompleted", skip_serializing_if = "Option::is_none")]
    pub profile_completed: Option<bool>,
    /// Logged in user only
    #[serde(rename = "ratingSystem", skip_serializing_if = "Option::is_none")]
    pub rating_system: Option<RatingSystem>,
    #[serde(rename = "ratingsCount", skip_serializing_if = "Option::is_none")]
    pub ratings_count: Option<f32>,
    #[serde(rename = "reviewsCount", skip_serializing_if = "Option::is_none")]
    pub reviews_count: Option<f32>,
    /// Logged in user only. Toggle visibility of NSFW media and posts
    #[serde(rename = "sfwFilter", skip_serializing_if = "Option::is_none")]
    pub sfw_filter: Option<bool>,
    /// Logged in user only
    #[serde(rename = "shareToGlobal", skip_serializing_if = "Option::is_none")]
    pub share_to_global: Option<bool>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subscribedToNewsletter", skip_serializing_if = "Option::is_none")]
    pub subscribed_to_newsletter: Option<bool>,
    /// Logged in user only
    #[serde(rename = "theme", skip_serializing_if = "Option::is_none")]
    pub theme: Option<Theme>,
    /// Logged in user only
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Logged in user only
    #[serde(rename = "titleLanguagePreference canonical", skip_serializing_if = "Option::is_none")]
    pub title_language_preference_canonical: Option<TitleLanguagePreferenceCanonical>,
    /// ISO 8601 of last modification
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "waifuOrHusbando", skip_serializing_if = "Option::is_none")]
    pub waifu_or_husbando: Option<String>,
    /// Deprecated, use the `profileLinks` relationship
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl UsersAttributes {
    pub fn new() -> UsersAttributes {
        UsersAttributes {
            about: None,
            ao_pro: None,
            avatar: None,
            birthday: None,
            comments_count: None,
            confirmed: None,
            country: None,
            cover_image: None,
            created_at: None,
            email: None,
            facebook_id: None,
            favorites_count: None,
            feed_completed: None,
            followers_count: None,
            following_count: None,
            gender: None,
            has_password: None,
            language: None,
            life_spent_on_anime: None,
            likes_given_count: None,
            likes_received_count: None,
            location: None,
            media_reactions_count: None,
            name: None,
            password: None,
            past_names: None,
            posts_count: None,
            previous_email: None,
            pro_expires_at: None,
            pro_tier: None,
            profile_completed: None,
            rating_system: None,
            ratings_count: None,
            reviews_count: None,
            sfw_filter: None,
            share_to_global: None,
            slug: None,
            status: None,
            subscribed_to_newsletter: None,
            theme: None,
            time_zone: None,
            title: None,
            title_language_preference_canonical: None,
            updated_at: None,
            waifu_or_husbando: None,
            website: None,
        }
    }
}

/// Logged in user only
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingSystem {
    #[serde(rename = "advanced")]
    Advanced,
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "simple")]
    Simple,
}
/// Logged in user only
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Theme {
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
}
/// Logged in user only
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TitleLanguagePreferenceCanonical {
    #[serde(rename = "canonical")]
    Canonical,
    #[serde(rename = "romanized")]
    Romanized,
    #[serde(rename = "english")]
    English,
}

