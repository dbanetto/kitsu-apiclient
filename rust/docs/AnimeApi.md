# \AnimeApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**episodes_fetch_collection**](AnimeApi.md#episodes_fetch_collection) | **get** /episodes | Episodes_Fetch Collection
[**episodes_fetch_resource**](AnimeApi.md#episodes_fetch_resource) | **get** /episodes/{id} | Episodes_Fetch Resource
[**fetch_collection**](AnimeApi.md#fetch_collection) | **get** /anime | Fetch Collection
[**fetch_resource**](AnimeApi.md#fetch_resource) | **get** /anime/{id} | Fetch Resource
[**trending_anime_fetch_collection**](AnimeApi.md#trending_anime_fetch_collection) | **get** /trending/anime | Trending Anime_Fetch Collection



## episodes_fetch_collection

> crate::models::EpisodesFetchCollectionresponse episodes_fetch_collection(id)
Episodes_Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `number` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::EpisodesFetchCollectionresponse**](Episodes_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## episodes_fetch_resource

> crate::models::EpisodesFetchResourceresponse episodes_fetch_resource(id)
Episodes_Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id` `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `number` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::EpisodesFetchResourceresponse**](Episodes_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_collection

> crate::models::FetchCollectionresponse fetch_collection(id)
Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `season` | `winter`, `spring`, `summer`, `fall` `seasonYear` | `2017`, `2005,2006` `streamers` | `Crunchyroll` `ageRating` | `G`, `PG,R,R18`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchCollectionresponse**](FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_resource

> crate::models::FetchResourceresponse fetch_resource(id)
Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `season` | `winter`, `spring`, `summer`, `fall` `seasonYear` | `2017`, `2005,2006` `streamers` | `Crunchyroll` `ageRating` | `G`, `PG,R,R18`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchResourceresponse**](FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trending_anime_fetch_collection

> crate::models::TrendingAnimeFetchCollectionresponse trending_anime_fetch_collection()
Trending Anime_Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | N    | N     | N  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TrendingAnimeFetchCollectionresponse**](TrendingAnime_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

