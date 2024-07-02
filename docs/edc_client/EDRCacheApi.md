# EDRCacheApi

All URIs are relative to the base path of the server

| Method                                                          | HTTP request                                      | Description |
|-----------------------------------------------------------------|---------------------------------------------------|-------------|
| [**delete_edr**](EDRCacheApi.md#delete_edr)                     | **DELETE** /v1/edrs/{transferProcessId}           |             |
| [**get_edr_data_address**](EDRCacheApi.md#get_edr_data_address) | **POST** /v1/edrs/{transferProcessId}/dataaddress |             |
| [**query_edrs**](EDRCacheApi.md#query_edrs)                     | **GET** /v1/edrs/request                          |             |

## delete_edr

> edc_client::edr_cache_api::delete_edr(id)


Removes an EDR entry given the transfer process ID

### Parameters


| Name   | Type        | Description | Required   | Notes |
|--------|-------------|-------------|------------|-------|
| **id** | **String**  |             | [required] |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#edrcacheapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_edr_data_address

> edc_client::edr_cache_api::get_edr_data_address(id)


Gets the EDR data address with the given transfer process ID

### Parameters


| Name   | Type        | Description | Required   | Notes |
|--------|-------------|-------------|------------|-------|
| **id** | **String**  |             | [required] |       |

### Return type

[**edc_api::DataAddress**](../edc_api/DataAddress.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#edrcacheapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## query_edrs

> edc_client::edr_cache_api::query_edrs(query_spec)


Returns a list of all currently registered data plane instances

### Parameters


| Name           | Type                                             | Description | Required | Notes |
|----------------|--------------------------------------------------|-------------|----------|-------|
| **query_spec** | Option<[**QuerySpec**](../edc_api/QuerySpec.md)> |             |          |       |

### Return type

[**Vec<edc_api::EndpointDataReferenceEntry>**](../edc_api/EndpointDataReferenceEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#edrcacheapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

