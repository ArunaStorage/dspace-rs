use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::{json, Value};
use tracing::{debug, error, info};
use uuid::Uuid;
use dsp_api::contract_negotiation::contract_negotiation::NegotiationState;
use dsp_api::contract_negotiation::ContractNegotiation;
use odrl::functions::state_machine::{ProviderState, ProviderStateMachine};

// SharedState:
//     Key: ID of the contract negotiation
//     Value: Tuple of ContractNegotiation and ProviderStateMachine<ProviderState>, to keep track of the state of the negotiation
type SharedState = Arc<Mutex<HashMap<String, (ContractNegotiation, ProviderStateMachine<ProviderState>)>>>;

pub async fn get_negotiation(State(state): State<SharedState>, Path(pid): Path<String>) -> impl IntoResponse {
    info!("Get Negotiation called");
    debug!("Received Negotiation request for pid {:#?}", pid.clone());

    let state = state.lock().unwrap();

    match state.get(&pid) {
        Some((negotiation, _)) => {
            (StatusCode::OK, Json(negotiation.clone())).into_response()
        },
        None => {
            let err_msg = format!("A Contract Negotiation with the given pid {:#?} does not exist.", pid.clone());
            error!("{}", err_msg);
            (StatusCode::NOT_FOUND, Json(err_msg)).into_response()
        }
    }
}

pub async fn request_negotiation(headers: HeaderMap, State(state): State<SharedState>, Json(request_message): Json<Value>) -> impl IntoResponse {
    info!("Request Negotiation called");
    debug!("Received Contract Negotiation request with request message: {:#?}", request_message);

    let mut state = state.lock().unwrap();

    if let Some(provider_pid) = request_message["provider_pid"].as_str() {
        // TODO: Implement response to a Offer sent by the provider (provider pid provided)
    } else {
        // TODO: Implement new CN (no provider pid provided)

        let host = headers["host"].to_str().unwrap();

        if request_message["dspace:callbackAddress"].is_null() {
            let err_msg = "The callbackAddress is required and should be a URL indicating where messages to the Consumer should be sent in asynchronous settings.".to_string();
            error!("{}", err_msg);
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(err_msg)).into_response()
        }

        let mut fsm = ProviderStateMachine::new(
            host.clone(),
            request_message["dspace:callbackAddress"].as_str().unwrap(),
        );

        debug!("Initialized new State Machine; State: {}", fsm.state);

        let pid = Uuid::new_v4().to_string();

        let negotiation = ContractNegotiation {
            context: HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/dspace/v0.8/".to_string()))]),
            dsp_type: "dspace:ContractNegotiation".to_string(),
            consumer_pid: request_message["dspace:consumerPid"].as_str().unwrap().to_string(),
            provider_pid: pid.clone(),
            state: NegotiationState::REQUESTED,
        };

        let transition_message = format!("Requesting Contract Negotiation from {:#?}", request_message["dspace:callbackAddress"].as_str().unwrap());
        fsm.receive_contract_request(transition_message);

        debug!("State Machine transitioned to: {}", fsm.state);
        println!("FSM: {:#?}", fsm.clone()); // only for demonstration purposes, remove later

        state.insert(pid.clone(), (negotiation.clone(), fsm.clone()));

        return (StatusCode::CREATED, Json(negotiation.clone())).into_response()
    }

    (StatusCode::OK, Json("".clone())).into_response()
}

pub async fn make_offer() -> impl IntoResponse {

}

pub async fn accept_offer() -> impl IntoResponse {

}

pub async fn verify_agreement() -> impl IntoResponse {

}

pub async fn terminate_negotiation() -> impl IntoResponse {

}