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


/// struct for typed errors of method `delete_resource123456789`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteResource123456789Error {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_fetch_collection123456789101112`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFetchCollection123456789101112Error {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_fetch_resource123456789101112`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFetchResource123456789101112Error {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_action_logs_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupActionLogsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_action_logs_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupActionLogsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_bans_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupBansCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_bans_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupBansDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_bans_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupBansFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_bans_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupBansFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_categories_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupCategoriesCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_categories_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupCategoriesDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_categories_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupCategoriesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_categories_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupCategoriesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_categories_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupCategoriesUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_invites_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupInvitesCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_invites_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupInvitesDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_invites_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupInvitesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_invites_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupInvitesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_invites_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupInvitesUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_member_notes_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMemberNotesCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_member_notes_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMemberNotesDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_member_notes_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMemberNotesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_member_notes_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMemberNotesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_member_notes_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMemberNotesUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_members_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMembersCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_members_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMembersDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_members_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMembersFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_members_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMembersFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_members_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMembersUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_neighbors_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupNeighborsCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_neighbors_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupNeighborsDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_neighbors_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupNeighborsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_neighbors_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupNeighborsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_permissions_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupPermissionsCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_permissions_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupPermissionsDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_permissions_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupPermissionsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_permissions_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupPermissionsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_reports_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupReportsCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_reports_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupReportsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_reports_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupReportsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_reports_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupReportsUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_ticket_messages_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTicketMessagesCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_ticket_messages_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTicketMessagesDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_ticket_messages_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTicketMessagesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_ticket_messages_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTicketMessagesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_ticket_messages_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTicketMessagesUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_tickets_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTicketsCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_tickets_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTicketsFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_tickets_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTicketsFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `group_tickets_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTicketsUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `leader_chat_messages_create_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaderChatMessagesCreateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `leader_chat_messages_delete_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaderChatMessagesDeleteResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `leader_chat_messages_fetch_collection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaderChatMessagesFetchCollectionError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `leader_chat_messages_fetch_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaderChatMessagesFetchResourceError {
    Status400(crate::models::Error406),
    Status404(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `leader_chat_messages_update_resource`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaderChatMessagesUpdateResourceError {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `patch_update_resource12345678`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchUpdateResource12345678Error {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `post_create_resource12345678`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateResource12345678Error {
    Status400(crate::models::Error406),
    Status401(crate::models::Error406),
    Status403(crate::models::Error406),
    Status404(crate::models::Error406),
    Status409(crate::models::Error406),
    Status500(crate::models::Error406),
    UnknownValue(serde_json::Value),
}


    pub async fn delete_resource123456789(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<DeleteResource123456789Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/groups/{id}", configuration.base_path, id=id);
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
            let entity: Option<DeleteResource123456789Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_fetch_collection123456789101112(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FetchCollectionresponse13, Error<GetFetchCollection123456789101112Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/groups", configuration.base_path);
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
            let entity: Option<GetFetchCollection123456789101112Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_fetch_resource123456789101112(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::FetchResourceresponse13, Error<GetFetchResource123456789101112Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/groups/{id}", configuration.base_path, id=id);
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
            let entity: Option<GetFetchResource123456789101112Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_action_logs_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupActionLogsFetchCollectionresponse, Error<GroupActionLogsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-action-logs", configuration.base_path);
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
            let entity: Option<GroupActionLogsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_action_logs_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupActionLogsFetchResourceresponse, Error<GroupActionLogsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-action-logs/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupActionLogsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_bans_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupBansCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-bans", configuration.base_path);
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
            let entity: Option<GroupBansCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_bans_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupBansDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-bans/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupBansDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_bans_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupBansFetchCollectionresponse, Error<GroupBansFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-bans", configuration.base_path);
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
            let entity: Option<GroupBansFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_bans_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupBansFetchResourceresponse, Error<GroupBansFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-bans/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupBansFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_categories_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupCategoriesCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-categories", configuration.base_path);
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
            let entity: Option<GroupCategoriesCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_categories_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupCategoriesDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-categories/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupCategoriesDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_categories_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupCategoriesFetchCollectionresponse, Error<GroupCategoriesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-categories", configuration.base_path);
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
            let entity: Option<GroupCategoriesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_categories_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupCategoriesFetchResourceresponse, Error<GroupCategoriesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-categories/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupCategoriesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_categories_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupCategoriesUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-categories/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupCategoriesUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_invites_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupInvitesCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-invites", configuration.base_path);
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
            let entity: Option<GroupInvitesCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_invites_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupInvitesDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-invites/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupInvitesDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_invites_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupInvitesFetchCollectionresponse, Error<GroupInvitesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-invites", configuration.base_path);
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
            let entity: Option<GroupInvitesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_invites_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupInvitesFetchResourceresponse, Error<GroupInvitesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-invites/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupInvitesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_invites_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupInvitesUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-invites/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupInvitesUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_member_notes_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupMemberNotesCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-member-notes", configuration.base_path);
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
            let entity: Option<GroupMemberNotesCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_member_notes_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupMemberNotesDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-member-notes/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupMemberNotesDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_member_notes_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupMemberNotesFetchCollectionresponse, Error<GroupMemberNotesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-member-notes", configuration.base_path);
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
            let entity: Option<GroupMemberNotesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_member_notes_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupMemberNotesFetchResourceresponse, Error<GroupMemberNotesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-member-notes/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupMemberNotesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_member_notes_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupMemberNotesUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-member-notes/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupMemberNotesUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_members_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupMembersCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-members", configuration.base_path);
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
            let entity: Option<GroupMembersCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_members_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupMembersDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-members/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupMembersDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_members_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupMembersFetchCollectionresponse, Error<GroupMembersFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-members", configuration.base_path);
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
            let entity: Option<GroupMembersFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_members_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupMembersFetchResourceresponse, Error<GroupMembersFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-members/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupMembersFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_members_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupMembersUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-members/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupMembersUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_neighbors_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupNeighborsCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-neighbors", configuration.base_path);
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
            let entity: Option<GroupNeighborsCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_neighbors_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupNeighborsDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-neighbors/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupNeighborsDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_neighbors_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupNeighborsFetchCollectionresponse, Error<GroupNeighborsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-neighbors", configuration.base_path);
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
            let entity: Option<GroupNeighborsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_neighbors_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupNeighborsFetchResourceresponse, Error<GroupNeighborsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-neighbors/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupNeighborsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_permissions_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupPermissionsCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-permissions", configuration.base_path);
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
            let entity: Option<GroupPermissionsCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_permissions_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupPermissionsDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-permissions/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupPermissionsDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_permissions_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupPermissionsFetchCollectionresponse, Error<GroupPermissionsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-permissions", configuration.base_path);
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
            let entity: Option<GroupPermissionsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_permissions_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupPermissionsFetchResourceresponse, Error<GroupPermissionsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-permissions/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupPermissionsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_reports_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupReportsCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-reports", configuration.base_path);
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
            let entity: Option<GroupReportsCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_reports_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupReportsFetchCollectionresponse, Error<GroupReportsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-reports", configuration.base_path);
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
            let entity: Option<GroupReportsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_reports_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupReportsFetchResourceresponse, Error<GroupReportsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-reports/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupReportsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_reports_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupReportsUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-reports/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupReportsUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_ticket_messages_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupTicketMessagesCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-ticket-messages", configuration.base_path);
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
            let entity: Option<GroupTicketMessagesCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_ticket_messages_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupTicketMessagesDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-ticket-messages/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupTicketMessagesDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_ticket_messages_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupTicketMessagesFetchCollectionresponse, Error<GroupTicketMessagesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-ticket-messages", configuration.base_path);
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
            let entity: Option<GroupTicketMessagesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_ticket_messages_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupTicketMessagesFetchResourceresponse, Error<GroupTicketMessagesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-ticket-messages/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupTicketMessagesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_ticket_messages_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupTicketMessagesUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-ticket-messages/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupTicketMessagesUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_tickets_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupTicketsCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-tickets", configuration.base_path);
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
            let entity: Option<GroupTicketsCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_tickets_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupTicketsFetchCollectionresponse, Error<GroupTicketsFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-tickets", configuration.base_path);
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
            let entity: Option<GroupTicketsFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_tickets_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::GroupTicketsFetchResourceresponse, Error<GroupTicketsFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-tickets/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupTicketsFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn group_tickets_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<GroupTicketsUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/group-tickets/{id}", configuration.base_path, id=id);
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
            let entity: Option<GroupTicketsUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn leader_chat_messages_create_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<LeaderChatMessagesCreateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/leader-chat-messages", configuration.base_path);
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
            let entity: Option<LeaderChatMessagesCreateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn leader_chat_messages_delete_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<LeaderChatMessagesDeleteResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/leader-chat-messages/{id}", configuration.base_path, id=id);
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
            let entity: Option<LeaderChatMessagesDeleteResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn leader_chat_messages_fetch_collection(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::LeaderChatMessagesFetchCollectionresponse, Error<LeaderChatMessagesFetchCollectionError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/leader-chat-messages", configuration.base_path);
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
            let entity: Option<LeaderChatMessagesFetchCollectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn leader_chat_messages_fetch_resource(configuration: &configuration::Configuration, id: f64) -> Result<crate::models::LeaderChatMessagesFetchResourceresponse, Error<LeaderChatMessagesFetchResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/leader-chat-messages/{id}", configuration.base_path, id=id);
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
            let entity: Option<LeaderChatMessagesFetchResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn leader_chat_messages_update_resource(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<LeaderChatMessagesUpdateResourceError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/leader-chat-messages/{id}", configuration.base_path, id=id);
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
            let entity: Option<LeaderChatMessagesUpdateResourceError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn patch_update_resource12345678(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<PatchUpdateResource12345678Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/groups/{id}", configuration.base_path, id=id);
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
            let entity: Option<PatchUpdateResource12345678Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn post_create_resource12345678(configuration: &configuration::Configuration, id: f64, content_type: &str) -> Result<(), Error<PostCreateResource12345678Error>> {
        let client = &configuration.client;

        let uri_str = format!("{}/groups", configuration.base_path);
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
            let entity: Option<PostCreateResource12345678Error> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

