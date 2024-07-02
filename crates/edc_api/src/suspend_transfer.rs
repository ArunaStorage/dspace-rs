/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SuspendTransfer {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")] // TODO: Should this be "reason"?
    pub state: Option<String>,
}

impl SuspendTransfer {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_type: Option<String>, state: Option<String>) -> SuspendTransfer {
        SuspendTransfer {
            context,
            at_type,
            state,
        }
    }

    pub fn default() -> SuspendTransfer {
        SuspendTransfer {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("SuspendTransfer".to_string()),
            state: None,
        }
    }

}