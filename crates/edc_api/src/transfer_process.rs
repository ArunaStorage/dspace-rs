/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferProcess {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "callbackAddresses", skip_serializing_if = "Option::is_none")]
    pub callback_addresses: Option<Vec<crate::CallbackAddress>>,
    #[serde(rename = "contractAgreementId", skip_serializing_if = "Option::is_none")]
    pub contract_agreement_id: Option<String>,
    #[serde(rename = "counterPartyAddress", skip_serializing_if = "Option::is_none")]
    pub counter_party_address: Option<String>,
    #[serde(rename = "counterPartyId", skip_serializing_if = "Option::is_none")]
    pub counter_party_id: Option<String>,
    #[serde(rename = "dataDestination", skip_serializing_if = "Option::is_none")]
    pub data_destination: Option<Box<crate::DataAddress>>,
    #[serde(rename = "errorDetail", skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    #[serde(rename = "privateProperties", skip_serializing_if = "Option::is_none")]
    pub private_properties: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
}

impl TransferProcess {

    pub fn new(context: std::collections::HashMap<String, serde_json::Value>, at_type: Option<String>, at_id: Option<String>, callback_addresses: Option<Vec<crate::CallbackAddress>>,
               contract_agreement_id: Option<String>, counter_party_address: Option<String>, counter_party_id: Option<String>, data_destination: Option<Box<crate::DataAddress>>,
               error_detail: Option<String>, private_properties: Option<::std::collections::HashMap<String, String>>, protocol: Option<String>, state: Option<String>,
               r#type: Option<RHashType>) -> TransferProcess {
        TransferProcess {
            context,
            at_type,
            at_id,
            callback_addresses,
            contract_agreement_id,
            counter_party_address,
            counter_party_id,
            data_destination,
            error_detail,
            private_properties,
            protocol,
            state,
            r#type,
        }
    }

    pub fn default() -> TransferProcess {
        TransferProcess {
            context: std::collections::HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))]),
            at_type: Some("TransferProcess".to_string()),
            at_id: None,
            callback_addresses: None,
            contract_agreement_id: None,
            counter_party_address: None,
            counter_party_id: None,
            data_destination: None,
            error_detail: None,
            private_properties: None,
            protocol: None,
            state: None,
            r#type: None,
        }
    }

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "CONSUMER")]
    Consumer,
    #[serde(rename = "PROVIDER")]
    Provider,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Consumer
    }
}