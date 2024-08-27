/*
 * Transfer Process [ACK]
 * Sent by:    Consumer, Provider
 * Schema:     TTL Shape, JSON Schema
 *
 * The Transfer Process is an object returned by a Consumer or Provider indicating a successful state change happened.
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferProcess {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:state")]
    pub state: TransferState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransferState {
    #[serde(rename = "dspace:REQUESTED")]
    REQUESTED,
    #[serde(rename = "STARTED")]
    STARTED,
    #[serde(rename = "TERMINATED")]
    TERMINATED,
    #[serde(rename = "COMPLETED")]
    COMPLETED,
    #[serde(rename = "SUSPENDED")]
    SUSPENDED,
}

impl TransferProcess {
    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, dsp_type: String, provider_pid: String, consumer_pid: String, state: TransferState) -> TransferProcess {
        TransferProcess {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            state,
        }
    }
}