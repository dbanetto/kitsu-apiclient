# \MediaRelationsApi

All URIs are relative to *https://kitsu.io/api/edge*

Method | HTTP request | Description
------------- | ------------- | -------------
[**franchises_fetch_collection**](MediaRelationsApi.md#franchises_fetch_collection) | **get** /franchises | Franchises_Fetch Collection
[**franchises_fetch_resource**](MediaRelationsApi.md#franchises_fetch_resource) | **get** /franchises/{id} | Franchises_Fetch Resource
[**get_fetch_collection12**](MediaRelationsApi.md#get_fetch_collection12) | **get** /media-relationships | Fetch Collection
[**get_fetch_resource12**](MediaRelationsApi.md#get_fetch_resource12) | **get** /media-relationships/{id} | Fetch Resource
[**installments_fetch_collection**](MediaRelationsApi.md#installments_fetch_collection) | **get** /installments | Installments_Fetch Collection
[**installments_fetch_resource**](MediaRelationsApi.md#installments_fetch_resource) | **get** /installments/{id} | Installments_Fetch Resource
[**mappings_fetch_collection**](MediaRelationsApi.md#mappings_fetch_collection) | **get** /mappings | Mappings_Fetch Collection
[**mappings_fetch_resource**](MediaRelationsApi.md#mappings_fetch_resource) | **get** /mappings/{id} | Mappings_Fetch Resource



## franchises_fetch_collection

> crate::models::FranchisesFetchCollectionresponse franchises_fetch_collection(id)
Franchises_Fetch Collection

**Status**: Deprecated. Use `media-relationships` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FranchisesFetchCollectionresponse**](Franchises_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## franchises_fetch_resource

> crate::models::FranchisesFetchResourceresponse franchises_fetch_resource(id)
Franchises_Fetch Resource

**Status**: Deprecated. Use `media-relationships` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :----

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FranchisesFetchResourceresponse**](Franchises_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_collection12

> crate::models::FetchCollectionresponse3 get_fetch_collection12(id)
Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `role` | | Values of the `role` attribute `sourceType` | `sourceId` | `destinationId` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchCollectionresponse3**](FetchCollectionresponse3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fetch_resource12

> crate::models::FetchResourceresponse3 get_fetch_resource12(id)
Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `role` | | Values of the `role` attribute `sourceType` | `sourceId` | `destinationId` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::FetchResourceresponse3**](FetchResourceresponse3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## installments_fetch_collection

> crate::models::InstallmentsFetchCollectionresponse installments_fetch_collection(id)
Installments_Fetch Collection

**Status**: Deprecated. Use `media-relationships` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::InstallmentsFetchCollectionresponse**](Installments_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## installments_fetch_resource

> crate::models::InstallmentsFetchResourceresponse installments_fetch_resource(id)
Installments_Fetch Resource

**Status**: Deprecated. Use `media-relationships` instead.  **Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `mediaType` | `anime`, `anime,manga` | **DEPRECATED** - use `kind` `mediaId` | | **DEPRECATED** - use `anime_id`, `manga_id` or `drama_id`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::InstallmentsFetchResourceresponse**](Installments_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mappings_fetch_collection

> crate::models::MappingsFetchCollectionresponse mappings_fetch_collection(id)
Mappings_Fetch Collection

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `externalSite` | `myanimelist/anime` `externalId` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::MappingsFetchCollectionresponse**](Mappings_FetchCollectionresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mappings_fetch_resource

> crate::models::MappingsFetchResourceresponse mappings_fetch_resource(id)
Mappings_Fetch Resource

**Authorisation**  Authenticated | Role      | GET | POST | PATCH | DELETE ------------: | --------: | :-: | :--: | :---: | :----: No            | None      | Y   | N    | N     | N Yes           | None      | Y   | N    | N     | N Yes           | Admin     | Y   | Y    | Y     | Y  **Filters**  Filters can be used in `camelCase` or `snake_case` format  Filter | Example | Notes -----: | :------ | :---- `externalSite` | `myanimelist/anime` `externalId` |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** |  | [required] |

### Return type

[**crate::models::MappingsFetchResourceresponse**](Mappings_FetchResourceresponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

