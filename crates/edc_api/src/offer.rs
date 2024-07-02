/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

/// ODRL offer
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Offer {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "@id")]
    pub at_id: String,
    #[serde(rename = "assigner")]
    pub assigner: String,
    #[serde(rename = "target")]
    pub target: String,
}

impl Offer {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_type: Option<String>, at_id: String, assigner: String, target: String) -> Offer {
        Offer {
            context,
            at_type,
            at_id,
            assigner,
            target,
        }
    }

    pub fn default() -> Offer {
        Offer {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("Offer".to_string()),
            at_id: String::new(),
            assigner: String::new(),
            target: String::new(),
        }
    }

}