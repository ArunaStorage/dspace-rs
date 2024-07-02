# DataplaneSelectorApi

All URIs are relative to the base path of the server

| Method                                             | HTTP request                   | Description |
|----------------------------------------------------|--------------------------------|-------------|
| [**add_entry**](DataplaneSelectorApi.md#add_entry) | **POST** /v2/dataplanes        | deprecated  |
| [**find**](DataplaneSelectorApi.md#find)           | **POST** /v2/dataplanes/select | deprecated  |
| [**get_all**](DataplaneSelectorApi.md#get_all)     | **GET** /v2/dataplanes         |             |

## add_entry

> edc_client::dataplane_selector::add_entry(data_plane_instance_schema)


Adds one datatplane instance to the internal database of the selector

### Parameters


| Name                           | Type                                                                         | Description | Required | Notes |
|--------------------------------|------------------------------------------------------------------------------|-------------|----------|-------|
| **data_plane_instance_schema** | Option<[**DataPlaneInstanceSchema**](../edc_api/DataPlaneInstanceSchema.md)> |             |          |       |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#dataplaneselectorapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## find

> edc_client::dataplane_selector::find(selection_request_schema)


Finds the best fitting data plane instance for a particular query

### Parameters


| Name                         | Type                                                                       | Description | Required | Notes |
|------------------------------|----------------------------------------------------------------------------|-------------|----------|-------|
| **selection_request_schema** | Option<[**SelectionRequestSchema**](../edc_api/SelectionRequestSchema.md)> |             |          |       |

### Return type

[**edc_api::DataPlaneInstanceSchema**](../edc_api/DataPlaneInstanceSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#dataplaneselectorapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## get_all

> edc_client::dataplane_selector_api::get_all()


Returns a list of all currently registered data plane instances

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#dataplaneselectorapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

