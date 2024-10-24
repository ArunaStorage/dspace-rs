use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tracing::{debug, error, info};
use dsp_api::contract_negotiation::ContractNegotiation;
use odrl::functions::state_machine::{ProviderState, ProviderStateMachine};

// SharedState:
//     Key: ID of the contract negotiation
//     Value: Tuple of ContractNegotiation and ProviderStateMachine<ProviderState>, to keep track of the state of the negotiation
type SharedState = Arc<Mutex<HashMap<String, (ContractNegotiation, ProviderStateMachine<ProviderState>)>>>;

pub async fn get_negotiation(State(state): State<SharedState>, Path(id): Path<String>) -> impl IntoResponse {
    info!("Get Negotiation called");
    debug!("Received Negotiation request for id {:#?}", id.clone());

    let state = state.lock().unwrap();

    match state.get(&id) {
        Some((negotiation, _)) => {
            (StatusCode::OK, Json(negotiation.clone())).into_response()
        },
        None => {
            let err_msg = format!("A Contract Negotiation with the given id {:#?} does not exist.", id.clone());
            error!("{}", err_msg);
            (StatusCode::NOT_FOUND, Json(err_msg)).into_response()
        }
    }
}

pub async fn request_negotiation() -> impl IntoResponse {

}

pub async fn make_offer() -> impl IntoResponse {

}

pub async fn accept_offer() -> impl IntoResponse {

}

pub async fn verify_agreement() -> impl IntoResponse {

}

pub async fn terminate_negotiation() -> impl IntoResponse {

}