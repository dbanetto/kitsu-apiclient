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

- API version: 1.0
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
*AnimeApi* | [**episodes_fetch_collection**](docs/AnimeApi.md#episodes_fetch_collection) | **get** /episodes | Episodes_Fetch Collection
*AnimeApi* | [**episodes_fetch_resource**](docs/AnimeApi.md#episodes_fetch_resource) | **get** /episodes/{id} | Episodes_Fetch Resource
*AnimeApi* | [**fetch_collection**](docs/AnimeApi.md#fetch_collection) | **get** /anime | Fetch Collection
*AnimeApi* | [**fetch_resource**](docs/AnimeApi.md#fetch_resource) | **get** /anime/{id} | Fetch Resource
*AnimeApi* | [**trending_anime_fetch_collection**](docs/AnimeApi.md#trending_anime_fetch_collection) | **get** /trending/anime | Trending Anime_Fetch Collection
*CategoriesApi* | [**category_favorites_fetch_collection**](docs/CategoriesApi.md#category_favorites_fetch_collection) | **get** /category-favorites | Category Favorites_Fetch Collection
*CategoriesApi* | [**category_favorites_fetch_resource**](docs/CategoriesApi.md#category_favorites_fetch_resource) | **get** /category-favorites/{id} | Category Favorites_Fetch Resource
*CategoriesApi* | [**create_resource**](docs/CategoriesApi.md#create_resource) | **post** /category-favorites | Create Resource
*CategoriesApi* | [**delete_resource**](docs/CategoriesApi.md#delete_resource) | **delete** /category-favorites/{id} | Delete Resource
*CategoriesApi* | [**get_fetch_collection1**](docs/CategoriesApi.md#get_fetch_collection1) | **get** /categories | Fetch Collection
*CategoriesApi* | [**get_fetch_resource1**](docs/CategoriesApi.md#get_fetch_resource1) | **get** /categories/{id} | Fetch Resource
*CategoriesApi* | [**update_resource**](docs/CategoriesApi.md#update_resource) | **patch** /category-favorites/{id} | Update Resource
*CharactersApi* | [**characters_create_resource**](docs/CharactersApi.md#characters_create_resource) | **post** /characters | Characters_Create Resource
*CharactersApi* | [**characters_delete_resource**](docs/CharactersApi.md#characters_delete_resource) | **delete** /characters/{id} | Characters_Delete Resource
*CharactersApi* | [**characters_fetch_collection**](docs/CharactersApi.md#characters_fetch_collection) | **get** /characters | Characters_Fetch Collection
*CharactersApi* | [**characters_fetch_resource**](docs/CharactersApi.md#characters_fetch_resource) | **get** /characters/{id} | Characters_Fetch Resource
*CharactersApi* | [**characters_update_resource**](docs/CharactersApi.md#characters_update_resource) | **patch** /characters/{id} | Characters_Update Resource
*CharactersApi* | [**delete_resource1234567**](docs/CharactersApi.md#delete_resource1234567) | **delete** /anime-characters/{id} | Delete Resource
*CharactersApi* | [**get_fetch_collection12345678910**](docs/CharactersApi.md#get_fetch_collection12345678910) | **get** /anime-characters | Fetch Collection
*CharactersApi* | [**get_fetch_resource12345678910**](docs/CharactersApi.md#get_fetch_resource12345678910) | **get** /anime-characters/{id} | Fetch Resource
*CharactersApi* | [**manga_characters_create_resource**](docs/CharactersApi.md#manga_characters_create_resource) | **post** /manga-characters | Manga Characters_Create Resource
*CharactersApi* | [**manga_characters_delete_resource**](docs/CharactersApi.md#manga_characters_delete_resource) | **delete** /manga-characters/{id} | Manga Characters_Delete Resource
*CharactersApi* | [**manga_characters_fetch_collection**](docs/CharactersApi.md#manga_characters_fetch_collection) | **get** /manga-characters | Manga Characters_Fetch Collection
*CharactersApi* | [**manga_characters_fetch_resource**](docs/CharactersApi.md#manga_characters_fetch_resource) | **get** /manga-characters/{id} | Manga Characters_Fetch Resource
*CharactersApi* | [**manga_characters_update_resource**](docs/CharactersApi.md#manga_characters_update_resource) | **patch** /manga-characters/{id} | Manga Characters_Update Resource
*CharactersApi* | [**patch_update_resource123456**](docs/CharactersApi.md#patch_update_resource123456) | **patch** /anime-characters/{id} | Update Resource
*CharactersApi* | [**post_create_resource123456**](docs/CharactersApi.md#post_create_resource123456) | **post** /anime-characters | Create Resource
*CommentsApi* | [**comment_likes_create_resource**](docs/CommentsApi.md#comment_likes_create_resource) | **post** /comment-likes | Comment Likes_Create Resource
*CommentsApi* | [**comment_likes_delete_resource**](docs/CommentsApi.md#comment_likes_delete_resource) | **delete** /comment-likes/{id} | Comment Likes_Delete Resource
*CommentsApi* | [**comment_likes_fetch_collection**](docs/CommentsApi.md#comment_likes_fetch_collection) | **get** /comment-likes | Comment Likes_Fetch Collection
*CommentsApi* | [**comment_likes_fetch_resource**](docs/CommentsApi.md#comment_likes_fetch_resource) | **get** /comment-likes/{id} | Comment Likes_Fetch Resource
*CommentsApi* | [**delete_resource123456**](docs/CommentsApi.md#delete_resource123456) | **delete** /comments/{id} | Delete Resource
*CommentsApi* | [**get_fetch_collection123456789**](docs/CommentsApi.md#get_fetch_collection123456789) | **get** /comments | Fetch Collection
*CommentsApi* | [**get_fetch_resource123456789**](docs/CommentsApi.md#get_fetch_resource123456789) | **get** /comments/{id} | Fetch Resource
*CommentsApi* | [**patch_update_resource12345**](docs/CommentsApi.md#patch_update_resource12345) | **patch** /comments/{id} | Update Resource
*CommentsApi* | [**post_create_resource12345**](docs/CommentsApi.md#post_create_resource12345) | **post** /comments | Create Resource
*GroupsApi* | [**delete_resource123456789**](docs/GroupsApi.md#delete_resource123456789) | **delete** /groups/{id} | Delete Resource
*GroupsApi* | [**get_fetch_collection123456789101112**](docs/GroupsApi.md#get_fetch_collection123456789101112) | **get** /groups | Fetch Collection
*GroupsApi* | [**get_fetch_resource123456789101112**](docs/GroupsApi.md#get_fetch_resource123456789101112) | **get** /groups/{id} | Fetch Resource
*GroupsApi* | [**group_action_logs_fetch_collection**](docs/GroupsApi.md#group_action_logs_fetch_collection) | **get** /group-action-logs | Group Action Logs_Fetch Collection
*GroupsApi* | [**group_action_logs_fetch_resource**](docs/GroupsApi.md#group_action_logs_fetch_resource) | **get** /group-action-logs/{id} | Group Action Logs_Fetch Resource
*GroupsApi* | [**group_bans_create_resource**](docs/GroupsApi.md#group_bans_create_resource) | **post** /group-bans | Group Bans_Create Resource
*GroupsApi* | [**group_bans_delete_resource**](docs/GroupsApi.md#group_bans_delete_resource) | **delete** /group-bans/{id} | Group Bans_Delete Resource
*GroupsApi* | [**group_bans_fetch_collection**](docs/GroupsApi.md#group_bans_fetch_collection) | **get** /group-bans | Group Bans_Fetch Collection
*GroupsApi* | [**group_bans_fetch_resource**](docs/GroupsApi.md#group_bans_fetch_resource) | **get** /group-bans/{id} | Group Bans_Fetch Resource
*GroupsApi* | [**group_categories_create_resource**](docs/GroupsApi.md#group_categories_create_resource) | **post** /group-categories | Group Categories_Create Resource
*GroupsApi* | [**group_categories_delete_resource**](docs/GroupsApi.md#group_categories_delete_resource) | **delete** /group-categories/{id} | Group Categories_Delete Resource
*GroupsApi* | [**group_categories_fetch_collection**](docs/GroupsApi.md#group_categories_fetch_collection) | **get** /group-categories | Group Categories_Fetch Collection
*GroupsApi* | [**group_categories_fetch_resource**](docs/GroupsApi.md#group_categories_fetch_resource) | **get** /group-categories/{id} | Group Categories_Fetch Resource
*GroupsApi* | [**group_categories_update_resource**](docs/GroupsApi.md#group_categories_update_resource) | **patch** /group-categories/{id} | Group Categories_Update Resource
*GroupsApi* | [**group_invites_create_resource**](docs/GroupsApi.md#group_invites_create_resource) | **post** /group-invites | Group Invites_Create Resource
*GroupsApi* | [**group_invites_delete_resource**](docs/GroupsApi.md#group_invites_delete_resource) | **delete** /group-invites/{id} | Group Invites_Delete Resource
*GroupsApi* | [**group_invites_fetch_collection**](docs/GroupsApi.md#group_invites_fetch_collection) | **get** /group-invites | Group Invites_Fetch Collection
*GroupsApi* | [**group_invites_fetch_resource**](docs/GroupsApi.md#group_invites_fetch_resource) | **get** /group-invites/{id} | Group Invites_Fetch Resource
*GroupsApi* | [**group_invites_update_resource**](docs/GroupsApi.md#group_invites_update_resource) | **patch** /group-invites/{id} | Group Invites_Update Resource
*GroupsApi* | [**group_member_notes_create_resource**](docs/GroupsApi.md#group_member_notes_create_resource) | **post** /group-member-notes | Group Member Notes_Create Resource
*GroupsApi* | [**group_member_notes_delete_resource**](docs/GroupsApi.md#group_member_notes_delete_resource) | **delete** /group-member-notes/{id} | Group Member Notes_Delete Resource
*GroupsApi* | [**group_member_notes_fetch_collection**](docs/GroupsApi.md#group_member_notes_fetch_collection) | **get** /group-member-notes | Group Member Notes_Fetch Collection
*GroupsApi* | [**group_member_notes_fetch_resource**](docs/GroupsApi.md#group_member_notes_fetch_resource) | **get** /group-member-notes/{id} | Group Member Notes_Fetch Resource
*GroupsApi* | [**group_member_notes_update_resource**](docs/GroupsApi.md#group_member_notes_update_resource) | **patch** /group-member-notes/{id} | Group Member Notes_Update Resource
*GroupsApi* | [**group_members_create_resource**](docs/GroupsApi.md#group_members_create_resource) | **post** /group-members | Group Members_Create Resource
*GroupsApi* | [**group_members_delete_resource**](docs/GroupsApi.md#group_members_delete_resource) | **delete** /group-members/{id} | Group Members_Delete Resource
*GroupsApi* | [**group_members_fetch_collection**](docs/GroupsApi.md#group_members_fetch_collection) | **get** /group-members | Group Members_Fetch Collection
*GroupsApi* | [**group_members_fetch_resource**](docs/GroupsApi.md#group_members_fetch_resource) | **get** /group-members/{id} | Group Members_Fetch Resource
*GroupsApi* | [**group_members_update_resource**](docs/GroupsApi.md#group_members_update_resource) | **patch** /group-members/{id} | Group Members_Update Resource
*GroupsApi* | [**group_neighbors_create_resource**](docs/GroupsApi.md#group_neighbors_create_resource) | **post** /group-neighbors | Group Neighbors_Create Resource
*GroupsApi* | [**group_neighbors_delete_resource**](docs/GroupsApi.md#group_neighbors_delete_resource) | **delete** /group-neighbors/{id} | Group Neighbors_Delete Resource
*GroupsApi* | [**group_neighbors_fetch_collection**](docs/GroupsApi.md#group_neighbors_fetch_collection) | **get** /group-neighbors | Group Neighbors_Fetch Collection
*GroupsApi* | [**group_neighbors_fetch_resource**](docs/GroupsApi.md#group_neighbors_fetch_resource) | **get** /group-neighbors/{id} | Group Neighbors_Fetch Resource
*GroupsApi* | [**group_permissions_create_resource**](docs/GroupsApi.md#group_permissions_create_resource) | **post** /group-permissions | Group Permissions_Create Resource
*GroupsApi* | [**group_permissions_delete_resource**](docs/GroupsApi.md#group_permissions_delete_resource) | **delete** /group-permissions/{id} | Group Permissions_Delete Resource
*GroupsApi* | [**group_permissions_fetch_collection**](docs/GroupsApi.md#group_permissions_fetch_collection) | **get** /group-permissions | Group Permissions_Fetch Collection
*GroupsApi* | [**group_permissions_fetch_resource**](docs/GroupsApi.md#group_permissions_fetch_resource) | **get** /group-permissions/{id} | Group Permissions_Fetch Resource
*GroupsApi* | [**group_reports_create_resource**](docs/GroupsApi.md#group_reports_create_resource) | **post** /group-reports | Group Reports_Create Resource
*GroupsApi* | [**group_reports_fetch_collection**](docs/GroupsApi.md#group_reports_fetch_collection) | **get** /group-reports | Group Reports_Fetch Collection
*GroupsApi* | [**group_reports_fetch_resource**](docs/GroupsApi.md#group_reports_fetch_resource) | **get** /group-reports/{id} | Group Reports_Fetch Resource
*GroupsApi* | [**group_reports_update_resource**](docs/GroupsApi.md#group_reports_update_resource) | **patch** /group-reports/{id} | Group Reports_Update Resource
*GroupsApi* | [**group_ticket_messages_create_resource**](docs/GroupsApi.md#group_ticket_messages_create_resource) | **post** /group-ticket-messages | Group Ticket Messages_Create Resource
*GroupsApi* | [**group_ticket_messages_delete_resource**](docs/GroupsApi.md#group_ticket_messages_delete_resource) | **delete** /group-ticket-messages/{id} | Group Ticket Messages_Delete Resource
*GroupsApi* | [**group_ticket_messages_fetch_collection**](docs/GroupsApi.md#group_ticket_messages_fetch_collection) | **get** /group-ticket-messages | Group Ticket Messages_Fetch Collection
*GroupsApi* | [**group_ticket_messages_fetch_resource**](docs/GroupsApi.md#group_ticket_messages_fetch_resource) | **get** /group-ticket-messages/{id} | Group Ticket Messages_Fetch Resource
*GroupsApi* | [**group_ticket_messages_update_resource**](docs/GroupsApi.md#group_ticket_messages_update_resource) | **patch** /group-ticket-messages/{id} | Group Ticket Messages_Update Resource
*GroupsApi* | [**group_tickets_create_resource**](docs/GroupsApi.md#group_tickets_create_resource) | **post** /group-tickets | Group Tickets_Create Resource
*GroupsApi* | [**group_tickets_fetch_collection**](docs/GroupsApi.md#group_tickets_fetch_collection) | **get** /group-tickets | Group Tickets_Fetch Collection
*GroupsApi* | [**group_tickets_fetch_resource**](docs/GroupsApi.md#group_tickets_fetch_resource) | **get** /group-tickets/{id} | Group Tickets_Fetch Resource
*GroupsApi* | [**group_tickets_update_resource**](docs/GroupsApi.md#group_tickets_update_resource) | **patch** /group-tickets/{id} | Group Tickets_Update Resource
*GroupsApi* | [**leader_chat_messages_create_resource**](docs/GroupsApi.md#leader_chat_messages_create_resource) | **post** /leader-chat-messages | Leader Chat Messages_Create Resource
*GroupsApi* | [**leader_chat_messages_delete_resource**](docs/GroupsApi.md#leader_chat_messages_delete_resource) | **delete** /leader-chat-messages/{id} | Leader Chat Messages_Delete Resource
*GroupsApi* | [**leader_chat_messages_fetch_collection**](docs/GroupsApi.md#leader_chat_messages_fetch_collection) | **get** /leader-chat-messages | Leader Chat Messages_Fetch Collection
*GroupsApi* | [**leader_chat_messages_fetch_resource**](docs/GroupsApi.md#leader_chat_messages_fetch_resource) | **get** /leader-chat-messages/{id} | Leader Chat Messages_Fetch Resource
*GroupsApi* | [**leader_chat_messages_update_resource**](docs/GroupsApi.md#leader_chat_messages_update_resource) | **patch** /leader-chat-messages/{id} | Leader Chat Messages_Update Resource
*GroupsApi* | [**patch_update_resource12345678**](docs/GroupsApi.md#patch_update_resource12345678) | **patch** /groups/{id} | Update Resource
*GroupsApi* | [**post_create_resource12345678**](docs/GroupsApi.md#post_create_resource12345678) | **post** /groups | Create Resource
*MangaApi* | [**chapters_fetch_collection**](docs/MangaApi.md#chapters_fetch_collection) | **get** /chapters | Chapters_Fetch Collection
*MangaApi* | [**chapters_fetch_resource**](docs/MangaApi.md#chapters_fetch_resource) | **get** /chapters/{id} | Chapters_Fetch Resource
*MangaApi* | [**get_fetch_collection**](docs/MangaApi.md#get_fetch_collection) | **get** /manga | Fetch Collection
*MangaApi* | [**get_fetch_resource**](docs/MangaApi.md#get_fetch_resource) | **get** /manga/{id} | Fetch Resource
*MangaApi* | [**trending_manga_fetch_collection**](docs/MangaApi.md#trending_manga_fetch_collection) | **get** /trending/manga | Trending Manga_Fetch Collection
*MediaFollowsApi* | [**delete_resource1**](docs/MediaFollowsApi.md#delete_resource1) | **delete** /media-follows/{id} | Delete Resource
*MediaFollowsApi* | [**get_fetch_collection123**](docs/MediaFollowsApi.md#get_fetch_collection123) | **get** /media-follows | Fetch Collection
*MediaFollowsApi* | [**get_fetch_resource123**](docs/MediaFollowsApi.md#get_fetch_resource123) | **get** /media-follows/{id} | Fetch Resource
*MediaFollowsApi* | [**media_attribute_votes_create_resource**](docs/MediaFollowsApi.md#media_attribute_votes_create_resource) | **post** /media-attribute-votes | Media Attribute Votes_Create Resource
*MediaFollowsApi* | [**media_attribute_votes_delete_resource**](docs/MediaFollowsApi.md#media_attribute_votes_delete_resource) | **delete** /media-attribute-votes/{id} | Media Attribute Votes_Delete Resource
*MediaFollowsApi* | [**media_attribute_votes_fetch_collection**](docs/MediaFollowsApi.md#media_attribute_votes_fetch_collection) | **get** /media-attribute-votes | Media Attribute Votes_Fetch Collection
*MediaFollowsApi* | [**media_attribute_votes_fetch_resource**](docs/MediaFollowsApi.md#media_attribute_votes_fetch_resource) | **get** /media-attribute-votes/{id} | Media Attribute Votes_Fetch Resource
*MediaFollowsApi* | [**media_attribute_votes_update_resource**](docs/MediaFollowsApi.md#media_attribute_votes_update_resource) | **patch** /media-attribute-votes/{id} | Media Attribute Votes_Update Resource
*MediaFollowsApi* | [**media_attributes_fetch_collection**](docs/MediaFollowsApi.md#media_attributes_fetch_collection) | **get** /media-attributes | Media Attributes_Fetch Collection
*MediaFollowsApi* | [**media_attributes_fetch_resource**](docs/MediaFollowsApi.md#media_attributes_fetch_resource) | **get** /media-attributes/{id} | Media Attributes_Fetch Resource
*MediaFollowsApi* | [**patch_update_resource**](docs/MediaFollowsApi.md#patch_update_resource) | **patch** /media-follows/{id} | Update Resource
*MediaFollowsApi* | [**post_create_resource**](docs/MediaFollowsApi.md#post_create_resource) | **post** /media-follows | Create Resource
*MediaRelationsApi* | [**franchises_fetch_collection**](docs/MediaRelationsApi.md#franchises_fetch_collection) | **get** /franchises | Franchises_Fetch Collection
*MediaRelationsApi* | [**franchises_fetch_resource**](docs/MediaRelationsApi.md#franchises_fetch_resource) | **get** /franchises/{id} | Franchises_Fetch Resource
*MediaRelationsApi* | [**get_fetch_collection12**](docs/MediaRelationsApi.md#get_fetch_collection12) | **get** /media-relationships | Fetch Collection
*MediaRelationsApi* | [**get_fetch_resource12**](docs/MediaRelationsApi.md#get_fetch_resource12) | **get** /media-relationships/{id} | Fetch Resource
*MediaRelationsApi* | [**installments_fetch_collection**](docs/MediaRelationsApi.md#installments_fetch_collection) | **get** /installments | Installments_Fetch Collection
*MediaRelationsApi* | [**installments_fetch_resource**](docs/MediaRelationsApi.md#installments_fetch_resource) | **get** /installments/{id} | Installments_Fetch Resource
*MediaRelationsApi* | [**mappings_fetch_collection**](docs/MediaRelationsApi.md#mappings_fetch_collection) | **get** /mappings | Mappings_Fetch Collection
*MediaRelationsApi* | [**mappings_fetch_resource**](docs/MediaRelationsApi.md#mappings_fetch_resource) | **get** /mappings/{id} | Mappings_Fetch Resource
*PostsApi* | [**delete_resource12345**](docs/PostsApi.md#delete_resource12345) | **delete** /posts/{id} | Delete Resource
*PostsApi* | [**get_fetch_collection12345678**](docs/PostsApi.md#get_fetch_collection12345678) | **get** /posts | Fetch Collection
*PostsApi* | [**get_fetch_resource12345678**](docs/PostsApi.md#get_fetch_resource12345678) | **get** /posts/{id} | Fetch Resource
*PostsApi* | [**patch_update_resource1234**](docs/PostsApi.md#patch_update_resource1234) | **patch** /posts/{id} | Update Resource
*PostsApi* | [**post_create_resource1234**](docs/PostsApi.md#post_create_resource1234) | **post** /posts | Create Resource
*PostsApi* | [**post_follows_create_resource**](docs/PostsApi.md#post_follows_create_resource) | **post** /post-follows | Post Follows_Create Resource
*PostsApi* | [**post_follows_delete_resource**](docs/PostsApi.md#post_follows_delete_resource) | **delete** /post-follows/{id} | Post Follows_Delete Resource
*PostsApi* | [**post_follows_fetch_collection**](docs/PostsApi.md#post_follows_fetch_collection) | **get** /post-follows | Post Follows_Fetch Collection
*PostsApi* | [**post_follows_fetch_resource**](docs/PostsApi.md#post_follows_fetch_resource) | **get** /post-follows/{id} | Post Follows_Fetch Resource
*PostsApi* | [**post_likes_create_resource**](docs/PostsApi.md#post_likes_create_resource) | **post** /post-likes | Post Likes_Create Resource
*PostsApi* | [**post_likes_delete_resource**](docs/PostsApi.md#post_likes_delete_resource) | **delete** /post-likes/{id} | Post Likes_Delete Resource
*PostsApi* | [**post_likes_fetch_collection**](docs/PostsApi.md#post_likes_fetch_collection) | **get** /post-likes | Post Likes_Fetch Collection
*PostsApi* | [**post_likes_fetch_resource**](docs/PostsApi.md#post_likes_fetch_resource) | **get** /post-likes/{id} | Post Likes_Fetch Resource
*ProducersStaffApi* | [**anime_staff_create_resource**](docs/ProducersStaffApi.md#anime_staff_create_resource) | **post** /anime-staff | Anime Staff_Create Resource
*ProducersStaffApi* | [**anime_staff_delete_resource**](docs/ProducersStaffApi.md#anime_staff_delete_resource) | **delete** /anime-staff/{id} | Anime Staff_Delete Resource
*ProducersStaffApi* | [**anime_staff_fetch_collection**](docs/ProducersStaffApi.md#anime_staff_fetch_collection) | **get** /anime-staff | Anime Staff_Fetch Collection
*ProducersStaffApi* | [**anime_staff_fetch_resource**](docs/ProducersStaffApi.md#anime_staff_fetch_resource) | **get** /anime-staff/{id} | Anime Staff_Fetch Resource
*ProducersStaffApi* | [**anime_staff_update_resource**](docs/ProducersStaffApi.md#anime_staff_update_resource) | **patch** /anime-staff/{id} | Anime Staff_Update Resource
*ProducersStaffApi* | [**castings_create_resource**](docs/ProducersStaffApi.md#castings_create_resource) | **post** /castings | Castings_Create Resource
*ProducersStaffApi* | [**castings_delete_resource**](docs/ProducersStaffApi.md#castings_delete_resource) | **delete** /castings/{id} | Castings_Delete Resource
*ProducersStaffApi* | [**castings_fetch_collection**](docs/ProducersStaffApi.md#castings_fetch_collection) | **get** /castings | Castings_Fetch Collection
*ProducersStaffApi* | [**castings_fetch_resource**](docs/ProducersStaffApi.md#castings_fetch_resource) | **get** /castings/{id} | Castings_Fetch Resource
*ProducersStaffApi* | [**castings_update_resource**](docs/ProducersStaffApi.md#castings_update_resource) | **patch** /castings/{id} | Castings_Update Resource
*ProducersStaffApi* | [**delete_resource12345678**](docs/ProducersStaffApi.md#delete_resource12345678) | **delete** /anime-productions/{id} | Delete Resource
*ProducersStaffApi* | [**get_fetch_collection1234567891011**](docs/ProducersStaffApi.md#get_fetch_collection1234567891011) | **get** /anime-productions | Fetch Collection
*ProducersStaffApi* | [**get_fetch_resource1234567891011**](docs/ProducersStaffApi.md#get_fetch_resource1234567891011) | **get** /anime-productions/{id} | Fetch Resource
*ProducersStaffApi* | [**manga_staff_create_resource**](docs/ProducersStaffApi.md#manga_staff_create_resource) | **post** /manga-staff | Manga Staff_Create Resource
*ProducersStaffApi* | [**manga_staff_delete_resource**](docs/ProducersStaffApi.md#manga_staff_delete_resource) | **delete** /manga-staff/{id} | Manga Staff_Delete Resource
*ProducersStaffApi* | [**manga_staff_fetch_collection**](docs/ProducersStaffApi.md#manga_staff_fetch_collection) | **get** /manga-staff | Manga Staff_Fetch Collection
*ProducersStaffApi* | [**manga_staff_fetch_resource**](docs/ProducersStaffApi.md#manga_staff_fetch_resource) | **get** /manga-staff/{id} | Manga Staff_Fetch Resource
*ProducersStaffApi* | [**manga_staff_update_resource**](docs/ProducersStaffApi.md#manga_staff_update_resource) | **patch** /manga-staff/{id} | Manga Staff_Update Resource
*ProducersStaffApi* | [**patch_update_resource1234567**](docs/ProducersStaffApi.md#patch_update_resource1234567) | **patch** /anime-productions/{id} | Update Resource
*ProducersStaffApi* | [**people_create_resource**](docs/ProducersStaffApi.md#people_create_resource) | **post** /people | People_Create Resource
*ProducersStaffApi* | [**people_delete_resource**](docs/ProducersStaffApi.md#people_delete_resource) | **delete** /people/{id} | People_Delete Resource
*ProducersStaffApi* | [**people_fetch_collection**](docs/ProducersStaffApi.md#people_fetch_collection) | **get** /people | People_Fetch Collection
*ProducersStaffApi* | [**people_fetch_resource**](docs/ProducersStaffApi.md#people_fetch_resource) | **get** /people/{id} | People_Fetch Resource
*ProducersStaffApi* | [**people_update_resource**](docs/ProducersStaffApi.md#people_update_resource) | **patch** /people/{id} | People_Update Resource
*ProducersStaffApi* | [**post_create_resource1234567**](docs/ProducersStaffApi.md#post_create_resource1234567) | **post** /anime-productions | Create Resource
*ProducersStaffApi* | [**producers_create_resource**](docs/ProducersStaffApi.md#producers_create_resource) | **post** /producers | Producers_Create Resource
*ProducersStaffApi* | [**producers_delete_resource**](docs/ProducersStaffApi.md#producers_delete_resource) | **delete** /producers/{id} | Producers_Delete Resource
*ProducersStaffApi* | [**producers_fetch_collection**](docs/ProducersStaffApi.md#producers_fetch_collection) | **get** /producers | Producers_Fetch Collection
*ProducersStaffApi* | [**producers_fetch_resource**](docs/ProducersStaffApi.md#producers_fetch_resource) | **get** /producers/{id} | Producers_Fetch Resource
*ProducersStaffApi* | [**producers_update_resource**](docs/ProducersStaffApi.md#producers_update_resource) | **patch** /producers/{id} | Producers_Update Resource
*ReactionsApi* | [**delete_resource1234**](docs/ReactionsApi.md#delete_resource1234) | **delete** /media-reaction-votes/{id} | Delete Resource
*ReactionsApi* | [**get_fetch_collection1234567**](docs/ReactionsApi.md#get_fetch_collection1234567) | **get** /media-reaction-votes | Fetch Collection
*ReactionsApi* | [**get_fetch_resource1234567**](docs/ReactionsApi.md#get_fetch_resource1234567) | **get** /media-reaction-votes/{id} | Fetch Resource
*ReactionsApi* | [**media_reactions_create_resource**](docs/ReactionsApi.md#media_reactions_create_resource) | **post** /media-reactions | Media Reactions_Create Resource
*ReactionsApi* | [**media_reactions_delete_resource**](docs/ReactionsApi.md#media_reactions_delete_resource) | **delete** /media-reactions/{id} | Media Reactions_Delete Resource
*ReactionsApi* | [**media_reactions_fetch_collection**](docs/ReactionsApi.md#media_reactions_fetch_collection) | **get** /media-reactions | Media Reactions_Fetch Collection
*ReactionsApi* | [**media_reactions_fetch_resource**](docs/ReactionsApi.md#media_reactions_fetch_resource) | **get** /media-reactions/{id} | Media Reactions_Fetch Resource
*ReactionsApi* | [**media_reactions_update_resource**](docs/ReactionsApi.md#media_reactions_update_resource) | **patch** /media-reactions/{id} | Media Reactions_Update Resource
*ReactionsApi* | [**patch_update_resource123**](docs/ReactionsApi.md#patch_update_resource123) | **patch** /media-reaction-votes/{id} | Update Resource
*ReactionsApi* | [**post_create_resource123**](docs/ReactionsApi.md#post_create_resource123) | **post** /media-reaction-votes | Create Resource
*ReactionsApi* | [**review_likes_create_resource**](docs/ReactionsApi.md#review_likes_create_resource) | **post** /review-likes | Review Likes_Create Resource
*ReactionsApi* | [**review_likes_delete_resource**](docs/ReactionsApi.md#review_likes_delete_resource) | **delete** /review-likes/{id} | Review Likes_Delete Resource
*ReactionsApi* | [**review_likes_fetch_collection**](docs/ReactionsApi.md#review_likes_fetch_collection) | **get** /review-likes | Review Likes_Fetch Collection
*ReactionsApi* | [**review_likes_fetch_resource**](docs/ReactionsApi.md#review_likes_fetch_resource) | **get** /review-likes/{id} | Review Likes_Fetch Resource
*ReactionsApi* | [**review_likes_update_resource**](docs/ReactionsApi.md#review_likes_update_resource) | **patch** /review-likes/{id} | Review Likes_Update Resource
*ReactionsApi* | [**reviews_create_resource**](docs/ReactionsApi.md#reviews_create_resource) | **post** /reviews | Reviews_Create Resource
*ReactionsApi* | [**reviews_delete_resource**](docs/ReactionsApi.md#reviews_delete_resource) | **delete** /reviews/{id} | Reviews_Delete Resource
*ReactionsApi* | [**reviews_fetch_collection**](docs/ReactionsApi.md#reviews_fetch_collection) | **get** /reviews | Reviews_Fetch Collection
*ReactionsApi* | [**reviews_fetch_resource**](docs/ReactionsApi.md#reviews_fetch_resource) | **get** /reviews/{id} | Reviews_Fetch Resource
*ReactionsApi* | [**reviews_update_resource**](docs/ReactionsApi.md#reviews_update_resource) | **patch** /reviews/{id} | Reviews_Update Resource
*ReportsApi* | [**delete_resource12345678910**](docs/ReportsApi.md#delete_resource12345678910) | **delete** /reports/{id} | Delete Resource
*ReportsApi* | [**get_fetch_collection12345678910111213**](docs/ReportsApi.md#get_fetch_collection12345678910111213) | **get** /reports | Fetch Collection
*ReportsApi* | [**get_fetch_resource12345678910111213**](docs/ReportsApi.md#get_fetch_resource12345678910111213) | **get** /reports/{id} | Fetch Resource
*ReportsApi* | [**patch_update_resource123456789**](docs/ReportsApi.md#patch_update_resource123456789) | **patch** /reports/{id} | Update Resource
*ReportsApi* | [**post_create_resource123456789**](docs/ReportsApi.md#post_create_resource123456789) | **post** /reports | Create Resource
*SiteAnnouncementsApi* | [**get_fetch_collection1234567891011121314**](docs/SiteAnnouncementsApi.md#get_fetch_collection1234567891011121314) | **get** /site-announcements | Fetch Collection
*SiteAnnouncementsApi* | [**get_fetch_resource1234567891011121314**](docs/SiteAnnouncementsApi.md#get_fetch_resource1234567891011121314) | **get** /site-announcements/{id} | Fetch Resource
*StreamersApi* | [**get_fetch_collection1234**](docs/StreamersApi.md#get_fetch_collection1234) | **get** /streamers | Fetch Collection
*StreamersApi* | [**get_fetch_resource1234**](docs/StreamersApi.md#get_fetch_resource1234) | **get** /streamers/{id} | Fetch Resource
*StreamersApi* | [**streaming_links_fetch_collection**](docs/StreamersApi.md#streaming_links_fetch_collection) | **get** /streaming-links | Streaming Links_Fetch Collection
*StreamersApi* | [**streaming_links_fetch_resource**](docs/StreamersApi.md#streaming_links_fetch_resource) | **get** /streaming-links/{id} | Streaming Links_Fetch Resource
*UserLibrariesApi* | [**delete_resource123**](docs/UserLibrariesApi.md#delete_resource123) | **delete** /library-entries/{id} | Delete Resource
*UserLibrariesApi* | [**get_fetch_collection123456**](docs/UserLibrariesApi.md#get_fetch_collection123456) | **get** /library-entries | Fetch Collection
*UserLibrariesApi* | [**get_fetch_resource123456**](docs/UserLibrariesApi.md#get_fetch_resource123456) | **get** /library-entries/{id} | Fetch Resource
*UserLibrariesApi* | [**library_entry_logs_create_resource**](docs/UserLibrariesApi.md#library_entry_logs_create_resource) | **post** /library-entry-logs | Library Entry Logs_Create Resource
*UserLibrariesApi* | [**library_entry_logs_delete_resource**](docs/UserLibrariesApi.md#library_entry_logs_delete_resource) | **delete** /library-entry-logs/{id} | Library Entry Logs_Delete Resource
*UserLibrariesApi* | [**library_entry_logs_fetch_collection**](docs/UserLibrariesApi.md#library_entry_logs_fetch_collection) | **get** /library-entry-logs | Library Entry Logs_Fetch Collection
*UserLibrariesApi* | [**library_entry_logs_fetch_resource**](docs/UserLibrariesApi.md#library_entry_logs_fetch_resource) | **get** /library-entry-logs/{id} | Library Entry Logs_Fetch Resource
*UserLibrariesApi* | [**library_entry_logs_update_resource**](docs/UserLibrariesApi.md#library_entry_logs_update_resource) | **patch** /library-entry-logs/{id} | Library Entry Logs_Update Resource
*UserLibrariesApi* | [**library_events_delete_resource**](docs/UserLibrariesApi.md#library_events_delete_resource) | **delete** /library-events/{id} | Library Events_Delete Resource
*UserLibrariesApi* | [**library_events_fetch_collection**](docs/UserLibrariesApi.md#library_events_fetch_collection) | **get** /library-events | Library Events_Fetch Collection
*UserLibrariesApi* | [**library_events_fetch_resource**](docs/UserLibrariesApi.md#library_events_fetch_resource) | **get** /library-events/{id} | Library Events_Fetch Resource
*UserLibrariesApi* | [**list_imports_create_resource**](docs/UserLibrariesApi.md#list_imports_create_resource) | **post** /list-imports | List Imports_Create Resource
*UserLibrariesApi* | [**list_imports_fetch_collection**](docs/UserLibrariesApi.md#list_imports_fetch_collection) | **get** /list-imports | List Imports_Fetch Collection
*UserLibrariesApi* | [**list_imports_fetch_resource**](docs/UserLibrariesApi.md#list_imports_fetch_resource) | **get** /list-imports/{id} | List Imports_Fetch Resource
*UserLibrariesApi* | [**patch_update_resource12**](docs/UserLibrariesApi.md#patch_update_resource12) | **patch** /library-entries/{id} | Update Resource
*UserLibrariesApi* | [**post_create_resource12**](docs/UserLibrariesApi.md#post_create_resource12) | **post** /library-entries | Create Resource
*UsersApi* | [**delete_resource12**](docs/UsersApi.md#delete_resource12) | **delete** /blocks/{id} | Delete Resource
*UsersApi* | [**favorites_create_resource**](docs/UsersApi.md#favorites_create_resource) | **post** /favorites | Favorites_Create Resource
*UsersApi* | [**favorites_delete_resource**](docs/UsersApi.md#favorites_delete_resource) | **delete** /favorites/{id} | Favorites_Delete Resource
*UsersApi* | [**favorites_fetch_collection**](docs/UsersApi.md#favorites_fetch_collection) | **get** /favorites | Favorites_Fetch Collection
*UsersApi* | [**favorites_fetch_resource**](docs/UsersApi.md#favorites_fetch_resource) | **get** /favorites/{id} | Favorites_Fetch Resource
*UsersApi* | [**favorites_update_resource**](docs/UsersApi.md#favorites_update_resource) | **patch** /favorites/{id} | Favorites_Update Resource
*UsersApi* | [**follows_create_resource**](docs/UsersApi.md#follows_create_resource) | **post** /follows | Follows_Create Resource
*UsersApi* | [**follows_delete_resource**](docs/UsersApi.md#follows_delete_resource) | **delete** /follows/{id} | Follows_Delete Resource
*UsersApi* | [**follows_fetch_collection**](docs/UsersApi.md#follows_fetch_collection) | **get** /follows | Follows_Fetch Collection
*UsersApi* | [**follows_fetch_resource**](docs/UsersApi.md#follows_fetch_resource) | **get** /follows/{id} | Follows_Fetch Resource
*UsersApi* | [**follows_update_resource**](docs/UsersApi.md#follows_update_resource) | **patch** /follows/{id} | Follows_Update Resource
*UsersApi* | [**get_fetch_collection12345**](docs/UsersApi.md#get_fetch_collection12345) | **get** /blocks | Fetch Collection
*UsersApi* | [**get_fetch_resource12345**](docs/UsersApi.md#get_fetch_resource12345) | **get** /blocks/{id} | Fetch Resource
*UsersApi* | [**linked_accounts_create_resource**](docs/UsersApi.md#linked_accounts_create_resource) | **post** /linked-accounts | Linked Accounts_Create Resource
*UsersApi* | [**linked_accounts_delete_resource**](docs/UsersApi.md#linked_accounts_delete_resource) | **delete** /linked-accounts/{id} | Linked Accounts_Delete Resource
*UsersApi* | [**linked_accounts_fetch_collection**](docs/UsersApi.md#linked_accounts_fetch_collection) | **get** /linked-accounts | Linked Accounts_Fetch Collection
*UsersApi* | [**linked_accounts_fetch_resource**](docs/UsersApi.md#linked_accounts_fetch_resource) | **get** /linked-accounts/{id} | Linked Accounts_Fetch Resource
*UsersApi* | [**linked_accounts_update_resource**](docs/UsersApi.md#linked_accounts_update_resource) | **patch** /linked-accounts/{id} | Linked Accounts_Update Resource
*UsersApi* | [**patch_update_resource1**](docs/UsersApi.md#patch_update_resource1) | **patch** /blocks/{id} | Update Resource
*UsersApi* | [**post_create_resource1**](docs/UsersApi.md#post_create_resource1) | **post** /blocks | Create Resource
*UsersApi* | [**profile_link_sites_fetch_collection**](docs/UsersApi.md#profile_link_sites_fetch_collection) | **get** /profile-link-sites | Profile Link Sites_Fetch Collection
*UsersApi* | [**profile_link_sites_fetch_resource**](docs/UsersApi.md#profile_link_sites_fetch_resource) | **get** /profile-link-sites/{id} | Profile Link Sites_Fetch Resource
*UsersApi* | [**profile_links_create_resource**](docs/UsersApi.md#profile_links_create_resource) | **post** /profile-links | Profile Links_Create Resource
*UsersApi* | [**profile_links_delete_resource**](docs/UsersApi.md#profile_links_delete_resource) | **delete** /profile-links/{id} | Profile Links_Delete Resource
*UsersApi* | [**profile_links_fetch_collection**](docs/UsersApi.md#profile_links_fetch_collection) | **get** /profile-links | Profile Links_Fetch Collection
*UsersApi* | [**profile_links_fetch_resource**](docs/UsersApi.md#profile_links_fetch_resource) | **get** /profile-links/{id} | Profile Links_Fetch Resource
*UsersApi* | [**profile_links_update_resource**](docs/UsersApi.md#profile_links_update_resource) | **patch** /profile-links/{id} | Profile Links_Update Resource
*UsersApi* | [**roles_create_resource**](docs/UsersApi.md#roles_create_resource) | **post** /roles | Roles_Create Resource
*UsersApi* | [**roles_delete_resource**](docs/UsersApi.md#roles_delete_resource) | **delete** /roles/{id} | Roles_Delete Resource
*UsersApi* | [**roles_fetch_collection**](docs/UsersApi.md#roles_fetch_collection) | **get** /roles | Roles_Fetch Collection
*UsersApi* | [**roles_fetch_resource**](docs/UsersApi.md#roles_fetch_resource) | **get** /roles/{id} | Roles_Fetch Resource
*UsersApi* | [**roles_update_resource**](docs/UsersApi.md#roles_update_resource) | **patch** /roles/{id} | Roles_Update Resource
*UsersApi* | [**stats_delete_resource**](docs/UsersApi.md#stats_delete_resource) | **delete** /stats/{id} | Stats_Delete Resource
*UsersApi* | [**stats_fetch_collection**](docs/UsersApi.md#stats_fetch_collection) | **get** /stats | Stats_Fetch Collection
*UsersApi* | [**stats_fetch_resource**](docs/UsersApi.md#stats_fetch_resource) | **get** /stats/{id} | Stats_Fetch Resource
*UsersApi* | [**user_roles_create_resource**](docs/UsersApi.md#user_roles_create_resource) | **post** /user-roles | User Roles_Create Resource
*UsersApi* | [**user_roles_delete_resource**](docs/UsersApi.md#user_roles_delete_resource) | **delete** /user-roles/{id} | User Roles_Delete Resource
*UsersApi* | [**user_roles_fetch_collection**](docs/UsersApi.md#user_roles_fetch_collection) | **get** /user-roles | User Roles_Fetch Collection
*UsersApi* | [**user_roles_fetch_resource**](docs/UsersApi.md#user_roles_fetch_resource) | **get** /user-roles/{id} | User Roles_Fetch Resource
*UsersApi* | [**users_create_resource**](docs/UsersApi.md#users_create_resource) | **post** /users | Users_Create Resource
*UsersApi* | [**users_delete_resource**](docs/UsersApi.md#users_delete_resource) | **delete** /users/{id} | Users_Delete Resource
*UsersApi* | [**users_fetch_collection**](docs/UsersApi.md#users_fetch_collection) | **get** /users | Users_Fetch Collection
*UsersApi* | [**users_fetch_resource**](docs/UsersApi.md#users_fetch_resource) | **get** /users/{id} | Users_Fetch Resource
*UsersApi* | [**users_update_resource**](docs/UsersApi.md#users_update_resource) | **patch** /users/{id} | Users_Update Resource


## Documentation For Models

 - [ActionLogs](docs/ActionLogs.md)
 - [ActionPerformed](docs/ActionPerformed.md)
 - [Activity](docs/Activity.md)
 - [AgeRating](docs/AgeRating.md)
 - [AllCategories](docs/AllCategories.md)
 - [AllTime](docs/AllTime.md)
 - [Ama](docs/Ama.md)
 - [Anime](docs/Anime.md)
 - [Anime1](docs/Anime1.md)
 - [AnimeAttributes](docs/AnimeAttributes.md)
 - [AnimeCharacters](docs/AnimeCharacters.md)
 - [AnimeCharacters1](docs/AnimeCharacters1.md)
 - [AnimeCharactersAttributes](docs/AnimeCharactersAttributes.md)
 - [AnimeMediaAttribute](docs/AnimeMediaAttribute.md)
 - [AnimeProductions](docs/AnimeProductions.md)
 - [AnimeProductions1](docs/AnimeProductions1.md)
 - [AnimeProductionsAttributes](docs/AnimeProductionsAttributes.md)
 - [AnimeStaff](docs/AnimeStaff.md)
 - [AnimeStaff1](docs/AnimeStaff1.md)
 - [AnimeStaffAttributes](docs/AnimeStaffAttributes.md)
 - [AnimeStaffFetchCollectionresponse](docs/AnimeStaffFetchCollectionresponse.md)
 - [AnimeStaffFetchResourceresponse](docs/AnimeStaffFetchResourceresponse.md)
 - [Application](docs/Application.md)
 - [Assignee](docs/Assignee.md)
 - [Avatar](docs/Avatar.md)
 - [Avatar1](docs/Avatar1.md)
 - [Blocked](docs/Blocked.md)
 - [Blocks](docs/Blocks.md)
 - [Blocks1](docs/Blocks1.md)
 - [BlocksAttributes](docs/BlocksAttributes.md)
 - [Castings](docs/Castings.md)
 - [Castings2](docs/Castings2.md)
 - [CastingsAttributes](docs/CastingsAttributes.md)
 - [CastingsFetchCollectionresponse](docs/CastingsFetchCollectionresponse.md)
 - [CastingsFetchResourceresponse](docs/CastingsFetchResourceresponse.md)
 - [Categories](docs/Categories.md)
 - [Categories2](docs/Categories2.md)
 - [CategoriesAttributes](docs/CategoriesAttributes.md)
 - [Category](docs/Category.md)
 - [CategoryFavorites](docs/CategoryFavorites.md)
 - [CategoryFavoritesAttributes](docs/CategoryFavoritesAttributes.md)
 - [CategoryFavoritesFetchCollectionresponse](docs/CategoryFavoritesFetchCollectionresponse.md)
 - [CategoryFavoritesFetchResourceresponse](docs/CategoryFavoritesFetchResourceresponse.md)
 - [ChangedData](docs/ChangedData.md)
 - [Chapters](docs/Chapters.md)
 - [Chapters1](docs/Chapters1.md)
 - [ChaptersAttributes](docs/ChaptersAttributes.md)
 - [ChaptersFetchCollectionresponse](docs/ChaptersFetchCollectionresponse.md)
 - [ChaptersFetchResourceresponse](docs/ChaptersFetchResourceresponse.md)
 - [Character](docs/Character.md)
 - [Characters](docs/Characters.md)
 - [CharactersAttributes](docs/CharactersAttributes.md)
 - [CharactersFetchCollectionresponse](docs/CharactersFetchCollectionresponse.md)
 - [CharactersFetchResourceresponse](docs/CharactersFetchResourceresponse.md)
 - [Comment](docs/Comment.md)
 - [CommentLikes](docs/CommentLikes.md)
 - [CommentLikesAttributes](docs/CommentLikesAttributes.md)
 - [CommentLikesFetchCollectionresponse](docs/CommentLikesFetchCollectionresponse.md)
 - [CommentLikesFetchResourceresponse](docs/CommentLikesFetchResourceresponse.md)
 - [Comments](docs/Comments.md)
 - [Comments1](docs/Comments1.md)
 - [CommentsAttributes](docs/CommentsAttributes.md)
 - [CoverImage](docs/CoverImage.md)
 - [CoverImage4](docs/CoverImage4.md)
 - [Destination](docs/Destination.md)
 - [Dimensions](docs/Dimensions.md)
 - [Dimensions1](docs/Dimensions1.md)
 - [Dimensions10](docs/Dimensions10.md)
 - [Dimensions9](docs/Dimensions9.md)
 - [Drama](docs/Drama.md)
 - [Drama2](docs/Drama2.md)
 - [DramaAttributes](docs/DramaAttributes.md)
 - [DramaCharacters](docs/DramaCharacters.md)
 - [DramaStaff](docs/DramaStaff.md)
 - [DramasMediaAttribute](docs/DramasMediaAttribute.md)
 - [Episodes](docs/Episodes.md)
 - [Episodes1](docs/Episodes1.md)
 - [EpisodesAttributes](docs/EpisodesAttributes.md)
 - [EpisodesFetchCollectionresponse](docs/EpisodesFetchCollectionresponse.md)
 - [EpisodesFetchResourceresponse](docs/EpisodesFetchResourceresponse.md)
 - [Error](docs/Error.md)
 - [Error406](docs/Error406.md)
 - [Event](docs/Event.md)
 - [ExternalSite](docs/ExternalSite.md)
 - [Favorites](docs/Favorites.md)
 - [Favorites1](docs/Favorites1.md)
 - [FavoritesAttributes](docs/FavoritesAttributes.md)
 - [FavoritesFetchCollectionresponse](docs/FavoritesFetchCollectionresponse.md)
 - [FavoritesFetchResourceresponse](docs/FavoritesFetchResourceresponse.md)
 - [FetchCollectionresponse](docs/FetchCollectionresponse.md)
 - [FetchCollectionresponse1](docs/FetchCollectionresponse1.md)
 - [FetchCollectionresponse10](docs/FetchCollectionresponse10.md)
 - [FetchCollectionresponse11](docs/FetchCollectionresponse11.md)
 - [FetchCollectionresponse12](docs/FetchCollectionresponse12.md)
 - [FetchCollectionresponse13](docs/FetchCollectionresponse13.md)
 - [FetchCollectionresponse14](docs/FetchCollectionresponse14.md)
 - [FetchCollectionresponse15](docs/FetchCollectionresponse15.md)
 - [FetchCollectionresponse2](docs/FetchCollectionresponse2.md)
 - [FetchCollectionresponse3](docs/FetchCollectionresponse3.md)
 - [FetchCollectionresponse4](docs/FetchCollectionresponse4.md)
 - [FetchCollectionresponse5](docs/FetchCollectionresponse5.md)
 - [FetchCollectionresponse6](docs/FetchCollectionresponse6.md)
 - [FetchCollectionresponse7](docs/FetchCollectionresponse7.md)
 - [FetchCollectionresponse8](docs/FetchCollectionresponse8.md)
 - [FetchCollectionresponse9](docs/FetchCollectionresponse9.md)
 - [FetchResourceresponse](docs/FetchResourceresponse.md)
 - [FetchResourceresponse1](docs/FetchResourceresponse1.md)
 - [FetchResourceresponse10](docs/FetchResourceresponse10.md)
 - [FetchResourceresponse11](docs/FetchResourceresponse11.md)
 - [FetchResourceresponse12](docs/FetchResourceresponse12.md)
 - [FetchResourceresponse13](docs/FetchResourceresponse13.md)
 - [FetchResourceresponse14](docs/FetchResourceresponse14.md)
 - [FetchResourceresponse15](docs/FetchResourceresponse15.md)
 - [FetchResourceresponse2](docs/FetchResourceresponse2.md)
 - [FetchResourceresponse3](docs/FetchResourceresponse3.md)
 - [FetchResourceresponse4](docs/FetchResourceresponse4.md)
 - [FetchResourceresponse5](docs/FetchResourceresponse5.md)
 - [FetchResourceresponse6](docs/FetchResourceresponse6.md)
 - [FetchResourceresponse7](docs/FetchResourceresponse7.md)
 - [FetchResourceresponse8](docs/FetchResourceresponse8.md)
 - [FetchResourceresponse9](docs/FetchResourceresponse9.md)
 - [FirstMessage](docs/FirstMessage.md)
 - [Followed](docs/Followed.md)
 - [Follower](docs/Follower.md)
 - [Followers](docs/Followers.md)
 - [Following](docs/Following.md)
 - [Follows](docs/Follows.md)
 - [FollowsAttributes](docs/FollowsAttributes.md)
 - [FollowsFetchCollectionresponse](docs/FollowsFetchCollectionresponse.md)
 - [FollowsFetchResourceresponse](docs/FollowsFetchResourceresponse.md)
 - [Franchise](docs/Franchise.md)
 - [Franchises](docs/Franchises.md)
 - [FranchisesAttributes](docs/FranchisesAttributes.md)
 - [FranchisesFetchCollectionresponse](docs/FranchisesFetchCollectionresponse.md)
 - [FranchisesFetchResourceresponse](docs/FranchisesFetchResourceresponse.md)
 - [Genres](docs/Genres.md)
 - [Genres1](docs/Genres1.md)
 - [GenresAttributes](docs/GenresAttributes.md)
 - [Group](docs/Group.md)
 - [GroupActionLogs](docs/GroupActionLogs.md)
 - [GroupActionLogsAttributes](docs/GroupActionLogsAttributes.md)
 - [GroupActionLogsFetchCollectionresponse](docs/GroupActionLogsFetchCollectionresponse.md)
 - [GroupActionLogsFetchResourceresponse](docs/GroupActionLogsFetchResourceresponse.md)
 - [GroupBans](docs/GroupBans.md)
 - [GroupBansAttributes](docs/GroupBansAttributes.md)
 - [GroupBansFetchCollectionresponse](docs/GroupBansFetchCollectionresponse.md)
 - [GroupBansFetchResourceresponse](docs/GroupBansFetchResourceresponse.md)
 - [GroupCategories](docs/GroupCategories.md)
 - [GroupCategoriesAttributes](docs/GroupCategoriesAttributes.md)
 - [GroupCategoriesFetchCollectionresponse](docs/GroupCategoriesFetchCollectionresponse.md)
 - [GroupCategoriesFetchResourceresponse](docs/GroupCategoriesFetchResourceresponse.md)
 - [GroupInvites](docs/GroupInvites.md)
 - [GroupInvitesAttributes](docs/GroupInvitesAttributes.md)
 - [GroupInvitesFetchCollectionresponse](docs/GroupInvitesFetchCollectionresponse.md)
 - [GroupInvitesFetchResourceresponse](docs/GroupInvitesFetchResourceresponse.md)
 - [GroupMember](docs/GroupMember.md)
 - [GroupMemberNotes](docs/GroupMemberNotes.md)
 - [GroupMemberNotesAttributes](docs/GroupMemberNotesAttributes.md)
 - [GroupMemberNotesFetchCollectionresponse](docs/GroupMemberNotesFetchCollectionresponse.md)
 - [GroupMemberNotesFetchResourceresponse](docs/GroupMemberNotesFetchResourceresponse.md)
 - [GroupMembers](docs/GroupMembers.md)
 - [GroupMembersAttributes](docs/GroupMembersAttributes.md)
 - [GroupMembersFetchCollectionresponse](docs/GroupMembersFetchCollectionresponse.md)
 - [GroupMembersFetchResourceresponse](docs/GroupMembersFetchResourceresponse.md)
 - [GroupNeighbors](docs/GroupNeighbors.md)
 - [GroupNeighborsAttributes](docs/GroupNeighborsAttributes.md)
 - [GroupNeighborsFetchCollectionresponse](docs/GroupNeighborsFetchCollectionresponse.md)
 - [GroupNeighborsFetchResourceresponse](docs/GroupNeighborsFetchResourceresponse.md)
 - [GroupPermissions](docs/GroupPermissions.md)
 - [GroupPermissionsAttributes](docs/GroupPermissionsAttributes.md)
 - [GroupPermissionsFetchCollectionresponse](docs/GroupPermissionsFetchCollectionresponse.md)
 - [GroupPermissionsFetchResourceresponse](docs/GroupPermissionsFetchResourceresponse.md)
 - [GroupReports](docs/GroupReports.md)
 - [GroupReportsAttributes](docs/GroupReportsAttributes.md)
 - [GroupReportsFetchCollectionresponse](docs/GroupReportsFetchCollectionresponse.md)
 - [GroupReportsFetchResourceresponse](docs/GroupReportsFetchResourceresponse.md)
 - [GroupTicketMessages](docs/GroupTicketMessages.md)
 - [GroupTicketMessagesAttributes](docs/GroupTicketMessagesAttributes.md)
 - [GroupTicketMessagesFetchCollectionresponse](docs/GroupTicketMessagesFetchCollectionresponse.md)
 - [GroupTicketMessagesFetchResourceresponse](docs/GroupTicketMessagesFetchResourceresponse.md)
 - [GroupTickets](docs/GroupTickets.md)
 - [GroupTicketsAttributes](docs/GroupTicketsAttributes.md)
 - [GroupTicketsFetchCollectionresponse](docs/GroupTicketsFetchCollectionresponse.md)
 - [GroupTicketsFetchResourceresponse](docs/GroupTicketsFetchResourceresponse.md)
 - [Groups](docs/Groups.md)
 - [GroupsAttributes](docs/GroupsAttributes.md)
 - [Image](docs/Image.md)
 - [Image1](docs/Image1.md)
 - [InputFile](docs/InputFile.md)
 - [Installments](docs/Installments.md)
 - [Installments1](docs/Installments1.md)
 - [InstallmentsAttributes](docs/InstallmentsAttributes.md)
 - [InstallmentsFetchCollectionresponse](docs/InstallmentsFetchCollectionresponse.md)
 - [InstallmentsFetchResourceresponse](docs/InstallmentsFetchResourceresponse.md)
 - [Invites](docs/Invites.md)
 - [Item](docs/Item.md)
 - [Kind](docs/Kind.md)
 - [Kind1](docs/Kind1.md)
 - [Large](docs/Large.md)
 - [Large9](docs/Large9.md)
 - [LeaderChatMessages](docs/LeaderChatMessages.md)
 - [LeaderChatMessages2](docs/LeaderChatMessages2.md)
 - [LeaderChatMessagesAttributes](docs/LeaderChatMessagesAttributes.md)
 - [LeaderChatMessagesFetchCollectionresponse](docs/LeaderChatMessagesFetchCollectionresponse.md)
 - [LeaderChatMessagesFetchResourceresponse](docs/LeaderChatMessagesFetchResourceresponse.md)
 - [LibraryEntries](docs/LibraryEntries.md)
 - [LibraryEntries1](docs/LibraryEntries1.md)
 - [LibraryEntriesAttributes](docs/LibraryEntriesAttributes.md)
 - [LibraryEntry](docs/LibraryEntry.md)
 - [LibraryEntryLogs](docs/LibraryEntryLogs.md)
 - [LibraryEntryLogs1](docs/LibraryEntryLogs1.md)
 - [LibraryEntryLogsAttributes](docs/LibraryEntryLogsAttributes.md)
 - [LibraryEntryLogsFetchCollectionresponse](docs/LibraryEntryLogsFetchCollectionresponse.md)
 - [LibraryEntryLogsFetchResourceresponse](docs/LibraryEntryLogsFetchResourceresponse.md)
 - [LibraryEvents](docs/LibraryEvents.md)
 - [LibraryEventsAttributes](docs/LibraryEventsAttributes.md)
 - [LibraryEventsFetchCollectionresponse](docs/LibraryEventsFetchCollectionresponse.md)
 - [LibraryEventsFetchResourceresponse](docs/LibraryEventsFetchResourceresponse.md)
 - [Likes](docs/Likes.md)
 - [LinkedAccount](docs/LinkedAccount.md)
 - [LinkedAccounts](docs/LinkedAccounts.md)
 - [LinkedAccounts1](docs/LinkedAccounts1.md)
 - [LinkedAccountsAttributes](docs/LinkedAccountsAttributes.md)
 - [LinkedAccountsFetchCollectionresponse](docs/LinkedAccountsFetchCollectionresponse.md)
 - [LinkedAccountsFetchResourceresponse](docs/LinkedAccountsFetchResourceresponse.md)
 - [Links](docs/Links.md)
 - [Links1](docs/Links1.md)
 - [Links144](docs/Links144.md)
 - [Links265](docs/Links265.md)
 - [ListImports](docs/ListImports.md)
 - [ListImportsAttributes](docs/ListImportsAttributes.md)
 - [ListImportsFetchCollectionresponse](docs/ListImportsFetchCollectionresponse.md)
 - [ListImportsFetchResourceresponse](docs/ListImportsFetchResourceresponse.md)
 - [Manga](docs/Manga.md)
 - [Manga1](docs/Manga1.md)
 - [MangaAttributes](docs/MangaAttributes.md)
 - [MangaCharacters](docs/MangaCharacters.md)
 - [MangaCharacters1](docs/MangaCharacters1.md)
 - [MangaCharactersAttributes](docs/MangaCharactersAttributes.md)
 - [MangaCharactersFetchCollectionresponse](docs/MangaCharactersFetchCollectionresponse.md)
 - [MangaCharactersFetchResourceresponse](docs/MangaCharactersFetchResourceresponse.md)
 - [MangaMediaAttribute](docs/MangaMediaAttribute.md)
 - [MangaStaff](docs/MangaStaff.md)
 - [MangaStaff1](docs/MangaStaff1.md)
 - [MangaStaffAttributes](docs/MangaStaffAttributes.md)
 - [MangaStaffFetchCollectionresponse](docs/MangaStaffFetchCollectionresponse.md)
 - [MangaStaffFetchResourceresponse](docs/MangaStaffFetchResourceresponse.md)
 - [MangaType](docs/MangaType.md)
 - [Mappings](docs/Mappings.md)
 - [Mappings1](docs/Mappings1.md)
 - [MappingsAttributes](docs/MappingsAttributes.md)
 - [MappingsFetchCollectionresponse](docs/MappingsFetchCollectionresponse.md)
 - [MappingsFetchResourceresponse](docs/MappingsFetchResourceresponse.md)
 - [Media](docs/Media.md)
 - [MediaAttributeVotes](docs/MediaAttributeVotes.md)
 - [MediaAttributeVotesAttributes](docs/MediaAttributeVotesAttributes.md)
 - [MediaAttributeVotesFetchCollectionresponse](docs/MediaAttributeVotesFetchCollectionresponse.md)
 - [MediaAttributeVotesFetchResourceresponse](docs/MediaAttributeVotesFetchResourceresponse.md)
 - [MediaAttributes](docs/MediaAttributes.md)
 - [MediaAttributesAttributes](docs/MediaAttributesAttributes.md)
 - [MediaAttributesFetchCollectionresponse](docs/MediaAttributesFetchCollectionresponse.md)
 - [MediaAttributesFetchResourceresponse](docs/MediaAttributesFetchResourceresponse.md)
 - [MediaFollows](docs/MediaFollows.md)
 - [MediaReaction](docs/MediaReaction.md)
 - [MediaReactionVotes](docs/MediaReactionVotes.md)
 - [MediaReactionVotesAttributes](docs/MediaReactionVotesAttributes.md)
 - [MediaReactions](docs/MediaReactions.md)
 - [MediaReactionsAttributes](docs/MediaReactionsAttributes.md)
 - [MediaReactionsFetchCollectionresponse](docs/MediaReactionsFetchCollectionresponse.md)
 - [MediaReactionsFetchResourceresponse](docs/MediaReactionsFetchResourceresponse.md)
 - [MediaRelationships](docs/MediaRelationships.md)
 - [MediaRelationships1](docs/MediaRelationships1.md)
 - [MediaRelationshipsAttributes](docs/MediaRelationshipsAttributes.md)
 - [Medium](docs/Medium.md)
 - [Medium5](docs/Medium5.md)
 - [Members](docs/Members.md)
 - [Messages](docs/Messages.md)
 - [Meta](docs/Meta.md)
 - [Meta1](docs/Meta1.md)
 - [Meta12](docs/Meta12.md)
 - [Meta13](docs/Meta13.md)
 - [Meta14](docs/Meta14.md)
 - [Meta39](docs/Meta39.md)
 - [Meta5](docs/Meta5.md)
 - [Moderator](docs/Moderator.md)
 - [Naughty](docs/Naughty.md)
 - [Neighbors](docs/Neighbors.md)
 - [NextUnit](docs/NextUnit.md)
 - [Notes](docs/Notes.md)
 - [NotificationSettings](docs/NotificationSettings.md)
 - [OneSignalPlayers](docs/OneSignalPlayers.md)
 - [Parent](docs/Parent.md)
 - [People](docs/People.md)
 - [PeopleAttributes](docs/PeopleAttributes.md)
 - [PeopleFetchCollectionresponse](docs/PeopleFetchCollectionresponse.md)
 - [PeopleFetchResourceresponse](docs/PeopleFetchResourceresponse.md)
 - [Permission](docs/Permission.md)
 - [Permissions](docs/Permissions.md)
 - [Person](docs/Person.md)
 - [PinnedPost](docs/PinnedPost.md)
 - [Post](docs/Post.md)
 - [PostFollows](docs/PostFollows.md)
 - [PostFollowsAttributes](docs/PostFollowsAttributes.md)
 - [PostFollowsFetchCollectionresponse](docs/PostFollowsFetchCollectionresponse.md)
 - [PostFollowsFetchResourceresponse](docs/PostFollowsFetchResourceresponse.md)
 - [PostLikes](docs/PostLikes.md)
 - [PostLikes1](docs/PostLikes1.md)
 - [PostLikesAttributes](docs/PostLikesAttributes.md)
 - [PostLikesFetchCollectionresponse](docs/PostLikesFetchCollectionresponse.md)
 - [PostLikesFetchResourceresponse](docs/PostLikesFetchResourceresponse.md)
 - [PosterImage](docs/PosterImage.md)
 - [Posts](docs/Posts.md)
 - [PostsAttributes](docs/PostsAttributes.md)
 - [PrimaryMedia](docs/PrimaryMedia.md)
 - [Privacy](docs/Privacy.md)
 - [Producer](docs/Producer.md)
 - [Producers](docs/Producers.md)
 - [ProducersAttributes](docs/ProducersAttributes.md)
 - [ProducersFetchCollectionresponse](docs/ProducersFetchCollectionresponse.md)
 - [ProducersFetchResourceresponse](docs/ProducersFetchResourceresponse.md)
 - [ProfileLinkSite](docs/ProfileLinkSite.md)
 - [ProfileLinkSites](docs/ProfileLinkSites.md)
 - [ProfileLinkSitesAttributes](docs/ProfileLinkSitesAttributes.md)
 - [ProfileLinkSitesFetchCollectionresponse](docs/ProfileLinkSitesFetchCollectionresponse.md)
 - [ProfileLinkSitesFetchResourceresponse](docs/ProfileLinkSitesFetchResourceresponse.md)
 - [ProfileLinks](docs/ProfileLinks.md)
 - [ProfileLinks1](docs/ProfileLinks1.md)
 - [ProfileLinksAttributes](docs/ProfileLinksAttributes.md)
 - [ProfileLinksFetchCollectionresponse](docs/ProfileLinksFetchCollectionresponse.md)
 - [ProfileLinksFetchResourceresponse](docs/ProfileLinksFetchResourceresponse.md)
 - [Rank](docs/Rank.md)
 - [RatingSystem](docs/RatingSystem.md)
 - [ReactionSkipped](docs/ReactionSkipped.md)
 - [Reason](docs/Reason.md)
 - [Relationships](docs/Relationships.md)
 - [Relationships1](docs/Relationships1.md)
 - [Relationships10](docs/Relationships10.md)
 - [Relationships11](docs/Relationships11.md)
 - [Relationships12](docs/Relationships12.md)
 - [Relationships13](docs/Relationships13.md)
 - [Relationships14](docs/Relationships14.md)
 - [Relationships15](docs/Relationships15.md)
 - [Relationships16](docs/Relationships16.md)
 - [Relationships17](docs/Relationships17.md)
 - [Relationships18](docs/Relationships18.md)
 - [Relationships19](docs/Relationships19.md)
 - [Relationships2](docs/Relationships2.md)
 - [Relationships20](docs/Relationships20.md)
 - [Relationships21](docs/Relationships21.md)
 - [Relationships22](docs/Relationships22.md)
 - [Relationships23](docs/Relationships23.md)
 - [Relationships24](docs/Relationships24.md)
 - [Relationships25](docs/Relationships25.md)
 - [Relationships26](docs/Relationships26.md)
 - [Relationships27](docs/Relationships27.md)
 - [Relationships28](docs/Relationships28.md)
 - [Relationships29](docs/Relationships29.md)
 - [Relationships3](docs/Relationships3.md)
 - [Relationships30](docs/Relationships30.md)
 - [Relationships31](docs/Relationships31.md)
 - [Relationships32](docs/Relationships32.md)
 - [Relationships33](docs/Relationships33.md)
 - [Relationships35](docs/Relationships35.md)
 - [Relationships36](docs/Relationships36.md)
 - [Relationships37](docs/Relationships37.md)
 - [Relationships38](docs/Relationships38.md)
 - [Relationships39](docs/Relationships39.md)
 - [Relationships4](docs/Relationships4.md)
 - [Relationships41](docs/Relationships41.md)
 - [Relationships42](docs/Relationships42.md)
 - [Relationships43](docs/Relationships43.md)
 - [Relationships44](docs/Relationships44.md)
 - [Relationships45](docs/Relationships45.md)
 - [Relationships46](docs/Relationships46.md)
 - [Relationships47](docs/Relationships47.md)
 - [Relationships48](docs/Relationships48.md)
 - [Relationships49](docs/Relationships49.md)
 - [Relationships5](docs/Relationships5.md)
 - [Relationships50](docs/Relationships50.md)
 - [Relationships52](docs/Relationships52.md)
 - [Relationships53](docs/Relationships53.md)
 - [Relationships54](docs/Relationships54.md)
 - [Relationships55](docs/Relationships55.md)
 - [Relationships56](docs/Relationships56.md)
 - [Relationships57](docs/Relationships57.md)
 - [Relationships59](docs/Relationships59.md)
 - [Relationships6](docs/Relationships6.md)
 - [Relationships60](docs/Relationships60.md)
 - [Relationships7](docs/Relationships7.md)
 - [Relationships8](docs/Relationships8.md)
 - [Relationships9](docs/Relationships9.md)
 - [Replies](docs/Replies.md)
 - [Reports](docs/Reports.md)
 - [Reports2](docs/Reports2.md)
 - [ReportsAttributes](docs/ReportsAttributes.md)
 - [Resource](docs/Resource.md)
 - [Review](docs/Review.md)
 - [ReviewLikes](docs/ReviewLikes.md)
 - [ReviewLikesAttributes](docs/ReviewLikesAttributes.md)
 - [ReviewLikesFetchCollectionresponse](docs/ReviewLikesFetchCollectionresponse.md)
 - [ReviewLikesFetchResourceresponse](docs/ReviewLikesFetchResourceresponse.md)
 - [Reviews](docs/Reviews.md)
 - [Reviews1](docs/Reviews1.md)
 - [ReviewsAttributes](docs/ReviewsAttributes.md)
 - [ReviewsFetchCollectionresponse](docs/ReviewsFetchCollectionresponse.md)
 - [ReviewsFetchResourceresponse](docs/ReviewsFetchResourceresponse.md)
 - [Role](docs/Role.md)
 - [Role1](docs/Role1.md)
 - [Role2](docs/Role2.md)
 - [Role3](docs/Role3.md)
 - [Role4](docs/Role4.md)
 - [Roles](docs/Roles.md)
 - [RolesAttributes](docs/RolesAttributes.md)
 - [RolesFetchCollectionresponse](docs/RolesFetchCollectionresponse.md)
 - [RolesFetchResourceresponse](docs/RolesFetchResourceresponse.md)
 - [Sender](docs/Sender.md)
 - [ShowType](docs/ShowType.md)
 - [SiteAnnouncements](docs/SiteAnnouncements.md)
 - [SiteAnnouncementsAttributes](docs/SiteAnnouncementsAttributes.md)
 - [Small](docs/Small.md)
 - [Small10](docs/Small10.md)
 - [Small9](docs/Small9.md)
 - [Source](docs/Source.md)
 - [Source2](docs/Source2.md)
 - [SpoiledUnit](docs/SpoiledUnit.md)
 - [Stats](docs/Stats.md)
 - [Stats1](docs/Stats1.md)
 - [StatsAttributes](docs/StatsAttributes.md)
 - [StatsData](docs/StatsData.md)
 - [StatsFetchCollectionresponse](docs/StatsFetchCollectionresponse.md)
 - [StatsFetchResourceresponse](docs/StatsFetchResourceresponse.md)
 - [Status](docs/Status.md)
 - [Status1](docs/Status1.md)
 - [Status2](docs/Status2.md)
 - [Status5](docs/Status5.md)
 - [Status6](docs/Status6.md)
 - [Status9](docs/Status9.md)
 - [StatusCounts](docs/StatusCounts.md)
 - [Strategy](docs/Strategy.md)
 - [Streamer](docs/Streamer.md)
 - [Streamers](docs/Streamers.md)
 - [StreamersAttributes](docs/StreamersAttributes.md)
 - [StreamingLinks](docs/StreamingLinks.md)
 - [StreamingLinks1](docs/StreamingLinks1.md)
 - [StreamingLinksAttributes](docs/StreamingLinksAttributes.md)
 - [StreamingLinksFetchCollectionresponse](docs/StreamingLinksFetchCollectionresponse.md)
 - [StreamingLinksFetchResourceresponse](docs/StreamingLinksFetchResourceresponse.md)
 - [Subtype](docs/Subtype.md)
 - [Subtype1](docs/Subtype1.md)
 - [Subtype2](docs/Subtype2.md)
 - [SyncStatus](docs/SyncStatus.md)
 - [Target](docs/Target.md)
 - [TargetGroup](docs/TargetGroup.md)
 - [TargetUser](docs/TargetUser.md)
 - [Theme](docs/Theme.md)
 - [Thumbnail](docs/Thumbnail.md)
 - [Ticket](docs/Ticket.md)
 - [Tickets](docs/Tickets.md)
 - [Tiny](docs/Tiny.md)
 - [Tiny9](docs/Tiny9.md)
 - [TitleLanguagePreferencecanonical](docs/TitleLanguagePreferencecanonical.md)
 - [Titles](docs/Titles.md)
 - [Titles1](docs/Titles1.md)
 - [Titles4](docs/Titles4.md)
 - [Token](docs/Token.md)
 - [TrendingAnimeFetchCollectionresponse](docs/TrendingAnimeFetchCollectionresponse.md)
 - [TrendingMangaFetchCollectionresponse](docs/TrendingMangaFetchCollectionresponse.md)
 - [Unit](docs/Unit.md)
 - [Uploads](docs/Uploads.md)
 - [User](docs/User.md)
 - [UserRoles](docs/UserRoles.md)
 - [UserRoles2](docs/UserRoles2.md)
 - [UserRolesAttributes](docs/UserRolesAttributes.md)
 - [UserRolesFetchCollectionresponse](docs/UserRolesFetchCollectionresponse.md)
 - [UserRolesFetchResourceresponse](docs/UserRolesFetchResourceresponse.md)
 - [Users](docs/Users.md)
 - [UsersAttributes](docs/UsersAttributes.md)
 - [UsersFetchCollectionresponse](docs/UsersFetchCollectionresponse.md)
 - [UsersFetchResourceresponse](docs/UsersFetchResourceresponse.md)
 - [Videos](docs/Videos.md)
 - [Votes](docs/Votes.md)
 - [Waifu](docs/Waifu.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



