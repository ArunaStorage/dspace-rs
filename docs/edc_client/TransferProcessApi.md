# TransferProcessApi

All URIs are relative to the base path of the server

| Method                                                                                 | HTTP request                                    | Description |
|----------------------------------------------------------------------------------------|-------------------------------------------------|-------------|
| [**deprovision_transfer_process**](TransferProcessApi.md#deprovision_transfer_process) | **POST** /v2/transferprocesses/{id}/deprovision |             |
| [**get_transfer_process**](TransferProcessApi.md#get_transfer_process)                 | **GET** /v2/transferprocesses/{id}              |             |
| [**get_transfer_process_state**](TransferProcessApi.md#get_transfer_process_state)     | **GET** /v2/transferprocesses/{id}/state        |             |
| [**initiate_transfer_process**](TransferProcessApi.md#initiate_transfer_process)       | **POST** /v2/transferprocesses                  |             |
| [**query_transfer_processes**](TransferProcessApi.md#query_transfer_processes)         | **POST** /v2/transferprocesses/request          |             |
| [**resume_transfer_process**](TransferProcessApi.md#resume_transfer_process)           | **POST** /v2/transferprocesses/{id}/resume      |             |
| [**suspend_transfer_process**](TransferProcessApi.md#suspend_transfer_process)         | **POST** /v2/transferprocesses/{id}/suspend     |             |
| [**terminate_transfer_process**](TransferProcessApi.md#terminate_transfer_process)     | **POST** /v2/transferprocesses/{id}/terminate   |             |

## deprovision_transfer_process

> edc_client::transfer_process_api::deprovision_transfer_process(id)


Requests the deprovisioning of resources associated with a transfer process. Due to the asynchronous nature of transfers, a successful response only indicates that the request was successfully received. This may take a long time, so clients must poll the /{id}/state endpoint to track the state.

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

[[Back to top]](#transferprocessapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_transfer_process

> edc_client::transfer_process_api::get_transfer_process(id)


Gets an transfer process with the given ID

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

[**edc_api::TransferProcess**](../edc_api/TransferProcess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#transferprocessapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_transfer_process_state

> edc_client::transfer_process_api::get_transfer_process_state(id)


Gets the state of a transfer process with the given ID

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

[**edc_api::TransferState**](../edc_api/TransferState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#transferprocessapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## initiate_transfer_process

> edc_client::transfer_process_api::initiate_transfer_process(transfer_request)


Initiates a data transfer with the given parameters. Please note that successfully invoking this endpoint only means that the transfer was initiated. Clients must poll the /{id}/state endpoint to track the state

### Parameters


| Name                 | Type                                                         | Description | Required | Notes |
|----------------------|--------------------------------------------------------------|-------------|----------|-------|
| **transfer_request** | Option<[**TransferRequest**](../edc_api/TransferRequest.md)> |             |          |       |

### Return type

[**edc_api::IdResponse**](../edc_api/IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#transferprocessapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## query_transfer_processes

> edc_client::transfer_process_api::query_transfer_processes(query_spec)


Returns all transfer process according to a query

### Parameters


| Name           | Type                                             | Description | Required | Notes |
|----------------|--------------------------------------------------|-------------|----------|-------|
| **query_spec** | Option<[**QuerySpec**](../edc_api/QuerySpec.md)> |             |          |       |

### Return type

[**Vec<edc_api::TransferProcess>**](../edc_api/TransferProcess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#transferprocessapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## resume_transfer_process

> edc_client::transfer_process_api::resume_transfer_process(id)


Requests the resumption of a suspended transfer process. Due to the asynchronous nature of transfers, a successful response only indicates that the request was successfully received. This may take a long time, so clients must poll the /{id}/state endpoint to track the state

### Parameters


| Name                   | Type                                                             | Description | Required   | Notes |
|------------------------|------------------------------------------------------------------|-------------|------------|-------|
| **id**                 | **String**                                                       |             | [required] |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#transferprocessapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)




## suspend_transfer_process

> edc_client::transfer_process_api::suspend_transfer_process(id)


Requests the suspension of a transfer process. Due to the asynchronous nature of transfers, a successful response only indicates that the request was successfully received. This may take a long time, so clients must poll the /{id}/state endpoint to track the state

### Parameters


| Name                   | Type                                                             | Description | Required   | Notes |
|------------------------|------------------------------------------------------------------|-------------|------------|-------|
| **id**                 | **String**                                                       |             | [required] |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#transferprocessapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## terminate_transfer_process

> edc_client::transfer_process_api::terminate_transfer_process(id, terminate_transfer)


Requests the termination of a transfer process. Due to the asynchronous nature of transfers, a successful response only indicates that the request was successfully received. Clients must poll the /{id}/state endpoint to track the state.

### Parameters


| Name                   | Type                                                             | Description | Required   | Notes |
|------------------------|------------------------------------------------------------------|-------------|------------|-------|
| **id**                 | **String**                                                       |             | [required] |       |
| **terminate_transfer** | Option<[**TerminateTransfer**](../edc_api/TerminateTransfer.md)> |             |            |       |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#transferprocessapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

