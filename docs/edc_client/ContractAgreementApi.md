# ContractAgreementApi

All URIs are relative to the base path of the server

| Method                                                                                         | HTTP request                                    | Description |
|------------------------------------------------------------------------------------------------|-------------------------------------------------|-------------|
| [**get_agreement_by_id**](ContractAgreementApi.md#get_agreement_by_id)                         | **GET** /v2/contractagreements/{id}             |             |
| [**get_negotiation_by_agreement_id**](ContractAgreementApi.md#get_negotiation_by_agreement_id) | **GET** /v2/contractagreements/{id}/negotiation |             |
| [**query_all_agreements**](ContractAgreementApi.md#query_all_agreements)                       | **POST** /v2/contractagreements/request         |             |

## get_agreement_by_id

> edc_client::contract_agreement_api::get_agreement_by_id(id)


Gets an contract agreement with the given ID

### Parameters


| Name   | Type       | Description | Required   | Notes |
|--------|------------|-------------|------------|-------|
| **id** | **String** |             | [required] |       |

### Return type

[**edc:api::ContractAgreement**](../edc_api/ContractAgreement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#contractagreementapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_negotiation_by_agreement_id

> edc_client::contract_agreement_api::get_negotiation_by_agreement_id(id)


Gets a contract negotiation with the given contract agreement ID

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

[[Back to top]](#contractagreementapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## query_all_agreements

> edc_client::contract_agreement_api::query_all_agreements(query_spec)


Gets all contract agreements according to a particular query

### Parameters


| Name           | Type                                             | Description | Required | Notes |
|----------------|--------------------------------------------------|-------------|----------|-------|
| **query_spec** | Option<[**QuerySpec**](../edc_api/QuerySpec.md)> |             |          |       |

### Return type

[**Vec<edc_api::ContractAgreement>**](../edc_api/ContractAgreement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#contractagreementapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

