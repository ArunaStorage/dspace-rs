mod common;

#[cfg(test)]
mod aruna_connector_catalog_api_test {
    use crate::common;
    use crate::common::{PROVIDER_PROTOCOL, setup_consumer_configuration, setup_provider_configuration, get_dataset};

    #[tokio::test]
    async fn test_get_dataset() {

        common::get_dataset().await;

    }

}