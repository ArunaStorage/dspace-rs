# \ContractDefinitionApi

All URIs are relative to base path of the server

| Method                                                                                        | HTTP request                             | Description |
|-----------------------------------------------------------------------------------------------|------------------------------------------|-------------|
| [**create_contract_definition**](ContractDefinitionApi.md#create_contract_definition)         | **POST** /v2/contractdefinitions         |             |
| [**delete_contract_definition**](ContractDefinitionApi.md#delete_contract_definition)         | **DELETE** /v2/contractdefinitions/{id}  |             |
| [**get_contract_definition**](ContractDefinitionApi.md#get_contract_definition)               | **GET** /v2/contractdefinitions/{id}     |             |
| [**query_all_contract_definitions**](ContractDefinitionApi.md#query_all_contract_definitions) | **POST** /v2/contractdefinitions/request |             |
| [**update_contract_definition**](ContractDefinitionApi.md#update_contract_definition)         | **PUT** /v2/contractdefinitions          |             |

## create_contract_definition

> edc_client::contract_definition_api::create_contract_definition(contract_definition_input)


Creates a new contract definition

### Parameters


| Name                          | Type                                                                         | Description | Required | Notes |
|-------------------------------|------------------------------------------------------------------------------|-------------|----------|-------|
| **contract_definition_input** | Option<[**ContractDefinitionInput**](../edc_api/ContractDefinitionInput.md)> |             |          |       |

### Return type

[**edc_api::IdResponse**](../edc_api/IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#contractdefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## delete_contract_definition

> edc_client::contract_definition_api::delete_contract_definition(id)


Removes a contract definition with the given ID if possible. DANGER ZONE: Note that deleting contract definitions can have unexpected results, especially for contract offers that have been sent out or ongoing or contract negotiations.

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

[[Back to top]](#contractdefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_contract_definition

> edc_client::contract_definition_api::get_contract_definition(id)


Gets an contract definition with the given ID

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

[**edc_api::ContractDefinitionOutput**](../edc_api/ContractDefinitionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#contractdefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## query_all_contract_definitions

> edc_client::contract_definition_api::query_all_contract_definitions(query_spec)


Returns all contract definitions according to a query

### Parameters


| Name           | Type                                             | Description | Required | Notes |
|----------------|--------------------------------------------------|-------------|----------|-------|
| **query_spec** | Option<[**QuerySpec**](../edc_api/QuerySpec.md)> |             |          |       |

### Return type

[**Vec<edc_api::ContractDefinitionOutput>**](../edc_api/ContractDefinitionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#contractdefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## update_contract_definition

> update_contract_definition(contract_definition_input)


Updated a contract definition with the given ID. The supplied JSON structure must be a valid JSON-LD object

### Parameters


| Name                          | Type                                                                         | Description | Required | Notes |
|-------------------------------|------------------------------------------------------------------------------|-------------|----------|-------|
| **contract_definition_input** | Option<[**ContractDefinitionInput**](../edc_api/ContractDefinitionInput.md)> |             |          |       |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#contractdefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

