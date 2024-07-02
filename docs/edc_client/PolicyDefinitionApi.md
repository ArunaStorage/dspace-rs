# PolicyDefinitionApi

All URIs are relative to the base path of the server

| Method                                                                          | HTTP request                           | Description |
|---------------------------------------------------------------------------------|----------------------------------------|-------------|
| [**create_policy_definition**](PolicyDefinitionApi.md#create_policy_definition) | **POST** /v2/policydefinitions         |             |
| [**delete_policy_definition**](PolicyDefinitionApi.md#delete_policy_definition) | **DELETE** /v2/policydefinitions/{id}  |             |
| [**get_policy_definition**](PolicyDefinitionApi.md#get_policy_definition)       | **GET** /v2/policydefinitions/{id}     |             |
| [**query_policy_definitions**](PolicyDefinitionApi.md#query_policy_definitions) | **POST** /v2/policydefinitions/request |             |
| [**update_policy_definition**](PolicyDefinitionApi.md#update_policy_definition) | **PUT** /v2/policydefinitions/{id}     |             |

## create_policy_definition

> edc_client::policy_definition_api::create_policy_definition(policy_definition_input)


Creates a new policy definition

### Parameters


| Name                        | Type                                                                     | Description | Required | Notes |
|-----------------------------|--------------------------------------------------------------------------|-------------|----------|-------|
| **policy_definition_input** | Option<[**PolicyDefinitionInput**](../edc_api/PolicyDefinitionInput.md)> |             |          |       |

### Return type

[**edc_api::IdResponse**](../edc_api/IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#policydefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## delete_policy_definition

> edc_client::policy_definition_api::delete_policy_definition(id)


Removes a policy definition with the given ID if possible. Deleting a policy definition is only possible if that policy definition is not yet referenced by a contract definition, in which case an error is returned. DANGER ZONE: Note that deleting policy definitions can have unexpected results, do this at your own risk!

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

[[Back to top]](#policydefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_policy_definition

> edc_client::policy_definition_api::get_policy_definition(id)


Gets a policy definition with the given ID

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

[**edc_api::PolicyDefinitionOutput**](../edc_api/PolicyDefinitionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#policydefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## query_policy_definitions

> edc_client::policy_definition_api::query_policy_definitions(query_spec)


Returns all policy definitions according to a query

### Parameters


| Name           | Type                                             | Description | Required | Notes |
|----------------|--------------------------------------------------|-------------|----------|-------|
| **query_spec** | Option<[**QuerySpec**](../edc_api/QuerySpec.md)> |             |          |       |

### Return type

[**Vec<edc_api::PolicyDefinitionOutput>**](../edc_api/PolicyDefinitionOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#policydefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## update_policy_definition

> edc_client::policy_definition_api::update_policy_definition(id, policy_definition_input)


Updates an existing Policy, If the Policy is not found, an error is reported

### Parameters


| Name                        | Type                                                                     | Description | Required   | Notes |
|-----------------------------|--------------------------------------------------------------------------|-------------|------------|-------|
| **id**                      | **String**                                                               |             | [required] |       |
| **policy_definition_input** | Option<[**PolicyDefinitionInput**](../edc_api/PolicyDefinitionInput.md)> |             |            |       |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#policydefinitionapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

