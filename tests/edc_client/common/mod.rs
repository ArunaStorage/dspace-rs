extern crate edc_client;
extern crate odrl;

use edc_api::{AssetInput, ContractDefinitionInput, Criterion, DataAddress, PolicyDefinitionInput};
use edc_client::configuration::{ApiKey, Configuration};
use odrl::name_spaces as NameSpaces;

use uuid::Uuid;
use edc_client::{asset_api, contract_definition_api, policy_definition_api};

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

pub async fn setup_random_contract_definition(configuration: &Configuration) -> (String, String, String) {

    // Create asset with random id
    let asset = AssetInput {
        context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String(NameSpaces::EDC_NS.to_string()))]),
        at_id: Some(Uuid::new_v4().to_string()),
        at_type: Some("Asset".to_string()),
        data_address: Box::new(DataAddress {
            at_type: Some("HttpData".to_string()),
            r#type: Some("https://w3id.org/edc/v0.0.1/ns/DataAddress".to_string()),
            base_url: Some("https://jsonplaceholder.typicode.com/users".to_string()),
        }),
        private_properties: None,
        properties: Default::default(),
    };

    let asset_response = asset_api::create_asset(&configuration, Some(asset)).await.unwrap();

    let test_policy = r#"
    {
        "@context": "http://www.w3.org/ns/odrl.jsonld",
        "@type": "Set",
        "uid": "api_test_policy",
        "permission": [
            {
                "target": "http://example.com/asset:9898.movie",
                "action": "display",
                "constraint": [
                    {
                        "leftOperand": "spatial",
                        "operator": "eq",
                        "rightOperand": "https://www.wikidata.org/wiki/Q183",
                        "comment": "i.e Germany"
                    }
                ]
            }
        ]
    }
    "#;

    // Create policy with random id
    let policy_definition = PolicyDefinitionInput {
        context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String(NameSpaces::EDC_NS.to_string()))]),
        at_id: Some(Uuid::new_v4().to_string()),
        at_type: Some("PolicyDefinition".to_string()),
        policy: serde_json::from_str(test_policy).unwrap(),
    };

    let policy_response = policy_definition_api::create_policy_definition(&configuration, Some(policy_definition)).await.unwrap();

    // Create contract definition with random id containing previously created asset and policy
    let contract_definition = ContractDefinitionInput {
        context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String(NameSpaces::EDC_NS.to_string()))]),
        at_id: Some(Uuid::new_v4().to_string()),
        at_type: Some("ContractDefinition".to_string()),
        access_policy_id: policy_response.clone().at_id.unwrap(),
        assets_selector: vec![Criterion {
            at_type: Some("Criterion".to_string()),
            operand_left: serde_json::Value::from(format!("{}{}", NameSpaces::EDC_NS, "id")),
            operand_right: serde_json::Value::from(asset_response.clone().at_id.unwrap()),
            operator: "=".to_string(),
        }],
        contract_policy_id: policy_response.clone().at_id.unwrap(),
    };

    let definition_response = contract_definition_api::create_contract_definition(&configuration, Some(contract_definition)).await.unwrap();

    (
        asset_response.clone().at_id.unwrap(),
        policy_response.clone().at_id.unwrap(),
        definition_response.clone().at_id.unwrap(),
    )

}