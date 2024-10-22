extern crate dsp_client;

use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use dsp_client::contract_negotiation::negotiation_provider_api::GetNegotiationError;
use dsp_client::Error;
use edc_api::{AssetInput, ContractDefinitionInput, ContractOfferDescription, ContractRequest, Criterion, DataAddress, DatasetRequest, Offer, PolicyDefinitionInput};
use edc_client::{configuration::Configuration, asset_api, catalog_api, contract_definition_api, contract_negotiation_api, policy_definition_api};
use edc_client::configuration::ApiKey;
use odrl::name_spaces::{EDC_NS, LD_NS};

pub const PROVIDER_PROTOCOL: &str = "http://localhost:3000";
pub const PROVIDER_ID: &str = "aruna-connector";
pub const DATASPACE_PROTOCOL: &str = "dataspace-protocol-http";

pub fn setup_provider_configuration() -> Configuration {
    Configuration::new(
        "http://localhost:3000".to_string(),
        Some("okhttp/4.12.0".to_owned()),
        reqwest::Client::new(),
        None,
        None,
        None,
        None
    ).with_headers()
}

pub fn setup_consumer_configuration() -> Configuration {
    let mut consumer = Configuration::default();
    consumer.base_path = "http://localhost:19194/protocol".to_string();
    consumer.with_headers()
}

pub async fn setup_management_consumer() -> edc_client::configuration::Configuration {
    let mut management_consumer = edc_client::configuration::Configuration::default();
    management_consumer.base_path = "http://localhost:19193/management".to_owned();
    management_consumer.api_key = Some(ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    management_consumer.with_headers()
}

pub async fn get_dataset() {

    let dataset_request = DatasetRequest {
        context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
        at_type: Some("DatasetRequest".to_string()),
        at_id: Some("Test".to_string()),
        counter_party_address: Some(PROVIDER_PROTOCOL.to_string()),
        counter_party_id: Some(PROVIDER_ID.to_string()),
        protocol: Some(DATASPACE_PROTOCOL.to_string()),
        query_spec: None,
    };

    let consumer = setup_management_consumer().await;

    println!("Consumer: {:#?}", consumer);

    let dataset = catalog_api::get_dataset(&consumer, Some(dataset_request)).await.unwrap();

}
