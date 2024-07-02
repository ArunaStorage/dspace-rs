# ContractNegotiation

## Properties

| Name                      | Type                                                                                                                           | Notes      |
|---------------------------|--------------------------------------------------------------------------------------------------------------------------------|------------|
| **context**               | **std::collections::HashMap<String, [serde_json::Value](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)>** | [optional] |
| **at_id**                 | Option<**String**>                                                                                                             | [optional] |
| **at_type**               | Option<**String**>                                                                                                             | [optional] |
| **callback_addresses**    | Option<**Vec<[edc_api::CallbackAddress](CallbackAddress.md)>**>                                                                | [optional] |
| **contract_agreement_id** | Option<**String**>                                                                                                             | [optional] |
| **counter_party_address** | Option<**String**>                                                                                                             | [optional] |
| **counter_party_id**      | Option<**String**>                                                                                                             | [optional] |
| **error_detail**          | Option<**String**>                                                                                                             | [optional] |
| **protocol**              | Option<**String**>                                                                                                             | [optional] |
| **state**                 | Option<**String**>                                                                                                             | [optional] |
| **r#type**                | Option<**Enum {CONSUMER, PROVIDER}**>                                                                                          | [optional] |

[[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to README]](../../README.md)



