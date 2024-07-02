# SelectionRequestSchema

## Properties

| Name              | Type                                                                                                                                   | Notes      |
|-------------------|----------------------------------------------------------------------------------------------------------------------------------------|------------|
| **context**       | **std::collections::HashMap<**String**, [**serde_json::Value**](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)>** |            |
| **at_type**       | Option<**String**>                                                                                                                     | [optional] |
| **destination**   | Option<Box<[**edc_api::DataAddress**](DataAddress.md)>>                                                                                | [optional] |
| **source**        | Option<Box<[**edc_api::DataAddress**](DataAddress.md)>>                                                                                | [optional] |
| **strategy**      | Option<**String**>                                                                                                                     | [optional] |
| **transfer_type** | Option<**String**>                                                                                                                     | [optional] |

[[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to README]](../../README.md)


