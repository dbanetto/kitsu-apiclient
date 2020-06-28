# Rust API client for openapi

Kitsu is a modern anime discovery platform that helps you track the
anime you're watching, discover new anime and socialize with other
fans.

With the Kitsu API you can do everything the client can do and much
more.

API path: `https://kitsu.io/api/edge`

<!-- # Versioning

Every year, we release a new version of the API, numbered by the
last two digits of the current year. For example, the root URL of
this (the 2017) release is `https://kitsu.io/api/17/`.

No fields, endpoints, or resources will be removed until the next
year's release, but may be changed to return empty values (arrays,
empty strings, etc.) before then. Fields, endpoints, and resources
may be added throughout the lifetime of a release.

In addition to these versioned APIs, there is access to the same API
our website uses. However, this offers no guarantees: anything could
change at any time. We suggest you don't use this, but if you need
to, it can be accessed at `https://kitsu.io/api/edge/`.

## Life Cycle

Upon release of a new version, the previous version will be
maintained for one year or until usage drops below 2% of API
traffic.

During this period, it will not be updated to have any new fields,
endpoints, or resources. You are expected to keep your applications
running on the latest version of the API. For most applications,
upgrading should take no more than a few hours of work.
-->

# JSON API

The Kitsu API implements the JSON API specification. This means
there are some notable semantics to how you consume it, but
understanding it will take a lot of the work of using it out of your
hands.

These docs will include a short overview of the capabilities, but
you can consult the [JSON API Specification][jsonapi] for more
information.

You can be more specific about the data you want to retrieve by
using URL parameters and are outlined below.

**Note:** This documentation will display parameters with brackets
(`[` and `]`) for readability, but actual URLs will need to be
percent-encoded (`%5B` and `%5D`).

## Request Headers

As per the JSON API specification, all requests to the API should
contain these headers:

```
Accept: application/vnd.api+json
Content-Type: application/vnd.api+json
```

## Filtering and Search

Filtering lets you query data that contains certain matching
attributes or relationships. These take the form of
`filter[attribute]=value`. For example, you can request all the
anime of the Adventure category:

```
/anime?filter[categories]=adventure
```

For some models, you can also search based on the query text:

```
/anime?filter[text]=cowboy%20bebop
```

## Pagination

You can choose how much of a resource to receive by specifying
pagination parameters. Pagination is supported via `limit` and
`offset`. Resources are paginated in groups of `10` by default and can be
increased to a maximum of `20`.

`/anime?page[limit]=5&page[offset]=0`

The response will include URLs for the first, next and last page of
resources in the `links` object based on your request.

```
\"links\": {
    \"first\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=0\",
    \"next\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=5\",
    \"last\": \"https://kitsu.io/api/edge/anime?page[limit]=5&page[offset]=12062\"
}
```

## Sorting

Sorting by attributes is also supported. By default, sorts are
applied in ascending order. You can request a descending order by
prepending `-` to the parameter. You can use a comma-delimited list
to sort by multiple attributes.

`/users?sort=-followersCount,-followingCount`

## Includes

You can include related resources with `include=[relationship]`.
You can also specify successive relationships using a `.`. A
comma-delimited list can be used to request multiple relationships.

`/anime?include=categories,mediaRelationships.destination`

## Sparse Fieldsets

You can request a resource to only return a specific set of fields
in its response. For example, to only receive a user's name and
creation date:

`/users?fields[users]=name,createdAt`

## Client Implementations

JSON API has a great advantage in that since its standardised,
API-agnostic tools can be made to abstract away the semantics of
consuming and working with the data. It is recommended that you use
a JSON API client to implement the Kitsu API for this reason.

Many implementations in over 13 languages can be found on the
[JSON API website][jsonapi-client].

# HTTP Methods

Method   | Description
-------- | -----------
`GET`    | Fetch - retrieve resources
`POST`   | Create - create new resources
`PATCH`  | Update - (partially) modify existing resources
`DELETE` | Delete - remove resources

# Status Codes

Code  | Description
----- | -----------
`200` | OK - request succeeded (GET, PATCH, DELETE)
`201` | Created - new resource created (POST)
`204` | No Content - request succeeded (DELETE)
`400` | Bad Request - malformed request
`401` | Unauthorized - invalid or no authentication details provided
`404` | Not Found - resource does not exist
`406` | Not Acceptable - invalid `Accept` header
`5xx` | Server Error

# Tutorials

