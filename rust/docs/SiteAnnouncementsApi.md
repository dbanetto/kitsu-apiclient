# \SiteAnnouncementsApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_fetch_collection1234567891011121314**](SiteAnnouncementsApi.md#get_fetch_collection1234567891011121314) | **get** /site-announcements | Fetch Collection
[**get_fetch_resource1234567891011121314**](SiteAnnouncementsApi.md#get_fetch_resource1234567891011121314) | **get** /site-announcements/{id} | Fetch Resource



## get_fetch_collection1234567891011121314

> crate::models::FetchCollectionresponse15 get_fetch_collection1234567891011121314(id)
Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchCollectionresponse15**](FetchCollectionresponse15.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_resource1234567891011121314

> crate::models::FetchResourceresponse15 get_fetch_resource1234567891011121314(id)
Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchResourceresponse15**](FetchResourceresponse15.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

