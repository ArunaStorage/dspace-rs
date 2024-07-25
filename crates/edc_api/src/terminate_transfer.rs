/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TerminateTransfer {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl TerminateTransfer {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_type: Option<String>, reason: Option<String>) -> TerminateTransfer {
        TerminateTransfer {
            context,
            at_type,
            reason,
        }
    }

    pub fn default() -> TerminateTransfer {
        TerminateTransfer {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("TerminateTransfer".to_string()),
            reason: None,
        }
    }

}