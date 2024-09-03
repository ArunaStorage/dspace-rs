extern crate dsp_client;

use dsp_client::configuration::Configuration;

pub fn setup_provider_configuration() -> Configuration {
    let mut provider = Configuration::default();
    provider.base_path = "http://localhost:29194/protocol".to_string();
    provider.with_headers()
}

pub fn setup_consumer_configuration() -> Configuration {
    let mut consumer = Configuration::default();
    consumer.base_path = "http://localhost:19194/protocol".to_string();
    consumer.with_headers()
}
