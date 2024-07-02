# AssetApi

All URIs are relative to the base path of the server

| Method                                                           | HTTP request                | NOTE |
|------------------------------------------------------------------|-----------------------------|------|
| [**create_asset**](AssetApi.md#create_asset)                     | **POST** /v3/assets         |      |
| [**get_asset**](AssetApi.md#get_asset)                           | **GET** /v3/assets/{id}     |      |
| [**remove_asset**](AssetApi.md#remove_asset)                     | **DELETE** /v3/assets/{id}  |      |
| [**request_assets**](AssetApi.md#request_assets)                 | **POST** /v3/assets/request |      |
| [**update_asset**](AssetApi.md#update_asset)                     | **PUT** /v3/assets          |      |

## create_asset

> edc_client::asset_api::create_asset(asset_entry)


Creates a new asset together with a data address

### Parameters


| Name            | Type                                               | Description | Required | Notes |
|-----------------|----------------------------------------------------|-------------|----------|-------|
| **asset_entry** | Option<[**AssetInput**](../edc_api/AssetInput.md)> |             |          |       |

### Return type

[**edc_api::IdResponse**](../edc_api/IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#assetapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## get_asset

> edc_client::asset_api::get_asset(id)


Gets an asset with the given ID

### Parameters


| Name   | Type        | Description | Required   | Notes |
|--------|-------------|-------------|------------|-------|
| **id** | **String**  |             | [required] |       |

### Return type

[**edc_api::AssetOutput**](../edc_api/AssetOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#assetapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## remove_asset

> edc_client::asset_api::remove_asset(id)


Removes an asset with the given ID if possible. Deleting an asset is only possible if that asset is not yet referenced by a contract agreement, in which case an error is returned. DANGER ZONE: Note that deleting assets can have unexpected results, especially for contract offers that have been sent out or ongoing or contract negotiations.

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#assetapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## request_assets

> edc_client::asset_api::request_assets(query_spec)


 all assets according to a particular query

### Parameters


| Name           | Type                                             | Description | Required | Notes |
|----------------|--------------------------------------------------|-------------|----------|-------|
| **query_spec** | Option<[**QuerySpec**](../edc_api/QuerySpec.md)> |             |          |       |

### Return type

[**Vec<edc_api::AssetOutput>**](../edc_api/AssetOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#assetapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## update_asset

> edc_client::asset_update::update_asset(asset)


Updates an asset with the given ID if it exists. If the asset is not found, no further action is taken. DANGER ZONE: Note that updating assets can have unexpected results, especially for contract offers that have been sent out or are ongoing in contract negotiations.

### Parameters


| Name      | Type                                               | Description | Required | Notes |
|-----------|----------------------------------------------------|-------------|----------|-------|
| **asset** | Option<[**AssetInput**](../edc_api/AssetInput.md)> |             |          |       |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#assetapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)
