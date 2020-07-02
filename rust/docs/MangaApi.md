# \MangaApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chapters_fetch_collection**](MangaApi.md#chapters_fetch_collection) | **get** /chapters | Chapters_Fetch Collection
[**chapters_fetch_resource**](MangaApi.md#chapters_fetch_resource) | **get** /chapters/{id} | Chapters_Fetch Resource
[**get_fetch_collection**](MangaApi.md#get_fetch_collection) | **get** /manga | Fetch Collection
[**get_fetch_resource**](MangaApi.md#get_fetch_resource) | **get** /manga/{id} | Fetch Resource
[**trending_manga_fetch_collection**](MangaApi.md#trending_manga_fetch_collection) | **get** /trending/manga | Trending Manga_Fetch Collection



## chapters_fetch_collection

> crate::models::ChaptersFetchCollectionresponse chapters_fetch_collection(id)
Chapters_Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mangaId` | `2`, `8,987` `number` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::ChaptersFetchCollectionresponse**](Chapters_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chapters_fetch_resource

> crate::models::ChaptersFetchResourceresponse chapters_fetch_resource(id)
Chapters_Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mangaId` | `2`, `8,987` `number` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::ChaptersFetchResourceresponse**](Chapters_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_collection

> crate::models::FetchCollectionresponse1 get_fetch_collection(id)
Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `chapterCount` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchCollectionresponse1**](FetchCollectionresponse1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_resource

> crate::models::FetchResourceresponse1 get_fetch_resource(id)
Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `chapterCount` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchResourceresponse1**](FetchResourceresponse1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trending_manga_fetch_collection

> crate::models::TrendingMangaFetchCollectionresponse trending_manga_fetch_collection()
Trending Manga_Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TrendingMangaFetchCollectionresponse**](TrendingManga_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

