/*
 * Contract Agreement Message
 * Sent by:    Provider
 * Results in: AGREED, TERMINATED
 * Response:   ACK or ERROR
 * Schema:     TTL Shape, JSON Schema
 *
 * The Contract Agreement Message is sent by a Provider when it agrees to a contract. It contains the complete Agreement.
 *
 * The message must contain a consumerPid and a providerPid.
 *
 * The message must contain an ODRL Agreement.
 *
 * An Agreement must contain a timestamp property defined as an XSD DateTime type.
 *
 * An Agreement must contain an assigner and assignee. The contents of these properties are a dataspace-specific unique identifier of the Agreement parties.
 * Note that these identifiers are not necessarily the same as the identifiers of the Participant Agents negotiating the contract (e.g., Connectors).
 *
 * An Agreement must contain a odrl:target property. None of its Rules, however, must have any odrl:target attributes to prevent inconsistencies with the ODRL inferencing rules for compact policies.
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractAgreementMessage {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:agreement")]
    pub agreement: serde_json::Value,   //TODO: Maybe better to use a struct here
    #[serde(rename = "dspace:callbackAddress")]
    pub callback_address: String,
}

impl ContractAgreementMessage {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, dsp_type: String, provider_pid: String, consumer_pid: String,
               agreement: serde_json::Value, callback_address: String) -> ContractAgreementMessage {
        ContractAgreementMessage {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            agreement,
            callback_address,
        }
    }

    pub fn default() -> ContractAgreementMessage {
        ContractAgreementMessage {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/dspace/2024/1/context.json".to_string()))]),
            dsp_type: "dspace:ContractAgreementMessage".to_string(),
            provider_pid: String::new(),
            consumer_pid: String::new(),
            agreement: serde_json::Value::Null,
            callback_address: String::new(),
        }
    }
}