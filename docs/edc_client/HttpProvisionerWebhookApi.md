# HttpProvisionerWebhookApi

All URIs are relative to the base path of the server

| Method                                                                                | HTTP request                               | Description |
|---------------------------------------------------------------------------------------|--------------------------------------------|-------------|
| [**call_deprovision_webhook**](HttpProvisionerWebhookApi.md#call_deprovision_webhook) | **POST** /callback/{processId}/deprovision |             |
| [**call_provision_webhook**](HttpProvisionerWebhookApi.md#call_provision_webhook)     | **POST** /callback/{processId}/provision   |             |

## call_deprovision_webhook

> edc_client::http_provisioner_webhook_api::call_deprovision_webhook(process_id, deprovisioned_resource)


### Parameters


| Name                       | Type                                                                     | Description | Required   | Notes |
|----------------------------|--------------------------------------------------------------------------|-------------|------------|-------|
| **process_id**             | **String**                                                               |             | [required] |       |
| **deprovisioned_resource** | Option<[**DeprovisionedResource**](../edc_api/DeprovisionedResource.md)> |             |            |       |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#httpprovisionerwebhookapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## call_provision_webhook

> edc_client::http_provisioner_webhook_api::call_provision_webhook(process_id, provisioner_webhook_request)


### Parameters


| Name                            | Type                                                                             | Description | Required   | Notes |
|---------------------------------|----------------------------------------------------------------------------------|-------------|------------|-------|
| **process_id**                  | **String**                                                                       |             | [required] |       |
| **provisioner_webhook_request** | Option<[**ProvisionerWebhookRequest**](../edc_api/ProvisionerWebhookRequest.md)> |             |            |       |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#httpprovisionerwebhookapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)

