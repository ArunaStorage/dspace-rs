/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PolicyDefinitionOutput {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    /// ODRL policy
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
}

impl PolicyDefinitionOutput {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_id: Option<String>, at_type: Option<String>, policy: Option<serde_json::Value>) -> PolicyDefinitionOutput {
        PolicyDefinitionOutput {
            context,
            at_id,
            at_type,
            policy,
        }
    }

    pub fn default() -> PolicyDefinitionOutput {
        PolicyDefinitionOutput {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_id: None,
            at_type: Some("PolicyDefinition".to_string()),
            policy: None,
        }
    }

}