# \UserLibrariesApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_resource123**](UserLibrariesApi.md#delete_resource123) | **delete** /library-entries/{id} | Delete Resource
[**get_fetch_collection123456**](UserLibrariesApi.md#get_fetch_collection123456) | **get** /library-entries | Fetch Collection
[**get_fetch_resource123456**](UserLibrariesApi.md#get_fetch_resource123456) | **get** /library-entries/{id} | Fetch Resource
[**library_entry_logs_create_resource**](UserLibrariesApi.md#library_entry_logs_create_resource) | **post** /library-entry-logs | Library Entry Logs_Create Resource
[**library_entry_logs_delete_resource**](UserLibrariesApi.md#library_entry_logs_delete_resource) | **delete** /library-entry-logs/{id} | Library Entry Logs_Delete Resource
[**library_entry_logs_fetch_collection**](UserLibrariesApi.md#library_entry_logs_fetch_collection) | **get** /library-entry-logs | Library Entry Logs_Fetch Collection
[**library_entry_logs_fetch_resource**](UserLibrariesApi.md#library_entry_logs_fetch_resource) | **get** /library-entry-logs/{id} | Library Entry Logs_Fetch Resource
[**library_entry_logs_update_resource**](UserLibrariesApi.md#library_entry_logs_update_resource) | **patch** /library-entry-logs/{id} | Library Entry Logs_Update Resource
[**library_events_delete_resource**](UserLibrariesApi.md#library_events_delete_resource) | **delete** /library-events/{id} | Library Events_Delete Resource
[**library_events_fetch_collection**](UserLibrariesApi.md#library_events_fetch_collection) | **get** /library-events | Library Events_Fetch Collection
[**library_events_fetch_resource**](UserLibrariesApi.md#library_events_fetch_resource) | **get** /library-events/{id} | Library Events_Fetch Resource
[**list_imports_create_resource**](UserLibrariesApi.md#list_imports_create_resource) | **post** /list-imports | List Imports_Create Resource
[**list_imports_fetch_collection**](UserLibrariesApi.md#list_imports_fetch_collection) | **get** /list-imports | List Imports_Fetch Collection
[**list_imports_fetch_resource**](UserLibrariesApi.md#list_imports_fetch_resource) | **get** /list-imports/{id} | List Imports_Fetch Resource
[**patch_update_resource12**](UserLibrariesApi.md#patch_update_resource12) | **patch** /library-entries/{id} | Update Resource
[**post_create_resource12**](UserLibrariesApi.md#post_create_resource12) | **post** /library-entries | Create Resource



## delete_resource123

> delete_resource123(id, content_type)
Delete Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y*  | N    | N     | N Yes           | None      | Y*† | Y    | Y     | Y Yes           | Admin     | Y*  | N    | N     | N  \\* Resources with `private: true` are not accessible, unless authenticated as the user associated with the library entry  † Adult content will only be shown if enabled in the authenticated user's settings  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `animeId` | `1`, `43,2,300` `dramaId` | `following` | `kind` | `anime`, `anime,manga` | Filter by the related media type `mangaId` | `2`, `8,987` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `since` | `2017-01-28`, `2013-03-25T00:00:00.000Z` | Returns entries updated since the supplied ISO 8601 date `status` | `current`, `onHold` | Values of the `status` attribute `title` | | Search resource `userId` | `42603`, `2,7`  **Pagination**  The maximum page limit is `500` for this resource.

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


## get_fetch_collection123456

> crate::models::FetchCollectionresponse7 get_fetch_collection123456(id)
Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y*  | N    | N     | N Yes           | None      | Y*† | Y    | Y     | Y Yes           | Admin     | Y*  | N    | N     | N  \\* Resources with `private: true` are not accessible, unless authenticated as the user associated with the library entry  † Adult content will only be shown if enabled in the authenticated user's settings  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `animeId` | `1`, `43,2,300` `dramaId` | `following` | `kind` | `anime`, `anime,manga` | Filter by the related media type `mangaId` | `2`, `8,987` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `since` | `2017-01-28`, `2013-03-25T00:00:00.000Z` | Returns entries updated since the supplied ISO 8601 date `status` | `current`, `onHold` | Values of the `status` attribute `title` | | Search resource `userId` | `42603`, `2,7`  **Pagination**  The maximum page limit is `500` for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchCollectionresponse7**](FetchCollectionresponse7.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_resource123456

> crate::models::FetchResourceresponse7 get_fetch_resource123456(id)
Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y*  | N    | N     | N Yes           | None      | Y*† | Y    | Y     | Y Yes           | Admin     | Y*  | N    | N     | N  \\* Resources with `private: true` are not accessible, unless authenticated as the user associated with the library entry  † Adult content will only be shown if enabled in the authenticated user's settings  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `animeId` | `1`, `43,2,300` `dramaId` | `following` | `kind` | `anime`, `anime,manga` | Filter by the related media type `mangaId` | `2`, `8,987` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `since` | `2017-01-28`, `2013-03-25T00:00:00.000Z` | Returns entries updated since the supplied ISO 8601 date `status` | `current`, `onHold` | Values of the `status` attribute `title` | | Search resource `userId` | `42603`, `2,7`  **Pagination**  The maximum page limit is `500` for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchResourceresponse7**](FetchResourceresponse7.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## library_entry_logs_create_resource

> library_entry_logs_create_resource(id, content_type)
Library Entry Logs_Create Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `linkedAccountId` | `syncStatus` | `pending`, `success`, `error`  **Pagination**  The maximum page limit is unlimited for this resource.

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


## library_entry_logs_delete_resource

> library_entry_logs_delete_resource(id, content_type)
Library Entry Logs_Delete Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `linkedAccountId` | `syncStatus` | `pending`, `success`, `error`  **Pagination**  The maximum page limit is unlimited for this resource.

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


## library_entry_logs_fetch_collection

> crate::models::LibraryEntryLogsFetchCollectionresponse library_entry_logs_fetch_collection(id)
Library Entry Logs_Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `linkedAccountId` | `syncStatus` | `pending`, `success`, `error`  **Pagination**  The maximum page limit is unlimited for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::LibraryEntryLogsFetchCollectionresponse**](LibraryEntryLogs_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## library_entry_logs_fetch_resource

> crate::models::LibraryEntryLogsFetchResourceresponse library_entry_logs_fetch_resource(id)
Library Entry Logs_Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `linkedAccountId` | `syncStatus` | `pending`, `success`, `error`  **Pagination**  The maximum page limit is unlimited for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::LibraryEntryLogsFetchResourceresponse**](LibraryEntryLogs_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## library_entry_logs_update_resource

> library_entry_logs_update_resource(id, content_type)
Library Entry Logs_Update Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | Y    | Y     | Y Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `linkedAccountId` | `syncStatus` | `pending`, `success`, `error`  **Pagination**  The maximum page limit is unlimited for this resource.

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


## library_events_delete_resource

> library_events_delete_resource(id, content_type)
Library Events_Delete Resource

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | N    | N     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7` `createdAt` |

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


## library_events_fetch_collection

> crate::models::LibraryEventsFetchCollectionresponse library_events_fetch_collection(id)
Library Events_Fetch Collection

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | N    | N     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7` `createdAt` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::LibraryEventsFetchCollectionresponse**](LibraryEvents_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## library_events_fetch_resource

> crate::models::LibraryEventsFetchResourceresponse library_events_fetch_resource(id)
Library Events_Fetch Resource

**Status**: In development.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | N    | N     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7` `createdAt` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::LibraryEventsFetchResourceresponse**](LibraryEvents_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_imports_create_resource

> list_imports_create_resource(id, content_type)
List Imports_Create Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | N   | N    | N     | N Yes           | None      | Y   | Y    | N     | N Yes           | Admin     | N   | Y    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7`

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


## list_imports_fetch_collection

> crate::models::ListImportsFetchCollectionresponse list_imports_fetch_collection(id)
List Imports_Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | N   | N    | N     | N Yes           | None      | Y   | Y    | N     | N Yes           | Admin     | N   | Y    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::ListImportsFetchCollectionresponse**](ListImports_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_imports_fetch_resource

> crate::models::ListImportsFetchResourceresponse list_imports_fetch_resource(id)
List Imports_Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | N   | N    | N     | N Yes           | None      | Y   | Y    | N     | N Yes           | Admin     | N   | Y    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::ListImportsFetchResourceresponse**](ListImports_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_update_resource12

> patch_update_resource12(id, content_type)
Update Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y*  | N    | N     | N Yes           | None      | Y*† | Y    | Y     | Y Yes           | Admin     | Y*  | N    | N     | N  \\* Resources with `private: true` are not accessible, unless authenticated as the user associated with the library entry  † Adult content will only be shown if enabled in the authenticated user's settings  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `animeId` | `1`, `43,2,300` `dramaId` | `following` | `kind` | `anime`, `anime,manga` | Filter by the related media type `mangaId` | `2`, `8,987` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `since` | `2017-01-28`, `2013-03-25T00:00:00.000Z` | Returns entries updated since the supplied ISO 8601 date `status` | `current`, `onHold` | Values of the `status` attribute `title` | | Search resource `userId` | `42603`, `2,7`  **Pagination**  The maximum page limit is `500` for this resource.

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


## post_create_resource12

> post_create_resource12(id, content_type)
Create Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y*  | N    | N     | N Yes           | None      | Y*† | Y    | Y     | Y Yes           | Admin     | Y*  | N    | N     | N  \\* Resources with `private: true` are not accessible, unless authenticated as the user associated with the library entry  † Adult content will only be shown if enabled in the authenticated user's settings  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `animeId` | `1`, `43,2,300` `dramaId` | `following` | `kind` | `anime`, `anime,manga` | Filter by the related media type `mangaId` | `2`, `8,987` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `since` | `2017-01-28`, `2013-03-25T00:00:00.000Z` | Returns entries updated since the supplied ISO 8601 date `status` | `current`, `onHold` | Values of the `status` attribute `title` | | Search resource `userId` | `42603`, `2,7`  **Pagination**  The maximum page limit is `500` for this resource.

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

