# \MediaFollowsApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_resource1**](MediaFollowsApi.md#delete_resource1) | **delete** /media-follows/{id} | Delete Resource
[**get_fetch_collection123**](MediaFollowsApi.md#get_fetch_collection123) | **get** /media-follows | Fetch Collection
[**get_fetch_resource123**](MediaFollowsApi.md#get_fetch_resource123) | **get** /media-follows/{id} | Fetch Resource
[**media_attribute_votes_create_resource**](MediaFollowsApi.md#media_attribute_votes_create_resource) | **post** /media-attribute-votes | Media Attribute Votes_Create Resource
[**media_attribute_votes_delete_resource**](MediaFollowsApi.md#media_attribute_votes_delete_resource) | **delete** /media-attribute-votes/{id} | Media Attribute Votes_Delete Resource
[**media_attribute_votes_fetch_collection**](MediaFollowsApi.md#media_attribute_votes_fetch_collection) | **get** /media-attribute-votes | Media Attribute Votes_Fetch Collection
[**media_attribute_votes_fetch_resource**](MediaFollowsApi.md#media_attribute_votes_fetch_resource) | **get** /media-attribute-votes/{id} | Media Attribute Votes_Fetch Resource
[**media_attribute_votes_update_resource**](MediaFollowsApi.md#media_attribute_votes_update_resource) | **patch** /media-attribute-votes/{id} | Media Attribute Votes_Update Resource
[**media_attributes_fetch_collection**](MediaFollowsApi.md#media_attributes_fetch_collection) | **get** /media-attributes | Media Attributes_Fetch Collection
[**media_attributes_fetch_resource**](MediaFollowsApi.md#media_attributes_fetch_resource) | **get** /media-attributes/{id} | Media Attributes_Fetch Resource
[**patch_update_resource**](MediaFollowsApi.md#patch_update_resource) | **patch** /media-follows/{id} | Update Resource
[**post_create_resource**](MediaFollowsApi.md#post_create_resource) | **post** /media-follows | Create Resource



## delete_resource1

> delete_resource1(id, content_type)
Delete Resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |
**content_type** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_collection123

> crate::models::FetchCollectionresponse4 get_fetch_collection123(id)
Fetch Collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchCollectionresponse4**](FetchCollectionresponse4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_resource123

> crate::models::FetchResourceresponse4 get_fetch_resource123(id)
Fetch Resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchResourceresponse4**](FetchResourceresponse4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_attribute_votes_create_resource

> media_attribute_votes_create_resource(id, content_type)
Media Attribute Votes_Create Resource

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `userId` | `42603`, `2,7` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |
**content_type** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_attribute_votes_delete_resource

> media_attribute_votes_delete_resource(id, content_type)
Media Attribute Votes_Delete Resource

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `userId` | `42603`, `2,7` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |
**content_type** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_attribute_votes_fetch_collection

> crate::models::MediaAttributeVotesFetchCollectionresponse media_attribute_votes_fetch_collection(id)
Media Attribute Votes_Fetch Collection

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `userId` | `42603`, `2,7` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::MediaAttributeVotesFetchCollectionresponse**](MediaAttributeVotes_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_attribute_votes_fetch_resource

> crate::models::MediaAttributeVotesFetchResourceresponse media_attribute_votes_fetch_resource(id)
Media Attribute Votes_Fetch Resource

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `userId` | `42603`, `2,7` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::MediaAttributeVotesFetchResourceresponse**](MediaAttributeVotes_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_attribute_votes_update_resource

> media_attribute_votes_update_resource(id, content_type)
Media Attribute Votes_Update Resource

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `userId` | `42603`, `2,7` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |
**content_type** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_attributes_fetch_collection

> crate::models::MediaAttributesFetchCollectionresponse media_attributes_fetch_collection(id)
Media Attributes_Fetch Collection

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `slug` | |  **Pagination**  The maximum page limit is unlimited for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::MediaAttributesFetchCollectionresponse**](MediaAttributes_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_attributes_fetch_resource

> crate::models::MediaAttributesFetchResourceresponse media_attributes_fetch_resource(id)
Media Attributes_Fetch Resource

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `slug` | |  **Pagination**  The maximum page limit is unlimited for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::MediaAttributesFetchResourceresponse**](MediaAttributes_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_update_resource

> patch_update_resource(id, content_type)
Update Resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |
**content_type** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_create_resource

> post_create_resource(id, content_type)
Create Resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |
**content_type** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

