# ContractRequest

## Properties

| Name                      | Type                                                                                                                           | Description                                                            | Notes                    |
|---------------------------|--------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------|--------------------------|
| **context**               | **std::collections::HashMap<String, [serde_json::Value](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)>** |                                                                        |                          |
| **at_type**               | Option<**String**>                                                                                                             |                                                                        | [optional]               |
| **callback_addresses**    | Option<**Vec<[edc_api::CallbackAddress](CallbackAddress.md)>**>                                                                |                                                                        | [optional]               |
| **connector_address**     | Option<**String**>                                                                                                             | Please use counterPartyAddress instead, connectorAddress is deprecated | \[deprecated] [optional] |
| **counter_party_address** | **String**                                                                                                                     |                                                                        |                          |
| **offer**                 | Option<**[edc_api::ContractOfferDescription](ContractOfferDescription.md)**>                                                   |                                                                        | [optional]               |
| **policy**                | **[edc_api::Offer](Offer.md)**                                                                                                 |                                                                        |                          |
| **protocol**              | **String**                                                                                                                     |                                                                        |                          |
| **provider_id**           | Option<**String**>                                                                                                             | Please use policy.assigner instead, providerId is deprecated           | \[deprecated] [optional] |

[[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to README]](../../README.md)


