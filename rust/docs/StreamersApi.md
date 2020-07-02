# \StreamersApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_fetch_collection1234**](StreamersApi.md#get_fetch_collection1234) | **get** /streamers | Fetch Collection
[**get_fetch_resource1234**](StreamersApi.md#get_fetch_resource1234) | **get** /streamers/{id} | Fetch Resource
[**streaming_links_fetch_collection**](StreamersApi.md#streaming_links_fetch_collection) | **get** /streaming-links | Streaming Links_Fetch Collection
[**streaming_links_fetch_resource**](StreamersApi.md#streaming_links_fetch_resource) | **get** /streaming-links/{id} | Streaming Links_Fetch Resource



## get_fetch_collection1234

> crate::models::FetchCollectionresponse5 get_fetch_collection1234(id)
Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----  **Pagination**  The maximum page limit is unlimited for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchCollectionresponse5**](FetchCollectionresponse5.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_resource1234

> crate::models::FetchResourceresponse5 get_fetch_resource1234(id)
Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----  **Pagination**  The maximum page limit is unlimited for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchResourceresponse5**](FetchResourceresponse5.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## streaming_links_fetch_collection

> crate::models::StreamingLinksFetchCollectionresponse streaming_links_fetch_collection(id)
Streaming Links_Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::StreamingLinksFetchCollectionresponse**](StreamingLinks_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## streaming_links_fetch_resource

> crate::models::StreamingLinksFetchResourceresponse streaming_links_fetch_resource(id)
Streaming Links_Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::StreamingLinksFetchResourceresponse**](StreamingLinks_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

