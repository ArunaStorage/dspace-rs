# ContractNegotiationApi

All URIs are relative to the base path of the server

| Method                                                                                       | HTTP request                                     | Description |
|----------------------------------------------------------------------------------------------|--------------------------------------------------|-------------|
| [**get_agreement_for_negotiation**](ContractNegotiationApi.md#get_agreement_for_negotiation) | **GET** /v2/contractnegotiations/{id}/agreement  |             |
| [**get_negotiation**](ContractNegotiationApi.md#get_negotiation)                             | **GET** /v2/contractnegotiations/{id}            |             |
| [**get_negotiation_state**](ContractNegotiationApi.md#get_negotiation_state)                 | **GET** /v2/contractnegotiations/{id}/state      |             |
| [**initiate_contract_negotiation**](ContractNegotiationApi.md#initiate_contract_negotiation) | **POST** /v2/contractnegotiations                |             |
| [**query_negotiations**](ContractNegotiationApi.md#query_negotiations)                       | **POST** /v2/contractnegotiations/request        |             |
| [**terminate_negotiation**](ContractNegotiationApi.md#terminate_negotiation)                 | **POST** /v2/contractnegotiations/{id}/terminate |             |



## get_agreement_for_negotiation

> edc_client::contract_negotiation_api::get_agreement_for_negotiation(id)


Gets a contract agreement for a contract negotiation with the given ID

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

[**edc_api::ContractAgreement**](../edc_api/ContractAgreement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#contractnegotiationapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_negotiation

> edc_client::contract_negotiation_api::get_negotiation(id)


Gets a contract negotiation with the given ID

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

[**edc_api::ContractNegotiation**](../edc_api/ContractNegotiation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#contractnegotiationapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_negotiation_state

> edc_client::contract_negotiation_api::get_negotiation_state(id)


Gets the state of a contract negotiation with the given ID

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

[**edc_api::NegotiationState**](../edc_api/NegotiationState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#contractnegotiationapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## initiate_contract_negotiation

> edc_client::contract_negotiation_api::initiate_contract_negotiation(contract_request)


Initiates a contract negotiation for a given offer and with the given counter part. Please note that successfully invoking this endpoint only means that the negotiation was initiated. Clients must poll the /{id}/state endpoint to track the state

### Parameters


| Name                 | Type                                                         | Description | Required | Notes |
|----------------------|--------------------------------------------------------------|-------------|----------|-------|
| **contract_request** | Option<[**ContractRequest**](../edc_api/ContractRequest.md)> |             |          |       |

### Return type

[**edc_api::IdResponse**](../edc_api/IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#contractnegotiationapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## query_negotiations

> edc_client::contract_negotiation_api::query_negotiations(query_spec)


Returns all contract negotiations according to a query

### Parameters


| Name           | Type                                             | Description | Required | Notes |
|----------------|--------------------------------------------------|-------------|----------|-------|
| **query_spec** | Option<[**QuerySpec**](../edc_api/QuerySpec.md)> |             |          |       |

### Return type

[**Vec<edc_api::ContractNegotiation>**](../edc_api/ContractNegotiation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#contractnegotiationapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## terminate_negotiation

> edc_client::contract_negotiation_api::terminate_negotiation(id, terminate_negotiation_schema)


Terminates the contract negotiation.

### Parameters


| Name                             | Type                                                                               | Description | Required   | Notes |
|----------------------------------|------------------------------------------------------------------------------------|-------------|------------|-------|
| **id**                           | **String**                                                                         |             | [required] |       |
| **terminate_negotiation_schema** | Option<[**TerminateNegotiationSchema**](../edc_api/TerminateNegotiationSchema.md)> |             |            |       |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#contractnegotiationapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

