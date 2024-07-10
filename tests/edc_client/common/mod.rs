extern crate edc_client;
use edc_client::configuration::{ApiKey, Configuration};

pub const PROVIDER_PROTOCOL: &str = "http://provider-connector:9194/protocol";
pub const PROVIDER_ID: &str = "provider";

pub fn setup_provider_configuration() -> Configuration {
    let mut provider = Configuration::default();
    provider.base_path ="http://localhost:29193/management".to_string();
    provider.api_key = Some(ApiKey {
            prefix: Some("x-api-key".to_string()),
            key: "123456".to_owned(),
        });
    provider.with_headers()
}

pub fn setup_consumer_configuration() -> Configuration {
    let mut consumer = Configuration::default();
    consumer.base_path = "http://localhost:19193/management".to_owned();
    consumer.api_key = Some(ApiKey {
            prefix: Some("x-api-key".to_string()),
            key: "123456".to_owned(),
        });
    consumer.with_headers()
}