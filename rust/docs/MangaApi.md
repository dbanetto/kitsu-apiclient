# \MangaApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_collection**](MangaApi.md#fetch_collection) | **Get** /chapters | Fetch Collection
[**fetch_collection_0**](MangaApi.md#fetch_collection_0) | **Get** /manga | Fetch Collection
[**fetch_collection_1**](MangaApi.md#fetch_collection_1) | **Get** /trending/manga | Fetch Collection
[**fetch_resource**](MangaApi.md#fetch_resource) | **Get** /chapters/{id} | Fetch Resource
[**fetch_resource_0**](MangaApi.md#fetch_resource_0) | **Get** /manga/{id} | Fetch Resource



## fetch_collection

> crate::models::InlineResponse200 fetch_collection()
Fetch Collection

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_collection_0

> crate::models::InlineResponse200 fetch_collection_0()
Fetch Collection

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_collection_1

> crate::models::InlineResponse20061 fetch_collection_1()
Fetch Collection

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20061**](inline_response_200_61.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_resource

> crate::models::InlineResponse2009 fetch_resource(id)
Fetch Resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** |  | [required] |

### Return type

[**crate::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_resource_0

> crate::models::InlineResponse20039 fetch_resource_0(id)
Fetch Resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** |  | [required] |

### Return type

[**crate::models::InlineResponse20039**](inline_response_200_39.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

