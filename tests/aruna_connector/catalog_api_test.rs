mod common;

#[cfg(test)]
mod aruna_connector_catalog_api_test {
    use edc_client::catalog_api;

    use crate::common::{setup_management_consumer, DATASPACE_PROTOCOL};

    #[tokio::test]
    async fn test_get_dataset() {

        let request = edc_api::DatasetRequest {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("DatasetRequest".to_string()),
            at_id: Some("Test".to_string()),
            counter_party_address: Some("http://localhost:3000".to_string()),
            counter_party_id: Some("foobar".to_string()),
            protocol: Some(DATASPACE_PROTOCOL.to_string()),
            query_spec: None,
        };

        let config = setup_management_consumer().await;

        catalog_api::get_dataset(&config, Some(request)).await.unwrap();
    }

}