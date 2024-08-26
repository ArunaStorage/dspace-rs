/*
 * Contract Offer Message
 * Sent by:    Provider
 * Results in: AGREED, TERMINATED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Contract Offer Message is sent by a Provider to initiate a CN or to respond to a Contract Request Message sent by a Consumer.
 *
 * If the message includes a consumerPid property, the request will be associated with an existing CN.
 * If the message does not include a consumerPid, a new CN will be created on Consumer side and the Consumer selects an appropriate consumerPid.
 *
 * The Dataset id is not required but can be included when the Provider initiates a CN.
 *
 * Different to a Dataset (see DCAT Vocabulry Mapping), the Offer inside a ContractOfferMessage must have an odrl:target attribute.
 * However, it's contained Rules must not have any odrl:target attributes to prevent inconsistencies with the ODRL inferencing rules for compact policies.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractOfferMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid", skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<String>,
    #[serde(rename = "dspace:offer")]
    pub offer: serde_json::Value,   //TODO: Maybe better to use a struct here
    #[serde(rename = "dspace:callbackAddress")]
    pub callback_address: String,
}

impl ContractOfferMessage {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, dsp_type: String, provider_pid: String, consumer_pid: Option<String>,
               offer: serde_json::Value, callback_address: String) -> ContractOfferMessage {
        ContractOfferMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            offer,
            callback_address,
        }
    }

    pub fn default() -> ContractOfferMessage {
        ContractOfferMessage {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/dspace/2024/1/context.json".to_string()))]),
            dsp_type: "dspace:ContractOfferMessage".to_string(),
            provider_pid: String::new(),
            consumer_pid: None,
            offer: serde_json::Value::Null,
            callback_address: String::new(),
        }
    }
}