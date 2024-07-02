/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NegotiationState {
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl NegotiationState {

    pub fn new(state: Option<String>) -> NegotiationState {
        NegotiationState {
            state,
        }
    }

    pub fn default() -> NegotiationState {
        NegotiationState {
            state: None,
        }
    }

}