- [You and your Kitsu Anime library](https://github.com/pheyvaer/kitsu-tutorial/blob/master/index.md)

# Questions?

If you have any questions you can:

- Join our [Discord server][discord]

- Join our Slack by sending an email to josh@kitsu.io

- Ping [@wopian][wopian], [@matthewdias][matthewdias] or [@nuck][nuck] on Kitsu.

[jsonapi]:http://jsonapi.org/format/
[jsonapi-client]:http://jsonapi.org/implementations/#client-libraries
[wopian]:https://kitsu.io/users/wopian
[matthewdias]:https://kitsu.io/users/matthewdias
[nuck]:https://kitsu.io/users/nuck
[discord]:https://invite.gg/kitsu

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://kitsu.io/api/edge*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AnimeApi* | [**fetch_collection**](docs/AnimeApi.md#fetch_collection) | **Get** /anime | Fetch Collection
*AnimeApi* | [**fetch_collection_0**](docs/AnimeApi.md#fetch_collection_0) | **Get** /episodes | Fetch Collection
*AnimeApi* | [**fetch_collection_1**](docs/AnimeApi.md#fetch_collection_1) | **Get** /trending/anime | Fetch Collection
*AnimeApi* | [**fetch_resource**](docs/AnimeApi.md#fetch_resource) | **Get** /anime/{id} | Fetch Resource
*AnimeApi* | [**fetch_resource_0**](docs/AnimeApi.md#fetch_resource_0) | **Get** /episodes/{id} | Fetch Resource
*CategoriesApi* | [**create_resource**](docs/CategoriesApi.md#create_resource) | **Post** /category-favorites | Create Resource
*CategoriesApi* | [**delete_resource**](docs/CategoriesApi.md#delete_resource) | **Delete** /category-favorites/{id} | Delete Resource
*CategoriesApi* | [**fetch_collection**](docs/CategoriesApi.md#fetch_collection) | **Get** /categories | Fetch Collection
*CategoriesApi* | [**fetch_collection_0**](docs/CategoriesApi.md#fetch_collection_0) | **Get** /category-favorites | Fetch Collection
*CategoriesApi* | [**fetch_resource**](docs/CategoriesApi.md#fetch_resource) | **Get** /categories/{id} | Fetch Resource
*CategoriesApi* | [**fetch_resource_0**](docs/CategoriesApi.md#fetch_resource_0) | **Get** /category-favorites/{id} | Fetch Resource
*CategoriesApi* | [**update_resource**](docs/CategoriesApi.md#update_resource) | **Patch** /category-favorites/{id} | Update Resource
*CharactersApi* | [**create_resource**](docs/CharactersApi.md#create_resource) | **Post** /anime-characters | Create Resource
*CharactersApi* | [**create_resource_0**](docs/CharactersApi.md#create_resource_0) | **Post** /characters | Create Resource
*CharactersApi* | [**create_resource_1**](docs/CharactersApi.md#create_resource_1) | **Post** /manga-characters | Create Resource
*CharactersApi* | [**delete_resource**](docs/CharactersApi.md#delete_resource) | **Delete** /anime-characters/{id} | Delete Resource
*CharactersApi* | [**delete_resource_0**](docs/CharactersApi.md#delete_resource_0) | **Delete** /characters/{id} | Delete Resource
*CharactersApi* | [**delete_resource_1**](docs/CharactersApi.md#delete_resource_1) | **Delete** /manga-characters/{id} | Delete Resource
*CharactersApi* | [**fetch_collection**](docs/CharactersApi.md#fetch_collection) | **Get** /anime-characters | Fetch Collection
*CharactersApi* | [**fetch_collection_0**](docs/CharactersApi.md#fetch_collection_0) | **Get** /characters | Fetch Collection
*CharactersApi* | [**fetch_collection_1**](docs/CharactersApi.md#fetch_collection_1) | **Get** /manga-characters | Fetch Collection
*CharactersApi* | [**fetch_resource**](docs/CharactersApi.md#fetch_resource) | **Get** /anime-characters/{id} | Fetch Resource
*CharactersApi* | [**fetch_resource_0**](docs/CharactersApi.md#fetch_resource_0) | **Get** /characters/{id} | Fetch Resource
*CharactersApi* | [**fetch_resource_1**](docs/CharactersApi.md#fetch_resource_1) | **Get** /manga-characters/{id} | Fetch Resource
*CharactersApi* | [**update_resource**](docs/CharactersApi.md#update_resource) | **Patch** /anime-characters/{id} | Update Resource
*CharactersApi* | [**update_resource_0**](docs/CharactersApi.md#update_resource_0) | **Patch** /characters/{id} | Update Resource
*CharactersApi* | [**update_resource_1**](docs/CharactersApi.md#update_resource_1) | **Patch** /manga-characters/{id} | Update Resource
*CommentsApi* | [**create_resource**](docs/CommentsApi.md#create_resource) | **Post** /comment-likes | Create Resource
*CommentsApi* | [**create_resource_0**](docs/CommentsApi.md#create_resource_0) | **Post** /comments | Create Resource
*CommentsApi* | [**delete_resource**](docs/CommentsApi.md#delete_resource) | **Delete** /comment-likes/{id} | Delete Resource
*CommentsApi* | [**delete_resource_0**](docs/CommentsApi.md#delete_resource_0) | **Delete** /comments/{id} | Delete Resource
*CommentsApi* | [**fetch_collection**](docs/CommentsApi.md#fetch_collection) | **Get** /comment-likes | Fetch Collection
*CommentsApi* | [**fetch_collection_0**](docs/CommentsApi.md#fetch_collection_0) | **Get** /comments | Fetch Collection
*CommentsApi* | [**fetch_resource**](docs/CommentsApi.md#fetch_resource) | **Get** /comment-likes/{id} | Fetch Resource
*CommentsApi* | [**fetch_resource_0**](docs/CommentsApi.md#fetch_resource_0) | **Get** /comments/{id} | Fetch Resource
*CommentsApi* | [**update_resource**](docs/CommentsApi.md#update_resource) | **Patch** /comments/{id} | Update Resource
*GroupsApi* | [**create_resource**](docs/GroupsApi.md#create_resource) | **Post** /group-bans | Create Resource
*GroupsApi* | [**create_resource_0**](docs/GroupsApi.md#create_resource_0) | **Post** /group-categories | Create Resource
*GroupsApi* | [**create_resource_1**](docs/GroupsApi.md#create_resource_1) | **Post** /group-invites | Create Resource
*GroupsApi* | [**create_resource_10**](docs/GroupsApi.md#create_resource_10) | **Post** /leader-chat-messages | Create Resource
*GroupsApi* | [**create_resource_2**](docs/GroupsApi.md#create_resource_2) | **Post** /group-member-notes | Create Resource
*GroupsApi* | [**create_resource_3**](docs/GroupsApi.md#create_resource_3) | **Post** /group-members | Create Resource
*GroupsApi* | [**create_resource_4**](docs/GroupsApi.md#create_resource_4) | **Post** /group-neighbors | Create Resource
*GroupsApi* | [**create_resource_5**](docs/GroupsApi.md#create_resource_5) | **Post** /group-permissions | Create Resource
*GroupsApi* | [**create_resource_6**](docs/GroupsApi.md#create_resource_6) | **Post** /group-reports | Create Resource
*GroupsApi* | [**create_resource_7**](docs/GroupsApi.md#create_resource_7) | **Post** /group-ticket-messages | Create Resource
*GroupsApi* | [**create_resource_8**](docs/GroupsApi.md#create_resource_8) | **Post** /group-tickets | Create Resource
*GroupsApi* | [**create_resource_9**](docs/GroupsApi.md#create_resource_9) | **Post** /groups | Create Resource
*GroupsApi* | [**delete_resource**](docs/GroupsApi.md#delete_resource) | **Delete** /group-bans/{id} | Delete Resource
*GroupsApi* | [**delete_resource_0**](docs/GroupsApi.md#delete_resource_0) | **Delete** /group-categories/{id} | Delete Resource
*GroupsApi* | [**delete_resource_1**](docs/GroupsApi.md#delete_resource_1) | **Delete** /group-invites/{id} | Delete Resource
*GroupsApi* | [**delete_resource_2**](docs/GroupsApi.md#delete_resource_2) | **Delete** /group-member-notes/{id} | Delete Resource
*GroupsApi* | [**delete_resource_3**](docs/GroupsApi.md#delete_resource_3) | **Delete** /group-members/{id} | Delete Resource
*GroupsApi* | [**delete_resource_4**](docs/GroupsApi.md#delete_resource_4) | **Delete** /group-neighbors/{id} | Delete Resource
*GroupsApi* | [**delete_resource_5**](docs/GroupsApi.md#delete_resource_5) | **Delete** /group-permissions/{id} | Delete Resource
*GroupsApi* | [**delete_resource_6**](docs/GroupsApi.md#delete_resource_6) | **Delete** /group-ticket-messages/{id} | Delete Resource
*GroupsApi* | [**delete_resource_7**](docs/GroupsApi.md#delete_resource_7) | **Delete** /groups/{id} | Delete Resource
*GroupsApi* | [**delete_resource_8**](docs/GroupsApi.md#delete_resource_8) | **Delete** /leader-chat-messages/{id} | Delete Resource
*GroupsApi* | [**fetch_collection**](docs/GroupsApi.md#fetch_collection) | **Get** /group-action-logs | Fetch Collection
*GroupsApi* | [**fetch_collection_0**](docs/GroupsApi.md#fetch_collection_0) | **Get** /group-bans | Fetch Collection
*GroupsApi* | [**fetch_collection_1**](docs/GroupsApi.md#fetch_collection_1) | **Get** /group-categories | Fetch Collection
*GroupsApi* | [**fetch_collection_10**](docs/GroupsApi.md#fetch_collection_10) | **Get** /groups | Fetch Collection
*GroupsApi* | [**fetch_collection_11**](docs/GroupsApi.md#fetch_collection_11) | **Get** /leader-chat-messages | Fetch Collection
*GroupsApi* | [**fetch_collection_2**](docs/GroupsApi.md#fetch_collection_2) | **Get** /group-invites | Fetch Collection
*GroupsApi* | [**fetch_collection_3**](docs/GroupsApi.md#fetch_collection_3) | **Get** /group-member-notes | Fetch Collection
*GroupsApi* | [**fetch_collection_4**](docs/GroupsApi.md#fetch_collection_4) | **Get** /group-members | Fetch Collection
*GroupsApi* | [**fetch_collection_5**](docs/GroupsApi.md#fetch_collection_5) | **Get** /group-neighbors | Fetch Collection
*GroupsApi* | [**fetch_collection_6**](docs/GroupsApi.md#fetch_collection_6) | **Get** /group-permissions | Fetch Collection
*GroupsApi* | [**fetch_collection_7**](docs/GroupsApi.md#fetch_collection_7) | **Get** /group-reports | Fetch Collection
*GroupsApi* | [**fetch_collection_8**](docs/GroupsApi.md#fetch_collection_8) | **Get** /group-ticket-messages | Fetch Collection
*GroupsApi* | [**fetch_collection_9**](docs/GroupsApi.md#fetch_collection_9) | **Get** /group-tickets | Fetch Collection
*GroupsApi* | [**fetch_resource**](docs/GroupsApi.md#fetch_resource) | **Get** /group-action-logs/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_0**](docs/GroupsApi.md#fetch_resource_0) | **Get** /group-bans/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_1**](docs/GroupsApi.md#fetch_resource_1) | **Get** /group-categories/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_10**](docs/GroupsApi.md#fetch_resource_10) | **Get** /groups/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_11**](docs/GroupsApi.md#fetch_resource_11) | **Get** /leader-chat-messages/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_2**](docs/GroupsApi.md#fetch_resource_2) | **Get** /group-invites/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_3**](docs/GroupsApi.md#fetch_resource_3) | **Get** /group-member-notes/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_4**](docs/GroupsApi.md#fetch_resource_4) | **Get** /group-members/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_5**](docs/GroupsApi.md#fetch_resource_5) | **Get** /group-neighbors/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_6**](docs/GroupsApi.md#fetch_resource_6) | **Get** /group-permissions/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_7**](docs/GroupsApi.md#fetch_resource_7) | **Get** /group-reports/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_8**](docs/GroupsApi.md#fetch_resource_8) | **Get** /group-ticket-messages/{id} | Fetch Resource
*GroupsApi* | [**fetch_resource_9**](docs/GroupsApi.md#fetch_resource_9) | **Get** /group-tickets/{id} | Fetch Resource
*GroupsApi* | [**update_resource**](docs/GroupsApi.md#update_resource) | **Patch** /group-categories/{id} | Update Resource
*GroupsApi* | [**update_resource_0**](docs/GroupsApi.md#update_resource_0) | **Patch** /group-invites/{id} | Update Resource
*GroupsApi* | [**update_resource_1**](docs/GroupsApi.md#update_resource_1) | **Patch** /group-member-notes/{id} | Update Resource
*GroupsApi* | [**update_resource_2**](docs/GroupsApi.md#update_resource_2) | **Patch** /group-members/{id} | Update Resource
*GroupsApi* | [**update_resource_3**](docs/GroupsApi.md#update_resource_3) | **Patch** /group-reports/{id} | Update Resource
*GroupsApi* | [**update_resource_4**](docs/GroupsApi.md#update_resource_4) | **Patch** /group-ticket-messages/{id} | Update Resource
*GroupsApi* | [**update_resource_5**](docs/GroupsApi.md#update_resource_5) | **Patch** /group-tickets/{id} | Update Resource
*GroupsApi* | [**update_resource_6**](docs/GroupsApi.md#update_resource_6) | **Patch** /groups/{id} | Update Resource
*GroupsApi* | [**update_resource_7**](docs/GroupsApi.md#update_resource_7) | **Patch** /leader-chat-messages/{id} | Update Resource
*MangaApi* | [**fetch_collection**](docs/MangaApi.md#fetch_collection) | **Get** /chapters | Fetch Collection
*MangaApi* | [**fetch_collection_0**](docs/MangaApi.md#fetch_collection_0) | **Get** /manga | Fetch Collection
*MangaApi* | [**fetch_collection_1**](docs/MangaApi.md#fetch_collection_1) | **Get** /trending/manga | Fetch Collection
*MangaApi* | [**fetch_resource**](docs/MangaApi.md#fetch_resource) | **Get** /chapters/{id} | Fetch Resource
*MangaApi* | [**fetch_resource_0**](docs/MangaApi.md#fetch_resource_0) | **Get** /manga/{id} | Fetch Resource
*MediaFollowsApi* | [**create_resource**](docs/MediaFollowsApi.md#create_resource) | **Post** /media-attribute-votes | Create Resource
*MediaFollowsApi* | [**create_resource_0**](docs/MediaFollowsApi.md#create_resource_0) | **Post** /media-follows | Create Resource
*MediaFollowsApi* | [**delete_resource**](docs/MediaFollowsApi.md#delete_resource) | **Delete** /media-attribute-votes/{id} | Delete Resource
*MediaFollowsApi* | [**delete_resource_0**](docs/MediaFollowsApi.md#delete_resource_0) | **Delete** /media-follows/{id} | Delete Resource
*MediaFollowsApi* | [**fetch_collection**](docs/MediaFollowsApi.md#fetch_collection) | **Get** /media-attribute-votes | Fetch Collection
*MediaFollowsApi* | [**fetch_collection_0**](docs/MediaFollowsApi.md#fetch_collection_0) | **Get** /media-attributes | Fetch Collection
*MediaFollowsApi* | [**fetch_collection_1**](docs/MediaFollowsApi.md#fetch_collection_1) | **Get** /media-follows | Fetch Collection
*MediaFollowsApi* | [**fetch_resource**](docs/MediaFollowsApi.md#fetch_resource) | **Get** /media-attribute-votes/{id} | Fetch Resource
*MediaFollowsApi* | [**fetch_resource_0**](docs/MediaFollowsApi.md#fetch_resource_0) | **Get** /media-attributes/{id} | Fetch Resource
*MediaFollowsApi* | [**fetch_resource_1**](docs/MediaFollowsApi.md#fetch_resource_1) | **Get** /media-follows/{id} | Fetch Resource
*MediaFollowsApi* | [**update_resource**](docs/MediaFollowsApi.md#update_resource) | **Patch** /media-attribute-votes/{id} | Update Resource
*MediaFollowsApi* | [**update_resource_0**](docs/MediaFollowsApi.md#update_resource_0) | **Patch** /media-follows/{id} | Update Resource
*MediaRelationsApi* | [**fetch_collection**](docs/MediaRelationsApi.md#fetch_collection) | **Get** /franchises | Fetch Collection
*MediaRelationsApi* | [**fetch_collection_0**](docs/MediaRelationsApi.md#fetch_collection_0) | **Get** /installments | Fetch Collection
*MediaRelationsApi* | [**fetch_collection_1**](docs/MediaRelationsApi.md#fetch_collection_1) | **Get** /mappings | Fetch Collection
*MediaRelationsApi* | [**fetch_collection_2**](docs/MediaRelationsApi.md#fetch_collection_2) | **Get** /media-relationships | Fetch Collection
*MediaRelationsApi* | [**fetch_resource**](docs/MediaRelationsApi.md#fetch_resource) | **Get** /franchises/{id} | Fetch Resource
*MediaRelationsApi* | [**fetch_resource_0**](docs/MediaRelationsApi.md#fetch_resource_0) | **Get** /installments/{id} | Fetch Resource
*MediaRelationsApi* | [**fetch_resource_1**](docs/MediaRelationsApi.md#fetch_resource_1) | **Get** /mappings/{id} | Fetch Resource
*MediaRelationsApi* | [**fetch_resource_2**](docs/MediaRelationsApi.md#fetch_resource_2) | **Get** /media-relationships/{id} | Fetch Resource
*PostsApi* | [**create_resource**](docs/PostsApi.md#create_resource) | **Post** /post-follows | Create Resource
*PostsApi* | [**create_resource_0**](docs/PostsApi.md#create_resource_0) | **Post** /post-likes | Create Resource
*PostsApi* | [**create_resource_1**](docs/PostsApi.md#create_resource_1) | **Post** /posts | Create Resource
*PostsApi* | [**delete_resource**](docs/PostsApi.md#delete_resource) | **Delete** /post-follows/{id} | Delete Resource
*PostsApi* | [**delete_resource_0**](docs/PostsApi.md#delete_resource_0) | **Delete** /post-likes/{id} | Delete Resource
*PostsApi* | [**delete_resource_1**](docs/PostsApi.md#delete_resource_1) | **Delete** /posts/{id} | Delete Resource
*PostsApi* | [**fetch_collection**](docs/PostsApi.md#fetch_collection) | **Get** /post-follows | Fetch Collection
*PostsApi* | [**fetch_collection_0**](docs/PostsApi.md#fetch_collection_0) | **Get** /post-likes | Fetch Collection
*PostsApi* | [**fetch_collection_1**](docs/PostsApi.md#fetch_collection_1) | **Get** /posts | Fetch Collection
*PostsApi* | [**fetch_resource**](docs/PostsApi.md#fetch_resource) | **Get** /post-follows/{id} | Fetch Resource
*PostsApi* | [**fetch_resource_0**](docs/PostsApi.md#fetch_resource_0) | **Get** /post-likes/{id} | Fetch Resource
*PostsApi* | [**fetch_resource_1**](docs/PostsApi.md#fetch_resource_1) | **Get** /posts/{id} | Fetch Resource
*PostsApi* | [**update_resource**](docs/PostsApi.md#update_resource) | **Patch** /posts/{id} | Update Resource
*ProducersStaffApi* | [**create_resource**](docs/ProducersStaffApi.md#create_resource) | **Post** /anime-productions | Create Resource
*ProducersStaffApi* | [**create_resource_0**](docs/ProducersStaffApi.md#create_resource_0) | **Post** /anime-staff | Create Resource
*ProducersStaffApi* | [**create_resource_1**](docs/ProducersStaffApi.md#create_resource_1) | **Post** /castings | Create Resource
*ProducersStaffApi* | [**create_resource_2**](docs/ProducersStaffApi.md#create_resource_2) | **Post** /manga-staff | Create Resource
*ProducersStaffApi* | [**create_resource_3**](docs/ProducersStaffApi.md#create_resource_3) | **Post** /people | Create Resource
*ProducersStaffApi* | [**create_resource_4**](docs/ProducersStaffApi.md#create_resource_4) | **Post** /producers | Create Resource
*ProducersStaffApi* | [**delete_resource**](docs/ProducersStaffApi.md#delete_resource) | **Delete** /anime-productions/{id} | Delete Resource
*ProducersStaffApi* | [**delete_resource_0**](docs/ProducersStaffApi.md#delete_resource_0) | **Delete** /anime-staff/{id} | Delete Resource
*ProducersStaffApi* | [**delete_resource_1**](docs/ProducersStaffApi.md#delete_resource_1) | **Delete** /castings/{id} | Delete Resource
*ProducersStaffApi* | [**delete_resource_2**](docs/ProducersStaffApi.md#delete_resource_2) | **Delete** /manga-staff/{id} | Delete Resource
*ProducersStaffApi* | [**delete_resource_3**](docs/ProducersStaffApi.md#delete_resource_3) | **Delete** /people/{id} | Delete Resource
*ProducersStaffApi* | [**delete_resource_4**](docs/ProducersStaffApi.md#delete_resource_4) | **Delete** /producers/{id} | Delete Resource
*ProducersStaffApi* | [**fetch_collection**](docs/ProducersStaffApi.md#fetch_collection) | **Get** /anime-productions | Fetch Collection
*ProducersStaffApi* | [**fetch_collection_0**](docs/ProducersStaffApi.md#fetch_collection_0) | **Get** /anime-staff | Fetch Collection
*ProducersStaffApi* | [**fetch_collection_1**](docs/ProducersStaffApi.md#fetch_collection_1) | **Get** /castings | Fetch Collection
*ProducersStaffApi* | [**fetch_collection_2**](docs/ProducersStaffApi.md#fetch_collection_2) | **Get** /manga-staff | Fetch Collection
*ProducersStaffApi* | [**fetch_collection_3**](docs/ProducersStaffApi.md#fetch_collection_3) | **Get** /people | Fetch Collection
*ProducersStaffApi* | [**fetch_collection_4**](docs/ProducersStaffApi.md#fetch_collection_4) | **Get** /producers | Fetch Collection
*ProducersStaffApi* | [**fetch_resource**](docs/ProducersStaffApi.md#fetch_resource) | **Get** /anime-productions/{id} | Fetch Resource
*ProducersStaffApi* | [**fetch_resource_0**](docs/ProducersStaffApi.md#fetch_resource_0) | **Get** /anime-staff/{id} | Fetch Resource
*ProducersStaffApi* | [**fetch_resource_1**](docs/ProducersStaffApi.md#fetch_resource_1) | **Get** /castings/{id} | Fetch Resource
*ProducersStaffApi* | [**fetch_resource_2**](docs/ProducersStaffApi.md#fetch_resource_2) | **Get** /manga-staff/{id} | Fetch Resource
*ProducersStaffApi* | [**fetch_resource_3**](docs/ProducersStaffApi.md#fetch_resource_3) | **Get** /people/{id} | Fetch Resource
*ProducersStaffApi* | [**fetch_resource_4**](docs/ProducersStaffApi.md#fetch_resource_4) | **Get** /producers/{id} | Fetch Resource
*ProducersStaffApi* | [**update_resource**](docs/ProducersStaffApi.md#update_resource) | **Patch** /anime-productions/{id} | Update Resource
*ProducersStaffApi* | [**update_resource_0**](docs/ProducersStaffApi.md#update_resource_0) | **Patch** /anime-staff/{id} | Update Resource
*ProducersStaffApi* | [**update_resource_1**](docs/ProducersStaffApi.md#update_resource_1) | **Patch** /castings/{id} | Update Resource
*ProducersStaffApi* | [**update_resource_2**](docs/ProducersStaffApi.md#update_resource_2) | **Patch** /manga-staff/{id} | Update Resource
*ProducersStaffApi* | [**update_resource_3**](docs/ProducersStaffApi.md#update_resource_3) | **Patch** /people/{id} | Update Resource
*ProducersStaffApi* | [**update_resource_4**](docs/ProducersStaffApi.md#update_resource_4) | **Patch** /producers/{id} | Update Resource
*ReactionsApi* | [**create_resource**](docs/ReactionsApi.md#create_resource) | **Post** /media-reaction-votes | Create Resource
*ReactionsApi* | [**create_resource_0**](docs/ReactionsApi.md#create_resource_0) | **Post** /media-reactions | Create Resource
*ReactionsApi* | [**create_resource_1**](docs/ReactionsApi.md#create_resource_1) | **Post** /review-likes | Create Resource
*ReactionsApi* | [**create_resource_2**](docs/ReactionsApi.md#create_resource_2) | **Post** /reviews | Create Resource
*ReactionsApi* | [**delete_resource**](docs/ReactionsApi.md#delete_resource) | **Delete** /media-reaction-votes/{id} | Delete Resource
*ReactionsApi* | [**delete_resource_0**](docs/ReactionsApi.md#delete_resource_0) | **Delete** /media-reactions/{id} | Delete Resource
*ReactionsApi* | [**delete_resource_1**](docs/ReactionsApi.md#delete_resource_1) | **Delete** /review-likes/{id} | Delete Resource
*ReactionsApi* | [**delete_resource_2**](docs/ReactionsApi.md#delete_resource_2) | **Delete** /reviews/{id} | Delete Resource
*ReactionsApi* | [**fetch_collection**](docs/ReactionsApi.md#fetch_collection) | **Get** /media-reaction-votes | Fetch Collection
*ReactionsApi* | [**fetch_collection_0**](docs/ReactionsApi.md#fetch_collection_0) | **Get** /media-reactions | Fetch Collection
*ReactionsApi* | [**fetch_collection_1**](docs/ReactionsApi.md#fetch_collection_1) | **Get** /review-likes | Fetch Collection
*ReactionsApi* | [**fetch_collection_2**](docs/ReactionsApi.md#fetch_collection_2) | **Get** /reviews | Fetch Collection
*ReactionsApi* | [**fetch_resource**](docs/ReactionsApi.md#fetch_resource) | **Get** /media-reaction-votes/{id} | Fetch Resource
*ReactionsApi* | [**fetch_resource_0**](docs/ReactionsApi.md#fetch_resource_0) | **Get** /media-reactions/{id} | Fetch Resource
*ReactionsApi* | [**fetch_resource_1**](docs/ReactionsApi.md#fetch_resource_1) | **Get** /review-likes/{id} | Fetch Resource
*ReactionsApi* | [**fetch_resource_2**](docs/ReactionsApi.md#fetch_resource_2) | **Get** /reviews/{id} | Fetch Resource
*ReactionsApi* | [**update_resource**](docs/ReactionsApi.md#update_resource) | **Patch** /media-reaction-votes/{id} | Update Resource
*ReactionsApi* | [**update_resource_0**](docs/ReactionsApi.md#update_resource_0) | **Patch** /media-reactions/{id} | Update Resource
*ReactionsApi* | [**update_resource_1**](docs/ReactionsApi.md#update_resource_1) | **Patch** /review-likes/{id} | Update Resource
*ReactionsApi* | [**update_resource_2**](docs/ReactionsApi.md#update_resource_2) | **Patch** /reviews/{id} | Update Resource
*ReportsApi* | [**create_resource**](docs/ReportsApi.md#create_resource) | **Post** /reports | Create Resource
*ReportsApi* | [**delete_resource**](docs/ReportsApi.md#delete_resource) | **Delete** /reports/{id} | Delete Resource
*ReportsApi* | [**fetch_collection**](docs/ReportsApi.md#fetch_collection) | **Get** /reports | Fetch Collection
*ReportsApi* | [**fetch_resource**](docs/ReportsApi.md#fetch_resource) | **Get** /reports/{id} | Fetch Resource
*ReportsApi* | [**update_resource**](docs/ReportsApi.md#update_resource) | **Patch** /reports/{id} | Update Resource
*SiteAnnouncementsApi* | [**fetch_collection**](docs/SiteAnnouncementsApi.md#fetch_collection) | **Get** /site-announcements | Fetch Collection
*SiteAnnouncementsApi* | [**fetch_resource**](docs/SiteAnnouncementsApi.md#fetch_resource) | **Get** /site-announcements/{id} | Fetch Resource
*StreamersApi* | [**fetch_collection**](docs/StreamersApi.md#fetch_collection) | **Get** /streamers | Fetch Collection
*StreamersApi* | [**fetch_collection_0**](docs/StreamersApi.md#fetch_collection_0) | **Get** /streaming-links | Fetch Collection
*StreamersApi* | [**fetch_resource**](docs/StreamersApi.md#fetch_resource) | **Get** /streamers/{id} | Fetch Resource
*StreamersApi* | [**fetch_resource_0**](docs/StreamersApi.md#fetch_resource_0) | **Get** /streaming-links/{id} | Fetch Resource
*UserLibrariesApi* | [**create_resource**](docs/UserLibrariesApi.md#create_resource) | **Post** /library-entries | Create Resource
*UserLibrariesApi* | [**create_resource_0**](docs/UserLibrariesApi.md#create_resource_0) | **Post** /library-entry-logs | Create Resource
*UserLibrariesApi* | [**create_resource_1**](docs/UserLibrariesApi.md#create_resource_1) | **Post** /list-imports | Create Resource
*UserLibrariesApi* | [**delete_resource**](docs/UserLibrariesApi.md#delete_resource) | **Delete** /library-entries/{id} | Delete Resource
*UserLibrariesApi* | [**delete_resource_0**](docs/UserLibrariesApi.md#delete_resource_0) | **Delete** /library-entry-logs/{id} | Delete Resource
*UserLibrariesApi* | [**delete_resource_1**](docs/UserLibrariesApi.md#delete_resource_1) | **Delete** /library-events/{id} | Delete Resource
*UserLibrariesApi* | [**fetch_collection**](docs/UserLibrariesApi.md#fetch_collection) | **Get** /library-entries | Fetch Collection
*UserLibrariesApi* | [**fetch_collection_0**](docs/UserLibrariesApi.md#fetch_collection_0) | **Get** /library-entry-logs | Fetch Collection
*UserLibrariesApi* | [**fetch_collection_1**](docs/UserLibrariesApi.md#fetch_collection_1) | **Get** /library-events | Fetch Collection
*UserLibrariesApi* | [**fetch_collection_2**](docs/UserLibrariesApi.md#fetch_collection_2) | **Get** /list-imports | Fetch Collection
*UserLibrariesApi* | [**fetch_resource**](docs/UserLibrariesApi.md#fetch_resource) | **Get** /library-entries/{id} | Fetch Resource
*UserLibrariesApi* | [**fetch_resource_0**](docs/UserLibrariesApi.md#fetch_resource_0) | **Get** /library-entry-logs/{id} | Fetch Resource
*UserLibrariesApi* | [**fetch_resource_1**](docs/UserLibrariesApi.md#fetch_resource_1) | **Get** /library-events/{id} | Fetch Resource
*UserLibrariesApi* | [**fetch_resource_2**](docs/UserLibrariesApi.md#fetch_resource_2) | **Get** /list-imports/{id} | Fetch Resource
*UserLibrariesApi* | [**update_resource**](docs/UserLibrariesApi.md#update_resource) | **Patch** /library-entries/{id} | Update Resource
*UserLibrariesApi* | [**update_resource_0**](docs/UserLibrariesApi.md#update_resource_0) | **Patch** /library-entry-logs/{id} | Update Resource
*UsersApi* | [**create_resource**](docs/UsersApi.md#create_resource) | **Post** /blocks | Create Resource
*UsersApi* | [**create_resource_0**](docs/UsersApi.md#create_resource_0) | **Post** /favorites | Create Resource
*UsersApi* | [**create_resource_1**](docs/UsersApi.md#create_resource_1) | **Post** /follows | Create Resource
*UsersApi* | [**create_resource_2**](docs/UsersApi.md#create_resource_2) | **Post** /linked-accounts | Create Resource
*UsersApi* | [**create_resource_3**](docs/UsersApi.md#create_resource_3) | **Post** /profile-links | Create Resource
*UsersApi* | [**create_resource_4**](docs/UsersApi.md#create_resource_4) | **Post** /roles | Create Resource
*UsersApi* | [**create_resource_5**](docs/UsersApi.md#create_resource_5) | **Post** /user-roles | Create Resource
*UsersApi* | [**create_resource_6**](docs/UsersApi.md#create_resource_6) | **Post** /users | Create Resource
*UsersApi* | [**delete_resource**](docs/UsersApi.md#delete_resource) | **Delete** /blocks/{id} | Delete Resource
*UsersApi* | [**delete_resource_0**](docs/UsersApi.md#delete_resource_0) | **Delete** /favorites/{id} | Delete Resource
*UsersApi* | [**delete_resource_1**](docs/UsersApi.md#delete_resource_1) | **Delete** /follows/{id} | Delete Resource
*UsersApi* | [**delete_resource_2**](docs/UsersApi.md#delete_resource_2) | **Delete** /linked-accounts/{id} | Delete Resource
*UsersApi* | [**delete_resource_3**](docs/UsersApi.md#delete_resource_3) | **Delete** /profile-links/{id} | Delete Resource
*UsersApi* | [**delete_resource_4**](docs/UsersApi.md#delete_resource_4) | **Delete** /roles/{id} | Delete Resource
*UsersApi* | [**delete_resource_5**](docs/UsersApi.md#delete_resource_5) | **Delete** /stats/{id} | Delete Resource
*UsersApi* | [**delete_resource_6**](docs/UsersApi.md#delete_resource_6) | **Delete** /user-roles/{id} | Delete Resource
*UsersApi* | [**delete_resource_7**](docs/UsersApi.md#delete_resource_7) | **Delete** /users/{id} | Delete Resource
*UsersApi* | [**fetch_collection**](docs/UsersApi.md#fetch_collection) | **Get** /blocks | Fetch Collection
*UsersApi* | [**fetch_collection_0**](docs/UsersApi.md#fetch_collection_0) | **Get** /favorites | Fetch Collection
*UsersApi* | [**fetch_collection_1**](docs/UsersApi.md#fetch_collection_1) | **Get** /follows | Fetch Collection
*UsersApi* | [**fetch_collection_2**](docs/UsersApi.md#fetch_collection_2) | **Get** /linked-accounts | Fetch Collection
*UsersApi* | [**fetch_collection_3**](docs/UsersApi.md#fetch_collection_3) | **Get** /profile-link-sites | Fetch Collection
*UsersApi* | [**fetch_collection_4**](docs/UsersApi.md#fetch_collection_4) | **Get** /profile-links | Fetch Collection
*UsersApi* | [**fetch_collection_5**](docs/UsersApi.md#fetch_collection_5) | **Get** /roles | Fetch Collection
*UsersApi* | [**fetch_collection_6**](docs/UsersApi.md#fetch_collection_6) | **Get** /stats | Fetch Collection
*UsersApi* | [**fetch_collection_7**](docs/UsersApi.md#fetch_collection_7) | **Get** /user-roles | Fetch Collection
*UsersApi* | [**fetch_collection_8**](docs/UsersApi.md#fetch_collection_8) | **Get** /users | Fetch Collection
*UsersApi* | [**fetch_resource**](docs/UsersApi.md#fetch_resource) | **Get** /blocks/{id} | Fetch Resource
*UsersApi* | [**fetch_resource_0**](docs/UsersApi.md#fetch_resource_0) | **Get** /favorites/{id} | Fetch Resource
*UsersApi* | [**fetch_resource_1**](docs/UsersApi.md#fetch_resource_1) | **Get** /follows/{id} | Fetch Resource
*UsersApi* | [**fetch_resource_2**](docs/UsersApi.md#fetch_resource_2) | **Get** /linked-accounts/{id} | Fetch Resource
*UsersApi* | [**fetch_resource_3**](docs/UsersApi.md#fetch_resource_3) | **Get** /profile-link-sites/{id} | Fetch Resource
*UsersApi* | [**fetch_resource_4**](docs/UsersApi.md#fetch_resource_4) | **Get** /profile-links/{id} | Fetch Resource
*UsersApi* | [**fetch_resource_5**](docs/UsersApi.md#fetch_resource_5) | **Get** /roles/{id} | Fetch Resource
*UsersApi* | [**fetch_resource_6**](docs/UsersApi.md#fetch_resource_6) | **Get** /stats/{id} | Fetch Resource
*UsersApi* | [**fetch_resource_7**](docs/UsersApi.md#fetch_resource_7) | **Get** /user-roles/{id} | Fetch Resource
*UsersApi* | [**fetch_resource_8**](docs/UsersApi.md#fetch_resource_8) | **Get** /users/{id} | Fetch Resource
*UsersApi* | [**update_resource**](docs/UsersApi.md#update_resource) | **Patch** /blocks/{id} | Update Resource
*UsersApi* | [**update_resource_0**](docs/UsersApi.md#update_resource_0) | **Patch** /favorites/{id} | Update Resource
*UsersApi* | [**update_resource_1**](docs/UsersApi.md#update_resource_1) | **Patch** /follows/{id} | Update Resource
*UsersApi* | [**update_resource_2**](docs/UsersApi.md#update_resource_2) | **Patch** /linked-accounts/{id} | Update Resource
*UsersApi* | [**update_resource_3**](docs/UsersApi.md#update_resource_3) | **Patch** /profile-links/{id} | Update Resource
*UsersApi* | [**update_resource_4**](docs/UsersApi.md#update_resource_4) | **Patch** /roles/{id} | Update Resource
*UsersApi* | [**update_resource_5**](docs/UsersApi.md#update_resource_5) | **Patch** /users/{id} | Update Resource


## Documentation For Models

 - [Anime](docs/Anime.md)
 - [AnimeAttributes](docs/AnimeAttributes.md)
 - [AnimeAttributesCoverImage](docs/AnimeAttributesCoverImage.md)
 - [AnimeAttributesCoverImageMeta](docs/AnimeAttributesCoverImageMeta.md)
 - [AnimeAttributesCoverImageMetaDimensions](docs/AnimeAttributesCoverImageMetaDimensions.md)
 - [AnimeAttributesCoverImageMetaDimensionsLarge](docs/AnimeAttributesCoverImageMetaDimensionsLarge.md)
 - [AnimeAttributesPosterImage](docs/AnimeAttributesPosterImage.md)
 - [AnimeAttributesPosterImageMeta](docs/AnimeAttributesPosterImageMeta.md)
 - [AnimeAttributesPosterImageMetaDimensions](docs/AnimeAttributesPosterImageMetaDimensions.md)
 - [AnimeAttributesTitles](docs/AnimeAttributesTitles.md)
 - [AnimeCharacters](docs/AnimeCharacters.md)
 - [AnimeCharactersAttributes](docs/AnimeCharactersAttributes.md)
 - [AnimeProductions](docs/AnimeProductions.md)
 - [AnimeProductionsAttributes](docs/AnimeProductionsAttributes.md)
 - [AnimeStaff](docs/AnimeStaff.md)
 - [AnimeStaffAttributes](docs/AnimeStaffAttributes.md)
 - [Blocks](docs/Blocks.md)
 - [BlocksAttributes](docs/BlocksAttributes.md)
 - [Castings](docs/Castings.md)
 - [CastingsAttributes](docs/CastingsAttributes.md)
 - [Categories](docs/Categories.md)
 - [CategoriesAttributes](docs/CategoriesAttributes.md)
 - [CategoryFavorites](docs/CategoryFavorites.md)
 - [CategoryFavoritesAttributes](docs/CategoryFavoritesAttributes.md)
 - [Chapters](docs/Chapters.md)
 - [ChaptersAttributes](docs/ChaptersAttributes.md)
 - [Characters](docs/Characters.md)
 - [CharactersAttributes](docs/CharactersAttributes.md)
 - [CharactersAttributesImage](docs/CharactersAttributesImage.md)
 - [CommentLikes](docs/CommentLikes.md)
 - [CommentLikesAttributes](docs/CommentLikesAttributes.md)
 - [Comments](docs/Comments.md)
 - [CommentsAttributes](docs/CommentsAttributes.md)
 - [Drama](docs/Drama.md)
 - [DramaAttributes](docs/DramaAttributes.md)
 - [DramaRelationships](docs/DramaRelationships.md)
 - [Episodes](docs/Episodes.md)
 - [EpisodesAttributes](docs/EpisodesAttributes.md)
 - [Error](docs/Error.md)
 - [Error400](docs/Error400.md)
 - [Error401](docs/Error401.md)
 - [Error403](docs/Error403.md)
 - [Error404](docs/Error404.md)
 - [Error406](docs/Error406.md)
 - [Error409](docs/Error409.md)
 - [Error500](docs/Error500.md)
 - [ErrorSource](docs/ErrorSource.md)
 - [Favorites](docs/Favorites.md)
 - [FavoritesAttributes](docs/FavoritesAttributes.md)
 - [Follows](docs/Follows.md)
 - [FollowsAttributes](docs/FollowsAttributes.md)
 - [Franchises](docs/Franchises.md)
 - [FranchisesAttributes](docs/FranchisesAttributes.md)
 - [FranchisesAttributesTitles](docs/FranchisesAttributesTitles.md)
 - [Genres](docs/Genres.md)
 - [GenresAttributes](docs/GenresAttributes.md)
 - [GroupActionLogs](docs/GroupActionLogs.md)
 - [GroupActionLogsAttributes](docs/GroupActionLogsAttributes.md)
 - [GroupBans](docs/GroupBans.md)
 - [GroupBansAttributes](docs/GroupBansAttributes.md)
 - [GroupCategories](docs/GroupCategories.md)
 - [GroupCategoriesAttributes](docs/GroupCategoriesAttributes.md)
 - [GroupInvites](docs/GroupInvites.md)
 - [GroupInvitesAttributes](docs/GroupInvitesAttributes.md)
 - [GroupMemberNotes](docs/GroupMemberNotes.md)
 - [GroupMemberNotesAttributes](docs/GroupMemberNotesAttributes.md)
 - [GroupMembers](docs/GroupMembers.md)
 - [GroupMembersAttributes](docs/GroupMembersAttributes.md)
 - [GroupNeighbors](docs/GroupNeighbors.md)
 - [GroupNeighborsAttributes](docs/GroupNeighborsAttributes.md)
 - [GroupPermissions](docs/GroupPermissions.md)
 - [GroupPermissionsAttributes](docs/GroupPermissionsAttributes.md)
 - [GroupReports](docs/GroupReports.md)
 - [GroupReportsAttributes](docs/GroupReportsAttributes.md)
 - [GroupTicketMessages](docs/GroupTicketMessages.md)
 - [GroupTicketMessagesAttributes](docs/GroupTicketMessagesAttributes.md)
 - [GroupTickets](docs/GroupTickets.md)
 - [GroupTicketsAttributes](docs/GroupTicketsAttributes.md)
 - [Groups](docs/Groups.md)
 - [GroupsAttributes](docs/GroupsAttributes.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponse2001](docs/InlineResponse2001.md)
 - [InlineResponse20010](docs/InlineResponse20010.md)
 - [InlineResponse20010Data](docs/InlineResponse20010Data.md)
 - [InlineResponse20010DataAttributes](docs/InlineResponse20010DataAttributes.md)
 - [InlineResponse20010DataAttributesImage](docs/InlineResponse20010DataAttributesImage.md)
 - [InlineResponse20010DataRelationships](docs/InlineResponse20010DataRelationships.md)
 - [InlineResponse20011](docs/InlineResponse20011.md)
 - [InlineResponse20011Data](docs/InlineResponse20011Data.md)
 - [InlineResponse20011DataRelationships](docs/InlineResponse20011DataRelationships.md)
 - [InlineResponse20012](docs/InlineResponse20012.md)
 - [InlineResponse20012Data](docs/InlineResponse20012Data.md)
 - [InlineResponse20012DataAttributes](docs/InlineResponse20012DataAttributes.md)
 - [InlineResponse20012DataRelationships](docs/InlineResponse20012DataRelationships.md)
 - [InlineResponse20013](docs/InlineResponse20013.md)
 - [InlineResponse20013Data](docs/InlineResponse20013Data.md)
 - [InlineResponse20013DataAttributes](docs/InlineResponse20013DataAttributes.md)
 - [InlineResponse20013DataRelationships](docs/InlineResponse20013DataRelationships.md)
 - [InlineResponse20014](docs/InlineResponse20014.md)
 - [InlineResponse20014Data](docs/InlineResponse20014Data.md)
 - [InlineResponse20014DataAttributes](docs/InlineResponse20014DataAttributes.md)
 - [InlineResponse20014DataRelationships](docs/InlineResponse20014DataRelationships.md)
 - [InlineResponse20015](docs/InlineResponse20015.md)
 - [InlineResponse20015Data](docs/InlineResponse20015Data.md)
 - [InlineResponse20015DataRelationships](docs/InlineResponse20015DataRelationships.md)
 - [InlineResponse20016](docs/InlineResponse20016.md)
 - [InlineResponse20016Data](docs/InlineResponse20016Data.md)
 - [InlineResponse20016DataAttributes](docs/InlineResponse20016DataAttributes.md)
 - [InlineResponse20016DataAttributesTitles](docs/InlineResponse20016DataAttributesTitles.md)
 - [InlineResponse20016DataRelationships](docs/InlineResponse20016DataRelationships.md)
 - [InlineResponse20017](docs/InlineResponse20017.md)
 - [InlineResponse20017Data](docs/InlineResponse20017Data.md)
 - [InlineResponse20017DataAttributes](docs/InlineResponse20017DataAttributes.md)
 - [InlineResponse20017DataRelationships](docs/InlineResponse20017DataRelationships.md)
 - [InlineResponse20018](docs/InlineResponse20018.md)
 - [InlineResponse20018Data](docs/InlineResponse20018Data.md)
 - [InlineResponse20018DataAttributes](docs/InlineResponse20018DataAttributes.md)
 - [InlineResponse20018DataRelationships](docs/InlineResponse20018DataRelationships.md)
 - [InlineResponse20019](docs/InlineResponse20019.md)
 - [InlineResponse20019Data](docs/InlineResponse20019Data.md)
 - [InlineResponse20019DataAttributes](docs/InlineResponse20019DataAttributes.md)
 - [InlineResponse2001Data](docs/InlineResponse2001Data.md)
 - [InlineResponse2001DataAttributes](docs/InlineResponse2001DataAttributes.md)
 - [InlineResponse2001DataLinks](docs/InlineResponse2001DataLinks.md)
 - [InlineResponse2001DataRelationships](docs/InlineResponse2001DataRelationships.md)
 - [InlineResponse2001DataRelationshipsAnime](docs/InlineResponse2001DataRelationshipsAnime.md)
 - [InlineResponse2001DataRelationshipsAnimeLinks](docs/InlineResponse2001DataRelationshipsAnimeLinks.md)
 - [InlineResponse2002](docs/InlineResponse2002.md)
 - [InlineResponse20020](docs/InlineResponse20020.md)
 - [InlineResponse20020Data](docs/InlineResponse20020Data.md)
 - [InlineResponse20020DataAttributes](docs/InlineResponse20020DataAttributes.md)
 - [InlineResponse20020DataRelationships](docs/InlineResponse20020DataRelationships.md)
 - [InlineResponse20021](docs/InlineResponse20021.md)
 - [InlineResponse20021Data](docs/InlineResponse20021Data.md)
 - [InlineResponse20021DataAttributes](docs/InlineResponse20021DataAttributes.md)
 - [InlineResponse20021DataRelationships](docs/InlineResponse20021DataRelationships.md)
 - [InlineResponse20022](docs/InlineResponse20022.md)
 - [InlineResponse20022Data](docs/InlineResponse20022Data.md)
 - [InlineResponse20022DataAttributes](docs/InlineResponse20022DataAttributes.md)
 - [InlineResponse20022DataRelationships](docs/InlineResponse20022DataRelationships.md)
 - [InlineResponse20023](docs/InlineResponse20023.md)
 - [InlineResponse20023Data](docs/InlineResponse20023Data.md)
 - [InlineResponse20023DataRelationships](docs/InlineResponse20023DataRelationships.md)
 - [InlineResponse20024](docs/InlineResponse20024.md)
 - [InlineResponse20024Data](docs/InlineResponse20024Data.md)
 - [InlineResponse20024DataAttributes](docs/InlineResponse20024DataAttributes.md)
 - [InlineResponse20024DataRelationships](docs/InlineResponse20024DataRelationships.md)
 - [InlineResponse20025](docs/InlineResponse20025.md)
 - [InlineResponse20025Data](docs/InlineResponse20025Data.md)
 - [InlineResponse20025DataAttributes](docs/InlineResponse20025DataAttributes.md)
 - [InlineResponse20025DataRelationships](docs/InlineResponse20025DataRelationships.md)
 - [InlineResponse20026](docs/InlineResponse20026.md)
 - [InlineResponse20026Data](docs/InlineResponse20026Data.md)
 - [InlineResponse20026DataAttributes](docs/InlineResponse20026DataAttributes.md)
 - [InlineResponse20026DataRelationships](docs/InlineResponse20026DataRelationships.md)
 - [InlineResponse20027](docs/InlineResponse20027.md)
 - [InlineResponse20027Data](docs/InlineResponse20027Data.md)
 - [InlineResponse20027DataAttributes](docs/InlineResponse20027DataAttributes.md)
 - [InlineResponse20027DataRelationships](docs/InlineResponse20027DataRelationships.md)
 - [InlineResponse20028](docs/InlineResponse20028.md)
 - [InlineResponse20028Data](docs/InlineResponse20028Data.md)
 - [InlineResponse20028DataAttributes](docs/InlineResponse20028DataAttributes.md)
 - [InlineResponse20028DataRelationships](docs/InlineResponse20028DataRelationships.md)
 - [InlineResponse20029](docs/InlineResponse20029.md)
 - [InlineResponse20029Data](docs/InlineResponse20029Data.md)
 - [InlineResponse20029DataAttributes](docs/InlineResponse20029DataAttributes.md)
 - [InlineResponse20029DataRelationships](docs/InlineResponse20029DataRelationships.md)
 - [InlineResponse20029DataRelationshipsFranchise](docs/InlineResponse20029DataRelationshipsFranchise.md)
 - [InlineResponse2002Data](docs/InlineResponse2002Data.md)
 - [InlineResponse2002DataAttributes](docs/InlineResponse2002DataAttributes.md)
 - [InlineResponse2002DataRelationships](docs/InlineResponse2002DataRelationships.md)
 - [InlineResponse2003](docs/InlineResponse2003.md)
 - [InlineResponse20030](docs/InlineResponse20030.md)
 - [InlineResponse20030Data](docs/InlineResponse20030Data.md)
 - [InlineResponse20030DataAttributes](docs/InlineResponse20030DataAttributes.md)
 - [InlineResponse20030DataRelationships](docs/InlineResponse20030DataRelationships.md)
 - [InlineResponse20031](docs/InlineResponse20031.md)
 - [InlineResponse20031Meta](docs/InlineResponse20031Meta.md)
 - [InlineResponse20031MetaStatusCounts](docs/InlineResponse20031MetaStatusCounts.md)
 - [InlineResponse20032](docs/InlineResponse20032.md)
 - [InlineResponse20032Data](docs/InlineResponse20032Data.md)
 - [InlineResponse20032DataAttributes](docs/InlineResponse20032DataAttributes.md)
 - [InlineResponse20032DataRelationships](docs/InlineResponse20032DataRelationships.md)
 - [InlineResponse20033](docs/InlineResponse20033.md)
 - [InlineResponse20033Data](docs/InlineResponse20033Data.md)
 - [InlineResponse20033DataAttributes](docs/InlineResponse20033DataAttributes.md)
 - [InlineResponse20033DataRelationships](docs/InlineResponse20033DataRelationships.md)
 - [InlineResponse20034](docs/InlineResponse20034.md)
 - [InlineResponse20034Data](docs/InlineResponse20034Data.md)
 - [InlineResponse20034DataAttributes](docs/InlineResponse20034DataAttributes.md)
 - [InlineResponse20034DataRelationships](docs/InlineResponse20034DataRelationships.md)
 - [InlineResponse20035](docs/InlineResponse20035.md)
 - [InlineResponse20035Data](docs/InlineResponse20035Data.md)
 - [InlineResponse20035DataAttributes](docs/InlineResponse20035DataAttributes.md)
 - [InlineResponse20035DataAttributesToken](docs/InlineResponse20035DataAttributesToken.md)
 - [InlineResponse20035DataAttributesTokenApplication](docs/InlineResponse20035DataAttributesTokenApplication.md)
 - [InlineResponse20035DataRelationships](docs/InlineResponse20035DataRelationships.md)
 - [InlineResponse20036](docs/InlineResponse20036.md)
 - [InlineResponse20036Data](docs/InlineResponse20036Data.md)
 - [InlineResponse20036DataAttributes](docs/InlineResponse20036DataAttributes.md)
 - [InlineResponse20036DataRelationships](docs/InlineResponse20036DataRelationships.md)
 - [InlineResponse20037](docs/InlineResponse20037.md)
 - [InlineResponse20037Data](docs/InlineResponse20037Data.md)
 - [InlineResponse20037DataAttributes](docs/InlineResponse20037DataAttributes.md)
 - [InlineResponse20037DataRelationships](docs/InlineResponse20037DataRelationships.md)
 - [InlineResponse20038](docs/InlineResponse20038.md)
 - [InlineResponse20038Data](docs/InlineResponse20038Data.md)
 - [InlineResponse20038DataRelationships](docs/InlineResponse20038DataRelationships.md)
 - [InlineResponse20039](docs/InlineResponse20039.md)
 - [InlineResponse20039Data](docs/InlineResponse20039Data.md)
 - [InlineResponse20039DataAttributes](docs/InlineResponse20039DataAttributes.md)
 - [InlineResponse20039DataRelationships](docs/InlineResponse20039DataRelationships.md)
 - [InlineResponse2003Data](docs/InlineResponse2003Data.md)
 - [InlineResponse2003DataAttributes](docs/InlineResponse2003DataAttributes.md)
 - [InlineResponse2003DataRelationships](docs/InlineResponse2003DataRelationships.md)
 - [InlineResponse2004](docs/InlineResponse2004.md)
 - [InlineResponse20040](docs/InlineResponse20040.md)
 - [InlineResponse20040Data](docs/InlineResponse20040Data.md)
 - [InlineResponse20040DataAttributes](docs/InlineResponse20040DataAttributes.md)
 - [InlineResponse20040DataRelationships](docs/InlineResponse20040DataRelationships.md)
 - [InlineResponse20041](docs/InlineResponse20041.md)
 - [InlineResponse20041Data](docs/InlineResponse20041Data.md)
 - [InlineResponse20041DataAttributes](docs/InlineResponse20041DataAttributes.md)
 - [InlineResponse20041DataRelationships](docs/InlineResponse20041DataRelationships.md)
 - [InlineResponse20042](docs/InlineResponse20042.md)
 - [InlineResponse20042Data](docs/InlineResponse20042Data.md)
 - [InlineResponse20042DataAttributes](docs/InlineResponse20042DataAttributes.md)
 - [InlineResponse20042DataRelationships](docs/InlineResponse20042DataRelationships.md)
 - [InlineResponse20043](docs/InlineResponse20043.md)
 - [InlineResponse20043Data](docs/InlineResponse20043Data.md)
 - [InlineResponse20043DataLinks](docs/InlineResponse20043DataLinks.md)
 - [InlineResponse20043DataRelationships](docs/InlineResponse20043DataRelationships.md)
 - [InlineResponse20044](docs/InlineResponse20044.md)
 - [InlineResponse20044Data](docs/InlineResponse20044Data.md)
 - [InlineResponse20044DataRelationships](docs/InlineResponse20044DataRelationships.md)
 - [InlineResponse20045](docs/InlineResponse20045.md)
 - [InlineResponse20045Data](docs/InlineResponse20045Data.md)
 - [InlineResponse20045DataAttributes](docs/InlineResponse20045DataAttributes.md)
 - [InlineResponse20045DataRelationships](docs/InlineResponse20045DataRelationships.md)
 - [InlineResponse20046](docs/InlineResponse20046.md)
 - [InlineResponse20046Data](docs/InlineResponse20046Data.md)
 - [InlineResponse20046DataAttributes](docs/InlineResponse20046DataAttributes.md)
 - [InlineResponse20047](docs/InlineResponse20047.md)
 - [InlineResponse20047Data](docs/InlineResponse20047Data.md)
 - [InlineResponse20047DataAttributes](docs/InlineResponse20047DataAttributes.md)
 - [InlineResponse20047DataRelationships](docs/InlineResponse20047DataRelationships.md)
 - [InlineResponse20048](docs/InlineResponse20048.md)
 - [InlineResponse20048Data](docs/InlineResponse20048Data.md)
 - [InlineResponse20048DataRelationships](docs/InlineResponse20048DataRelationships.md)
 - [InlineResponse20049](docs/InlineResponse20049.md)
 - [InlineResponse20049Data](docs/InlineResponse20049Data.md)
 - [InlineResponse20049DataAttributes](docs/InlineResponse20049DataAttributes.md)
 - [InlineResponse20049DataRelationships](docs/InlineResponse20049DataRelationships.md)
 - [InlineResponse2004Data](docs/InlineResponse2004Data.md)
 - [InlineResponse2004DataAttributes](docs/InlineResponse2004DataAttributes.md)
 - [InlineResponse2004DataAttributesCoverImage](docs/InlineResponse2004DataAttributesCoverImage.md)
 - [InlineResponse2004DataAttributesCoverImageMeta](docs/InlineResponse2004DataAttributesCoverImageMeta.md)
 - [InlineResponse2004DataAttributesCoverImageMetaDimensions](docs/InlineResponse2004DataAttributesCoverImageMetaDimensions.md)
 - [InlineResponse2004DataAttributesCoverImageMetaDimensionsLarge](docs/InlineResponse2004DataAttributesCoverImageMetaDimensionsLarge.md)
 - [InlineResponse2004DataAttributesPosterImage](docs/InlineResponse2004DataAttributesPosterImage.md)
 - [InlineResponse2004DataAttributesPosterImageMeta](docs/InlineResponse2004DataAttributesPosterImageMeta.md)
 - [InlineResponse2004DataAttributesPosterImageMetaDimensions](docs/InlineResponse2004DataAttributesPosterImageMetaDimensions.md)
 - [InlineResponse2004DataAttributesRatingFrequencies](docs/InlineResponse2004DataAttributesRatingFrequencies.md)
 - [InlineResponse2004DataAttributesTitles](docs/InlineResponse2004DataAttributesTitles.md)
 - [InlineResponse2004DataRelationships](docs/InlineResponse2004DataRelationships.md)
 - [InlineResponse2005](docs/InlineResponse2005.md)
 - [InlineResponse20050](docs/InlineResponse20050.md)
 - [InlineResponse20050Data](docs/InlineResponse20050Data.md)
 - [InlineResponse20050DataAttributes](docs/InlineResponse20050DataAttributes.md)
 - [InlineResponse20050DataRelationships](docs/InlineResponse20050DataRelationships.md)
 - [InlineResponse20051](docs/InlineResponse20051.md)
 - [InlineResponse20051Data](docs/InlineResponse20051Data.md)
 - [InlineResponse20051DataAttributes](docs/InlineResponse20051DataAttributes.md)
 - [InlineResponse20052](docs/InlineResponse20052.md)
 - [InlineResponse20052Data](docs/InlineResponse20052Data.md)
 - [InlineResponse20052DataAttributes](docs/InlineResponse20052DataAttributes.md)
 - [InlineResponse20052DataRelationships](docs/InlineResponse20052DataRelationships.md)
 - [InlineResponse20053](docs/InlineResponse20053.md)
 - [InlineResponse20053Data](docs/InlineResponse20053Data.md)
 - [InlineResponse20053DataAttributes](docs/InlineResponse20053DataAttributes.md)
 - [InlineResponse20053DataRelationships](docs/InlineResponse20053DataRelationships.md)
 - [InlineResponse20054](docs/InlineResponse20054.md)
 - [InlineResponse20054Data](docs/InlineResponse20054Data.md)
 - [InlineResponse20054DataRelationships](docs/InlineResponse20054DataRelationships.md)
 - [InlineResponse20055](docs/InlineResponse20055.md)
 - [InlineResponse20055Data](docs/InlineResponse20055Data.md)
 - [InlineResponse20055DataAttributes](docs/InlineResponse20055DataAttributes.md)
 - [InlineResponse20055DataRelationships](docs/InlineResponse20055DataRelationships.md)
 - [InlineResponse20056](docs/InlineResponse20056.md)
 - [InlineResponse20056Data](docs/InlineResponse20056Data.md)
 - [InlineResponse20056DataAttributes](docs/InlineResponse20056DataAttributes.md)
 - [InlineResponse20056DataRelationships](docs/InlineResponse20056DataRelationships.md)
 - [InlineResponse20057](docs/InlineResponse20057.md)
 - [InlineResponse20057Data](docs/InlineResponse20057Data.md)
 - [InlineResponse20057DataAttributes](docs/InlineResponse20057DataAttributes.md)
 - [InlineResponse20058](docs/InlineResponse20058.md)
 - [InlineResponse20058Data](docs/InlineResponse20058Data.md)
 - [InlineResponse20058DataAttributes](docs/InlineResponse20058DataAttributes.md)
 - [InlineResponse20058DataAttributesStatsData](docs/InlineResponse20058DataAttributesStatsData.md)
 - [InlineResponse20058DataAttributesStatsDataAllCategories](docs/InlineResponse20058DataAttributesStatsDataAllCategories.md)
 - [InlineResponse20058DataAttributesStatsDataAllTime](docs/InlineResponse20058DataAttributesStatsDataAllTime.md)
 - [InlineResponse20058DataAttributesStatsDataAllYears](docs/InlineResponse20058DataAttributesStatsDataAllYears.md)
 - [InlineResponse20059](docs/InlineResponse20059.md)
 - [InlineResponse20059Data](docs/InlineResponse20059Data.md)
 - [InlineResponse20059DataAttributes](docs/InlineResponse20059DataAttributes.md)
 - [InlineResponse20059DataRelationships](docs/InlineResponse20059DataRelationships.md)
 - [InlineResponse2005Data](docs/InlineResponse2005Data.md)
 - [InlineResponse2005DataAttributes](docs/InlineResponse2005DataAttributes.md)
 - [InlineResponse2005DataRelationships](docs/InlineResponse2005DataRelationships.md)
 - [InlineResponse2006](docs/InlineResponse2006.md)
 - [InlineResponse20060](docs/InlineResponse20060.md)
 - [InlineResponse20060Data](docs/InlineResponse20060Data.md)
 - [InlineResponse20060DataAttributes](docs/InlineResponse20060DataAttributes.md)
 - [InlineResponse20060DataRelationships](docs/InlineResponse20060DataRelationships.md)
 - [InlineResponse20061](docs/InlineResponse20061.md)
 - [InlineResponse20062](docs/InlineResponse20062.md)
 - [InlineResponse20062Data](docs/InlineResponse20062Data.md)
 - [InlineResponse20062DataRelationships](docs/InlineResponse20062DataRelationships.md)
 - [InlineResponse20063](docs/InlineResponse20063.md)
 - [InlineResponse20063Data](docs/InlineResponse20063Data.md)
 - [InlineResponse20063DataAttributes](docs/InlineResponse20063DataAttributes.md)
 - [InlineResponse20063DataAttributesAvatar](docs/InlineResponse20063DataAttributesAvatar.md)
 - [InlineResponse20063DataAttributesAvatarMeta](docs/InlineResponse20063DataAttributesAvatarMeta.md)
 - [InlineResponse20063DataAttributesAvatarMetaDimensions](docs/InlineResponse20063DataAttributesAvatarMetaDimensions.md)
 - [InlineResponse20063DataAttributesAvatarMetaDimensionsLarge](docs/InlineResponse20063DataAttributesAvatarMetaDimensionsLarge.md)
 - [InlineResponse20063DataAttributesAvatarMetaDimensionsSmall](docs/InlineResponse20063DataAttributesAvatarMetaDimensionsSmall.md)
 - [InlineResponse20063DataAttributesCoverImage](docs/InlineResponse20063DataAttributesCoverImage.md)
 - [InlineResponse20063DataAttributesCoverImageMeta](docs/InlineResponse20063DataAttributesCoverImageMeta.md)
 - [InlineResponse20063DataAttributesCoverImageMetaDimensions](docs/InlineResponse20063DataAttributesCoverImageMetaDimensions.md)
 - [InlineResponse20063DataRelationships](docs/InlineResponse20063DataRelationships.md)
 - [InlineResponse2006Data](docs/InlineResponse2006Data.md)
 - [InlineResponse2006DataAttributes](docs/InlineResponse2006DataAttributes.md)
 - [InlineResponse2006DataRelationships](docs/InlineResponse2006DataRelationships.md)
 - [InlineResponse2007](docs/InlineResponse2007.md)
 - [InlineResponse2007Data](docs/InlineResponse2007Data.md)
 - [InlineResponse2007DataAttributes](docs/InlineResponse2007DataAttributes.md)
 - [InlineResponse2007DataRelationships](docs/InlineResponse2007DataRelationships.md)
 - [InlineResponse2008](docs/InlineResponse2008.md)
 - [InlineResponse2008Data](docs/InlineResponse2008Data.md)
 - [InlineResponse2008DataRelationships](docs/InlineResponse2008DataRelationships.md)
 - [InlineResponse2009](docs/InlineResponse2009.md)
 - [InlineResponse2009Data](docs/InlineResponse2009Data.md)
 - [InlineResponse2009DataAttributes](docs/InlineResponse2009DataAttributes.md)
 - [InlineResponse2009DataAttributesThumbnail](docs/InlineResponse2009DataAttributesThumbnail.md)
 - [InlineResponse2009DataAttributesThumbnailMeta](docs/InlineResponse2009DataAttributesThumbnailMeta.md)
 - [InlineResponse2009DataAttributesTitles](docs/InlineResponse2009DataAttributesTitles.md)
 - [InlineResponse2009DataRelationships](docs/InlineResponse2009DataRelationships.md)
 - [InlineResponse200Links](docs/InlineResponse200Links.md)
 - [InlineResponse200Meta](docs/InlineResponse200Meta.md)
 - [InlineResponse400](docs/InlineResponse400.md)
 - [Installments](docs/Installments.md)
 - [InstallmentsAttributes](docs/InstallmentsAttributes.md)
 - [LeaderChatMessages](docs/LeaderChatMessages.md)
 - [LeaderChatMessagesAttributes](docs/LeaderChatMessagesAttributes.md)
 - [LibraryEntries](docs/LibraryEntries.md)
 - [LibraryEntriesAttributes](docs/LibraryEntriesAttributes.md)
 - [LibraryEntryLogs](docs/LibraryEntryLogs.md)
 - [LibraryEntryLogsAttributes](docs/LibraryEntryLogsAttributes.md)
 - [LibraryEvents](docs/LibraryEvents.md)
 - [LibraryEventsAttributes](docs/LibraryEventsAttributes.md)
 - [LinkedAccounts](docs/LinkedAccounts.md)
 - [LinkedAccountsAttributes](docs/LinkedAccountsAttributes.md)
 - [ListImports](docs/ListImports.md)
 - [ListImportsAttributes](docs/ListImportsAttributes.md)
 - [Manga](docs/Manga.md)
 - [MangaAttributes](docs/MangaAttributes.md)
 - [MangaCharacters](docs/MangaCharacters.md)
 - [MangaCharactersAttributes](docs/MangaCharactersAttributes.md)
 - [MangaStaff](docs/MangaStaff.md)
 - [MangaStaffAttributes](docs/MangaStaffAttributes.md)
 - [Mappings](docs/Mappings.md)
 - [MappingsAttributes](docs/MappingsAttributes.md)
 - [MediaAttributeVotes](docs/MediaAttributeVotes.md)
 - [MediaAttributeVotesAttributes](docs/MediaAttributeVotesAttributes.md)
 - [MediaAttributes](docs/MediaAttributes.md)
 - [MediaAttributesAttributes](docs/MediaAttributesAttributes.md)
 - [MediaFollows](docs/MediaFollows.md)
 - [MediaReactionVotes](docs/MediaReactionVotes.md)
 - [MediaReactionVotesAttributes](docs/MediaReactionVotesAttributes.md)
 - [MediaReactions](docs/MediaReactions.md)
 - [MediaReactionsAttributes](docs/MediaReactionsAttributes.md)
 - [MediaRelationships](docs/MediaRelationships.md)
 - [MediaRelationshipsAttributes](docs/MediaRelationshipsAttributes.md)
 - [People](docs/People.md)
 - [PeopleAttributes](docs/PeopleAttributes.md)
 - [PostFollows](docs/PostFollows.md)
 - [PostFollowsAttributes](docs/PostFollowsAttributes.md)
 - [PostLikes](docs/PostLikes.md)
 - [PostLikesAttributes](docs/PostLikesAttributes.md)
 - [Posts](docs/Posts.md)
 - [PostsAttributes](docs/PostsAttributes.md)
 - [Producers](docs/Producers.md)
 - [ProducersAttributes](docs/ProducersAttributes.md)
 - [ProfileLinkSites](docs/ProfileLinkSites.md)
 - [ProfileLinkSitesAttributes](docs/ProfileLinkSitesAttributes.md)
 - [ProfileLinks](docs/ProfileLinks.md)
 - [ProfileLinksAttributes](docs/ProfileLinksAttributes.md)
 - [Reports](docs/Reports.md)
 - [ReportsAttributes](docs/ReportsAttributes.md)
 - [ReviewLikes](docs/ReviewLikes.md)
 - [ReviewLikesAttributes](docs/ReviewLikesAttributes.md)
 - [Reviews](docs/Reviews.md)
 - [ReviewsAttributes](docs/ReviewsAttributes.md)
 - [Roles](docs/Roles.md)
 - [RolesAttributes](docs/RolesAttributes.md)
 - [SiteAnnouncements](docs/SiteAnnouncements.md)
 - [SiteAnnouncementsAttributes](docs/SiteAnnouncementsAttributes.md)
 - [Stats](docs/Stats.md)
 - [StatsAttributes](docs/StatsAttributes.md)
 - [Streamers](docs/Streamers.md)
 - [StreamersAttributes](docs/StreamersAttributes.md)
 - [StreamingLinks](docs/StreamingLinks.md)
 - [StreamingLinksAttributes](docs/StreamingLinksAttributes.md)
 - [UserRoles](docs/UserRoles.md)
 - [UserRolesAttributes](docs/UserRolesAttributes.md)
 - [Users](docs/Users.md)
 - [UsersAttributes](docs/UsersAttributes.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



