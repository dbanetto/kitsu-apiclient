# StatsData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total** | Option<**f32**> | Total media in collection. Omitted for **amount-consumed** kinds | [optional]
**total_media** | Option<**f32**> | Omitted if not **category-breakdown** or **favorite-year** kind | [optional]
**activity** | Option<[**Vec<crate::models::Activity>**](activity.md)> | Omitted if not **activity-history** kind | [optional]
**all_categories** | Option<[**crate::models::AllCategories**](all_categories.md)> |  | [optional]
**all_time** | Option<[**crate::models::AllTime**](all_time.md)> |  | [optional]
**all_years** | Option<**::std::collections::HashMap<String, f32>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


