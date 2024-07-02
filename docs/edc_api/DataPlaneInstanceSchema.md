# DataPlaneInstanceSchema

## Properties

| Name                     | Type                                                                                                                           | Notes      |
|--------------------------|--------------------------------------------------------------------------------------------------------------------------------|------------|
| **context**              | **std::collections::HashMap<String, [serde_json::Value](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)>** |            |
| **at_id**                | Option<**String**>                                                                                                             | [optional] |
| **at_type**              | Option<**String**>                                                                                                             | [optional] |
| **allowed_dest_types**   | **Vec\<String>**                                                                                                               |            |
| **allowed_source_types** | **Vec\<String>**                                                                                                               |            |
| **last_active**          | Option<**i64**>                                                                                                                | [optional] |
| **turn_count**           | Option<**i32**>                                                                                                                | [optional] |
| **url**                  | **String**                                                                                                                     |            |

[[Back to Model list]](../../crates/edc_api/README.md#documentation-for-models) [[Back to API list]](../../crates/edc_client/README.md#documentation-for-api-endpoints) [[Back to README]](../../README.md)


