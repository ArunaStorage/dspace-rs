use axum::{routing::{get, post, put, delete}, Router, ServiceExt};
use serde::{Deserialize, Serialize};
use tracing::{Level};
use tracing_subscriber;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


pub mod api {
    pub mod contract_agreement;
    pub mod contract_definition;
    pub mod contract_negotiation;
}

use api::{contract_agreement, contract_definition, contract_negotiation};

#[tokio::main]
async fn main() {

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();


    // Shared states to store
    let shared_contract_definitions_state = Arc::new(Mutex::new(HashMap::new())); // Contract Definitions
    let shared_contract_negotiations_state = Arc::new(Mutex::new(HashMap::new())); // Contract Negotiations
    //let shared_contract_agreement_states = Arc::new(Mutex::new(HashMap::new())); // Contract Agreements


    // routes for contract agreements
    let contract_agreement_routes = Router::new()
        // Contract Agreements
        // `POST /v2/contractagreements/request` goes to `query_all_agreements`
        .route("/request", post(contract_agreement::query_all_agreements))
        // `GET /v2/contractagreements/{id}` goes to `get_agreement_by_id`
        .route("/:id", get(contract_agreement::get_agreement_by_id))
        // `GET /v2/contractagreements/{id}/negotiation` goes to `get_negotiation_by_agreement_id`
        .route("/:id/negotiation", get(contract_agreement::get_negotiation_by_agreement_id));
        // add shared state to the app
        //.with_state(shared_contract_agreement_states);

    // routes for contract definitions
    let contract_definitions_routes = Router::new()
        // Contract Definitions
        // `PUT /v2/contractdefinitions` goes to `update_contract_definition`
        .route("/", put(contract_definition::update_contract_definition))
        // `POST /v2/contractdefinitions` goes to `create_contract_definition`
        .route("/", post(contract_definition::create_contract_definition))
        // `POST /v2/contractdefinitions/{id}/request` goes to `request_contract_definition`
        .route("/request", post(contract_definition::request_contract_definition))
        // `GET /v2/contractdefinitions/{id}` goes to `get_contract_definition`
        .route("/:id", get(contract_definition::get_contract_definition))
        // `DELETE /v2/contractdefinitions/{id}` goes to `delete_contract_definition`
        .route("/:id", delete(contract_definition::delete_contract_definition))
        // add shared state to the app
        .with_state(shared_contract_definitions_state);

    // routes for contract negotiations
    let contract_negotiations_routes = Router::new()
        // Contract Negotiations
        // `POST /v2/contractnegotiations` goes to `initiate_contract_negotiation`
        .route("/", post(contract_negotiation::initiate_contract_negotiation))
        // `POST /v2/contractnegotiations/request` goes to `request_contract_negotiation`
        .route("/request", post(contract_negotiation::request_contract_negotiation))
        // `GET /v2/contractnegotiations/{id}` goes to `get_contract_negotiation`
        .route("/:id", get(contract_negotiation::get_contract_negotiation))
        // `GET /v2/contractnegotiations/{id}/agreement` goes to `get_agreement_by_negotiation_id`
        .route("/:id/agreement", get(contract_negotiation::get_agreement_by_negotiation_id))
        // `GET /v2/contractnegotiations/{id}/state` goes to `get_negotiation_state`
        .route("/:id/state", get(contract_negotiation::get_negotiation_state))
        // `POST /v2/contractnegotiations/{id}/terminate` goes to `terminate_contract_negotiation`
        .route("/:id/terminate", post(contract_negotiation::terminate_contract_negotiation))
        // add shared state to the app
        .with_state(shared_contract_negotiations_state);


    // create our app by nesting the routes
    let api_routes = Router::new()
        .nest("/v2/contractagreements", contract_agreement_routes)
        .nest("/v2/contractdefinitions", contract_definitions_routes)
        .nest("/v2/contractnegotiations", contract_negotiations_routes);


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, api_routes.into_make_service()).await.unwrap();

}
