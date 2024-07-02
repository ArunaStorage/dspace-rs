# CatalogApi

All URIs are relative to the base path of the server

| Method                                               | HTTP request                         | Description |
|------------------------------------------------------|--------------------------------------|-------------|
| [**get_dataset**](CatalogApi.md#get_dataset)         | **POST** /v2/catalog/dataset/request |             |
| [**request_catalog**](CatalogApi.md#request_catalog) | **POST** /v2/catalog/request         |             |

## get_dataset

> edc_client::catalog_api::get_dataset(dataset_request)


### Parameters


| Name                | Type                                                       | Description | Required | Notes |
|---------------------|------------------------------------------------------------|-------------|----------|-------|
| **dataset_request** | Option<[**DatasetRequest**](../edc_api/DatasetRequest.md)> |             |          |       |

### Return type

[**serde_json::Value**](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#catalogapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

## request_catalog

> edc_client::catalog_api::request_catalog(catalog_request)


### Parameters


| Name                | Type                                                       | Description | Required | Notes |
|---------------------|------------------------------------------------------------|-------------|----------|-------|
| **catalog_request** | Option<[**CatalogRequest**](../edc_api/CatalogRequest.md)> |             |          |       |

### Return type

[**serde_json::Value**](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#catalogapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

