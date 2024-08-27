/*
 * Catalog [ACK]
 * Sent by:    Provider
 * Schema:     TTL Shape, JSON Schema
 *
 * The Catalog contains all Datasets which the requester shall see.
 */
use crate::catalog::{DataService, Dataset};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:dataset")]
    pub datasets: Vec<Dataset>,
    #[serde(rename = "dcat:service")]
    pub service: Vec<DataService>,
    #[serde(rename = "dspace:participantId", skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    #[serde(rename = "foaf:homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
}

impl Catalog {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, dsp_type: String, datasets: Vec<Dataset>, service: Vec<DataService>,
               participant_id: Option<String>, homepage: Option<String>) -> Catalog {
        Catalog {
            context,
            dsp_type,
            datasets,
            service,
            participant_id,
            homepage,
        }
    }

    pub fn default() -> Catalog {
        Catalog {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/dspace/2024/1/context.json".to_string()))]),
            dsp_type: "dspace:Catalog".to_string(),
            datasets: Vec::new(),
            service: Vec::new(),
            participant_id: None,
            homepage: None,
        }
    }

}