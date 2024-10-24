mod common;

mod aruna_connector_negotiation_provider_api_test {
    use dsp_client::contract_negotiation::negotiation_provider_api;

    use crate::common::{setup_dsp_provider_configuration};

    #[tokio::test]
    async fn test_get_unknown_negotiation() {

        let config = setup_dsp_provider_configuration().await;
        let pid = "Test";

        let result = negotiation_provider_api::get_negotiation(&config, pid).await;
        assert!(result.is_err());
    }
}