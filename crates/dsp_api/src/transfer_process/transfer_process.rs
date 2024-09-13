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
    pub state: TransferStateType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransferStateType {
    TransferState(TransferState),
    EDCTransferState(EDCTransferState),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransferState {
    #[serde(rename = "dspace:REQUESTED", alias = "https://w3id.org/dspace/v0.8/REQUESTED")]
    REQUESTED,
    #[serde(rename = "dspace:STARTED", alias = "https://w3id.org/dspace/v0.8/STARTED")]
    STARTED,
    #[serde(rename = "dspace:TERMINATED", alias = "https://w3id.org/dspace/v0.8/TERMINATED")]
    TERMINATED,
    #[serde(rename = "dspace:COMPLETED", alias = "https://w3id.org/dspace/v0.8/COMPLETED")]
    COMPLETED,
    #[serde(rename = "dspace:SUSPENDED", alias = "https://w3id.org/dspace/v0.8/SUSPENDED")]
    SUSPENDED,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EDCTransferState {
    #[serde(rename = "INITIAL")]
    Initial,
    #[serde(rename = "PROVISIONING")]
    Provisioning,
    #[serde(rename = "PROVISIONING_REQUESTED")]
    ProvisioningRequested,
    #[serde(rename = "PROVISIONED")]
    Provisioned,
    #[serde(rename = "REQUESTING")]
    Requesting,
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "SUSPENDING")]
    Suspending,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "RESUMING")]
    Resuming,
    #[serde(rename = "RESUMED")]
    Resumed,
    #[serde(rename = "COMPLETING")]
    Completing,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "TERMINATING")]
    Terminating,
    #[serde(rename = "TERMINATED")]
    Terminated,
    #[serde(rename = "DEPROVISIONING")]
    Deprovisioning,
    #[serde(rename = "DEPROVISIONING_REQUESTED")]
    DeprovisioningRequested,
    #[serde(rename = "DEPROVISIONED")]
    Deprovisioned,
}

impl TransferProcess {
    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, dsp_type: String, provider_pid: String, consumer_pid: String, state: TransferStateType) -> TransferProcess {
        TransferProcess {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            state,
        }
    }
}