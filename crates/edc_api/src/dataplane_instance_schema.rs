/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataPlaneInstanceSchema {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "allowedDestTypes")]
    pub allowed_dest_types: Vec<String>,
    #[serde(rename = "allowedSourceTypes")]
    pub allowed_source_types: Vec<String>,
    #[serde(rename = "lastActive", skip_serializing_if = "Option::is_none")]
    pub last_active: Option<i64>,
    #[serde(rename = "turnCount", skip_serializing_if = "Option::is_none")]
    pub turn_count: Option<i32>,
    #[serde(rename = "url")]
    pub url: String,
}

impl DataPlaneInstanceSchema {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_id: Option<String>, at_type: Option<String>, allowed_dest_types: Vec<String>,
               allowed_source_types: Vec<String>, last_active: Option<i64>, turn_count: Option<i32>, url: String) -> DataPlaneInstanceSchema {
        DataPlaneInstanceSchema {
            context,
            at_id,
            at_type,
            allowed_dest_types,
            allowed_source_types,
            last_active,
            turn_count,
            url,
        }
    }

    pub fn default() -> DataPlaneInstanceSchema {
        DataPlaneInstanceSchema {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("DataPlaneInstance".to_string()),
            at_id: None,
            allowed_dest_types: Vec::new(),
            allowed_source_types: Vec::new(),
            last_active: None,
            turn_count: None,
            url: String::new(),
        }
    }

}