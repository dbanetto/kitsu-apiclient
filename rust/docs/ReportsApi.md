# \ReportsApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_resource12345678910**](ReportsApi.md#delete_resource12345678910) | **delete** /reports/{id} | Delete Resource
[**get_fetch_collection12345678910111213**](ReportsApi.md#get_fetch_collection12345678910111213) | **get** /reports | Fetch Collection
[**get_fetch_resource12345678910111213**](ReportsApi.md#get_fetch_resource12345678910111213) | **get** /reports/{id} | Fetch Resource
[**patch_update_resource123456789**](ReportsApi.md#patch_update_resource123456789) | **patch** /reports/{id} | Update Resource
[**post_create_resource123456789**](ReportsApi.md#post_create_resource123456789) | **post** /reports | Create Resource



## delete_resource12345678910

> delete_resource12345678910(id, content_type)
Delete Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | N   | N    | N     | N Yes           | None      | N   | Y    | Y     | N Yes           | Admin     | Y   | N    | Y     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7` `naughtyId` | `naughtyType` | `Post`, `Comment`, `Review`, `Reaction` `status` | `current`, `onHold` | Values of the `status` attribute `reason` | `nsfw`, `spoiler`

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


## get_fetch_collection12345678910111213

> crate::models::FetchCollectionresponse14 get_fetch_collection12345678910111213(id)
Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | N   | N    | N     | N Yes           | None      | N   | Y    | Y     | N Yes           | Admin     | Y   | N    | Y     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7` `naughtyId` | `naughtyType` | `Post`, `Comment`, `Review`, `Reaction` `status` | `current`, `onHold` | Values of the `status` attribute `reason` | `nsfw`, `spoiler`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchCollectionresponse14**](FetchCollectionresponse14.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_resource12345678910111213

> crate::models::FetchResourceresponse14 get_fetch_resource12345678910111213(id)
Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | N   | N    | N     | N Yes           | None      | N   | Y    | Y     | N Yes           | Admin     | Y   | N    | Y     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7` `naughtyId` | `naughtyType` | `Post`, `Comment`, `Review`, `Reaction` `status` | `current`, `onHold` | Values of the `status` attribute `reason` | `nsfw`, `spoiler`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchResourceresponse14**](FetchResourceresponse14.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_update_resource123456789

> patch_update_resource123456789(id, content_type)
Update Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | N   | N    | N     | N Yes           | None      | N   | Y    | Y     | N Yes           | Admin     | Y   | N    | Y     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7` `naughtyId` | `naughtyType` | `Post`, `Comment`, `Review`, `Reaction` `status` | `current`, `onHold` | Values of the `status` attribute `reason` | `nsfw`, `spoiler`

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


## post_create_resource123456789

> post_create_resource123456789(id, content_type)
Create Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | N   | N    | N     | N Yes           | None      | N   | Y    | Y     | N Yes           | Admin     | Y   | N    | Y     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `userId` | `42603`, `2,7` `naughtyId` | `naughtyType` | `Post`, `Comment`, `Review`, `Reaction` `status` | `current`, `onHold` | Values of the `status` attribute `reason` | `nsfw`, `spoiler`

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

