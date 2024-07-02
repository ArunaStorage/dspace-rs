# TransferRequest

## Properties

| Name                      | Type                                                                                                                           | Description                                                                          | Notes                    |
|---------------------------|--------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------|--------------------------|
| **context**               | **std::collections::HashMap<String, [serde_json::Value](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)>** |                                                                                      |                          |
| **at_type**               | Option<**String**>                                                                                                             |                                                                                      | [optional]               |
| **asset_id**              | **String**                                                                                                                     |                                                                                      |                          |
| **callback_addresses**    | Option<**Vec<[edc_api::CallbackAddress](CallbackAddress.md)>**>                                                                |                                                                                      | [optional]               |
| **connector_address**     | Option<**String**>                                                                                                             | Please use counterPartyAddress instead, connectorAddress is deprecated               | \[deprecated] [optional] |
| **connector_id**          | Option<**String**>                                                                                                             | Provider connector id is stored in the contract agreement, connectorId is deprecated | \[deprecated] [optional] |
| **contract_id**           | **String**                                                                                                                     |                                                                                      |                          |
| **counter_party_address** | **String**                                                                                                                     |                                                                                      |                          |
| **data_destination**      | Box<[**edc_api::DataAddress**](DataAddress.md)>                                                                                |                                                                                      |                          |
| **private_properties**    | Option<**::std::collections::HashMap<String, String>**>                                                                        |                                                                                      | [optional]               |
| **protocol**              | **String**                                                                                                                     |                                                                                      |                          |
| **transfer_type**         | **String**                                                                                                                     |                                                                                      |                          |

[[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to README]](../../README.md)


