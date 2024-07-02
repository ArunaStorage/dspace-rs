# SecretApi

All URIs are relative to the base path of the server

| Method                                          | HTTP request                | NOTE |
|-------------------------------------------------|-----------------------------|------|
| [**create_secret**](SecretApi.md#create_secret) | **POST** /v1/secrets        |      |
| [**delete_secret**](SecretApi.md#delete_secret) | **DELETE** /v1/secrets/{id} |      |
| [**get_secret**](SecretApi.md#get_secret)       | **GET** /v1/secrets/{id}    |      |
| [**update_secret**](SecretApi.md#update_secret) | **PUT** /v1/secrets         |      |

## create_secret

> edc_client::secret_api::create_secret(secret_input)


Creates a new secret

### Parameters


| Name             | Type                                                 | Description | Required | Notes |
|------------------|------------------------------------------------------|-------------|----------|-------|
| **secret_input** | Option<[**SecretInput**](../edc_api/SecretInput.md)> |             |          |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#secretapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## delete_secret

> edc_client::secret_api::delete_secrete(id)


Removes a secret with the given ID if possible

### Parameters


| Name   | Type        | Description | Required   | Notes |
|--------|-------------|-------------|------------|-------|
| **id** | **String**  |             | [required] |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#secretapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## get_secret

> edc_client::secret_api::get_asset(id)


Gets a secret with the given ID

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

[**edc_api::SecretOutput**](../edc_api/SecretOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#secretapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)



## update_secret

> edc_client::secret::update_secret(secret_input)


Updates a secret with the given ID if it exists. If the secret is not found, no further action is taken

### Parameters


| Name             | Type                                                 | Description | Required | Notes |
|------------------|------------------------------------------------------|-------------|----------|-------|
| **secret_input** | Option<[**SecretInput**](../edc_api/SecretInput.md)> |             |          |       |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#secretapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

