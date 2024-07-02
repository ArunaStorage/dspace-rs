# ApplicationObservabilityApi

All URIs are relative to the base path of the server

| Method                                                            | HTTP request             | Note       |
|-------------------------------------------------------------------|--------------------------|------------|
| [**check_health**](ApplicationObservabilityApi.md#check_health)   | **GET** /check/health    | deprecated |
| [**get_liveness**](ApplicationObservabilityApi.md#get_liveness)   | **GET** /check/liveness  | deprecated |
| [**get_readiness**](ApplicationObservabilityApi.md#get_readiness) | **GET** /check/readiness | deprecated |
| [**get_startup**](ApplicationObservabilityApi.md#get_startup)     | **GET** /check/startup   | deprecated |

## check_health

> edc_client::application_observability_api::check_health()


Performs a liveness probe to determine whether the runtime is working properly.

### Parameters

This endpoint does not need any parameter.

### Return type

[**edc_api::HealthStatus**](../edc_api/HealthStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](ApplicationObservabilityApi.md#applicationobservabilityapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models) 


## get_liveness

> edc_client::application_observability_api::get_liveness()


Performs a liveness probe to determine whether the runtime is working properly.

### Parameters

This endpoint does not need any parameter.

### Return type

[**edc_api::HealthStatus**](../edc_api/HealthStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](ApplicationObservabilityApi.md#applicationobservabilityapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_readiness

> edc_client::application_observability_api::get_readiness()


Performs a readiness probe to determine whether the runtime is able to accept requests.

### Parameters

This endpoint does not need any parameter.

### Return type

[**edc_api::HealthStatus**](../edc_api/HealthStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](ApplicationObservabilityApi.md#applicationobservabilityapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models)


## get_startup

> edc_client::application_observability_api::get_startup()


Performs a startup probe to determine whether the runtime has completed startup.

### Parameters

This endpoint does not need any parameter.

### Return type

[**edc_api::HealthStatus**](../edc_api/HealthStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](ApplicationObservabilityApi.md#applicationobservabilityapi) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models) 

