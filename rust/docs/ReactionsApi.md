# \ReactionsApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_resource1234**](ReactionsApi.md#delete_resource1234) | **delete** /media-reaction-votes/{id} | Delete Resource
[**get_fetch_collection1234567**](ReactionsApi.md#get_fetch_collection1234567) | **get** /media-reaction-votes | Fetch Collection
[**get_fetch_resource1234567**](ReactionsApi.md#get_fetch_resource1234567) | **get** /media-reaction-votes/{id} | Fetch Resource
[**media_reactions_create_resource**](ReactionsApi.md#media_reactions_create_resource) | **post** /media-reactions | Media Reactions_Create Resource
[**media_reactions_delete_resource**](ReactionsApi.md#media_reactions_delete_resource) | **delete** /media-reactions/{id} | Media Reactions_Delete Resource
[**media_reactions_fetch_collection**](ReactionsApi.md#media_reactions_fetch_collection) | **get** /media-reactions | Media Reactions_Fetch Collection
[**media_reactions_fetch_resource**](ReactionsApi.md#media_reactions_fetch_resource) | **get** /media-reactions/{id} | Media Reactions_Fetch Resource
[**media_reactions_update_resource**](ReactionsApi.md#media_reactions_update_resource) | **patch** /media-reactions/{id} | Media Reactions_Update Resource
[**patch_update_resource123**](ReactionsApi.md#patch_update_resource123) | **patch** /media-reaction-votes/{id} | Update Resource
[**post_create_resource123**](ReactionsApi.md#post_create_resource123) | **post** /media-reaction-votes | Create Resource
[**review_likes_create_resource**](ReactionsApi.md#review_likes_create_resource) | **post** /review-likes | Review Likes_Create Resource
[**review_likes_delete_resource**](ReactionsApi.md#review_likes_delete_resource) | **delete** /review-likes/{id} | Review Likes_Delete Resource
[**review_likes_fetch_collection**](ReactionsApi.md#review_likes_fetch_collection) | **get** /review-likes | Review Likes_Fetch Collection
[**review_likes_fetch_resource**](ReactionsApi.md#review_likes_fetch_resource) | **get** /review-likes/{id} | Review Likes_Fetch Resource
[**review_likes_update_resource**](ReactionsApi.md#review_likes_update_resource) | **patch** /review-likes/{id} | Review Likes_Update Resource
[**reviews_create_resource**](ReactionsApi.md#reviews_create_resource) | **post** /reviews | Reviews_Create Resource
[**reviews_delete_resource**](ReactionsApi.md#reviews_delete_resource) | **delete** /reviews/{id} | Reviews_Delete Resource
[**reviews_fetch_collection**](ReactionsApi.md#reviews_fetch_collection) | **get** /reviews | Reviews_Fetch Collection
[**reviews_fetch_resource**](ReactionsApi.md#reviews_fetch_resource) | **get** /reviews/{id} | Reviews_Fetch Resource
[**reviews_update_resource**](ReactionsApi.md#reviews_update_resource) | **patch** /reviews/{id} | Reviews_Update Resource



## delete_resource1234

> delete_resource1234(id, content_type)
Delete Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaReactionId` | `1`, `24334` `userId` | `42603`, `2,7`

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


## get_fetch_collection1234567

> crate::models::FetchCollectionresponse8 get_fetch_collection1234567(id)
Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaReactionId` | `1`, `24334` `userId` | `42603`, `2,7`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchCollectionresponse8**](FetchCollectionresponse8.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_resource1234567

> crate::models::FetchResourceresponse8 get_fetch_resource1234567(id)
Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaReactionId` | `1`, `24334` `userId` | `42603`, `2,7`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchResourceresponse8**](FetchResourceresponse8.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_reactions_create_resource

> media_reactions_create_resource(id, content_type)
Media Reactions_Create Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `upVotesCount` | `2`, `7` `userId` | `42603`, `2,7` `animeId` | `1`, `43,2,300` `dramaId` | `mangaId` | `2`, `8,987` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind`

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


## media_reactions_delete_resource

> media_reactions_delete_resource(id, content_type)
Media Reactions_Delete Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `upVotesCount` | `2`, `7` `userId` | `42603`, `2,7` `animeId` | `1`, `43,2,300` `dramaId` | `mangaId` | `2`, `8,987` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind`

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


## media_reactions_fetch_collection

> crate::models::MediaReactionsFetchCollectionresponse media_reactions_fetch_collection(id)
Media Reactions_Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `upVotesCount` | `2`, `7` `userId` | `42603`, `2,7` `animeId` | `1`, `43,2,300` `dramaId` | `mangaId` | `2`, `8,987` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::MediaReactionsFetchCollectionresponse**](MediaReactions_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_reactions_fetch_resource

> crate::models::MediaReactionsFetchResourceresponse media_reactions_fetch_resource(id)
Media Reactions_Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `upVotesCount` | `2`, `7` `userId` | `42603`, `2,7` `animeId` | `1`, `43,2,300` `dramaId` | `mangaId` | `2`, `8,987` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::MediaReactionsFetchResourceresponse**](MediaReactions_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_reactions_update_resource

> media_reactions_update_resource(id, content_type)
Media Reactions_Update Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `createdAt` | `upVotesCount` | `2`, `7` `userId` | `42603`, `2,7` `animeId` | `1`, `43,2,300` `dramaId` | `mangaId` | `2`, `8,987` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind`

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


## patch_update_resource123

> patch_update_resource123(id, content_type)
Update Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaReactionId` | `1`, `24334` `userId` | `42603`, `2,7`

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


## post_create_resource123

> post_create_resource123(id, content_type)
Create Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaReactionId` | `1`, `24334` `userId` | `42603`, `2,7`

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


## review_likes_create_resource

> review_likes_create_resource(id, content_type)
Review Likes_Create Resource

**Status**: Deprecated. Use `media-reaction-votes` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `reviewId` | `4`, `14792` `userId` | `42603`, `2,7`

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


## review_likes_delete_resource

> review_likes_delete_resource(id, content_type)
Review Likes_Delete Resource

**Status**: Deprecated. Use `media-reaction-votes` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `reviewId` | `4`, `14792` `userId` | `42603`, `2,7`

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


## review_likes_fetch_collection

> crate::models::ReviewLikesFetchCollectionresponse review_likes_fetch_collection(id)
Review Likes_Fetch Collection

**Status**: Deprecated. Use `media-reaction-votes` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `reviewId` | `4`, `14792` `userId` | `42603`, `2,7`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::ReviewLikesFetchCollectionresponse**](ReviewLikes_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## review_likes_fetch_resource

> crate::models::ReviewLikesFetchResourceresponse review_likes_fetch_resource(id)
Review Likes_Fetch Resource

**Status**: Deprecated. Use `media-reaction-votes` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `reviewId` | `4`, `14792` `userId` | `42603`, `2,7`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::ReviewLikesFetchResourceresponse**](ReviewLikes_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## review_likes_update_resource

> review_likes_update_resource(id, content_type)
Review Likes_Update Resource

**Status**: Deprecated. Use `media-reaction-votes` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | N     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `reviewId` | `4`, `14792` `userId` | `42603`, `2,7`

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


## reviews_create_resource

> reviews_create_resource(id, content_type)
Reviews_Create Resource

**Status**: Deprecated. Use `media-reactions` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `userId` | `42603`, `2,7`

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


## reviews_delete_resource

> reviews_delete_resource(id, content_type)
Reviews_Delete Resource

**Status**: Deprecated. Use `media-reactions` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `userId` | `42603`, `2,7`

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


## reviews_fetch_collection

> crate::models::ReviewsFetchCollectionresponse reviews_fetch_collection(id)
Reviews_Fetch Collection

**Status**: Deprecated. Use `media-reactions` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `userId` | `42603`, `2,7`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::ReviewsFetchCollectionresponse**](Reviews_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reviews_fetch_resource

> crate::models::ReviewsFetchResourceresponse reviews_fetch_resource(id)
Reviews_Fetch Resource

**Status**: Deprecated. Use `media-reactions` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `userId` | `42603`, `2,7`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::ReviewsFetchResourceresponse**](Reviews_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reviews_update_resource

> reviews_update_resource(id, content_type)
Reviews_Update Resource

**Status**: Deprecated. Use `media-reactions` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `userId` | `42603`, `2,7`

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

