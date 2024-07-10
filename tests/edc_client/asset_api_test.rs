mod common;

#[cfg(test)]
mod asset_api_create_test {

    extern crate edc_api;
    extern crate edc_client;

    use crate::common::setup_provider_configuration;
    use edc_api::{AssetInput, DataAddress};
    use edc_client::{asset_api, Error};

    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert("name".to_string(), serde_json::Value::String("test".to_string()));
        properties.insert("foo".to_string(), serde_json::Value::String("bar".to_string()));
        let mut data_address = DataAddress::default();
        data_address.at_type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address);
        asset.properties = properties;

        let response = asset_api::create_asset(&configuration, Some(asset)).await.unwrap();

        assert!(response.at_id.is_some());
        assert_eq!(response.at_id.unwrap(), id.clone());
        assert!(response.created_at.is_some());
        assert!(response.created_at.unwrap() > 0);

    }

    #[tokio::test]
    async fn test_create_malformed_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert("name".to_string(), serde_json::Value::String("test".to_string()));
        properties.insert("foo".to_string(), serde_json::Value::String("bar".to_string()));
        let mut data_address = DataAddress::default();

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address);
        asset.properties = properties;
        asset.context = std::collections::HashMap::new();

        let response = asset_api::create_asset(&configuration, Some(asset)).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::BAD_REQUEST);
            },
            _ => panic!("Expected Status Code 400, because of missing mandatory @type in DataAddress"),
        }
    }

    #[tokio::test]
    async fn test_create_duplicate_id_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert("name".to_string(), serde_json::Value::String("test".to_string()));
        properties.insert("foo".to_string(), serde_json::Value::String("bar".to_string()));
        let mut data_address = DataAddress::default();
        data_address.at_type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address);
        asset.properties = properties;

        let _ = asset_api::create_asset(&configuration, Some(asset.clone())).await;

        let response = asset_api::create_asset(&configuration, Some(asset.clone())).await; // This should fail, because an asset with the same id already exists

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::CONFLICT);
            },
            _ => panic!("Expected Status Code 409, because an asset with that ID already exists"),
        }

    }

}

#[cfg(test)]
mod asset_api_get_test {

    extern crate edc_api;
    extern crate edc_client;

    use crate::common::setup_provider_configuration;
    use edc_api::{AssetInput, DataAddress};
    use edc_client::{asset_api, Error};

    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert("name".to_string(), serde_json::Value::String("test".to_string()));
        properties.insert("foo".to_string(), serde_json::Value::String("bar".to_string()));
        let mut data_address = DataAddress::default();
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address.clone());
        asset.properties = properties.clone();

        let _ = asset_api::create_asset(&configuration, Some(asset)).await.unwrap();

        let response = asset_api::get_asset(&configuration, &id.clone()).await.unwrap();

        // Check for correct properties
        assert!(response.properties.is_some());
        assert_eq!(response.properties.clone().unwrap().get("foo").unwrap(), &serde_json::Value::String("bar".to_string()));
        assert_eq!(response.properties.clone().unwrap().get("name").unwrap(), &serde_json::Value::String("test".to_string()));

        // Check for correct data address
        assert!(response.data_address.is_some());
        assert_eq!(response.data_address.unwrap(), Box::new(data_address.clone()));

    }

    #[tokio::test]
    async fn test_get_asset_with_unknown_id() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();

        let response = asset_api::get_asset(&configuration, &id.clone()).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            },
            _ => panic!("Expected Status Code 404, because no asset with that ID exists"),
        }
    }

}

#[cfg(test)]
mod asset_api_delete_test {

    extern crate edc_api;
    extern crate edc_client;

    use crate::common::setup_provider_configuration;
    use edc_api::{AssetInput, DataAddress};
    use edc_client::{asset_api, Error};

    use uuid::Uuid;

    #[tokio::test]
    async fn test_delete_asset() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert("name".to_string(), serde_json::Value::String("test".to_string()));
        properties.insert("foo".to_string(), serde_json::Value::String("bar".to_string()));
        let mut data_address = DataAddress::default();
        data_address.r#type = Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string());

        let mut asset = AssetInput::default();
        asset.at_id = Some(id.clone());
        asset.data_address = Box::new(data_address.clone());
        asset.properties = properties.clone();

        let _ = asset_api::create_asset(&configuration, Some(asset)).await.unwrap();

        let response = asset_api::remove_asset(&configuration, &id.clone()).await;

        assert!(response.is_ok());

    }

    #[tokio::test]
    async fn test_delete_asset_with_unknown_id() {
        let configuration = setup_provider_configuration();

        let id = Uuid::new_v4().to_string();

        let response = asset_api::remove_asset(&configuration, &id.clone()).await;

        assert!(response.is_err());
        match response {
            Err(Error::ResponseError(response)) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
            },
            _ => panic!("Expected Status Code 404, because no asset with that ID exists"),
        }
    }

    /* TODO: Check for Error 409, after contract negotiation api is tested
             Assets that are part of a contract negotiation cannot be deleted */

}