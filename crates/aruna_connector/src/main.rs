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

    let shared_state = Arc::new(Mutex::new(HashMap::new())); // Contract Definitions

    // build our application with a route
    let prototype = Router::new()
        // Contract Agreements
        // `POST /v2/contractagreements/request` goes to `query_all_agreements`
        .route("/v2/contractagreements/request", post(contract_agreement::query_all_agreements))
        // `GET /v2/contractagreements/{id}` goes to `get_agreement_by_id`
        .route("/v2/contractagreements/:id", get(contract_agreement::get_agreement_by_id))
        // `GET /v2/contractagreements/{id}/negotiation` goes to `get_negotiation_by_agreement_id`
        .route("/v2/contractagreements/:id/negotiation", get(contract_agreement::get_negotiation_by_agreement_id))

        // Contract Definitions
        // `PUT /v2/contractdefinitions` goes to `update_contract_definition`
        .route("/v2/contractdefinitions", put(contract_definition::update_contract_definition))
        // `POST /v2/contractdefinitions` goes to `create_contract_definition`
        .route("/v2/contractdefinitions", post(contract_definition::create_contract_definition))
        // `POST /v2/contractdefinitions/{id}/request` goes to `request_contract_definition`
        .route("/v2/contractdefinitions/request", post(contract_definition::request_contract_definition))
        // `GET /v2/contractdefinitions/{id}` goes to `get_contract_definition`
        .route("/v2/contractdefinitions/:id", get(contract_definition::get_contract_definition))
        // `DELETE /v2/contractdefinitions/{id}` goes to `delete_contract_definition`
        .route("/v2/contractdefinitions/:id", delete(contract_definition::delete_contract_definition))

        // Contract Negotiations
        // `POST /v2/contractnegotiations` goes to `initiate_contract_negotiation`
        .route("/v2/contractnegotiations", post(contract_negotiation::initiate_contract_negotiation))
        // `POST /v2/contractnegotiations/request` goes to `query_all_contract_negotiations`
        .route("/v2/contractnegotiations/request", post(contract_negotiation::query_all_contract_negotiations))
        // `GET /v2/contractnegotiations/{id}` goes to `get_contract_negotiation`
        .route("/v2/contractnegotiations/:id", get(contract_negotiation::get_contract_negotiation))
        // `GET /v2/contractnegotiations/{id}/agreement` goes to `get_agreement_by_negotiation_id`
        .route("/v2/contractnegotiations/:id/agreement", get(contract_negotiation::get_agreement_by_negotiation_id))
        // `GET /v2/contractnegotiations/{id}/state` goes to `get_negotiation_state`
        .route("/v2/contractnegotiations/:id/state", get(contract_negotiation::get_negotiation_state))
        // `POST /v2/contractnegotiations/{id}/terminate` goes to `terminate_contract_negotiation`
        .route("/v2/contractnegotiations/:id/terminate", post(contract_negotiation::terminate_contract_negotiation))

        // Transfer Process


        // `GET /` goes to `root`
        .route("/", get(root))

        // add shared state to the app
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, prototype.into_make_service()).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
